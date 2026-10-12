# Operator guidance changes

Scope: default human presentation and a static `alibi guide` command. Collection,
evaluation, exit codes and JSON Receipt V2 are unchanged. Historical source
qualification remains QUALIFIED_WITH_LIMITATIONS; changed builds are not
silently labeled as the exact qualified source or binary by the packager.

UNKNOWN explanations derive from canonical source status/completeness and
required-source roles. Unavailable and unsupported are distinct. Malformed
sources are described as invalid or inconsistent; this broad category does not
pretend to distinguish syntax from a state-transition race without the exact
source detail. Partial observations are insufficient. Evaluator uncertainty
with otherwise available sources points to its summary and evidence. Default
output now includes evidence for UNKNOWN without requiring verbose mode.
No message promises that rerunning or obtaining more privilege will resolve it.

CLEAR boundaries identify curated log patterns, supported filesystem evidence,
EDAC observations or sampled local networking, as applicable. Source completion
and sample times remain in the envelope; no impossible atomic snapshot is
claimed. A kernel-related application problem can coexist with kernel-scream
CLEAR when it leaves no recognized retained log event.

Formalization consideration (owner: implementer): the proposition is that these
additive presentations preserve canonical verdicts and receipt semantics. Tests
cover every source-status category, missing records, evaluator uncertainty,
unchanged JSON before/after rendering, scope text and the public guide command.
Collection/evaluation/JSON implementations are unchanged. Practical checks and
source comparison suffice for this presentation change; they do not prove human
comprehension or reduced diagnostic time, which remain field-study questions.

Issues: #5 UNKNOWN, #6 symptom discovery, #7 CLEAR boundaries. Receipt export,
comparison, new diagnostic families and Constellation integration are not added.
