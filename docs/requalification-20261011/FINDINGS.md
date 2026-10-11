# Post-holdout findings

Historical decisions and raw evidence are preserved in the original campaign. This successor's evidence root, abbreviated R, is `/data/git/alibi-qualification-evidence-20261011/guest-lab/requalification-20261011`. Source repair acceptance is separate from guest acceptance and release disposition.

| Finding | Original counterexample and violated contract | Bounded repair |
|---|---|---|
| H05 — F0/F2/F3 | A same-name interface replacement during sampling supplied counters from different objects, yet the network accusation was CLEAR. | Hold one interface directory and its counter descriptors across both samples. Read failures cannot be repaired by reopening a pathname. Require comparable evidence; otherwise UNKNOWN. |
| H04 — F1 | Benign EDAC registration text was treated as a memory error. | Require a supported positive error report; recognize a narrow informational grammar and reject ambiguous EDAC formats as inadequate for exoneration. |
| H06 — F3 | Default human output omitted material required-source details retained by JSON, even when another source established DEGRADED. | Render the canonical source envelope and finding source roles in default human output. Verbose mode is not necessary to discover required-source limitations. |
| Review continuation case — F0/F2 | In the first repair candidate, an informational EDAC first line could consume an unknown continuation in one journal record and allow CLEAR. | Classify the first physical line; a nonpositive multiline record is unrecognized. A complete independently positive first line remains positive. |

All three original defects were reproduced in fresh guests against the old executable (`R/baseline-repro`). H05 replacement chronology was independently established inside the guaranteed counter-sampling interval. It does not establish packet loss, and packet loss is not the oracle for this identity defect.

The corrected continuation reproducer is `R/continuation-before-corrected`: complete collector input reached the evaluator and failed CLEAR-versus-UNKNOWN. The earlier `R/continuation-before` fixture had incorrect journal indentation and failed completeness instead; it is preserved as a fixture error, not supporting semantic evidence. The provisional executable and both reproducer records remain retained. The successor passes the new regression and full automated checks in `R/repair-checks-2`.

Additional disclosed guest cases C01–C04 exercise reused names/MACs/indices, rename plus name reuse, replacement completed before collection, and a same-object namespace round trip. Their independent old-binary results are in `R/churn-baseline`. C01 reproduced the false CLEAR despite identifier reuse. C02 already returned UNKNOWN. C03 and C04 returned allowable bounded CLEAR; continuous namespace residence is not a claimed property.

The interface repair assumes genuine sysfs attributes and coherent proc/sys views. It does not establish all topology history, uninterrupted namespace membership, hidden driver resets or an atomic host snapshot. EDAC fixtures test evidence interpretation; they do not qualify physical memory-error detection. No universal EDAC interpreter or event-tracking subsystem was added. Receipt V2 remains unchanged.

Infrastructure exceptions are recorded independently of product findings. The first disclosed successor batch stopped before launching G09 because its QMP control socket pathname exceeded the Unix socket limit. Its earlier Linux 6.8 boot also stopped before the scenario due to a 6.5 module-release declaration in the archive. These are qualification infrastructure gaps, not Alibi failures or passes. Original records remain immutable; successor execution must be reconciled explicitly.

Final guest outcomes and release blockers are recorded in RESULTS.md and QUALIFICATION.md after independent evaluation. This document does not itself certify a repair or release.
