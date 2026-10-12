use crate::collect::{Facts, SourceId};
use crate::eval::{
    base_verdict, first_plus_more, patterns, Evaluator, Finding, SourceRole, Verdict,
};

const SOURCES: &[SourceRole] = &[
    SourceRole::required(SourceId::Edac),
    SourceRole::required(SourceId::KernelLog),
];

pub struct Memrot;

impl Evaluator for Memrot {
    fn accusation(&self) -> &'static str {
        "memrot"
    }

    fn source_roles(&self) -> &'static [SourceRole] {
        SOURCES
    }

    fn evaluate_evidence(&self, facts: &Facts) -> Finding {
        let mut finding = Finding::new(self.accusation());
        let mut required_failures: Vec<String> = Vec::new();

        match facts.edac.get() {
            Some(edac) => {
                let ue: u64 = edac.controllers.iter().filter_map(|c| c.ue).sum();
                let ce: u64 = edac.controllers.iter().filter_map(|c| c.ce).sum();
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
                if facts.edac.is_complete()
                    && edac
                        .controllers
                        .iter()
                        .all(|controller| controller.ue.is_some() && controller.ce.is_some())
                    && ue == 0
                    && ce == 0
                {
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
                for line in &log.lines {
                    if patterns::edac_message(line) == patterns::EdacMessage::Unrecognized {
                        let detail = format!("unrecognized EDAC log report; memory-error interpretation unavailable: {}", line);
                        required_failures.push(detail.clone());
                        finding.skipped.push(detail);
                    }
                }
                let (matches, _) = patterns::matches_for(&log.lines, self.accusation());
                for (line, label) in matches {
                    finding
                        .indications
                        .push(format!("retained kernel-log report: {}: {}", label, line));
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
            Verdict::Clear => {
                "observed EDAC counters zero; no recognized memory errors in acquired retained log"
                    .to_string()
            }
            Verdict::Degraded => first_plus_more(&finding.indications),
            Verdict::Unknown => {
                let mut parts = required_failures;
                if log_ok {
                    parts.push(
                        "no recognized kernel memory-error signature matched — hardware memory integrity not established"
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
    use crate::collect::{Edac, EdacController, KernelLog, Source, Status};
    use crate::eval::fixtures::{self, src_ok};
    use std::time::Duration;

    #[test]
    fn clear_on_clean_fixture() {
        let finding = Memrot.evaluate(&fixtures::clean_facts());
        assert_eq!(finding.verdict, Verdict::Clear);
        assert_eq!(
            finding.summary,
            "observed EDAC counters zero; no recognized memory errors in acquired retained log"
        );
    }

    #[test]
    fn degraded_on_uncorrected_errors() {
        let mut facts = fixtures::clean_facts();
        facts.edac = src_ok(Edac {
            controllers: vec![EdacController {
                name: "mc0".to_string(),
                ue: Some(2),
                ce: Some(0),
            }],
        });
        let finding = Memrot.evaluate(&facts);
        assert_eq!(finding.verdict, Verdict::Degraded);
        assert!(finding.indications[0].contains("ue=2"));
    }

    #[test]
    fn degraded_on_corrected_errors() {
        let mut facts = fixtures::clean_facts();
        facts.edac = src_ok(Edac {
            controllers: vec![EdacController {
                name: "mc0".to_string(),
                ue: Some(0),
                ce: Some(7),
            }],
        });
        let finding = Memrot.evaluate(&facts);
        assert_eq!(finding.verdict, Verdict::Degraded);
        assert!(finding.indications[0].contains("ce=7"));
    }

    #[test]
    fn partial_edac_preserves_positive_evidence() {
        let mut facts = fixtures::clean_facts();
        facts.edac = Source::partial(
            Edac {
                controllers: vec![EdacController {
                    name: "mc0".to_string(),
                    ue: Some(2),
                    ce: None,
                }],
            },
            Status::Unavailable,
            "mc0/ce_count: permission denied",
            "/sys/devices/system/edac/mc",
            Duration::from_millis(1),
        );
        let finding = Memrot.evaluate(&facts);
        assert_eq!(finding.verdict, Verdict::Degraded);
        assert!(finding.indications.iter().any(|i| i.contains("ue=2")));
    }

    #[test]
    fn partial_edac_without_indication_cannot_clear() {
        let mut facts = fixtures::clean_facts();
        facts.edac = Source::partial(
            Edac {
                controllers: vec![EdacController {
                    name: "mc0".to_string(),
                    ue: Some(0),
                    ce: None,
                }],
            },
            Status::Unavailable,
            "mc0/ce_count: disappeared",
            "/sys/devices/system/edac/mc",
            Duration::from_millis(1),
        );
        let finding = Memrot.evaluate(&facts);
        assert_eq!(finding.verdict, Verdict::Unknown);
        assert!(!finding
            .evidence
            .iter()
            .any(|e| e.contains("EDAC counters zero")));
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
    fn registration_is_not_a_memory_error_when_edac_is_absent() {
        let mut facts = fixtures::clean_facts();
        facts.edac = Source::failed(Status::Unsupported, "no controllers", Duration::ZERO);
        facts.kernel_log = src_ok(KernelLog {
            lines: vec!["<6>[ 0.0] EDAC MC0: Giving out device to module fixture_edac controller fixture: DEV fixture (POLLED)".to_string()],
            method: "dmesg".to_string(),
        });
        let finding = Memrot.evaluate(&facts);
        assert_eq!(finding.verdict, Verdict::Unknown);
        assert!(finding.indications.is_empty());
    }

    #[test]
    fn unknown_edac_form_blocks_clear_but_retains_supported_positives() {
        let mut facts = fixtures::clean_facts();
        facts.kernel_log = src_ok(KernelLog {
            lines: vec!["EDAC unusual_driver: report format unavailable".to_string()],
            method: "dmesg".to_string(),
        });
        let finding = Memrot.evaluate(&facts);
        assert_eq!(finding.verdict, Verdict::Unknown);
        assert!(finding
            .skipped
            .iter()
            .any(|line| line.contains("unrecognized EDAC")));
        facts
            .kernel_log
            .value
            .as_mut()
            .unwrap()
            .lines
            .push("EDAC MC0: 2 UE error on DIMM0".to_string());
        assert_eq!(Memrot.evaluate(&facts).verdict, Verdict::Degraded);
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
