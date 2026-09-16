pub mod kernel_scream;
pub mod memrot;
pub mod network_choke_point;
pub mod patterns;
pub mod silent_killer;

#[cfg(test)]
pub mod fixtures;

use crate::collect::Facts;
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verdict {
    Clear,
    Degraded,
    Unknown,
    NotApplicable,
}

impl Verdict {
    pub fn as_str(self) -> &'static str {
        match self {
            Verdict::Clear => "CLEAR",
            Verdict::Degraded => "DEGRADED",
            Verdict::Unknown => "UNKNOWN",
            Verdict::NotApplicable => "NOT_APPLICABLE",
        }
    }
}

#[derive(Debug)]
pub struct Finding {
    pub accusation: &'static str,
    pub verdict: Verdict,
    pub summary: String,
    pub indications: Vec<String>,
    pub evidence: Vec<String>,
    pub skipped: Vec<String>,
    pub time_to_verdict: Duration,
}

impl Finding {
    pub fn new(accusation: &'static str) -> Self {
        Finding {
            accusation,
            verdict: Verdict::Clear,
            summary: String::new(),
            indications: Vec::new(),
            evidence: Vec::new(),
            skipped: Vec::new(),
            time_to_verdict: Duration::ZERO,
        }
    }
}

pub trait Evaluator {
    fn accusation(&self) -> &'static str;
    fn evaluate(&self, facts: &Facts) -> Finding;
}

pub fn registry() -> Vec<Box<dyn Evaluator>> {
    vec![
        Box::new(kernel_scream::KernelScream),
        Box::new(silent_killer::SilentKiller),
        Box::new(memrot::Memrot),
        Box::new(network_choke_point::NetworkChokePoint),
    ]
}

pub fn base_verdict(indications: &[String], required_failures: &[String]) -> Verdict {
    if !indications.is_empty() {
        Verdict::Degraded
    } else if !required_failures.is_empty() {
        Verdict::Unknown
    } else {
        Verdict::Clear
    }
}

pub fn first_plus_more(indications: &[String]) -> String {
    match indications.len() {
        0 => String::new(),
        1 => indications[0].clone(),
        n => format!("{} (+{} more)", indications[0], n - 1),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_verdict_degraded_outranks_required_failure() {
        let indications = vec!["evidence of failure".to_string()];
        let failures = vec!["kernel log unavailable".to_string()];
        assert_eq!(base_verdict(&indications, &failures), Verdict::Degraded);
    }

    #[test]
    fn base_verdict_unknown_on_required_failure_only() {
        let failures = vec!["kernel log unavailable".to_string()];
        assert_eq!(base_verdict(&[], &failures), Verdict::Unknown);
    }

    #[test]
    fn base_verdict_clear_when_all_sources_ok() {
        assert_eq!(base_verdict(&[], &[]), Verdict::Clear);
    }
}
