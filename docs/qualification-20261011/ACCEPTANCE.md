# Independent acceptance review

Reviewer and decision owner: `acceptance_review`, independent of implementation and oracle design. Review date: 2026-10-11. Scope: candidate source `0427a7c0a81bb0b8212cc2e9a093453354121119`, its route-mask change relative to `6f8ad5fba4ed6f3fb9f32c9930377a0a48e06931`, preserved local reproductions, command-boundary trials, and campaign disposition. No hidden holdouts were opened. This reviewer did not execute the tests, rebuild the binaries, or run a guest; the test statements below report inspection of producer evidence.

**Disposition: accept the narrow parser repair and the recommendation to HOLD release qualification. No Linux guest/environment set is qualified by this review.** Unresolved demonstrated collector/parser defects support `HELD_DEFECT`; missing guest and holdout evidence independently prevents a positive release decision.

## Evidence identity and verification

Evidence root: `/data/git/alibi-qualification-evidence-20261011`. SHA-256 identities:

| Object relative to evidence root | SHA-256 |
|---|---|
| `alibi-baseline` | `6e05e61cde2c092148bf2788f3dd9560a5d5dc05c83f7685fc5020d1bf1bc859` |
| `alibi-candidate` | `503a8f3e283b4a1d843bb2e643a8d43b5f7074516cd4a5b37a3bdc5b681fc3f3` |
| `local-boundary/baseline/COMPLETE.json` | `a5184f29c79dd8f5df05e4b39efdaaf95be4c6c61fd31110f767cbec33fe960e` |
| `local-boundary/candidate/COMPLETE.json` | `a5c48791be169c0e7c465d3157652f7ec5f6203a10caaec781b657d21e1b3b0b` |
| `local-reproducers-baseline/RECEIPT.json` | `58b96ad800fe33100cdc4a6e12f1848d7523a1975ba62022e40f8085a3b1948b` |
| `local-reproducers-route-mask-repair/RECEIPT.json` | `fed86491b57037e1547b773498dd48ca18114a5ff9c1f3979f9943fd14cecb2c` |
| `route-mask-repair-checks/RECEIPT.json` | `c19e0388a1e7906a6468e06a499dbca722c18dd0273fe5000ed2c2d1aacdd0e1` |
| `local-evidence-spec.json` | `5d473c4ce919ca9e91fdfb3dd89164e4f2a046fe86b58c5753a533e40aeb5e87` |

Recomputed all 124 stdout/stderr hashes across the four local-result receipts, both executable hashes, and the local runner/spec hashes; all matched. Independently inspected `ORACLE-REVIEW.md`, SHA-256 `8fa0d560d97a537bface6c1b43a6f9aa0772656dc0eb4c29695decbfc838a1f9`. Hash agreement establishes artifact consistency, not an independent rebuild or proof that a source revision produced those binary bytes. Source-to-binary correspondence rests on the campaign's recorded controlled build and custody.

The baseline test log reports 102 unit and 2 integration tests passing. The candidate log reports 104 unit and 2 integration tests passing. Candidate format, test, clippy and release commands have exit-zero records. No schema or golden-receipt change appears in the product diff.

## Accepted conclusions and challenges

The route repair requires a zero mask as well as a zero destination, and rejects invalid mask width/hex. A legal zero-address `/8` or `/32` is independently not `/0`; this oracle does not merely restate implementation behavior. Original helper results are true for both narrow prefixes; candidate results are false, with a genuine default still true. This is sufficient to accept the bounded parser correction. It does not validate policy routing, all route flags, temporal consistency, or actual guest route collection.

The eight predeclared CLI cases ran once against each of two binaries. They establish eight distinct command-boundary scenarios, not sixteen independent real-host faults. LE01/02/03/05 correctly produce UNKNOWN with inadequate required sources and null MTTI. LE07 correctly identifies the journal fallback and clears its admitted formatting control. The runner only automatically compares allowed verdicts; this reviewer additionally inspected receipt source limitations and fallback identity rather than treating `allowed_verdict=true` as complete acceptance.

LE04 and LE08 are fixture-scoped false CLEAR/evidence/receipt failures under the declared oracle: invalid textual bytes or known non-log output become complete negative evidence and innocence. They do not establish a genuine kernel emitting those malformed representations. LE08 depends on external knowledge of the fixture; arbitrary message bodies cannot be authenticated by a timestamp prefix. These findings justify stricter acquisition contracts, not a universal text-authentication claim.

LE06 establishes a command deadline failure: collection succeeds after approximately four seconds although the declared deadline is three seconds. Its allowed-verdict mismatch must not automatically count as semantic false CLEAR: eventually collected benign content can remain sufficient for a bounded negative result. The original 100 ms helper also took approximately one second. These are F4 observations, not proof of infinite hang, a stalled physical device, or receipt timestamp falsification. The CLI receipt actually records approximately 4001 ms.

Impossible df capacity, unrecognized mount access mode and the caught UTF-8 slicing panic remain concrete parser-boundary failures. Full CLI filesystem false CLEAR and normal-kernel production of those malformed inputs were not demonstrated. EDAC overflow, broad pattern attribution, interface-name association and unbounded memory growth remain source-review hypotheses; they must not enter observed-failure counts.

The initial LE06 direct probes reported surviving process-group identities. Root subsequently recorded both exact groups with no members in `local-boundary/PROCESS-RECONCILIATION.json`. This resolves the observed cleanup question; initial EOF alone would not have resolved it. The source helper was retired, so helper replay requires rebuilding it from retained source and the matching library. Equal malformed-log and healthy-control receipt hashes are possible because receipts contain line counts and summaries rather than raw log bytes; fixture/input hashes remain necessary for replay identity.

## Qualification limits and decision

No disposable Linux guest fault experiment or final holdout ran. The unreadable kernel and unsuitable configured storage are concrete laboratory blockers, not PASS results. Two supported Linux environments, compound kernel faults, temporal host transitions, resource behavior and hardware coverage remain unqualified. Read-only ordinary host controls and command substitutions do not substitute for them. A prepared guest harness is not a working guest laboratory until boot and independent fault/oracle evidence exist.

Useful exoneration is established only for the narrowly declared formatting/fallback control (one eligible case per binary). Its denominator cannot become all eight cases, the healthy development host, or a claimed Linux population. Real-host useful-exoneration rate and false-CLEAR incidence are unmeasured. No observed unsupported positive attribution or host mutation in these runs establishes neither their universal absence nor a passed release gate.

The reported regression success and one accepted repair do not discharge unresolved F2/F4 failures, fixture-scoped F0/F3 observations, or F7 guest/holdout gaps. A future qualification must preserve the original failures, freeze any successor repair, rerun affected real-kernel cases, and execute still-unseen holdouts under a separately admitted environment. This record accepts evidence-backed HOLD, not completion of the originally requested real-host target.

## Formalization and custody

Propositions reviewed: zero-address non-/0 routes cannot satisfy default-route existence; inadequate required evidence cannot justify CLEAR; a collector deadline covers output completion. Exact source and result identities are above. Counterexamples are bounded to the named inputs and local command execution; no model establishes kernel correspondence or arbitrary scheduler behavior. Decision owner `acceptance_review` considers direct boundary examples and source inspection sufficient for this repair/HOLD decision. A future acquisition-state model could help with a concurrency repair but is not required or implemented here.

Only this small acceptance record and a lane completion notice are created by this review. Campaign root owns custody through release review, next review 2026-10-18; retain minimal source/result identities for that dependency. No build outputs, VM images, credentials or temporary substrate were created. Lane completion and this acceptance are separate records: the former announces readiness, the latter records this review's bounded decision.

## Final disposition-document review

Reviewed the following draft bytes at 2026-10-11T16:57:49.846227+00:00. Their counts, classification limits and HELD_DEFECT recommendation agree with the evidence reviewed above; no semantic correction is required. HARNESS and storage closeout are root-owned operational records outside this acceptance scope. Later edits to these documents require a separate review if they change the accepted claims.

- `RESULTS.md`: SHA-256 `8dd05645c5ade7b0e0d3c6547d0e5b58f1601ee519f77b3ef0bb53e22f793a8d`.
- `FINDINGS.md`: SHA-256 `5975f1c34d3656849f6373333000cbfb88f88d29abf683796520e3b9690f23f8`.
- `QUALIFICATION.md`: SHA-256 `22e744a58ddd79e72c22aed0b801c829949bdd6c1d9bb5d9ebf1d74c36180331`.
- `RELEASE-NOTES-DRAFT.md`: SHA-256 `50f8385c45d8ce6e81058b74f38d429d46bb9e3f35d8f7d1bde1ba1dd288257e`.

Matching retained baseline and candidate library hashes were also independently recomputed against their reproducer receipts. The source helper hash matches both reproducer identities. This verifies retained replay inputs; it is not a fresh execution.
