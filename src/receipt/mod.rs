pub mod human;
pub mod json;

use crate::collect::{
    CollectionPlan, Completeness, EvidenceWindow, Facts, Source, SourceId, Status,
};
use crate::eval::{Finding, Verdict};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub const TOOL: &str = "alibi";
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const RECEIPT_VERSION: u64 = 2;

#[derive(Debug)]
pub struct SourceRecord {
    pub name: &'static str,
    pub status: Status,
    pub completeness: Completeness,
    pub method: Option<String>,
    pub detail: Option<String>,
    pub elapsed: Duration,
    pub completed_at: Duration,
    pub window: Option<EvidenceWindow>,
}

#[derive(Debug)]
pub struct Receipt {
    pub host: String,
    pub started_at: String,
    pub decision_latency: Duration,
    pub mtti: Option<Duration>,
    pub interrogation_complete: Duration,
    pub innocence_established: bool,
    pub verdict: Verdict,
    pub findings: Vec<Finding>,
    pub boot_time: Option<u64>,
    pub sources: Vec<SourceRecord>,
}

pub fn build(
    facts: &Facts,
    findings: Vec<Finding>,
    started: SystemTime,
    elapsed: Duration,
    plan: &CollectionPlan,
) -> Receipt {
    let epoch = started
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let sources = source_records(facts, plan);
    Receipt {
        host: facts.hostname.clone(),
        started_at: crate::timeutil::rfc3339_utc(epoch),
        decision_latency: decision_latency(&findings),
        mtti: mtti(&findings),
        interrogation_complete: elapsed,
        innocence_established: innocence_established(&findings),
        verdict: overall_verdict(&findings),
        findings,
        boot_time: facts.boot_time,
        sources,
    }
}

pub fn overall_verdict(findings: &[Finding]) -> Verdict {
    let mut overall = Verdict::NotApplicable;
    for f in findings {
        if rank(f.verdict) > rank(overall) {
            overall = f.verdict;
        }
    }
    overall
}

fn rank(verdict: Verdict) -> u8 {
    match verdict {
        Verdict::Degraded => 3,
        Verdict::Unknown => 2,
        Verdict::Clear => 1,
        Verdict::NotApplicable => 0,
    }
}

pub fn innocence_established(findings: &[Finding]) -> bool {
    !findings.is_empty()
        && findings
            .iter()
            .all(|f| matches!(f.verdict, Verdict::Clear | Verdict::NotApplicable))
}

pub fn decision_latency(findings: &[Finding]) -> Duration {
    findings
        .iter()
        .map(|f| f.decided_at)
        .max()
        .unwrap_or_default()
}

pub fn mtti(findings: &[Finding]) -> Option<Duration> {
    if findings.is_empty() {
        return None;
    }
    findings.iter().try_fold(Duration::ZERO, |latest, finding| {
        let established_at = match finding.verdict {
            Verdict::Clear => finding.innocence_at?,
            Verdict::NotApplicable => finding.decided_at,
            Verdict::Degraded | Verdict::Unknown => return None,
        };
        Some(latest.max(established_at))
    })
}

pub fn exit_code(findings: &[Finding]) -> u8 {
    if findings.iter().any(|f| f.verdict == Verdict::Degraded) {
        1
    } else if findings.iter().any(|f| f.verdict == Verdict::Unknown) {
        2
    } else {
        0
    }
}

fn source_records(facts: &Facts, plan: &CollectionPlan) -> Vec<SourceRecord> {
    let mut records = Vec::new();
    if plan.contains(SourceId::KernelLog) {
        records.push(record(SourceId::KernelLog, &facts.kernel_log));
    }
    if plan.contains(SourceId::Mounts) {
        records.push(record(SourceId::Mounts, &facts.mounts));
    }
    if plan.contains(SourceId::FsUsage) {
        records.push(record(SourceId::FsUsage, &facts.fs_usage));
    }
    if plan.contains(SourceId::Edac) {
        records.push(record(SourceId::Edac, &facts.edac));
    }
    if plan.contains(SourceId::Net) {
        records.push(record(SourceId::Net, &facts.net));
    }
    if plan.contains(SourceId::Routes) {
        records.push(record(SourceId::Routes, &facts.routes));
    }
    if plan.contains(SourceId::Conntrack) {
        records.push(record(SourceId::Conntrack, &facts.conntrack));
    }
    records
}

// Presentation metadata only: collection status, required-source decisions and
// observation timestamps remain exactly as supplied by the collectors.
const KERNEL_LOG_SCOPE: &str = "completeness describes acquisition, not historical coverage; kernel-log collection requests retained current-boot records only; historical coverage, gaps and freshness are not established. Retained reports do not establish that a condition is still active. Collection completion times and sample windows are not log-history coverage.";

fn record<T>(id: SourceId, source: &Source<T>) -> SourceRecord {
    let detail = if id == SourceId::KernelLog {
        Some(match &source.detail {
            Some(detail) => format!("{}; {}", detail, KERNEL_LOG_SCOPE),
            None => KERNEL_LOG_SCOPE.to_string(),
        })
    } else {
        source.detail.clone()
    };
    SourceRecord {
        name: id.as_str(),
        status: source.status,
        completeness: source.completeness,
        method: source.method.clone(),
        detail,
        elapsed: source.elapsed,
        completed_at: source.completed_at,
        window: source.window,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collect::Status;
    use crate::eval::{fixtures, SourceRole};

    fn finding(accusation: &'static str, verdict: Verdict, ttv_ms: u64) -> Finding {
        let mut f = Finding::new(accusation);
        f.verdict = verdict;
        f.decided_at = Duration::from_millis(ttv_ms);
        f.innocence_at = (verdict == Verdict::Clear).then_some(f.decided_at);
        f
    }

    #[test]
    fn overall_verdict_precedence() {
        let findings = vec![
            finding("a", Verdict::Clear, 100),
            finding("b", Verdict::Unknown, 200),
            finding("c", Verdict::Degraded, 300),
        ];
        assert_eq!(overall_verdict(&findings), Verdict::Degraded);
        let findings = vec![
            finding("a", Verdict::Clear, 100),
            finding("b", Verdict::NotApplicable, 100),
        ];
        assert_eq!(overall_verdict(&findings), Verdict::Clear);
        let findings = vec![finding("a", Verdict::NotApplicable, 100)];
        assert_eq!(overall_verdict(&findings), Verdict::NotApplicable);
    }

    #[test]
    fn decision_latency_is_max_time_to_verdict() {
        let findings = vec![
            finding("a", Verdict::Clear, 120),
            finding("b", Verdict::Clear, 350),
            finding("c", Verdict::Unknown, 250),
        ];
        assert_eq!(decision_latency(&findings), Duration::from_millis(350));
    }

    #[test]
    fn mtti_exists_only_when_aggregate_innocence_is_established() {
        let innocent = vec![
            finding("a", Verdict::Clear, 120),
            finding("b", Verdict::NotApplicable, 350),
        ];
        assert_eq!(mtti(&innocent), Some(Duration::from_millis(350)));

        let unresolved = vec![
            finding("a", Verdict::Clear, 120),
            finding("b", Verdict::Unknown, 250),
        ];
        assert_eq!(mtti(&unresolved), None);

        let degraded = vec![finding("a", Verdict::Degraded, 90)];
        assert_eq!(mtti(&degraded), None);
    }

    #[test]
    fn mtti_requires_clear_finding_innocence_timestamp() {
        let mut clear = finding("a", Verdict::Clear, 120);
        clear.innocence_at = None;
        assert_eq!(mtti(&[clear]), None);
    }

    #[test]
    fn innocence_established_only_when_all_clear_or_not_applicable() {
        let findings = vec![
            finding("a", Verdict::Clear, 100),
            finding("b", Verdict::NotApplicable, 100),
        ];
        assert!(innocence_established(&findings));
        let findings = vec![
            finding("a", Verdict::Clear, 100),
            finding("b", Verdict::Unknown, 100),
        ];
        assert!(!innocence_established(&findings));
        let findings = vec![finding("a", Verdict::Degraded, 100)];
        assert!(!innocence_established(&findings));
    }

    #[test]
    fn envelope_membership_comes_from_actual_collection_plan() {
        let facts = fixtures::clean_facts();
        let mut f = finding("a", Verdict::Clear, 100);
        f.source_roles = vec![SourceRole::required(SourceId::Net)];

        let plan = CollectionPlan::new(vec![SourceId::Net, SourceId::Routes]);
        let records = source_records(&facts, &plan);
        let names: Vec<_> = records.iter().map(|record| record.name).collect();
        assert_eq!(names, vec!["net", "routes"]);
    }

    #[test]
    fn temporal_scope_survives_both_renderers_and_source_failure_details() {
        use crate::collect::KernelLog;
        use crate::eval::kernel_scream::KernelScream;
        use crate::eval::Evaluator;
        for (status, completeness, expected) in [
            (Status::Ok, Completeness::Complete, Verdict::Clear),
            (Status::Malformed, Completeness::Partial, Verdict::Unknown),
            (Status::Unavailable, Completeness::None, Verdict::Unknown),
            (Status::Unsupported, Completeness::None, Verdict::Unknown),
        ] {
            let mut facts = fixtures::clean_facts();
            facts.kernel_log.status = status;
            facts.kernel_log.completeness = completeness;
            facts.kernel_log.detail = Some("original acquisition detail".into());
            facts.kernel_log.value = (completeness != Completeness::None).then(|| KernelLog {
                lines: vec!["<6>[ 900.000000] retained quiet record".into()],
                method: "fixture".into(),
            });
            let finding = KernelScream.evaluate(&facts);
            assert_eq!(finding.verdict, expected);
            let plan = CollectionPlan::new(vec![SourceId::KernelLog]);
            let r = build(&facts, vec![finding], UNIX_EPOCH, Duration::ZERO, &plan);
            assert_eq!(r.verdict, expected);
            assert_eq!(r.sources[0].status, status);
            assert_eq!(r.sources[0].completeness, completeness);
            assert_eq!(r.sources[0].completed_at, facts.kernel_log.completed_at);
            assert!(r.sources[0].window.is_none());
            for output in [human::render(&r, false), json::render(&r)] {
                for text in [
                    "original acquisition detail",
                    "completeness describes acquisition, not historical coverage",
                    "retained current-boot records only",
                    "historical coverage, gaps and freshness are not established",
                    "not log-history coverage",
                ] {
                    assert!(output.contains(text), "missing {text}: {output}");
                }
            }
            assert!(human::render(&r, false).contains("acquisition_completeness="));
            assert_eq!(
                facts.kernel_log.detail.as_deref(),
                Some("original acquisition detail")
            );
        }
    }

    #[test]
    fn retained_reports_stay_degraded_and_absent_edac_stays_unknown() {
        use crate::eval::{
            kernel_scream::KernelScream, memrot::Memrot, network_choke_point::NetworkChokePoint,
            silent_killer::SilentKiller, Evaluator,
        };
        let mut facts = fixtures::clean_facts();
        facts.kernel_log.value.as_mut().unwrap().lines = (1..=5)
            .map(|t| format!("<6>[ {t}.000000] eno1: Link is Down"))
            .chain(["<3>[ 6.000000] EXT4-fs error: historical record".into()])
            .collect();
        for evaluator in [&NetworkChokePoint as &dyn Evaluator, &SilentKiller] {
            let finding = evaluator.evaluate(&facts);
            assert_eq!(finding.verdict, Verdict::Degraded);
            let plan =
                CollectionPlan::new(evaluator.source_roles().iter().map(|r| r.source).collect());
            let r = build(&facts, vec![finding], UNIX_EPOCH, Duration::ZERO, &plan);
            for output in [human::render(&r, false), json::render(&r)] {
                assert!(output.contains("retained kernel"));
                assert!(output.contains("do not establish that a condition is still active"));
                if evaluator.accusation() == "network-choke-point" {
                    assert!(output.contains("not a current flap rate"));
                }
            }
        }
        facts.kernel_log.value.as_mut().unwrap().lines = vec!["<6>[ 1.000000] quiet record".into()];
        facts.edac = Source::failed(Status::Unsupported, "no EDAC in fixture", Duration::ZERO);
        let memory = Memrot.evaluate(&facts);
        let kernel = KernelScream.evaluate(&facts);
        assert_eq!(memory.verdict, Verdict::Unknown);
        assert_eq!(kernel.verdict, Verdict::Clear);
        assert_eq!(overall_verdict(&[kernel, memory]), Verdict::Unknown);
    }
}
