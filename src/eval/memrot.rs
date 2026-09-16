use crate::collect::Facts;
use crate::eval::{base_verdict, first_plus_more, patterns, Evaluator, Finding, Verdict};

pub struct Memrot;

impl Evaluator for Memrot {
    fn accusation(&self) -> &'static str {
        "memrot"
    }

    fn evaluate(&self, facts: &Facts) -> Finding {
        let mut finding = Finding::new(self.accusation());
        let mut required_failures: Vec<String> = Vec::new();

        match facts.edac.get() {
            Some(edac) => {
                let ue: u64 = edac.controllers.iter().map(|c| c.1).sum();
                let ce: u64 = edac.controllers.iter().map(|c| c.2).sum();
                if ue > 0 {
                    finding.indications.push(format!(
                        "uncorrected memory errors reported by EDAC (ue={})",
                        ue
                    ));
                }
                if ce > 0 {
                    finding.indications.push(format!(
                        "corrected memory errors reported by EDAC (ce={}; trend matters)",
                        ce
                    ));
                }
                if ue == 0 && ce == 0 {
                    finding.evidence.push(format!(
                        "EDAC counters zero across {} memory controller(s)",
                        edac.controllers.len()
                    ));
                }
            }
            None => {
                let msg = format!("EDAC unavailable: {}", facts.edac.failure_reason());
                required_failures.push(msg.clone());
                finding.skipped.push(msg);
            }
        }

        let log_ok = facts.kernel_log.is_ok();
        match facts.kernel_log.get() {
            Some(log) => {
                let (matches, _) = patterns::matches_for(&log.lines, self.accusation());
                for (line, label) in matches {
                    finding.indications.push(format!("{}: {}", label, line));
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
            Verdict::Clear => "EDAC counters zero; no kernel-reported memory errors".to_string(),
            Verdict::Degraded => first_plus_more(&finding.indications),
            Verdict::Unknown => {
                let mut parts = required_failures;
                if log_ok {
                    parts.push(
                        "no kernel-reported memory errors — hardware memory integrity not established"
                            .to_string(),
                    );
                }
                parts.join("; ")
            }
            Verdict::NotApplicable => unreachable!(),
        };
        finding
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collect::{Edac, KernelLog, Source, Status};
    use crate::eval::fixtures::{self, src_ok};
    use std::time::Duration;

    #[test]
    fn clear_on_clean_fixture() {
        let finding = Memrot.evaluate(&fixtures::clean_facts());
        assert_eq!(finding.verdict, Verdict::Clear);
        assert_eq!(
            finding.summary,
            "EDAC counters zero; no kernel-reported memory errors"
        );
    }

    #[test]
    fn degraded_on_uncorrected_errors() {
        let mut facts = fixtures::clean_facts();
        facts.edac = src_ok(Edac {
            controllers: vec![("mc0".to_string(), 2, 0)],
        });
        let finding = Memrot.evaluate(&facts);
        assert_eq!(finding.verdict, Verdict::Degraded);
        assert!(finding.indications[0].contains("ue=2"));
    }

    #[test]
    fn degraded_on_corrected_errors() {
        let mut facts = fixtures::clean_facts();
        facts.edac = src_ok(Edac {
            controllers: vec![("mc0".to_string(), 0, 7)],
        });
        let finding = Memrot.evaluate(&facts);
        assert_eq!(finding.verdict, Verdict::Degraded);
        assert!(finding.indications[0].contains("ce=7"));
    }

    #[test]
    fn unknown_when_edac_unsupported_and_log_clean() {
        let mut facts = fixtures::clean_facts();
        facts.edac = Source::failed(
            Status::Unsupported,
            "platform exposes no EDAC (VM, or driver not loaded)",
            Duration::from_millis(1),
        );
        let finding = Memrot.evaluate(&facts);
        assert_eq!(finding.verdict, Verdict::Unknown);
        assert_ne!(finding.verdict, Verdict::Clear);
        assert!(finding
            .summary
            .contains("hardware memory integrity not established"));
    }

    #[test]
    fn degraded_on_edac_log_line() {
        let mut facts = fixtures::clean_facts();
        facts.kernel_log = src_ok(KernelLog {
            lines: vec!["EDAC MC0: 1 CE memory read error on DIMM_A1".to_string()],
            method: "dmesg".to_string(),
        });
        let finding = Memrot.evaluate(&facts);
        assert_eq!(finding.verdict, Verdict::Degraded);
    }

    #[test]
    fn edac_banner_log_lines_produce_no_indication() {
        let mut facts = fixtures::clean_facts();
        facts.kernel_log = src_ok(KernelLog {
            lines: vec![
                "EDAC MC: Ver: 3.0.0".to_string(),
                "EDAC amd64: F17h_M10h detected (node 0)".to_string(),
            ],
            method: "dmesg".to_string(),
        });
        let finding = Memrot.evaluate(&facts);
        assert_eq!(finding.verdict, Verdict::Clear);
        assert!(finding.indications.is_empty());
    }
}
