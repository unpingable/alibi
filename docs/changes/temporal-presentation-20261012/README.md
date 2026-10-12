# Temporal presentation, issue #19

Base product: `5aef9cd85486cb15b7d044ab44acc08c9aebeef2`.
Implementation checked: `b3d5e4a3c304e440ebb4d8b703efcecf26809565`.
Authorization: issue #19 only; verdicts and evidence requirements preserved.
Issue #20 remains gated pending a demonstrated benefit and approval before
changing collection order.

Canonical kernel-log source detail now explains acquisition versus historical
coverage in human and JSON receipts, including failure cases. Original collection
failure details are preserved. Human output labels `acquisition_completeness`;
JSON retains Receipt V2's `completeness` key and values. Log indications are
labeled retained reports, network flap counts do not imply a current rate, and
negative summaries speak about acquired retained records or observed state.
No prior-boot collection, time filtering, timestamp inference, pattern additions,
source requirements or aggregation changes are included.

## Validation and limits

133 unit tests, 1 CLI test and 3 receipt tests passed, as did formatting, Clippy
with warnings denied and release build. Receipt V2 golden JSON is unchanged.
The first run found one human-envelope assertion still using the old label;
that assertion was updated without weakening source/detail checks.

The twelve disclosed deterministic inputs from investigation #18 were run against
the changed library. Verdicts, aggregate innocence/MTTI, evidence roles, source
status/completeness/method/window and indication counts remained unchanged. The
collection scheduling function was byte-identical. These include old and recent
flaps, old filesystem errors, absent EDAC, incomplete historical knowledge and
the pre-sample log gap. The comparison script initially looked for sources at the
wrong JSON level; the checker was corrected to use `envelope.sources` and audited
the already captured outputs. No product change or rerun was used to resolve it.

[COMPARISON.json](COMPARISON.json) binds the driver and four pairs of fictional
receipt samples. The source harness is retained on the investigation branch at
`a02a169` under `scripts/investigations/temporal-20261012`. Samples are library
fixtures, not CLI observations of a real host; their artificial timing values
are not performance measurements. These are disclosed regression checks, not
new independent holdouts or Linux-host qualification.

The historical QUALIFIED_WITH_LIMITATIONS disposition remains unchanged and
applies to its exact historical candidate. This changed build does not inherit
that qualification. No release was published and no operator time savings are
claimed.

## Formalization consideration

Decision owner: implementation controller, subject to independent review.
Proposition: canonical presentation preserves evidence decisions while exposing
acquisition/history distinctions consistently. Scope: unchanged collectors,
required-source rules, predicates and JSON structure; changed strings and human
labels. Source diff review, same-receipt renderer tests and the twelve paired
fixture results suffice for this bounded change. No new temporal model is
introduced. They cannot prove historical retention, kernel scheduling behavior,
pattern recall or human understanding. Collection-order improvements would need
a separately admitted claim and correspondence argument.
