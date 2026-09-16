use crate::receipt::Receipt;

pub fn render(receipt: &Receipt, verbose: bool) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "ALIBI host={} elapsed={:.2}s mtti={:.2}s\n\n",
        receipt.host,
        receipt.elapsed.as_secs_f64(),
        receipt.mtti.as_secs_f64()
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
