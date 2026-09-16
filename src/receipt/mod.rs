pub mod human;
pub mod json;

use crate::collect::{Facts, Source, Status};
use crate::eval::{Finding, Verdict};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub const TOOL: &str = "alibi";
pub const VERSION: &str = "0.1.0";
pub const RECEIPT_VERSION: u64 = 1;

#[derive(Debug)]
pub struct SourceRecord {
    pub name: &'static str,
    pub status: Status,
    pub method: Option<String>,
    pub detail: Option<String>,
    pub elapsed: Duration,
}

#[derive(Debug)]
pub struct Receipt {
    pub host: String,
    pub started_at: String,
    pub elapsed: Duration,
    pub mtti: Duration,
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
) -> Receipt {
    let epoch = started
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    Receipt {
        host: facts.hostname.clone(),
        started_at: crate::timeutil::rfc3339_utc(epoch),
        elapsed,
        mtti: mtti(&findings),
        innocence_established: innocence_established(&findings),
        verdict: overall_verdict(&findings),
        findings,
        boot_time: facts.boot_time,
        sources: source_records(facts),
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
    findings
        .iter()
        .all(|f| matches!(f.verdict, Verdict::Clear | Verdict::NotApplicable))
}

pub fn mtti(findings: &[Finding]) -> Duration {
    findings
        .iter()
        .map(|f| f.time_to_verdict)
        .max()
        .unwrap_or_default()
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

fn source_records(facts: &Facts) -> Vec<SourceRecord> {
    vec![
        record("kernel-log", &facts.kernel_log),
        record("mounts", &facts.mounts),
        record("fs-usage", &facts.fs_usage),
        record("edac", &facts.edac),
        record("net", &facts.net),
        record("routes", &facts.routes),
        record("conntrack", &facts.conntrack),
    ]
}

fn record<T>(name: &'static str, source: &Source<T>) -> SourceRecord {
    SourceRecord {
        name,
        status: source.status,
        method: source.method.clone(),
        detail: source.detail.clone(),
        elapsed: source.elapsed,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn finding(accusation: &'static str, verdict: Verdict, ttv_ms: u64) -> Finding {
        let mut f = Finding::new(accusation);
        f.verdict = verdict;
        f.time_to_verdict = Duration::from_millis(ttv_ms);
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
    fn mtti_is_max_time_to_verdict() {
        let findings = vec![
            finding("a", Verdict::Clear, 120),
            finding("b", Verdict::Clear, 350),
            finding("c", Verdict::Unknown, 250),
        ];
        assert_eq!(mtti(&findings), Duration::from_millis(350));
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
}
