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
        section(&mut out, "skipped", &finding.skipped);
        if verbose {
            section(&mut out, "indications", &finding.indications);
            section(&mut out, "evidence", &finding.evidence);
        }
    }
    let innocence = if receipt.innocence_established {
        "innocence established"
    } else {
        "innocence not established"
    };
    out.push_str(&format!(
        "\nverdict: {} ({})\n",
        receipt.verdict.as_str(),
        innocence
    ));
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
            "  {} status={} completeness={} method={:?} elapsed_ms={} completed_at_ms={}\n",
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
}
