# Successor experiment inventory

The independent [disclosed oracle](DISCLOSED-ORACLE.md) defines conditions, allowable verdicts and useful-exoneration eligibility before outcome review. It is the detailed inventory for the 51 planned disclosed occurrences. Counts of VM boots, CLI invocations and independent conditions are deliberately separate.

| Sequence | Intended coverage | Evidence root beneath R |
|---|---|---|
| Old-binary H04/H05/H06, three guests | Reproduce all three prior defects before accepting repairs | `baseline-repro` |
| Old-binary C01–C04, four guests | Same-name/identifier replacement, rename, precollection replacement, same-object namespace round trip | `churn-baseline` |
| Repaired G cases and supplementary sequences, 41 completed planned guests | Previously passing controls, actual network/storage failures, evidence loss, command deadlines, filesystem identity transition, acquisition on a second kernel | `disclosed-batch` and its reconciled successor |
| Repaired H01–H06, six completed guests | Original holdouts as disclosed regression cases | `disclosed-successor-batch` |
| Repaired C01–C04, four completed guests | Additional disclosed interface-transition regressions | `disclosed-successor-batch` |
| Q01–Q10, ten once-only fresh cases | Independent post-freeze acceptance; original inputs preserved unchanged | `holdouts` and `fresh-holdout-batch` |

R is `/data/git/alibi-qualification-evidence-20261011/guest-lab/requalification-20261011`. Every actual guest has unique execution identity, captured serial stream, START and TERMINAL records, capture hashes, environment identity and condition observations. Successful transport alone does not imply a correct verdict or induced fault.

G30 (systemd compound environment) and G36 (precise route-read transition barrier) remain explicit unexecuted qualification gaps. H03 requires independent proof of its intended counter increase; a previous failed induction cannot become a pass by repeating UNKNOWN. Resource starvation, stalled storage and general service functionality remain outside this bounded campaign's established capabilities.

The first repaired disclosed batch retained 13 terminal boots before a prelaunch G09 control-socket pathname rejection. One of those 13, the second-kernel acquisition attempt, exited before the scenario due to a mismatched module declaration. The separately sealed successor executed the 38 unexecuted cases plus a module-free second-kernel retry. Original records are retained and counted as infrastructure outcomes; the failed boot is not an Alibi invocation. This reconciliation does not change the predeclared semantic oracle.

The finalized [results](RESULTS.md), [fresh oracle](FRESH-ORACLE.md) and [independent acceptance](ACCEPTANCE-FINAL.md) report correct CLEAR, incorrect CLEAR, supported and unsupported positive attribution, justified and unnecessary UNKNOWN, receipt integrity, execution failures and unestablished conditions. Useful-exoneration eligibility is independently established from actual sufficient evidence. Repeated healthy baselines and supplemental controls cannot inflate the count of independent representative conditions.

Final fresh inventory: Q01 loaded healthy conntrack; Q02 same-object administrative transition; Q03 surviving-object name permutation; Q04 actual missing default plus malformed unused ifindex (identity-inadequacy subclaim F7); Q05 full separate tmpfs plus healthy network; Q06 conntrack pressure/ample filesystem/absent EDAC; Q07 benign ECC-enable log substitution; Q08 actual read-only ext4 plus root-only df failure; Q09 real TUN drops plus benign initialization substitution; Q10 counter-decrease substitution. All ten executed once after freeze; nine complete oracles and one explicit partial oracle, not ten blanket passes.

## Evaluated representative matrix

All rows below use the 6.5 guest environment. Counts are selected findings/cases, not all CLI invocations. Substitution cases remain labeled in the detailed oracle. The second kernel covers only supplementary acquisition trials.

| Set | Accusation | Representative guest cases | CLEAR | DEGRADED | UNKNOWN emitted | NA | False CLEAR / unsupported positive / receipt or execution failure |
|---|---|---:|---:|---:|---:|---:|---|
| disclosed | kernel-scream | 6 | 1 | 2 | 3 | 0 | 0 / 0 / 0 |
| disclosed | silent-killer | 17 | 5 | 7 | 5 | 0 | 0 / 0 / 0 |
| disclosed | memrot | 4 | 0 | 0 | 4 | 0 | 0 / 0 / 0 |
| disclosed | network-choke-point | 29 | 12 | 8 | 8 | 1 | 0 / 0 / 0 |
| fresh | kernel-scream | 3 | 3 | 0 | 0 | 0 | 0 / 0 / 0 |
| fresh | silent-killer | 4 | 2 | 2 | 0 | 0 | 0 / 0 / 0 |
| fresh | memrot | 3 | 0 | 0 | 3 | 0 | 0 / 0 / 0 |
| fresh | network-choke-point | 8 | 3 | 3 | 2 | 0 | 0 / 0 / 0 |

H03 contributes an emitted UNKNOWN but its intended condition is unestablished. Q04 retains an evaluated route positive and a separate unqualified identity-inadequacy subclaim. G30/G36 are unexecuted. These gaps are not hidden by the zero product-failure columns. Machine-readable aggregation: R/QUALIFICATION-MATRIX.json; underlying per-finding receipt identities and eligibility remain in the independent reviews.
