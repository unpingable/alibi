use crate::collect::Facts;
use crate::eval::{base_verdict, patterns, Evaluator, Finding, Verdict};

pub struct KernelScream;

impl Evaluator for KernelScream {
    fn accusation(&self) -> &'static str {
        "kernel-scream"
    }

    fn evaluate(&self, facts: &Facts) -> Finding {
        let mut finding = Finding::new(self.accusation());
        let mut required_failures = Vec::new();
        let mut total = 0usize;
        let mut first_label = "";
        match facts.kernel_log.get() {
            Some(log) => {
                let (matches, n) = patterns::matches_for(&log.lines, self.accusation());
                total = n;
                for (line, label) in matches {
                    if first_label.is_empty() {
                        first_label = label;
                    }
                    finding.indications.push(format!("{}: {}", label, line));
                }
                if finding.indications.is_empty() {
                    finding.evidence.push(format!(
                        "kernel log acquired via {} ({} lines scanned)",
                        log.method,
                        log.lines.len()
                    ));
                }
            }
            None => {
                let msg = format!(
                    "kernel log unavailable: {}",
                    facts.kernel_log.failure_reason()
                );
                required_failures.push(msg.clone());
                finding.skipped.push(msg);
            }
        }
        finding.verdict = base_verdict(&finding.indications, &required_failures);
        finding.summary = match finding.verdict {
            Verdict::Clear => "no serious kernel/hardware events found".to_string(),
            Verdict::Degraded => format!(
                "{} serious kernel/hardware log events (e.g. {})",
                total, first_label
            ),
            Verdict::Unknown => format!(
                "kernel log unavailable: {}",
                facts.kernel_log.failure_reason()
            ),
            Verdict::NotApplicable => unreachable!(),
        };
        finding
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collect::{KernelLog, Source, Status};
    use crate::eval::fixtures;
    use std::time::Duration;

    fn facts_with_log(lines: Vec<String>) -> Facts {
        let mut facts = fixtures::clean_facts();
        facts.kernel_log = fixtures::src_ok(KernelLog {
            lines,
            method: "dmesg".to_string(),
        });
        facts
    }

    #[test]
    fn kernel_scream_unknown_when_kernel_log_unavailable() {
        let mut facts = fixtures::clean_facts();
        facts.kernel_log = Source::failed(
            Status::Unavailable,
            "dmesg: operation not permitted; journalctl: no entries",
            Duration::from_millis(1),
        );
        let finding = KernelScream.evaluate(&facts);
        assert_eq!(finding.verdict, Verdict::Unknown);
        assert!(finding.summary.contains("kernel log unavailable"));
        assert!(!finding.skipped.is_empty());
    }

    #[test]
    fn clear_on_clean_fixture() {
        let finding = KernelScream.evaluate(&fixtures::clean_facts());
        assert_eq!(finding.verdict, Verdict::Clear);
        assert_eq!(finding.summary, "no serious kernel/hardware events found");
        assert!(!finding.evidence.is_empty());
    }

    #[test]
    fn degraded_on_oops_line() {
        let facts = facts_with_log(vec!["Oops: 0000 [#1] SMP".to_string()]);
        let finding = KernelScream.evaluate(&facts);
        assert_eq!(finding.verdict, Verdict::Degraded);
        assert_eq!(finding.indications.len(), 1);
        assert!(finding.summary.contains("kernel oops"));
    }

    #[test]
    fn degraded_on_aer_line() {
        let facts = facts_with_log(vec![
            "pcieport 0000:00:1c.0: AER: PCIe Bus Error: severity=Uncorrected".to_string(),
        ]);
        let finding = KernelScream.evaluate(&facts);
        assert_eq!(finding.verdict, Verdict::Degraded);
    }

    #[test]
    fn degraded_on_hardware_error_line() {
        let facts = facts_with_log(vec![
            "mce: [Hardware Error]: CPU 0: Machine Check Exception".to_string(),
        ]);
        let finding = KernelScream.evaluate(&facts);
        assert_eq!(finding.verdict, Verdict::Degraded);
    }
}
