# Post-holdout execution results

R is `/data/git/alibi-qualification-evidence-20261011/guest-lab/requalification-20261011`. Historical results are not overwritten. Independent source acceptance, disclosed guest acceptance and final acceptance are separate records.

## Before repair and disclosed acceptance

Three old-binary guest runs reproduced H04/H05/H06. Four additional old-binary churn runs established another same-name/ifindex/MAC reuse counterexample and three controls. The corrected local journal-continuation regression exposed a false CLEAR in the first repair candidate; the invalid initial fixture is retained separately. These are repair evidence, not fresh acceptance successes.

The repaired disclosed sequence completed 51 planned guest cases. It used 52 actual boots because a Linux 6.8 archive initially rejected a mismatched module declaration before any Alibi invocation. An additional G09 pathname rejection occurred before QEMU launched. Both infrastructure exceptions are preserved in the reconciled chain, and neither is scored as a diagnostic pass. The successor executed only the 38 unexecuted cases and the separately identified second-kernel retry.

Independent acceptance verified 52 terminal/start/serial/stderr chains, 745 captured-file hashes, nine actual archived executables, 122 JSON receipts and one default-human receipt. See [ACCEPTANCE-DISCLOSED.md](ACCEPTANCE-DISCLOSED.md) and R/DISCLOSED-REVIEW.json.

| Disclosed representative outcome | Count |
|---|---:|
| Findings | 56 |
| Correct bounded CLEAR | 18 |
| Supported DEGRADED | 17 |
| UNKNOWN emitted | 20 |
| NOT_APPLICABLE | 1 |
| Independently eligible useful exonerations | 18/18 |
| False CLEAR | 0/54 evaluated applicable findings; 0/18 CLEAR |
| Unsupported positive attribution | 0 |
| Evidence/receipt failures | 0 |
| Product execution failures | 0 |

One UNKNOWN belongs to the intended-condition gap H03: all 48 independent drop samples were zero. Its actual incomplete evidence justifies uncertainty, but the intended positive-preservation experiment remains F7. The other 19 representative UNKNOWN findings have established inadequate evidence. G30 and G36 remain unexecuted gaps. No unexplained UNKNOWN occurred in an independently eligible representative CLEAR opportunity.

Repeated baselines and supplementary sequences are retained separately: all 122 JSON invocations contain 249 findings, including 156 useful CLEARs, 22 supported DEGRADED, 69 UNKNOWN and two NA. The 156/156 repeated-control denominator does not replace the 18/18 representative denominator. The separate human invocation is not counted as another independent condition.

H05 and C01 now return UNKNOWN with ENODEV from retained old counter descriptors during independently timed replacement. C03 replacement before collection and C04 same-object namespace movement retain useful CLEAR. H04 no longer attributes a memory error to benign registration text. H06 preserves supported route degradation while default human output exposes the denied required kernel log and its acquisition details. These conclusions are finite observed correspondence, not a proof over every Linux schedule.

## Measurement scope

R/MEASUREMENTS-DISCLOSED.json covers the seven old-binary baseline/churn boots and 52 repaired disclosed boots: 59 actual boots and 142 CLI invocations. The repaired candidate accounts for 123 invocations. Main-kernel wall time is median 2250 ms, p95 2490 ms, maximum 3200 ms; peak reported CLI RSS is 2560 KiB. The second-kernel acquisition subset has seven invocations, median 250 ms and maximum 3180 ms. Selected command deadlines intentionally contribute the slow tail. Mixed workloads and TCG scheduling do not establish a statistical performance regression or improvement.

The repaired disclosed JSON interrogation timer has minimum 66 ms, median 2104 ms, nearest-rank p95 2250 ms and maximum 3052 ms. Wall time includes process startup/output overhead; the receipt's timer has its documented narrower scope. No disclosed Alibi invocation crashed, hung, exceeded the outer timeout or lacked required captured output and status.

## Fresh acceptance boundary

R/FINAL-FREEZE.json binds source `7e04d3cb076a48f2e131d6284a16f6bbb836ff58`, executable SHA-256 `0760ea3529ac3c3d831038a71c46f82b6bd3d750fd4dbc52866399962e593b06`, passing automated checks and independent acceptance before the ten new cases were disclosed to root. The implementation agent remains excluded. Fresh outcomes and final measurements are appended only after the complete new producer chain is sealed and independently evaluated. No product tuning against those outcomes is permitted within this qualification.

## Fresh results

The ten fresh cases completed once, followed by separate author-oracle and skeptical acceptance reviews. [FRESH-ORACLE.md](FRESH-ORACLE.md) records the case-by-case evidence. Twenty JSON and six default-human invocations were captured; six paired human outputs retained the material canonical source envelope and per-accusation distinctions.

| Fresh primary outcome | Count |
|---|---:|
| Selected findings | 18 |
| Correct bounded CLEAR | 8 |
| Supported DEGRADED | 5 |
| Justified UNKNOWN | 5 |
| Real-required-source useful exoneration | 7/7 |
| Substituted-log useful exoneration | 1/1, separately scoped |
| False CLEAR | 0/18 evaluated applicable findings; 0/8 CLEAR |
| Unsupported attribution, product receipt or execution failures | 0 |
| Full case oracles established | 9 |
| Intended subclaims not established | Q04 malformed-ifindex identity-inadequacy requirement |

Q04's actual missing-route positive is independently supported. Its sealed expectation that malformed ifindex text must make net evidence partial was an oracle overconstraint: the frozen implementation establishes identity through retained kernel-object descriptors, not that unused textual attribute. Both reviewers independently preserve the expected-versus-observed mismatch as F7 for that intended subclaim. Neither the original expectation nor the observed receipt was changed, and Q04 is not advertised as a full case pass.

Real state transitions in Q02/Q03, actual TUN receive drops in Q09 and the explicit counter-history substitution in Q10 all have independently established overlap with the conservative common sample interval. Benign ECC configuration in Q07 is explicitly substituted evidence, not an actual EDAC event. Q08's kernel CLEAR uses genuine log evidence despite an unrelated df failure fixture. These distinctions determine eligibility, not the presence of the word CLEAR.

Ten per-boot healthy controls contribute 40 findings (30 CLEAR, ten UNKNOWN), separately from the 18 primary findings. Six human pairs contribute 13 selected findings and are not additional independent conditions. Most fresh conditions have one trial; no general nondeterminism rate is claimed.

## Final execution and measurement reconciliation

The complete successor contains **69 actual guest boots and 168 Alibi invocations**: seven old-binary counterexample/control boots; 52 disclosed repaired boots including one pre-scenario infrastructure failure; ten fresh boots. The original G09 refusal occurred before boot and is reported separately. The repaired candidate has 149 CLI invocations (142 JSON, seven human); the old binary has 19 (18 JSON, one human).

R/MEASUREMENTS-FINAL.json initially omitted the wrapped Q04-human invocation because argv contained unshare/sh rather than an alibi token. R/MEASUREMENT-CORRECTION.json preserves the mistake and binds the explicit verified inclusion in R/MEASUREMENTS-FINAL-V2.json: 26 fresh invocations and 168 overall. No case was rerun and no output changed.

| Executable / guest kernel | CLI invocations | Wall median / p95 / maximum | Maximum CLI RSS |
|---|---:|---|---:|
| Old binary, 6.5 | 19 | 2230 / 2400 / 2400 ms | 2560 KiB |
| Frozen repair, 6.5 | 142 | 2275 / 2430 / 3200 ms | 2560 KiB |
| Frozen repair, 6.8 acquisition subset | 7 | 250 / 3180 / 3180 ms | 2432 KiB |

The differing workload mixtures preclude a controlled regression comparison. No material resource increase is apparent in these reported CLI peaks, but the campaign does not establish a performance bound. Recognized human output is not JSON and remains explicitly classified as such by the measurement helper; it is not a missing-receipt failure. User-visible diagnostic exit statuses 1 and 2 represent DEGRADED/UNKNOWN outcomes, not crashes.

See [QUALIFICATION.md](QUALIFICATION.md) for the conservative release scope and [STORAGE-CLOSEOUT.md](STORAGE-CLOSEOUT.md) for artifact disposition. No fresh-case outcome was used for implementation tuning.
