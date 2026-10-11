# Independent oracle adjudication — corrected baseline

Baseline `6f8ad5fba4ed6f3fb9f32c9930377a0a48e06931`; batch seal `6eb810fa906258cdaf48b88a0e9c3ea32bcfe5c9a79d6920f827987bbbcc4237`. Whole producer chain was terminal before review. Verified 39 terminal/start/serial identities and 375 captured-file hashes without mismatch. This is baseline evidence, not acceptance of a repaired candidate.

All 39 guest occurrences completed, including one separately scoped five-invocation route sequence. The 38 G scenarios produced 30 primary JSON invocations and 38 findings: {'CLEAR': 16, 'NOT_APPLICABLE': 1, 'UNKNOWN': 11, 'DEGRADED': 10}. Nine intended cases remain F7; eight have no primary JSON and G13 ran without inducing the intended counter delta.

| Case | Actual primary verdicts | Independent oracle / limitation |
|---|---|---|
| G01 | kernel-scream:CLEAR; silent-killer:CLEAR; network-choke-point:CLEAR | Ample supported tmpfs root, quiet retained real log, independent interface counters and routes adequate. |
| G02 | network-choke-point:NOT_APPLICABLE | Loopback-only link domain independently established; NOT_APPLICABLE is not an exoneration. |
| G03 | kernel-scream:UNKNOWN; silent-killer:UNKNOWN; memrot:UNKNOWN; network-choke-point:UNKNOWN | Independent unprivileged log probe failed with permission denial; required log inadequate. |
| G04 | memrot:UNKNOWN | No EDAC controllers; memory hardware evidence unavailable. |
| G05 | network-choke-point:DEGRADED | Both raw route tables establish no usable default. |
| G06 | network-choke-point:DEGRADED | Actual unreachable default and no alternate-family usable default. |
| G07 | network-choke-point:DEGRADED | Actual blackhole default and no alternate-family usable default. |
| G08 | network-choke-point:CLEAR | Unused dummy administratively down; main default and sampled counters healthy. Down alone is outside named indication. |
| G09 | No primary JSON | Carrier controller not implemented in this occurrence; supplemental successor separate. **F7.** |
| G10 | network-choke-point:CLEAR | Unused next hop and failed probe; complete bounded network evidence. Reachability is outside accusation. |
| G11 | network-choke-point:CLEAR | Actual unavailable resolver query reaches timeout; complete bounded network evidence. DNS is outside accusation. |
| G12 | No primary JSON | nf_tables load failed because libcrc32c prerequisite omitted; no filtering condition established. **F7.** |
| G13 | network-choke-point:CLEAR | UDP generator ran but independent tx_dropped before/after both zero. Intended sampled-drop fault not induced; do not relabel as a passing healthy control. **F7.** |
| G14 | No primary JSON | No nonzero historical drops induced; intended condition absent. **F7.** |
| G15 | No primary JSON | Actual logged down-event sequence not induced in this occurrence; supplement separate. **F7.** |
| G16 | network-choke-point:DEGRADED | Independent conntrack count 120 with maximum 128 establishes pressure above 90 percent. |
| G17 | network-choke-point:CLEAR | Both conntrack files absent, other required network observations adequate. |
| G18 | silent-killer:DEGRADED | Actual ext4 free blocks critical and bounded fill failed; supported positive. |
| G19 | silent-killer:DEGRADED | Actual ext4 inode exhaustion after 245 creations, blocks remain available. |
| G20 | silent-killer:DEGRADED | Actual read-only ext4 remount and independently failed write. |
| G21 | silent-killer:CLEAR | Backing view write succeeds, intentional read-only bind write fails; backing filesystem remains writable and ample. |
| G22 | silent-killer:UNKNOWN | Actual visible root overlay unsupported; no false vacuous exoneration. |
| G23 | silent-killer:DEGRADED | One material tmpfs full while root ample. |
| G24 | No primary JSON | XFS module failed because libcrc32c omitted; shutdown not induced. **F7.** |
| G25 | No primary JSON | Device-mapper error table installed and write failed, but required original buffered read succeeded. Declared read/write conjunction not established; F7 retained. **F7.** |
| G26 | network-choke-point:UNKNOWN | Same-user independent log read succeeds and counter read fails EACCES in private permission view; required network evidence inadequate. |
| G27 | silent-killer:UNKNOWN | Malformed df command substitution; real target independently healthy, required acquired usage inadequate. |
| G28 | kernel-scream:UNKNOWN | Both log command substitutions return empty stdout; real log separate, acquired evidence inadequate. |
| G29 | silent-killer:CLEAR; network-choke-point:CLEAR | Actual DNS timeout coexists with ample filesystem and adequate bounded network evidence. |
| G30 | No primary JSON | No systemd service manager; intended compound condition unexecuted. **F7.** |
| G31 | network-choke-point:DEGRADED | Independent complete route absence plus independently denied logs. Positive survives evidence failure; limitation must remain visible. |
| G32 | network-choke-point:UNKNOWN | Actual veth deletion at uptime 9.23–9.29 lies inside conservatively bounded network sample interval; topology changes and required counters disappear. |
| G33 | silent-killer:DEGRADED | Actual ext4 critical capacity established before invocation and held; two further primary-condition repeats also DEGRADED. |
| G34 | silent-killer:CLEAR | Actual prior fill removed before invocation, ample supported capacity and quiet relevant log during collection. |
| G35 | silent-killer:CLEAR | Target ext4 unmounted between mount enumeration and genuine df; df samples parent tmpfs. Original ext4 was healthy. Receipt falsely treats wrong-identity required usage as complete. |
| G36 | No primary JSON | No robust route-read transition barrier; ordering unestablished. **F7.** |
| G37 | kernel-scream:CLEAR | Finite inherited output pipe lasts four seconds despite three-second collection deadline; late successful evidence accepted. Command substitution, not real kernel fault. |
| G38 | silent-killer:CLEAR; memrot:UNKNOWN; network-choke-point:CLEAR | Independent local HTTP500 application response with ample filesystem, healthy bounded network evidence and absent EDAC. |

Useful exoneration is **13/13 independently eligible primary findings**: G01 K/S/N; G08 N; G10 N; G11 N; G17 N; G21 S; G29 S/N; G34 S; G38 S/N. Eligibility follows the predeclared bounded accusation contract and independent real source observations. DNS, next-hop and HTTP failures do not invalidate those specific exonerations. This is not an accuracy score: nine intended scenarios are inconclusive, EDAC evidence is unavailable, and three other emitted CLEAR findings are excluded (G13 failed induction, G35 identity failure, G37 late command fixture). All ten DEGRADED findings have independently established named positives. Eleven UNKNOWN findings are appropriate for their unavailable/inadequate evidence.

G35 is **F2/F3 and contract-level F0**: mount 29, device 7:0, ext4 at `/run/alibi-matrix/mnt` existed immediately before interception. Its independently measured 7,139 free of 7,145 blocks shows it was healthy. Unmount returned zero; the pathname then identified parent tmpfs with different filesystem ID. Genuine GNU df read that parent. Alibi nevertheless reported two material mounts and complete required fs-usage, with CLEAR. No hidden ENOSPC is alleged. Required source/mountpoint identity validation is a bounded repair; it does not establish atomic mount generations or forbid all legitimate aliases.

G37 is **F4 plus fixture-scoped F0**: success after roughly four seconds violates the three-second collector deadline. No real kernel fault is inferred. Structural receipt algebra alone does not detect either this deadline failure or G35 identity disagreement. All 30 primary receipts otherwise satisfy selected verdict/exit precedence, innocence/MTTI and decision-offset consistency checks.

The separately captured route sequence returns CLEAR for real `0.0.0.0/8` and `0.0.0.0/32` routes although neither is a default. Missing-default control is DEGRADED; valid and restored defaults CLEAR. These are two additional route-specific F0 counterexamples, kept outside the 38-case primary denominator.

Primary mixed-scope runtime: n=30, min=76 ms, median=2085.0 ms, max=4059 ms. Two additional G33 repetitions also report DEGRADED. Most cases are single trials; broad repeatability and performance-regression conclusions remain unavailable.

Machine-readable case/accusation records: `/data/git/alibi-qualification-evidence-20261011/oracle-corrected-baseline.json`; SHA-256 `e31dd8ff3ae92897d1a4eec725a0b7d2579d0664e37dcd6935100badef22bcd8`. Corrected harness still needs independently specified G12/G24/G25 successors and carrier cases; those results must be separately sealed and adjudicated. Original failed-harness evidence remains preserved.

Formalization consideration: decision owner oracle_design; exact source, input seal and result identity above. Proposition: bounded CLEAR requires applicable evidence with correct observation identity; successful reads alone do not establish that identity. G35 is a concrete counterexample to joining by pathname. Finite raw-event correspondence and explicit eligibility rules suffice for this audit; a model cannot recover missing real-host correspondence. No atomic snapshot or full-log-history guarantee is established. Root owns evidence custody and reviews disposable replay substrate at final closeout; minimal results and receipts remain linked. This review is not independent release acceptance.
