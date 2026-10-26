use crate::collect::{Completeness, Status};
use crate::eval::{EvidenceRole, Finding, Verdict};
use crate::receipt::Receipt;

pub fn render(receipt: &Receipt, verbose: bool) -> String {
    let mut out = String::new();
    let mtti = receipt
        .mtti
        .map(|duration| format!("{:.2}s", duration.as_secs_f64()))
        .unwrap_or_else(|| "n/a".to_string());
    out.push_str(&format!(
        "ALIBI host={} decision={:.2}s mtti={} complete={:.2}s\n\n",
        receipt.host,
        receipt.decision_latency.as_secs_f64(),
        mtti,
        receipt.interrogation_complete.as_secs_f64()
    ));
    for finding in &receipt.findings {
        out.push_str(&format!(
            "{:<10}{:<20} {}\n",
            finding.verdict.as_str(),
            finding.accusation,
            finding.summary
        ));
        out.push_str(&format!(
            "          decided_at_ms={} innocence_at_ms={} sources:",
            finding.decided_at.as_millis(),
            finding
                .innocence_at
                .map(|d| d.as_millis().to_string())
                .unwrap_or_else(|| "n/a".to_string())
        ));
        for role in &finding.source_roles {
            out.push_str(&format!(
                " {}({})",
                role.source.as_str(),
                role.role.as_str()
            ));
        }
        out.push('\n');
        if finding.verdict == Verdict::Clear {
            out.push_str(&format!(
                "          boundary: {}\n",
                crate::explain::clear_boundary(finding.accusation)
            ));
        }
        if matches!(finding.verdict, Verdict::Unknown | Verdict::Degraded) {
            uncertainty(&mut out, receipt, finding);
        }
        section(&mut out, "skipped", &finding.skipped);
        if verbose || matches!(finding.verdict, Verdict::Unknown | Verdict::Degraded) {
            section(&mut out, "indications", &finding.indications);
        }
        if verbose || finding.verdict == Verdict::Unknown {
            section(&mut out, "evidence", &finding.evidence);
        }
    }
    let innocence = if receipt.innocence_established {
        "innocence established for selected accusations only"
    } else {
        "innocence not established"
    };
    out.push_str(&format!(
        "\nverdict: {} ({})\n",
        receipt.verdict.as_str(),
        innocence
    ));
    let selected = |name| {
        receipt
            .findings
            .iter()
            .any(|f| f.source_roles.iter().any(|r| r.source.as_str() == name))
    };
    let mut scope = Vec::new();
    if selected("mounts") {
        scope.push("mount state is a snapshot");
    }
    if selected("edac") {
        scope.push("EDAC counters are cumulative with unknown event ages");
    }
    if selected("net") {
        scope.push("network deltas cover only the recorded sample window");
    }
    if selected("kernel-log") {
        scope.push("retained log reports do not establish a currently active condition; event ages and historical continuity are not bounded");
    }
    if !scope.is_empty() {
        out.push_str(&format!("\nobservation scope: {}.\n", scope.join("; ")));
    }
    out.push_str(&format!(
        "\nenvelope: started_at={} boot_time={}\n",
        receipt.started_at,
        receipt
            .boot_time
            .map(crate::timeutil::rfc3339_utc)
            .unwrap_or_else(|| "unavailable".to_string())
    ));
    for source in &receipt.sources {
        out.push_str(&format!(
            "  {} status={} acquisition_completeness={} method={:?} elapsed_ms={} completed_at_ms={}\n",
            source.name,
            source.status.as_str(),
            source.completeness.as_str(),
            source.method.as_deref().unwrap_or("unavailable"),
            source.elapsed.as_millis(),
            source.completed_at.as_millis()
        ));
        if let Some(window) = source.window {
            out.push_str(&format!(
                "    window_started_at_ms={} window_ended_at_ms={} duration_ms={}\n",
                window.started_at.as_millis(),
                window.ended_at.as_millis(),
                window.duration().as_millis()
            ));
        }
        if let Some(detail) = &source.detail {
            out.push_str(&format!("    detail={:?}\n", detail));
        }
    }
    out
}

// Presentation uses the canonical source status and completeness, never guesses
// a cause by matching error text. Details retain the collector/evaluator's reason.
fn uncertainty(out: &mut String, receipt: &Receipt, finding: &Finding) {
    let mut explained = false;
    for role in &finding.source_roles {
        if role.role != EvidenceRole::Required {
            continue;
        }
        let source = receipt
            .sources
            .iter()
            .find(|s| s.name == role.source.as_str());
        let (kind, next) = match source {
            None => ("unavailable evidence", "required source has no receipt record; this receipt cannot establish exoneration"),
            Some(s) => match s.status {
                Status::Unavailable => ("unavailable evidence", "inspect the source failure below; rerunning only helps if access or availability changes"),
                Status::Unsupported => ("unsupported evidence source", "this environment/source is outside supported observation; repeating unchanged does not establish it"),
                Status::Malformed => ("invalid or inconsistent evidence", "inspect the recorded detail; a later stable observation may help but cannot reconstruct the missing interval"),
                Status::Ok if s.completeness != Completeness::Complete => ("insufficient evidence", "the source is incomplete; a complete applicable observation is needed"),
                Status::Ok => continue,
            },
        };
        explained = true;
        if let Some(source) = source {
            let guidance = match (source.name, source.status) {
                ("edac", Status::Unsupported) => Some("no EDAC controllers are exposed; check platform/driver support and consult BMC or hypervisor hardware evidence. Additional privilege cannot create absent EDAC evidence"),
                ("kernel-log", Status::Unavailable) => Some("inspect dmesg and journal failure details. If access was denied, obtain authorized kernel-log access; otherwise check command availability and retained current-boot logs"),
                _ => None,
            };
            if let Some(guidance) = guidance {
                out.push_str(&format!("          next investigation: {}\n", guidance));
            }
        }
        out.push_str(&format!(
            "          uncertainty: {}: {} — {}\n",
            role.source.as_str(),
            kind,
            next
        ));
    }
    if !explained && finding.verdict == Verdict::Unknown {
        out.push_str("          uncertainty: insufficient evidence for this accusation; inspect the summary and evidence below. Source availability alone does not establish exoneration.\n");
    }
}

fn section(out: &mut String, name: &str, items: &[String]) {
    if items.is_empty() {
        return;
    }
    out.push_str(&format!("          {}:\n", name));
    for item in items {
        out.push_str(&format!("            - {}\n", item));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::eval::Verdict;
    use std::time::Duration;

    fn receipt(mtti: Option<Duration>) -> Receipt {
        Receipt {
            host: "fixture-host".to_string(),
            started_at: "2026-09-15T00:00:00Z".to_string(),
            decision_latency: Duration::from_millis(120),
            mtti,
            interrogation_complete: Duration::from_millis(125),
            innocence_established: mtti.is_some(),
            verdict: if mtti.is_some() {
                Verdict::Clear
            } else {
                Verdict::Unknown
            },
            findings: vec![],
            boot_time: None,
            sources: vec![],
        }
    }

    #[test]
    fn source_specific_next_observations_and_temporal_scope_are_visible() {
        use crate::collect::SourceId;
        use crate::eval::SourceRole;
        use crate::receipt::SourceRecord;
        for (id, status, expected) in [
            (
                SourceId::Edac,
                Status::Unsupported,
                "Additional privilege cannot create absent EDAC evidence",
            ),
            (
                SourceId::KernelLog,
                Status::Unavailable,
                "If access was denied, obtain authorized kernel-log access",
            ),
        ] {
            let mut r = receipt(None);
            let mut f = Finding::new("memrot");
            f.verdict = Verdict::Unknown;
            f.source_roles = vec![SourceRole::required(id)];
            r.findings.push(f);
            r.sources.push(SourceRecord {
                name: id.as_str(),
                status,
                completeness: Completeness::None,
                method: None,
                detail: None,
                elapsed: Duration::ZERO,
                completed_at: Duration::ZERO,
                window: None,
            });
            let json_before = crate::receipt::json::render(&r);
            let text = render(&r, false);
            assert!(text.contains(expected));
            assert_eq!(text.contains("unknown event ages"), id == SourceId::Edac);
            assert_eq!(
                text.contains("historical continuity are not bounded"),
                id == SourceId::KernelLog
            );
            assert!(!text.contains("mount state is a snapshot"));
            assert!(!text.contains("network deltas"));
            assert_eq!(json_before, crate::receipt::json::render(&r));
            r.findings[0].verdict = Verdict::Degraded;
            assert!(render(&r, false).contains(expected));
        }
    }

    #[test]
    fn default_human_preserves_failed_required_source_envelope() {
        use crate::collect::{Completeness, SourceId, Status};
        use crate::eval::{Finding, SourceRole};
        use crate::receipt::SourceRecord;
        let mut receipt = receipt(None);
        let mut finding = Finding::new("network-choke-point");
        finding.verdict = Verdict::Degraded;
        finding.summary = "no default route".to_string();
        finding.source_roles = vec![SourceRole::required(SourceId::KernelLog)];
        receipt.findings.push(finding);
        receipt.sources.push(SourceRecord {
            name: "kernel-log",
            status: Status::Unavailable,
            completeness: Completeness::None,
            method: Some("dmesg; journalctl".to_string()),
            detail: Some("permission denied".to_string()),
            elapsed: Duration::from_millis(7),
            completed_at: Duration::from_millis(8),
            window: None,
        });
        let rendered = render(&receipt, false);
        for expected in [
            "kernel-log",
            "unavailable",
            "none",
            "dmesg; journalctl",
            "permission denied",
            "required",
            "elapsed_ms=7",
            "completed_at_ms=8",
        ] {
            assert!(
                rendered.contains(expected),
                "missing {expected}: {rendered}"
            );
        }
    }

    #[test]
    fn default_degraded_output_preserves_all_canonical_indications() {
        let mut r = receipt(None);
        r.verdict = Verdict::Degraded;
        let mut f = Finding::new("network-choke-point");
        f.verdict = Verdict::Degraded;
        f.summary = "sampled drops (+1 more)".into();
        f.indications = vec!["sampled drops".into(), "no usable default route".into()];
        f.evidence = vec!["expanded counter observations".into()];
        r.findings.push(f);
        let json_before = crate::receipt::json::render(&r);
        for verbose in [false, true] {
            let text = render(&r, verbose);
            for indication in &r.findings[0].indications {
                assert!(text.contains(&format!("- {indication}\n")));
                assert!(json_before.contains(indication));
            }
            assert_eq!(text.contains("expanded counter observations"), verbose);
            assert_eq!(text.matches("indications:").count(), 1);
        }
        assert_eq!(json_before, crate::receipt::json::render(&r));
    }

    #[test]
    fn unresolved_receipt_does_not_report_an_mtti_value() {
        let rendered = render(&receipt(None), false);
        assert!(rendered.contains("decision=0.12s mtti=n/a complete=0.12s"));
        assert!(rendered.contains("innocence not established"));
    }

    #[test]
    fn established_receipt_reports_mtti() {
        let rendered = render(&receipt(Some(Duration::from_millis(120))), false);
        assert!(rendered.contains("decision=0.12s mtti=0.12s complete=0.12s"));
        assert!(rendered.contains("innocence established"));
    }
    #[test]
    fn uncertainty_categories_preserve_canonical_receipt() {
        use crate::collect::SourceId;
        use crate::eval::SourceRole;
        use crate::receipt::SourceRecord;
        for (status, completeness, label) in [
            (
                Status::Unavailable,
                Completeness::None,
                "unavailable evidence",
            ),
            (
                Status::Unsupported,
                Completeness::None,
                "unsupported evidence source",
            ),
            (
                Status::Malformed,
                Completeness::Partial,
                "invalid or inconsistent evidence",
            ),
            (Status::Ok, Completeness::Partial, "insufficient evidence"),
        ] {
            let mut r = receipt(None);
            let mut f = Finding::new("network-choke-point");
            f.verdict = Verdict::Unknown;
            f.summary = "bounded observation unavailable".into();
            f.source_roles = vec![SourceRole::required(SourceId::Net)];
            f.evidence.push("preserved observation detail".into());
            r.findings.push(f);
            r.sources.push(SourceRecord {
                name: "net",
                status,
                completeness,
                method: Some("fixture observation".into()),
                detail: Some("source-specific reason".into()),
                elapsed: Duration::ZERO,
                completed_at: Duration::ZERO,
                window: None,
            });
            let json_before = crate::receipt::json::render(&r);
            let text = render(&r, false);
            assert!(text.contains(label));
            assert!(text.contains("source-specific reason"));
            assert!(text.contains("preserved observation detail"));
            assert!(text.contains("UNKNOWN"));
            assert!(!text.contains("sudo"));
            assert_eq!(json_before, crate::receipt::json::render(&r));
        }
    }

    #[test]
    fn missing_record_and_evaluator_uncertainty_are_not_hidden() {
        use crate::collect::SourceId;
        use crate::eval::SourceRole;
        let mut r = receipt(None);
        let mut f = Finding::new("network-choke-point");
        f.verdict = Verdict::Unknown;
        f.source_roles = vec![SourceRole::required(SourceId::Net)];
        r.findings.push(f);
        assert!(render(&r, false).contains("no receipt record"));
        r.findings[0].source_roles.clear();
        assert!(
            render(&r, false).contains("Source availability alone does not establish exoneration")
        );
    }

    #[test]
    fn clear_keeps_evidence_timing_and_scope_visible() {
        let mut r = receipt(Some(Duration::ZERO));
        let mut f = Finding::new("kernel-scream");
        f.summary = "no matched events".into();
        r.findings.push(f);
        let text = render(&r, false);
        assert!(text.contains("unlogged kernel faults"));
        assert!(text.contains("selected accusations only"));
        assert!(text.contains("decided_at_ms="));
        assert!(!text.contains("uncertainty:"));
    }
}
