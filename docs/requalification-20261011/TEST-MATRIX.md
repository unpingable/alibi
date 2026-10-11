# Successor experiment inventory

The independent [disclosed oracle](DISCLOSED-ORACLE.md) defines conditions, allowable verdicts and useful-exoneration eligibility before outcome review. It is the detailed inventory for the 51 planned disclosed occurrences. Counts of VM boots, CLI invocations and independent conditions are deliberately separate.

| Sequence | Intended coverage | Evidence root beneath R |
|---|---|---|
| Old-binary H04/H05/H06, three guests | Reproduce all three prior defects before accepting repairs | `baseline-repro` |
| Old-binary C01–C04, four guests | Same-name/identifier replacement, rename, precollection replacement, same-object namespace round trip | `churn-baseline` |
| Repaired G cases and supplementary sequences, 41 planned guests | Previously passing controls, actual network/storage failures, evidence loss, command deadlines, filesystem identity transition, acquisition on a second kernel | `disclosed-batch` and its reconciled successor |
| Repaired H01–H06, six planned guests | Original holdouts as disclosed regression cases | `disclosed-successor-batch` |
| Repaired C01–C04, four planned guests | Additional disclosed interface-transition regressions | `disclosed-successor-batch` |
| Q01–Q10, ten newly sealed cases | Independent post-freeze acceptance; case details remain with the sealed author until freeze | `holdouts`; execution location recorded after freeze |

R is `/data/git/alibi-qualification-evidence-20261011/guest-lab/requalification-20261011`. Every actual guest has unique execution identity, captured serial stream, START and TERMINAL records, capture hashes, environment identity and condition observations. Successful transport alone does not imply a correct verdict or induced fault.

G30 (systemd compound environment) and G36 (precise route-read transition barrier) remain explicit unexecuted qualification gaps. H03 requires independent proof of its intended counter increase; a previous failed induction cannot become a pass by repeating UNKNOWN. Resource starvation, stalled storage and general service functionality remain outside this bounded campaign's established capabilities.

The first repaired disclosed batch retained 13 terminal boots before a prelaunch G09 control-socket pathname rejection. One of those 13, the second-kernel acquisition attempt, exited before the scenario due to a mismatched module declaration. The separately sealed successor plan schedules the 38 unexecuted cases plus a module-free second-kernel retry. Original records are retained and counted as infrastructure outcomes; the failed boot is not an Alibi invocation. This reconciliation does not change the predeclared semantic oracle.

The eventual results matrix must report correct CLEAR, incorrect CLEAR, supported and unsupported positive attribution, justified and unnecessary UNKNOWN, receipt integrity, execution failures and unestablished conditions. Useful-exoneration eligibility is independently established from actual sufficient evidence. Repeated healthy baselines and supplemental controls cannot inflate the count of independent representative conditions.
