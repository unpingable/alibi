# Independent successor review notes

Owner: acceptance_review. Baseline source: `6a7b5012494b090726c4184644cdaccbaba7837e`. Status: design challenge, not implementation acceptance. New sealed cases have not been inspected. Original H04–H06 are disclosed regressions and cannot establish fresh final acceptance.

## Interface evidence composition

The required proposition is that every subtracted counter pair refers to the same kernel object, with all facts combined into a per-interface conclusion tied to that object. Repeated names, MAC addresses and ifindices alone are insufficient: identifiers can be reused and endpoint equality permits alternate transition histories.

Retaining opened sysfs attribute descriptors for both samples is a promising bounded repair. Opens must be relative to a held interface directory, or otherwise prove all attributes belong to that held generation. Independently reopening six name-based paths can mix initial objects even if each individual descriptor is later retained. Reads must rewind and reacquire values rather than return application buffers. Removal or inability to read must invalidate the required evidence; no pathname reopen fallback may silently substitute another object. Final operstate/carrier must be tied to the same object or excluded from incompatible combinations. Directory identity comparisons can corroborate visible endpoint correspondence but are not a complete history.

Primary source correspondence: [Linux v6.6 kernfs file implementation](https://raw.githubusercontent.com/torvalds/linux/v6.6/fs/kernfs/file.c) obtains an active node reference for sequential reads and returns ENODEV when that fails. This adjacent-version source supports the mechanism hypothesis, not direct qualification of both campaign kernels. Genuine guest deletion/replacement trials and healthy repeated reads are still required.

Important distinction: [Linux v6.5 device namespace transition](https://raw.githubusercontent.com/torvalds/linux/v6.5/net/core/dev.c), function `__dev_change_net_namespace`, preserves registered state, changes the device network namespace and uses device rename to update kobjects. It does not simply recreate all attributes. A retained descriptor may therefore survive a namespace round trip. This preserves object identity but does not establish continuous membership in the diagnosed namespace. Explicitly bound the conclusion; do not claim that held descriptors detect all namespace transitions. Same-object counter reset followed by recovery to a nondecreasing value is another unresolved history unless a separate source establishes it.

A loss-aware rtnetlink subscription is an alternative if transition exclusion is required, but requires subscription before observation, a defensible ending barrier, namespace correspondence, message validation and explicit failure on loss/truncation/overflow. Adding an unchecked listener would replace one unproved assumption with another. This is not presently a request to expand the implementation.

## EDAC attribution

Replace broad vocabulary matching with supported positive event semantics. Positive count and event position must matter; a benign registration line can contain arbitrary controller/device labels including error vocabulary. Unrecognized potentially relevant messages must not become healthy evidence merely because a new positive recognizer rejects them. Preserve known CE/UE event positives and explicit insufficiency for ambiguous records. Exact source/event correspondence and fixtures should be reviewed independently of expected output strings. A VM substitution establishes parser behavior, not a real hardware memory event.

## Human receipt fidelity

Default and verbose human output should both expose the promised envelope: source identity, role, status/completeness, method or its absence, limitation detail, acquisition timing/window, boot scope and unresolved checks. Additional prose may be verbose-only; required limitations cannot be hidden behind a count. Keep Receipt V2 JSON unchanged and compare both renderings against the same receipt data. A correct overall DEGRADED does not excuse hiding required missing evidence for another dimension.

## Formalization consideration and custody

Reviewer decision owner: acceptance_review. The durable nontrivial proposition is same-object composition across two reads, not global snapshot atomicity. A small abstract counterexample model can distinguish reused names from retained-object handles and state its lifetime assumptions. Kernel source correspondence plus actual guest identity-transition observations are necessary for that model to describe this implementation. No universal proof is claimed in these design notes; acceptance remains pending source and empirical evidence.

This lane creates only small review documents and immutable completion notices. Root owns retained replay/evidence custody through successor acceptance and the 2026-10-18 review event. No builds, guests, source edits or credentials are created by this reviewer.

## Independently reviewed successor baseline reproductions

Review time: 2026-10-11T20:23:58.469565+00:00. Batch `R/baseline-repro/COMPLETE.json` SHA-256 `ae7dbf429fa4bf32793fbe888e4060165ce341a3c65efc02dbdf1d111ecf890c`. Verified all three terminal/start/serial/stderr hash chains and all 35 captured-file hashes. These runs use the original frozen source `74e4143844c0535d3837dc67df625f8251197dad` and retained guest archive. They reproduce disclosed defects; they are not fresh holdout acceptance.

H04 again reports memory DEGRADED from the benign registration substitution with no EDAC hardware evidence: F1. H05 again reports network CLEAR and complete net evidence despite indices 4/3 changing to 6/5. Independent transition timestamps 8.44–8.64 lie strictly inside the guaranteed common sample interval 7.872–9.740, derived from capture bracket 7.63–9.89, interrogation completion 2.117 s and relative window 99–2110 ms. This reproduces F0/F2/F3 without alleging actual packet loss. H06 reproduces correct route-based DEGRADED with denied kernel-log detail in JSON, while default human output exposes only the generic three-skipped count: documented envelope F3, not an incorrect route verdict.

Disclosed churn fixture design review: require actual successful deletion/recreation and recorded index/MAC identity for C01/C03; C02 also changes the name set and therefore exercises the existing topology guard; C04 may legitimately preserve same-object endpoint evidence despite namespace movement under the approved scope. Fixed sleep alone does not prove timing overlap; independently recompute interval correspondence from actual records or classify the intended race F7. No new sealed specification was opened.

## Disclosed churn baseline acceptance

Verified four terminal/start/serial/stderr chains and 60 captured-file hashes. Batch seal SHA-256 `3683c0fa35cd911020553ed3cebb131d7db546c9d5735762dc8f1b1be5d2bdb6`. C01 retains names, MACs and indices after actual successful deletion/recreation and still gets baseline CLEAR: the replacement interval 8.17–8.44 is inside common sample interval 7.564–9.466, independently demonstrating that endpoint identifiers are insufficient. C02 rename/reuse at 8.26–8.47 lies inside 7.641–9.553 and correctly yields UNKNOWN from changed topology. C03 replacement ends at 7.24 before invocation starts 7.73 and correctly yields CLEAR. C04 records the same-index device in the other namespace and back at 8.25–8.67, inside 7.645–9.538; CLEAR is allowable solely under same-object endpoint scope. All four intended disclosed conditions were induced. These baseline results do not qualify the repaired executable.

## Provisional source review challenge

Candidate repair-1 executable SHA-256 `14a59ad879347764f4ced7db39caab150afab753de73196aa57bbf0b1c6b922b` is not accepted yet. Its net descriptor logic and human envelope address the reviewed structural requirements; bounded counter parsing now rejects overlong prefixes. A supported journal continuation can, however, append an unrecognized EDAC report to an informational first line while the current origin classifier returns Informational for the entire record. This may bypass its promised unknown-form refusal. Requested a collector-to-evaluator regression and conservative multiline handling before acceptance. The separately proposed whitespace-after-PRI issue is not an acquired-record defect because the collector requires the timestamp immediately after the priority prefix.
