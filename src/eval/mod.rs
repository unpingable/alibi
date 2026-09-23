pub mod kernel_scream;
pub mod memrot;
pub mod network_choke_point;
pub mod patterns;
pub mod silent_killer;

#[cfg(test)]
pub mod fixtures;

use crate::collect::{CollectionPlan, Facts, SourceId};
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verdict {
    Clear,
    Degraded,
    Unknown,
    NotApplicable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvidenceRole {
    Required,
    Supporting,
}

impl EvidenceRole {
    pub fn as_str(self) -> &'static str {
        match self {
            EvidenceRole::Required => "required",
            EvidenceRole::Supporting => "supporting",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SourceRole {
    pub source: SourceId,
    pub role: EvidenceRole,
}

impl SourceRole {
    pub const fn required(source: SourceId) -> Self {
        SourceRole {
            source,
            role: EvidenceRole::Required,
        }
    }

    pub const fn supporting(source: SourceId) -> Self {
        SourceRole {
            source,
            role: EvidenceRole::Supporting,
        }
    }
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
    pub source_roles: Vec<SourceRole>,
    pub decided_at: Duration,
    pub innocence_at: Option<Duration>,
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
            source_roles: Vec::new(),
            decided_at: Duration::ZERO,
            innocence_at: None,
        }
    }
}

pub trait Evaluator {
    fn accusation(&self) -> &'static str;
    fn source_roles(&self) -> &'static [SourceRole];
    fn evaluate_evidence(&self, facts: &Facts) -> Finding;

    fn evaluate(&self, facts: &Facts) -> Finding {
        let mut finding = self.evaluate_evidence(facts);
        if finding.source_roles.is_empty() {
            finding.source_roles = self.source_roles().to_vec();
        }
        enforce_required_sources(&mut finding, facts);
        finding
    }
}

pub fn registry() -> Vec<Box<dyn Evaluator>> {
    vec![
        Box::new(kernel_scream::KernelScream),
        Box::new(silent_killer::SilentKiller),
        Box::new(memrot::Memrot),
        Box::new(network_choke_point::NetworkChokePoint),
    ]
}

pub fn collection_plan(evaluators: &[Box<dyn Evaluator>]) -> CollectionPlan {
    CollectionPlan::new(
        evaluators
            .iter()
            .flat_map(|e| e.source_roles().iter().map(|r| r.source))
            .collect(),
    )
}

fn enforce_required_sources(finding: &mut Finding, facts: &Facts) {
    let incomplete: Vec<String> = finding
        .source_roles
        .iter()
        .filter(|r| r.role == EvidenceRole::Required && !facts.source_complete(r.source))
        .map(|r| format!("{}: {}", r.source.as_str(), facts.source_failure(r.source)))
        .collect();

    for failure in &incomplete {
        let message = format!("required evidence incomplete: {}", failure);
        if !finding.skipped.contains(&message) {
            finding.skipped.push(message);
        }
    }

    if !incomplete.is_empty() && matches!(finding.verdict, Verdict::Clear | Verdict::NotApplicable)
    {
        finding.verdict = Verdict::Unknown;
        finding.summary = format!("required evidence incomplete: {}", incomplete.join("; "));
    }
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

    #[test]
    fn kernel_only_collection_plan_excludes_network_sources() {
        let evaluators: Vec<_> = registry()
            .into_iter()
            .filter(|e| e.accusation() == "kernel-scream")
            .collect();
        let plan = collection_plan(&evaluators);
        assert!(plan.contains(SourceId::KernelLog));
        assert!(!plan.contains(SourceId::Net));
        assert!(!plan.contains(SourceId::Routes));
        assert!(!plan.contains(SourceId::Conntrack));
    }
}
