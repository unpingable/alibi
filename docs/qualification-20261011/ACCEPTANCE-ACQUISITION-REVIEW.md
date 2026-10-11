# Interim independent acquisition review

Reviewer/decision owner: `acceptance_review`; date 2026-10-11. Scope: acquisition implementation `f908852c7c8e85bbf94ba959f5af855d65bad4e5`, frozen executable SHA-256 `ca2a6b9ef258e3643d0feaabc46c530908699db28843f823f3207077d85245b1`. This successor review preserves the earlier acceptance unchanged. It is not release acceptance, a completed matrix review, or a holdout evaluation. No hidden holdouts were read; no product code, builds or VM execution were performed by this reviewer.

## Source and local evidence

The source replaces blocking reader joins with two UnixStream pairs whose parent endpoints are nonblocking. Each loop reads at most one bounded chunk per stream, checks the acquisition deadline, and polls child status only after both streams reach EOF. Byte caps and strict stdout decoding prevent retained prefixes or invalid UTF-8 from becoming complete output. Mixed valid/malformed framed logs become partial evidence; retained recognized positives remain available. The local partial-positive receipt shows DEGRADED, malformed/partial source, null MTTI and the acquisition limitation.

Verified the frozen executable and all six source hashes in `guest-lab/candidate-acquisition.receipt.json`, plus its checks/local-replay hashes. All eighteen local stdout/stderr hashes match `guest-lab/acquisition-local-final/RECEIPT.json` (SHA-256 `d4d70339fa91a38555289e550bfc9d7851d879b21c51fa4e4f252369b4f5ccc2`). Its nine cases have the declared outcomes: two adequate formatting controls CLEAR, one retained partial positive DEGRADED, six inadequate acquisitions UNKNOWN. Inherited stdout/stderr cases finish in approximately 3014/3013 ms including fallback, with finite fixture children recorded absent. Other case runtimes range from approximately 13 to 35 ms. No outer timeout fired.

These results are implementation-authored regressions, not an unseen independent oracle or nine real kernel faults. They demonstrate the stated boundary behaviors without qualifying arbitrary malformed output or all scheduling histories. The 8 MiB external test covers stdout; the recorded unit tests exercise caps on both streams. The test log reports 109 unit plus 2 integration tests passing, and fmt/clippy/release exit-zero records exist. This reviewer inspected that evidence rather than rerunning the tests.

## Lifecycle assumptions requiring explicit reconciliation

Failure cleanup signals the dedicated group before reaping its leader and polls retirement for at most the declared userspace allowance. This is a reasonable ordering under the assumption that the leader remains waitable until this code reaps it. It is not a proof of universal descendant retirement: successful commands may leave descendants that have closed the streams, group-leaving descendants are excluded, and delayed kernel retirement is reported rather than synchronously awaited.

An outstanding source-level question is whether inherited `SIGCHLD=SIG_IGN` or an applicable automatic-reaping disposition can invalidate the leader-identity assumption. A launcher can supply ignored signal state; if the leader is automatically reaped, `try_wait` can fail with ECHILD and the current failure path still calls group signaling using its numeric identity. No unrelated group was observed signaled and no PID-reuse experiment was requested. The independent review asked root to establish the actual disposition guarantee or run a bounded negative control that demonstrates/refutes the unpinned path. This is an authority-boundary assumption gap pending adjudication, not a demonstrated F5.

The final elapsed check precedes UTF-8 conversion; the code does not prove that conversion completes before an exact wall-clock instant. Scheduling/spawn/syscall limits already exclude a universal elapsed-time guarantee. Documentation should describe the observed acquisition gate, bounded work and explicit cleanup allowance, without extending it into a strict real-time proof.

## Original guest evidence and correspondence identity

Verified all 55 capture hashes in `guest-lab/acquisition-baseline-batch/guest/captures/CAPTURES.json`, SHA-256 `1156658e1b3a0cbcc719b9046710a18755b1b7eb49c6d95ea011fb50e0538525`. The guest terminal SHA-256 is `fffd88f73a7332ebc74113100d57aada9fe684198963b28b3d0b71bcd5676e51`. Baseline guest receipts show healthy-kernel and healthy-filesystem CLEAR, absent-EDAC UNKNOWN, non-log and invalid-encoding fixture CLEAR, and inherited-pipe CLEAR at receipt latency 4034 ms. The direct encoding/non-log probes independently capture the substituted bytes. Running substitutions in a guest remains command-boundary evidence, not a genuine kernel emitting malformed bytes. Healthy-control labels alone are not independent proof of every source's adequacy.

The first genuine guest-log replay and genuine journal correspondence receipts record executable hash `636e5de1864a38f4746de43adca55ceadb2d3238c449ffc16740b19bf097eb7b`, which differs from the frozen binary. Their stdout hashes and the captured guest-log input hash match, but they establish correspondence for that earlier executable. Root was notified; preserved successor exact-binary evidence is required before assigning their result to the freeze. No historical receipt may be silently corrected.

The full baseline matrix is separately executing under root supervision. This review does not infer completion or successful induction from its launch. Candidate actual-guest reruns and a frozen-binary journal successor remain pending at this interim writing. Final release judgment must additionally reconcile other unresolved parser findings, independently adjudicated matrix outcomes, holdouts and storage closeout.

## Formalization consideration and custody

The source's finite acquisition table is useful for success gating, but its identity invariant depends on a waitable leader and its timing property depends on bounded userspace work rather than kernel scheduling guarantees. Exact source and result identities above bind this review. Concrete local counterexamples and source correspondence are sufficient for this interim decision; no model checker or proof of PID lifecycle is claimed. Decision owner `acceptance_review`; root owns remediation and subsequent acceptance scope.

Only this small review and its eventual immutable completion notice are created by this lane. Root owns custody for the current repair/release review; revisit at candidate freeze and campaign closeout. No compiler output, temporary guest substrate, credentials or large artifact is created here.

## Frozen-candidate correspondence follow-up

The exact-binary correspondence gap is now resolved by successor evidence, without altering the earlier receipts. `guest-lab/acquisition-real-journal-frozen/RECEIPT.json`, SHA-256 `236e07edf9b037eb75676b7e2d761e83a1d234a43e13d64bcaf247c396f749a4`, identifies the frozen ca2a executable. Its four raw/CLI stdout/stderr hashes were independently verified. The ordinary CLI returns CLEAR using the explicitly named journal fallback. The raw capture and CLI are sequential observations, not proof of identical journal snapshots or global host innocence.

Two completed candidate guests independently exercise genuine dmesg collection and the original bounded substitutions using that same executable. Build manifest SHA-256 `820116f8152ce677beae66104a28d85fb4a74a1a0806b0ad1c557fcf2641058a` names ca2a at `/bin/alibi`; its archive identity matches both guest starts. This reviewer verified each start/serial/stderr hash and all 110 capture hashes across the two runs.

| Completed run beneath `guest-lab/baseline-matrix-batch` | Kernel | TERMINAL.json SHA-256 | Capture manifest SHA-256 |
|---|---|---|---|
| acquisition-candidate | 6.5.0-44 | `2caf01dd34f0acee3d01e2dc1b872a2b8a6080114f64ab47bdd8477567d825dc` | `5ab71f0cd69581d244075aa86e41639c6d9c8ad13d2f55884f4b59ada79be969` |
| second-kernel-acquisition | 6.8.0-90 | `222ca483ec451865ac8cec30feceb00dc55cb71879bc16708f145cd1f9f149f3` | `c3d665a8efb21fe08cda4eccabf589b1513ce7076e72516c30e3efb001cd4997` |

Both show healthy-kernel/filesystem and restored-kernel CLEAR, absent EDAC UNKNOWN, known non-log and invalid-encoding substitutions UNKNOWN, and inherited-pipe UNKNOWN at receipt latencies 3014/3018 ms respectively. This supports candidate acquisition behavior on those two actual guest kernels. The common minimal userspace means two kernel environments, not two independently qualified distributions. The substitution cases remain substitution evidence even inside those guests. No conclusion about the concurrently executing full matrix or final holdouts follows.

Interim disposition remains pending only for the newly identified process-lifecycle assumption and later whole-campaign acceptance; the old correspondence-hash issue is resolved within this scope.

## Interim terminal decision: lifecycle successor required

The bounded ignored-SIGCHLD launcher reproduced loss of waitable-child identity on the exact ca2a executable. The default-disposition formatting control returns CLEAR; the ignored-disposition invocation returns UNKNOWN with ECHILD for both collectors, followed by cleanup-incomplete ECHILD. Both stdout hashes match the preserved receipt `guest-lab/sigchld-baseline-direct/RECEIPT.json` (SHA-256 `ab00953ce66f07ea3fc1b8c03658145cdc338626adec90a4716b024b91b0b402`). Its control flow enters numeric group signaling after the wait error. This ordering is source-supported; direct syscall tracing was unavailable and no unrelated group signal or PID reuse was demonstrated. Do not classify the result as a demonstrated F5.

The original acquisition defects are locally repaired and reproduced correctly in two actual guest kernels, but final acceptance of the new cleanup authority claim remains HELD pending a bounded lifecycle successor. Recommended direction communicated to root and implementation: refuse before spawn for unsupported automatic-reaping signal disposition, avoid numeric group signaling after any wait-status ownership error, and explicitly exclude concurrent external reapers or process-global disposition mutation from supported library use. Normalizing process-global signal state is not necessary to preserve the ordinary standalone CLI path. Root owns the implementation decision and subsequent qualification.

This interim review is complete for f908852/ca2a. Any successor source/binary needs separately identified checks and acceptance; its success must not be attributed retroactively to this freeze. The full matrix and hidden holdouts remain outside this record.
