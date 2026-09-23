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
    let mut skipped_total = 0;
    for finding in &receipt.findings {
        out.push_str(&format!(
            "{:<10}{:<20} {}\n",
            finding.verdict.as_str(),
            finding.accusation,
            finding.summary
        ));
        skipped_total += finding.skipped.len();
        if verbose {
            section(&mut out, "indications", &finding.indications);
            section(&mut out, "evidence", &finding.evidence);
            section(&mut out, "skipped", &finding.skipped);
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
    if !verbose && skipped_total > 0 {
        out.push_str(&format!(
            "note: {} check(s) skipped or unavailable; --verbose for detail\n",
            skipped_total
        ));
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
