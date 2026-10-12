# Temporal evidence investigation — 2026-10-12

Investigation requested in [#18](https://github.com/unpingable/alibi/issues/18).
**Product inspected: `5aef9cd85486cb15b7d044ab44acc08c9aebeef2`.** Harness execution
source: `2af0c20` (full revision in results/MANIFEST.json). No files under `src/`,
Cargo metadata, golden receipts or verdict rules were changed. The external
assessment supplied no exact revision. Its 104-test count corresponds to the
historical next-stage report; its precise reviewed tree cannot be established.

Baseline checks actually executed: 131 unit tests, 1 CLI test, 3 receipt tests,
formatting, Clippy with warnings denied and release build all passed. Twelve
fixture receipts and an ordering trace were produced. This is deterministic
characterization, **not new Linux guest qualification or human usability evidence**.
The historical `QUALIFIED_WITH_LIMITATIONS` disposition remains historical and
unchanged; it does not qualify current changed-source binaries.

## Six findings reconciled

| Finding | Current evidence and disposition |
| --- | --- |
| F1, lost history | Real observational limitation, already documented. `src/collect/kernellog.rs` requests current-boot journal records and validates framed, newline-terminated records; arbitrary nonempty output no longer becomes complete. Empty/unusable output falls back or becomes inadequate. One valid surviving record can still yield complete acquisition. `docs/envelope.md` explicitly distinguishes acquisition from complete boot history. `src/receipt/human.rs` now prints the source envelope and CLEAR boundaries. The alleged missing human envelope is stale; absence of historical coverage remains real. |
| F2, mixed horizons | Confirmed. `src/eval/network_choke_point.rs` counts matching link-down lines across acquired retained logs without an age condition; `silent_killer.rs` likewise accepts retained filesystem error patterns alongside current mount/capacity observations. Five old and five recent link events both produce DEGRADED. The network flap count omits individual timestamps, unlike many direct pattern indications. The wording does not clearly say whether the condition remains active. |
| F3, different evidence bases | Useful distinction, not a proof that one entire accusation is strong and another weak. Current per-accusation CLEAR boundaries help, but state, cumulative counters, sampled deltas and retained events have different temporal meanings. Mount/capacity state does not establish all storage functionality; counters depend on scope/reset/lifetime and logs remain required for several accusations. No universal state-versus-event confidence ranking is justified. |
| F4, VM aggregate | Overstated. Missing required EDAC prevents memrot CLEAR without a positive indication. It prevents all-selected innocence/MTTI, but selecting kernel-scream can still CLEAR. DEGRADED outranks UNKNOWN in an aggregate. Current docs/help already describe selected findings and exit precedence; default UNKNOWN explanations distinguish unsupported/unavailable/invalid/incomplete. Unsupported does not prove permanent impossibility: driver/environment changes may change observability. |
| F5, recall | No broad watchdog claim remains in current README. Hung-task text alone is not recognized by the current table, as the fixture demonstrates. This is a documented-scope/discovery question, not authorization to add that family. Curated positives/negative controls and historical qualification do not measure recall against an independent real-log population. No comprehensive recall claim is justified. |
| F6, read ordering | Confirmed schedule seam. Kernel-log collection precedes network sampling. A deterministic substitute emits five events during the sample; the earlier snapshot contains none and the evaluator clears. A later-log control degrades. This is compatible with the current stated non-atomic, acquired-evidence boundary; it cannot establish no flaps throughout the network sample. It is not by itself a demonstrated violation of the narrower current CLEAR contract. |

Source locations are relative to the pinned product revision, not floating main.
The collector's numeric timestamp framing validates representation; it does not
parse event time into a coverage model or authenticate source contents. The trust
assumption is also already stated in the envelope, including substituted commands
and coherent kernel views. The external review's claim that it was wholly unstated
is stale.

## Reproduction and independent ground truth

Run from this investigation checkout with an admitted bulk target directory:

```sh
CARGO_TARGET_DIR=/your/admitted/bulk/target \
  python3 scripts/investigations/temporal-20261012/run.py /your/new/results-directory
```

Requires the existing Rust toolchain and Python 3. The output directory must not
exist. The driver constructs explicit facts independently of production fixture
helpers and invokes the unchanged public evaluators and receipt renderers. It also
extracts **verbatim** `collect_facts_for` from the inspected source, compiles that
function with deterministic collector substitutes and records the actual call
order. Substitutes do not access host kernel interfaces, execute commands or sleep.
This tests the real scheduling function against a declared event schedule; it does
not test actual carrier behavior or kernel log publication delay. Timestamp offsets
inside the substituted schedule are fixture values, not measured wall-clock latency.

The fixture oracle is the explicit input/event schedule. Expected outputs are
characterization assertions, not acceptance of desirable product semantics.
Changing them would require revisiting the investigation, not silently weakening a
regression. The real library is compiled independently of the substitutions.

| Case | Actual | What it establishes |
| --- | --- | --- |
| clean-network | CLEAR | Negative control with all required fixture observations |
| old-flaps-300-day-boot | DEGRADED | Five records near boot still count at day 300 |
| recent-flaps | DEGRADED | Moving only log timestamps near day 300 does not change classification |
| old-fs-error-current-rw | DEGRADED | Retained error survives current rw/capacity observations; no proof of ongoing malfunction |
| retained-log-clean | CLEAR | Negative retained-log control |
| unknown-retention | CLEAR | A valid late retained record supplies no proof of earlier coverage |
| retained-panic-control | DEGRADED | A present recognized panic is detected |
| hung-task-not-recognized | CLEAR | This synthetic signature is outside the current pattern table |
| no-edac | UNKNOWN | Missing required EDAC does not clear memrot |
| selected-kernel-no-edac | CLEAR | Unselected EDAC does not block kernel-scream |
| flaps-after-log-snapshot | CLEAR | Pre-sample log misses declared during-sample events |
| same-fixture-log-after-sample-control | DEGRADED | Identical net facts plus post-sample log expose those events |

Additional aggregate assertions verify UNKNOWN without positives, and DEGRADED
precedence. No hidden panic/reboot, real memory error or real network fault was
induced. In F1 two histories—no earlier error versus an earlier error omitted by
retention—can supply the **same** retained input. Earliest and latest timestamps
cannot distinguish those histories. Even a boot-time first record does not prove
there are no internal gaps. Boot time, read completion time, oldest retained event
and full historical coverage are four distinct concepts.

## Receipt V2: what existing fields can and cannot say

| Existing field | Sound use | Unsound reinterpretation |
| --- | --- | --- |
| Source status/completeness | Acquisition success and known loss | Proof of complete history or pattern recall |
| Source method/detail | Current-boot retained-log source; coverage unknown; explicit acquisition limitations | Inferring absent events were never emitted |
| Source completed_at / elapsed | When collection completed and acquisition duration | Timestamp of the last event or end of guaranteed historical coverage |
| Source window | Existing acquisition/sample interval, relative to interrogation start | Boot-relative retained-log event bounds silently put in the same field |
| Finding indications/evidence/skipped/summary | Label retained historical reports, current state and counter interval separately | Equating a historical indication with a condition still active now |
| Receipt boot_time/started_at | Boot and invocation context | Proof of event retention, clock correspondence or continuous observation |

V2's existing strings can communicate the important distinctions in both human
and JSON output without a schema change. A renderer-only warning would leave
machine consumers without the same explanation; canonical detail/finding strings
are the preferable location. A typed event-span, continuity or basis-class contract
would be a new machine-readable design decision, not something to smuggle into
`window`. Full raw negative logs are not retained in a receipt, so consumers cannot
in general derive observed event bounds later from the receipt alone.

## Smallest defensible proposals — not implemented

1. **Presentation first:** put an explicit retained-current-boot / historical
   coverage unknown explanation in canonical kernel-log detail; label
   `completeness` as acquisition completeness in human output. Say retained-log
   link events do not establish a current flap rate, and distinguish historical
   filesystem reports from present mount/capacity state. Align summary/explain
   wording with already-published CLEAR boundaries. Preserve indications, required
   sources, aggregation and V2 structure. Test semantic consistency of both
   renderers. This is a proposed bounded follow-up, pending investigation review.
2. **Optional observed timestamp bounds:** only if useful after (1), report oldest
   and newest *observed record* timestamps, with clock basis and unknown gaps.
   Do not call this coverage start/end. Parsing must handle journal continuations,
   missing/odd timestamps and clock assumptions conservatively. Do not make a new
   required source or change CLEAR based on those bounds without approval.
3. **Collection order:** consider log acquisition after network sampling only as
   an explicit evidence-acquisition change requiring operator approval in this
   investigation. It reduces this particular seam, but delayed log emission,
   post-read events, overwritten records and non-atomic state remain. Test both
   orderings, source completion times, other selected accusations and command
   deadlines; qualify real guest behavior before making a stronger coverage claim.
4. **Scope and recall:** keep pattern coverage curated and explicit. Gather
   authorized real incident examples through #3 before proposing additional
   signatures; record inclusion/exclusion rationale and independent corpus limits.
   Do not add hung-task detection just to satisfy this synthetic example.
5. **VM guidance:** a short all-selected versus selected-only example can explain
   aggregate UNKNOWN without treating absent EDAC as N/A or automatically advising
   additional privilege. Avoid claiming unsupported evidence is permanently absent.

Do **not** filter old errors automatically, add confidence scores, collect prior
boots by default, turn UNKNOWN into N/A, or expand diagnostic families. A recognized
old event can be consequential; age alone does not settle relevance. These are
proposals, not authorization to change verdict semantics or publish a release.

## Formalization consideration and custody

Owner: root investigation controller; independent reviewer records acceptance
separately. Proposition: identical retained evidence cannot establish which of
two different unobserved histories occurred; a pre-sample snapshot cannot contain
later fixture events. The explicit finite schedule and identical-input argument,
paired controls, source identity and actual outputs suffice for this bounded
investigation. They prove neither kernel timing guarantees nor comprehensive
fault recognition. No new formal model is needed to establish these limited
counterexamples; stronger continuity semantics would require a separately scoped
model and implementation correspondence.

`results/MANIFEST.json` binds the executed driver, extracted function, executable,
stdout and twelve pairs of receipts. Text/JSON results are tracked; the generated
executable and build outputs are disposable after review. Local custody is
`/data/git/alibi-temporal-evidence-20261012`, owner root; the checkpoint binds
source, durable execution, storage and retirement. Retain minimal inputs/results
for the #18 decision, reviewed at resolution or 2026-10-19. No SSH keys or VMs were
created. No new guest-qualified binary or release artifact was produced.
