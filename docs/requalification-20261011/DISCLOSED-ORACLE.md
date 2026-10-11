# Predisclosed oracle and usefulness eligibility

Owner/independent reviewer: `acceptance_review`. Declared before outcome review on 2026-10-11. Candidate source `7e04d3cb076a48f2e131d6284a16f6bbb836ff58`, executable SHA-256 `0760ea3529ac3c3d831038a71c46f82b6bd3d750fd4dbc52866399962e593b06`. Scope: the planned 51 disclosed guest occurrences only. Fresh sealed specifications are not read or incorporated here. Review of outcomes begins only after the complete producer chain seals.

The oracle comes from the original independently reviewed accusation contracts and actual Linux condition proofs, not the successor decision code. Each row is conditional on independent induction, observation order and applicable evidence. Missing independent proof is F7, never an automatic pass. A sufficient independently supported positive for the same accusation may justify DEGRADED despite another required source failing; its limitation must remain visible. It does not justify an unrelated accusation. A stricter UNKNOWN is correct when required evidence is actually inadequate, but unexplained refusal in an independently established sufficient-evidence control reduces useful exoneration.

K = kernel-scream; S = silent-killer; M = memrot; N = network-choke-point. C = bounded CLEAR; D = supported DEGRADED; U = UNKNOWN; NA = NOT_APPLICABLE. Table outcomes describe the intended condition absent additional independently established indications. Unexpected conditions are adjudicated explicitly rather than changing a passing expectation after seeing output.

| Case | Independent condition required | Intended conclusion | Useful C eligibility |
|---|---|---|---|
| G01 | Ample supported root, real quiet retained log, adequate counters/default | K C; S C; N C | 3 |
| G02 | Loopback-only domain | N NA | 0; NA is not exoneration |
| G03 | Same-user required log permission denial | K/S/M/N U | 0 |
| G04 | No EDAC controllers | M U | 0 |
| G05 | Neither family has usable default | N D | 0 |
| G06 | Unreachable default only | N D | 0 |
| G07 | Blackhole default only | N D | 0 |
| G08 | Unused administratively down dummy, adequate main route/counters | N C | 1 |
| G09 | Actual carrier zero, valid route, no sampled errors/drops or repeated log threshold | N C | 1; connectivity not claimed |
| G10 | Incorrect next hop and failed probe, required bounded sources adequate | N C | 1; reachability excluded |
| G11 | Resolver timeout, adequate named network evidence | N C | 1; DNS excluded |
| G12 | Filtering rule hit and failed probe, sampled interface counters stable | N C | 1; filtering excluded |
| G13 | Real TUN rejection counters independently increase inside guaranteed sample interior | N D | 0 |
| G14 | Stopped TUN generation, stable nonzero historic drops throughout bracket | N C | 1 |
| G15 | At least three actual applicable down events independently logged | N D | 0 |
| G16 | Independently established conntrack pressure over named threshold | N D | 0 |
| G17 | Conntrack absent; other required network evidence adequate | N C | 1 |
| G18 | Material ext4 critically short of free blocks | S D | 0 |
| G19 | Material ext4 inode exhaustion, distinct from block exhaustion | S D | 0 |
| G20 | Actual read-only filesystem backing, failed write | S D | 0 |
| G21 | Read-only bind over same ample writable backing | S C | 1 |
| G22 | Unsupported visible root filesystem | S U | 0 |
| G23 | One material tmpfs full, unrelated root ample | S D | 0 |
| G24 | XFS shutdown verified by failed actual I/O and real shutdown log | K D; S D | 0; df limitations retained |
| G25 | Guest device-mapper error target, failed actual I/O and relevant kernel messages | K D; S D | 0; no hardware equivalence |
| G26 | Required counter denied to diagnosis identity, log readable | N U | 0 |
| G27 | Acquired malformed df substitution | S U | 0 |
| G28 | Empty acquired log outputs | K U | 0 |
| G29 | DNS timeout coexists with ample filesystem/adequate bounded network evidence | S C; N C | 2 |
| G30 | Systemd compound environment absent from admitted substrate | Unexecuted F7 | 0 |
| G31 | No default and required log denied independently | N D with limitation | 0 |
| G32 | Interface disappearance strictly during counter sample | N U, or D only from separate supported positive | 0 |
| G33 | Critical ext4 capacity established before each interrogation | S D on all condition repeats | 0 |
| G34 | Prior fill removed before interrogation; independently ample supported capacity | S C | 1 |
| G35 | Real unmount replaces old ext4 pathname with parent filesystem during df acquisition | S U, retaining identity mismatch; separate D requires supported positive | 0 |
| G36 | No established route-read transition barrier | Unexecuted F7 | 0 |
| G37 | Acquired stream remains open beyond command deadline | K U, deadline limitation; overrun is F4 | 0 |
| G38 | Real HTTP500 while filesystem/network bounded sources adequate and EDAC absent | S C; M U; N C | 2 |

G13/G14 use real TUN successor induction, not the previously failed veth induction. Preserve those historical failures rather than crediting them as controls. The planned G35 repeat and G33 within-occurrence repeats measure repeatability and do not add independent representative scenarios. Main G representatives contain 46 findings and provisionally 16 useful-C opportunities if their independent conditions reproduce. G30/G36 remain explicit gaps despite omission from the runnable plan.

## Other disclosed sequences

- Filesystem identity controls: FS01 actual writable ext4 at a pathname containing internal double spaces, FS02 read-only bind with the same writable backing, FS03 restored root. Expect S C in each if direct write/read/statfs/mount/df observations establish sufficient evidence. All three are useful opportunities; they are supplementary, not additional G representatives.
- Route sequence: actual usable default C; absent default D; zero-network /8 D; zero-host /32 D; restored usable default C. Both C controls are useful opportunities. An actual alternate-family usable default changes applicability and must be recorded.
- Acquisition sequences on each kernel: real healthy K C, healthy S C, absent EDAC M U, non-log output K U, invalid UTF-8 K U, inherited-pipe timeout K U, restored real K C. Three useful opportunities per kernel if adequate real sources are independently verified. Command-format probes are oracle tools, not Alibi findings. These seven invocations qualify only that acquisition subset on the second kernel.
- Every captured baseline/restored control is evaluated and retained. Their usefulness is reported separately from representative primary cases, so many repeated healthy baselines cannot inflate independent-condition coverage.

## Original holdouts, now disclosed regressions

| Case | Required independent condition | Allowable intended verdict / receipt | Useful C eligibility |
|---|---|---|---|
| H01 | Live conntrack outside private view; required pair member missing inside | N U unless a separate supported positive; record actual private count-read limitation | 0 |
| H02 | Open handle proves critical hidden lower filesystem; ample different upper filesystem covers pathname | S U unless a supported collected positive; never credit upper capacity for lower | 0 |
| H03 | Intended positive drops plus missing counter; actual induction independently proved | N D if usable positive preserved; U if only inadequacy observed. Intended condition is F7 when drops remain zero | 0 |
| H04 | Benign EDAC registration substitution; no hardware memory evidence | M U, no unsupported positive | 0 |
| H05 | Same-name interface replacement inside guaranteed sample interval | N U, or D only from independently comparable separate positive; net limitation recorded | 0 |
| H06 | Actual missing route plus denied log, JSON and default human invocations | N D; default human exposes denied required source identity/status/method or its absence/detail | 0 |

These six are not fresh final acceptance cases. H03's prior failed induction remains historical F7 and the successor must independently establish whether that condition occurred. No pass is obtained merely by matching its prior UNKNOWN.

## New disclosed churn cases

| Case | Required independent condition | Allowable intended verdict | Useful C eligibility |
|---|---|---|---|
| C01 | Successful deletion/recreation with reused names, MACs and indices strictly inside sample | N U; D only with separate comparable positive | 0 |
| C02 | Original device renamed and original name reused, both generations independently observed | N U; D only with separate comparable positive | 0 |
| C03 | Replacement completed before collection, stable complete healthy successor during sample | N C | 1 |
| C04 | Same device moved away and back, object identity preserved and complete non-indicating endpoints | N C is allowable for same-object endpoints; U if actual required reads fail | 1 when independent observation establishes sufficient acquisition evidence |

C04 does not qualify uninterrupted namespace residence. For in-window cases calculate the intersection of all possible sample intervals using independent invocation brackets plus recorded duration and sample offsets; an injection sleep alone is insufficient. Require margins beyond timestamp resolution. For C01 prove actual replacement as well as endpoint identifier equality. For C03 prove completion before the diagnosis starts.

## Scoring and formalization

Do not merge F7 into success or treat NA as useful exoneration. Count false CLEAR both per evaluated applicable finding and per emitted evaluable CLEAR. Count useful exoneration only where the independent oracle establishes sufficient applicable evidence for C; report every eligible UNKNOWN as lost utility, with cause. Positive preservation and human fidelity receive their own columns. Supplementary sequences and repeated controls remain separately counted. Provisional representative opportunities are 16 from G plus two from C; H adds none. Final denominators depend on actual condition proof, and every exclusion must remain explicit.

Finite propositions: valid counter subtraction requires one object identity; positive attribution requires independently interpretable evidence; a canonical limitation cannot disappear in human rendering. Reviewer owns this declared oracle. Actual kernel/utility observations, interval calculations and receipt invariants establish correspondence for finite executions; an abstract model alone cannot establish kernel behavior or fault induction. No general snapshot atomicity or log authenticity is claimed. Root owns custody and storage; this small declaration is retained through final acceptance or review2026-10-18.
