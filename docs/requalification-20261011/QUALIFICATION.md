# Successor release assessment

**Disposition: QUALIFIED_WITH_LIMITATIONS**, for the exact repaired executable and bounded environments below. This is a conservative engineering qualification recommendation, not publication authorization or universal Linux correctness. It does not replace the original campaign's historical HELD_DEFECT record. Independent final acceptance is recorded separately in [ACCEPTANCE-FINAL.md](ACCEPTANCE-FINAL.md), with its authoritative [kernel-version correction](ACCEPTANCE-CORRECTION.md).

Product source: `7e04d3cb076a48f2e131d6284a16f6bbb836ff58`. Executable SHA-256: `0760ea3529ac3c3d831038a71c46f82b6bd3d750fd4dbc52866399962e593b06`. R/FINAL-FREEZE.json binds the source, binary, passing checks and independent disclosed acceptance before fresh-case disclosure. R is `/data/git/alibi-qualification-evidence-20261011/guest-lab/requalification-20261011`.

The three demonstrated defects have been eliminated in their reproduced cases and additional disclosed boundary cases: mixed interface generations no longer justify CLEAR; benign EDAC registration no longer supports an error attribution; default human output no longer conceals the demonstrated required-source limitation. Independent review also caught and repaired a multiline-log false CLEAR before the final freeze. No product change followed fresh holdout disclosure or execution.

The repaired candidate passes 127 unit tests, three receipt integration tests, formatting, Clippy with warnings denied and release build. Independent disclosed review covers 51 completed planned guest occurrences. Ten previously unseen cases then ran against the frozen candidate. The complete successor record contains 69 actual QEMU boots and 168 Alibi invocations, including old-binary counterexamples and one pre-scenario infrastructure failure. Those historical failures are not mixed into repaired-candidate success counts.

Disclosed representative usefulness is 18/18. Fresh usefulness is 7/7 independently eligible real-source conclusions, plus 1/1 explicitly substituted benign-log conclusion. No false CLEAR, unsupported positive attribution, critical execution failure or receipt-integrity failure was demonstrated in the repaired evaluated set. These finite, deliberately selected observations are not population error-rate estimates or proof of absence. Read-only review and observed execution found no authority violation; this is not a complete syscall-level proof of all possible execution.

## Exactly what can be trusted

| Accusation or interface | Bounded conclusion supported by this evidence | Conditions and exclusions |
|---|---|---|
| `kernel-scream` | A complete acquired retained log with no recognized serious indication can support bounded CLEAR; malformed, denied or deadline-failed required acquisition yields uncertainty. | Supported command framing and genuine relevant source; no assertion about overwritten history, source authenticity, every possible kernel message or physical hardware health. |
| `silent-killer` | Adequately observed supported material filesystems can be exonerated for the implemented capacity, inode, read-only-backing and logged filesystem-error conditions. | Supported root/mount model, adequate targeted usage and log evidence, coherent source/mountpoint correspondence. General service functionality, slow storage and arbitrary mount-generation replacement are excluded. |
| `network-choke-point` | Comparable same-object counter samples, adequate routes/logs and applicable conntrack evidence can support bounded CLEAR. Replacement/reused names cannot supply the held object's second counter observation. | Genuine sysfs semantics and coherent proc/sys views. Two-second endpoint scope; no DNS, remote-service, filtering, continuous availability, complete topology-history or hidden driver-reset guarantee. |
| `memrot` | EDAC-less guests correctly remain UNKNOWN; supported versus benign/ambiguous EDAC text receives narrower interpretation. | No real physical memory-error induction or hardware-memory exoneration has been qualified. Synthetic parser checks remain explicitly synthetic. |
| Human/JSON receipts | Material required-source limitations remain visible even when another source justifies DEGRADED; accusation-specific outcomes and aggregate innocence agree. | Tested canonical Receipt V2 semantics and default human rendering; usability and operator comprehension remain unmeasured. |

The full disclosed matrix and fresh set used Linux 6.5.0-44-generic under QEMU TCG with constructed GNU/Dash/BusyBox userspace. Linux 6.8.0-90-generic covers only the acquisition subset. These are independently booted kernels, not two complete distribution qualifications or physical hardware tests. Device-mapper failures and TUN drops have their actual guest meanings; no hardware equivalence is asserted.

## Retained limitations and the oracle disagreement

H03 still failed to induce its intended drops; G30 lacks the systemd compound environment; G36 lacks the precise route-read transition barrier. These remain qualification gaps. Resource starvation, slow/stalled storage and general service diagnosis remain outside the established capability set.

Fresh Q04 expected malformed ifindex text to invalidate required identity evidence. The sealed expectation is preserved unchanged, as is the observed complete net source. Both independent reviewers concluded that this was an oracle overconstraint: the normative requirement is sound object correspondence, and the implementation uses retained directory/counter descriptors rather than ifindex text. The actual no-default-route DEGRADED is supported. The intended identity-inadequacy subclaim remains F7 and is not counted as a full case pass. This explicit gap motivates retaining the WITH_LIMITATIONS disposition rather than presenting an unqualified ten-for-ten success.

What could still fool or exceed the tool includes valid-looking stale/substituted logs, unobserved or hidden counter resets, filesystem replacement that preserves printed names, source views outside the coherent-host assumption and indications outside the curated patterns. A receipt cannot establish facts its sources did not preserve. CLEAR never means that the host, service, endpoint or physical hardware is healthy.

## Release decision and next gate

The new evidence supports a narrowly bounded v0.1 engineering recommendation with the exclusions above. It does not support a broad health claim. No push, merge, tag or publication occurred. The next product-validation gate is a real operator comparison against conventional manual investigation, measuring time to eliminate an innocent subsystem, manual checks avoided and understanding of UNKNOWN/CLEAR and receipts. That study has not been performed, and reduced human mean time to innocence is not claimed here.

Root owns the assessment and exact replay custody; independent reviewers own their acceptance records. The finite evidence/identity propositions and limits are recorded in REPAIR-DESIGN.md, ACCEPTANCE-SOURCE.md, ACCEPTANCE-DISCLOSED.md and ACCEPTANCE-FINAL.md. Practical kernel correspondence tests, interval arithmetic and independent evidence review support these bounded claims; no all-schedules formal proof is claimed. Storage and credential disposition are in STORAGE-CLOSEOUT.md, with retained evidence review by 2026-10-18.
