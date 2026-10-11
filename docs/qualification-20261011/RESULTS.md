# Results — bounded execution, release held

Implementation baseline `6f8ad5fba4ed6f3fb9f32c9930377a0a48e06931`; frozen route-repair candidate `0427a7c0a81bb0b8212cc2e9a093453354121119`. Evidence root `/data/git/alibi-qualification-evidence-20261011`. No guest has booted. None of the 38 disclosed guest cases or six hidden holdouts has been executed. Do not count the following local fixtures or read-only development-environment runs toward the 30 real-host experiment target.

## Executed work

Baseline: 102 unit + 2 receipt tests passed; candidate: 104 unit + 2 receipt tests passed. Both passed fmt, clippy with warnings denied, release build. Exact commands/status/times/logs: baseline.json, baseline-*.log, route-mask-repair-checks/RECEIPT.json. Test counts are not diagnostic qualification counts.

Both binaries ran the three minimized library scenarios and three external-CLI kernel-log substitutions in local-reproducers-{baseline,route-mask-repair}/RECEIPT.json. Route /8 and /32 returned true before repair, false after; /0 stayed true. Malformed capacity/access-mode parsing and caught mount escape panic persist. The 100 ms inherited-pipe reproducer returned successful output after approximately 1001 ms on both builds.

Eight independent oracle-specified CLI cases were then executed against both preserved binaries. Fixture specification was sealed before execution; its hash is `5d473c4ce919ca9e91fdfb3dd89164e4f2a046fe86b58c5753a533e40aeb5e87`. Raw binary outputs, direct acquisition-command observations, per-command modes, exit statuses and monotonic times are in local-boundary/{baseline,candidate}/COMPLETE.json and adjacent case directories. Distinct experiment specification is not multiplied by two implementations or counted again as a new condition.

| Case | Baseline | Candidate | CLI seconds baseline / candidate |
|---|---|---|---|
| LE01 | UNKNOWN | UNKNOWN | 0.0015 / 0.0018 |
| LE02 | UNKNOWN | UNKNOWN | 0.0267 / 0.0270 |
| LE03 | UNKNOWN | UNKNOWN | 0.0522 / 0.0522 |
| LE04 | CLEAR | CLEAR | 0.0268 / 0.0268 |
| LE05 | UNKNOWN | UNKNOWN | 0.0523 / 0.0521 |
| LE06 | CLEAR | CLEAR | 4.0031 / 4.0026 |
| LE07 | CLEAR | CLEAR | 0.0524 / 0.0522 |
| LE08 | CLEAR | CLEAR | 0.0267 / 0.0268 |

LE01 missing executables, LE02 execution permission denied, LE03 nonzero acquisition with positive stdout, and LE05 empty/no-entries fallback correctly yield UNKNOWN under the independent oracle. LE03's partial stdout is discarded; UNKNOWN is sound under the explicitly allowed policy and is not a demonstrated unsupported attribution. LE07 is a CLEAR formatting/fallback control only. LE04 invalid UTF-8 and LE08 known non-log substitutions produce fixture-scoped false CLEAR and overstate source completeness. LE06 completes after its declared 3-second acquisition deadline and nevertheless succeeds: demonstrated F4. Count its deadline violation separately from semantic false CLEAR; eventual benign bytes alone do not establish unjustified exoneration.

All eight CLI invocations per binary exited normally, with exits consistent with emitted verdicts. No outer 7-second cutoff fired. No crash occurred in these CLI cases. The malformed mount helper caught a parser panic: this does not prove a real CLI crash on kernel-produced mount syntax. Output growth and stalled-device behavior were not tested. Per-invocation memory/resource consumption was not measured; a 256 MiB service limit is an execution constraint, not a measured consumption result.

A detached service executed the two local fixture rounds; invocation `89b10c965747489dbb8d0c03bf0dc570`. A separate six-second probe demonstrated producer continuation after launch-tool return. This establishes only the tested supervisor/launch boundary, not provider recovery or VM recovery. Both services finished. Two direct-command process groups initially remained observable at EOF; later exact-group reconciliation found no members. See local-boundary/PROCESS-RECONCILIATION.json. No producer was restarted or replaced.

Seven separate read-only development-environment CLI controls captured version, usage, four selected accusations and all accusations. Five diagnostic receipts contain eight findings: K/S CLEAR, M UNKNOWN and N DEGRADED where requested. Network evidence included sampled drops and absent default routes. These are observations without an independently controlled healthy/fault oracle, so none establishes host exoneration soundness. Raw controls remain private local evidence, not sanitized public examples.

## Qualification measurements

| Accusation | Families specified | Qualified environments | Real guest trials | Correct bounded CLEAR / non-CLEAR / justified UNKNOWN | Demonstrated guest F0/F1/F2-F3/F4 | Guest cases unexecuted |
|---|---|---:|---:|---|---|---|
| kernel-scream | controls, logs, storage errors, evidence failures, compound | 0 | 0 | N/A | N/A | All applicable TEST-MATRIX rows |
| silent-killer | capacity, inodes, read-only, root applicability, storage, temporal | 0 | 0 | N/A | N/A | All applicable TEST-MATRIX rows |
| memrot | EDAC absence, evidence boundaries, compound | 0 | 0 | N/A | N/A | All applicable TEST-MATRIX rows |
| network-choke-point | routes, drops, links, conntrack, scope controls, temporal | 0 | 0 | N/A | N/A | All applicable TEST-MATRIX rows |

Real-host useful exoneration rate: **N/A (zero oracle-eligible real-host findings)**. Real-host false-CLEAR incidence: **unmeasured**, not zero. Local LE fixture results per binary: two semantic false CLEAR / eight evaluated findings, and two / four emitted CLEAR; one further emitted CLEAR accompanies an independently counted deadline failure. The sole predeclared adequate formatting/fallback control cleared (1/1); that is not a host-usefulness estimate. Four inadequate-source UNKNOWN results are justified under these fixtures. NA was not exercised by this local set. F1 and F5 were not demonstrated; their absence is not qualified.

Local CLI times above show sample size and long deadline case rather than a misleading aggregate accuracy or population p95. Same defects recur on both builds, but that is not a repeated independent guest trial. Environmental sensitivity beyond this host/toolchain is unmeasured. A route-only parser repair cannot address log provenance, command lifetime or mount observation consistency.

## Custody and limits

Preserve raw receipts plus fixture bytes and exact binary/library identities. No fabricated fault induction, inaccessible-source simulation or parser test is reported as a broken Linux guest. Guest temporal, compound, resource-exhaustion and second-environment results remain F7. Service functionality/resource starvation/storage-path latency remain F6 where not in the named accusation inventory. Final holdout results: **UNEXECUTED**, six cases remain sealed; no tuning from them occurred.
