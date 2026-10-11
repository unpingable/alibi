# Independent TUN successor review

Sealed four-trial batch `cb29b6c81df1cb81fdaa4c320b205224c8c2afb5d3590310358cbb06708402af`; verified four terminal/start/serial chains and80 captured-file hashes. No mismatch. These are real Linux TUN receive rejections of invalid frame input, observed through actual sysfs counters. They establish neither physical NIC faults nor transport-loss equivalence.

| Occurrence | Independently established condition | Result |
|---|---|---|
| baseline-G14 | Stopped generator; stable nonzero counter [732] throughout independently bracketed process. | CLEAR |
| final-G13 | Guaranteed sample interior [6.29, 8.166]; independent inner samples [(6.38, 94), (8.11, 417)] establish actual delta. | DEGRADED |
| final-G14 | Stopped generator; stable nonzero counter [728] throughout independently bracketed process. | CLEAR |
| final-G13-repeat | Guaranteed sample interior [6.161, 8.040000000000001]; independent inner samples [(6.21, 73), (8.0, 408)] establish actual delta. | DEGRADED |

The baseline historical control holds732 drops unchanged; final historical control holds728 unchanged. Generator HOLD begins more than three seconds before each interrogation and actual independent traces bracket its entirety. Other required network evidence is complete; no matching positive exists. Their CLEARs are useful bounded exonerations despite nonzero cumulative drops. Both final sampled-drop attempts instead show independently increasing counters strictly inside the possible Alibi sampling interval, so DEGRADED is justified without treating the reported delta as ground truth.

**Consolidated disclosed candidate representatives:**36 primary invocations across the38-case inventory,46 findings: CLEAR16, DEGRADED16, UNKNOWN13, NOT_APPLICABLE1. Useful exoneration **16/16**. False CLEAR **0/45 evaluated applicable representative findings,0/16 evaluated CLEAR findings**. G30 (systemd compound) and G36 (ordered route transition) remain F7. The additional final G13 repeat is a separate justified DEGRADED trial, not a new scenario. Original veth G13/G14 failed-induction records remain F7; their historical extra CLEAR is not erased or silently counted as PASS.

Exact candidate source `74e4143844c0535d3837dc67df625f8251197dad`; executable `eb1ef3e9ac00462707e8e416d1e9496c916d074b7af06ec03e1673387a64f1fe`. Baseline G14 source `6f8ad5fba4ed6f3fb9f32c9930377a0a48e06931`. Candidate controls preceding all three candidate trials also CLEAR, excluded from representative usefulness denominator.

Machine-readable interval calculations and evidence pointers: `/data/git/alibi-qualification-evidence-20261011/oracle-tun-replay.json`, SHA-256 `b8ed2b73cc8b1963ece8e1632040232ec3c778f889130d0cc3315ba6885c5d21`. Final six holdouts remain sealed and unevaluated; this is not release acceptance.

Formalization consideration: oracle_design; exact source and result identities above. Shared guest monotonic capture and receipt completion bound the process origin; intersection of possible sample intervals supplies a guaranteed interior. Require independent samples inside it with30ms margin, not merely before/after the whole CLI. For historical counters, stopped generation plus full-process stability supplies the negative oracle. Finite interval arithmetic and actual traces suffice for this bounded correspondence, without atomic snapshot, physical-fault or exhaustive timing claims. Root owns custody and final closeout review.

Acceptance correction: the original report counted37 representative invocations in error.35 original invocations already include G13; replacing it adds0 and G14 adds1, totaling36 (=38 inventory minus G30/G36). Original sealed JSON remains unchanged. Authoritative successor `/data/git/alibi-qualification-evidence-20261011/oracle-tun-replay-corrected.json`, SHA-256 `26cb54c168f37d4a4b95b3fe7df7c69d42302d16988d5d737a653ceb1d816fb2`; all46 findings and45 applicable/16 eligible denominators are unchanged.
