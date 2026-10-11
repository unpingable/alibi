# Independent final disclosed-matrix review

Candidate `74e4143844c0535d3837dc67df625f8251197dad`, executable SHA-256 `eb1ef3e9ac00462707e8e416d1e9496c916d074b7af06ec03e1673387a64f1fe`. Batch seal `f75f359071fe9b9cd0e4c2107324c6208e65b68264294fcfe3933763b21a181e`. All43 occurrences reached normal guest termination before review. Verified 605 captured-file hashes and43 terminal/start/serial chains; 99 JSON receipts pass aggregate exit/verdict/innocence/MTTI and finding-offset checks. QMP ledgers hash-match and control sockets were retired.

The38 primary G scenarios yielded 35 JSON invocations and 45 findings: {'UNKNOWN': 13, 'CLEAR': 16, 'NOT_APPLICABLE': 1, 'DEGRADED': 15}. Independently eligible useful exoneration: **15/15**. Demonstrated false CLEAR: **0/43 evaluated applicable findings and0/15 evaluated CLEAR findings**. These denominators exclude unresolved G13: it emitted one additional CLEAR, but its intended drop condition was not induced. Four intended scenarios remain F7; three have no primary receipt. The unresolved case is explicitly retained, not counted as PASS.

| Case | Actual primary findings | Independent oracle / limitation |
|---|---|---|
| G01 | kernel-scream: CLEAR; silent-killer: CLEAR; network-choke-point: CLEAR | Ample supported tmpfs root, quiet retained real log, independent interface counters and routes adequate. |
| G02 | network-choke-point: NOT_APPLICABLE | Loopback-only link domain independently established; NOT_APPLICABLE is not an exoneration. |
| G03 | kernel-scream: UNKNOWN; silent-killer: UNKNOWN; memrot: UNKNOWN; network-choke-point: UNKNOWN | Independent unprivileged log probe failed with permission denial; required log inadequate. |
| G04 | memrot: UNKNOWN | No EDAC controllers; memory hardware evidence unavailable. |
| G05 | network-choke-point: DEGRADED | Both raw route tables establish no usable default. |
| G06 | network-choke-point: DEGRADED | Actual unreachable default and no alternate-family usable default. |
| G07 | network-choke-point: DEGRADED | Actual blackhole default and no alternate-family usable default. |
| G08 | network-choke-point: CLEAR | Unused dummy administratively down; main default and sampled counters healthy. Down alone is outside named indication. |
| G09 | network-choke-point: CLEAR | Actual independently observed carrier=0 with retained valid default, zero named counter indication and fewer than three actual down events; restored carrier=1. Bounded CLEAR does not claim connectivity. |
| G10 | network-choke-point: CLEAR | Unused next hop and failed probe; complete bounded network evidence. Reachability is outside accusation. |
| G11 | network-choke-point: CLEAR | Actual unavailable resolver query reaches timeout; complete bounded network evidence. DNS is outside accusation. |
| G12 | network-choke-point: CLEAR | Actual nft rule counter packets 1 bytes 84, failed local probe and stable interface counters. Packet filtering lies outside named interface-drop criterion. |
| G13 | network-choke-point: CLEAR | UDP generator ran but independent tx_dropped before/after both zero. Intended sampled-drop fault not induced; do not relabel as a passing healthy control. **F7.** |
| G14 | No primary receipt | No nonzero historical drops induced; intended condition absent. **F7.** |
| G15 | network-choke-point: DEGRADED | Six QMP transitions independently reflected by guest carrier and three new actual eth0-attributed kernel down messages. Positive criterion established. |
| G16 | network-choke-point: DEGRADED | Independent conntrack count 120 with maximum 128 establishes pressure above 90 percent. |
| G17 | network-choke-point: CLEAR | Both conntrack files absent, other required network observations adequate. |
| G18 | silent-killer: DEGRADED | Actual ext4 free blocks critical and bounded fill failed; supported positive. |
| G19 | silent-killer: DEGRADED | Actual ext4 inode exhaustion after 245 creations, blocks remain available. |
| G20 | silent-killer: DEGRADED | Actual read-only ext4 remount and independently failed write. |
| G21 | silent-killer: CLEAR | Backing view write succeeds, intentional read-only bind write fails; backing filesystem remains writable and ample. |
| G22 | silent-killer: UNKNOWN | Actual visible root overlay unsupported; no false vacuous exoneration. |
| G23 | silent-killer: DEGRADED | One material tmpfs full while root ample. |
| G24 | kernel-scream: DEGRADED; silent-killer: DEGRADED | Shutdown command zero, subsequent actual write Input/output error, real XFS shutdown log. K/S positives justified; S receipt retains partial df EIO limitation. |
| G25 | kernel-scream: DEGRADED; silent-killer: DEGRADED | Actual linear-to-error device mapper transition, direct read and synced write both fail EIO, real ext4/Buffer I/O kernel messages. K/S positives justified; no hardware equivalence inferred. |
| G26 | network-choke-point: UNKNOWN | Same-user independent log read succeeds and counter read fails EACCES in private permission view; required network evidence inadequate. |
| G27 | silent-killer: UNKNOWN | Malformed df command substitution; real target independently healthy, required acquired usage inadequate. |
| G28 | kernel-scream: UNKNOWN | Both log command substitutions return empty stdout; real log separate, acquired evidence inadequate. |
| G29 | silent-killer: CLEAR; network-choke-point: CLEAR | Actual DNS timeout coexists with ample filesystem and adequate bounded network evidence. |
| G30 | No primary receipt | No systemd service manager; intended compound condition unexecuted. **F7.** |
| G31 | network-choke-point: DEGRADED | Independent complete route absence plus independently denied logs. Positive survives evidence failure; limitation must remain visible. |
| G32 | network-choke-point: UNKNOWN | Actual veth deletion at uptime 9.06–9.12; command bracket8.07–10.32 and relative sample96–2107ms bound transition strictly inside sample. Partial topology/counter evidence produces UNKNOWN. |
| G33 | silent-killer: DEGRADED | Actual ext4 critical capacity established before invocation and held; two further primary-condition repeats also DEGRADED. |
| G34 | silent-killer: CLEAR | Actual prior fill removed before invocation, ample supported capacity and quiet relevant log during collection. |
| G35 | silent-killer: UNKNOWN | Actual unmount9.22–9.30 replaces target ext4 pathname with parent tmpfs; genuine df identity mismatch retained in partial malformed fs-usage; UNKNOWN. Independent repeat also UNKNOWN. |
| G36 | No primary receipt | No robust route-read transition barrier; ordering unestablished. **F7.** |
| G37 | kernel-scream: UNKNOWN | Finite inherited pipe exceeds3s collector deadline; collector rejects late evidence and reports timeout including output drain; unavailable fallback, UNKNOWN in3044ms. |
| G38 | silent-killer: CLEAR; memrot: UNKNOWN; network-choke-point: CLEAR | Independent local HTTP500 application response with ample filesystem, healthy bounded network evidence and absent EDAC. |

G35 wrong-filesystem evidence now yields UNKNOWN with exact expected/observed source and mountpoint identity in the receipt. Its independently repeated occurrence also yields UNKNOWN. Both actual original ext4 and replacement tmpfs were healthy; this verifies refusal to join incompatible observations, not discovery of a hidden full filesystem. Separate actual spaced mount, spaced read-only bind and restored controls all CLEAR. Lexical identity validation does not guarantee mount-generation continuity or general alias equivalence.

G37 and both separate kernel acquisition sequences reject inherited-pipe evidence at the declared three-second deadline, including output drain, and report UNKNOWN. A few milliseconds of collection bookkeeping is not a newly asserted real-time scheduling bound. Actual K/S controls remain CLEAR; malformed/non-UTF8 command outputs UNKNOWN; absent EDAC UNKNOWN. Receipt log completeness describes acquired retained output, not proof that no earlier records were lost or that a log source is authentic.

The five-invocation real route sequence clears valid defaults and degrades missing/default-like zero /8 and /32 entries. Separate G33 repeat2/repeat3 both DEGRADED; separate G35 repeat UNKNOWN. These are repeated attempts, not new independent scenarios. Most other conditions have one primary occurrence. No crash, hang, receipt disagreement or unsupported positive is demonstrated in this disclosed candidate round; this is not proof of absence.

Primary runtime n=35: min80ms, median2088ms, nearest-rank p952195ms, max3045ms. Mixed accusation scope precludes a performance-regression estimate. Historical baseline failures remain separately recorded; they are not counted as candidate failures.

Machine-readable primary cases, per-accusation eligibility and all occurrence receipt hashes: `/data/git/alibi-qualification-evidence-20261011/oracle-final-matrix.json`, SHA-256 `39d5be92a20a2126519d3b932d0051f403357819043553dfb01e02e8605f8c8f`. TUN successors and final six holdouts remain separate, unevaluated here. No release disposition is conferred before independent acceptance and holdouts.

Formalization consideration: oracle_design owns this finite adjudication. Proposition: each credited verdict is bounded by independent condition proof, applicable required evidence and observed collection chronology. Exact source/result/receipt identities above. Concrete counterexamples from the baseline drove bounded identity/timeout repairs; explicit traces and receipt-algebra checks suffice for the tested finite cases, while identity generation, retained-history coverage and scheduler timing remain unproved. Root owns custody and reviews replay substrate at campaign closeout.
