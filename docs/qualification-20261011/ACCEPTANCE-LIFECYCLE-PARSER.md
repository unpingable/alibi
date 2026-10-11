# Independent lifecycle and parser repair acceptance

Reviewer/decision owner: `acceptance_review`, 2026-10-11. Source reviewed: `2ab19ade70a0e2ecb37cf90f115e1cd79e41d992`, relative to acquisition candidate `f908852c7c8e85bbf94ba959f5af855d65bad4e5`. Frozen executable SHA-256: `c84a5ab3a8492d345cec534f15ed43069f8f9ed5f398d54e6a071a4116352f0e`; retained library SHA-256: `269695775607ec16fba366ef2ff18d949202ce3d6be14572f6e1eecc91883796`.

**Accept the bounded lifecycle and three parser repairs for their documented local scope. This is not overall release or real-host matrix acceptance.** Corrected guest matrix, exact-successor guest repeats and final holdouts are outside this review. Earlier acceptance records remain unchanged. This reviewer made no product edits, built no binaries, executed no guest and inspected no hidden holdouts.

## Evidence inspected

Evidence root: `/data/git/alibi-qualification-evidence-20261011/guest-lab`. Recomputed binary, library, all four source and all four receipt hashes named by `candidate-lifecycle-parser.receipt.json`; all match. Recomputed all thirty stdout/stderr hashes across the following local result sets; all match. Read the actual result content and source/test correspondence, not just producer success flags.

| Evidence | SHA-256 |
|---|---|
| `lifecycle-parser-final-checks/COMPLETE.json` | `6f8a8cebea322fac2174c0cf3b96fcb5969cbe0a61ce5c525ab748697acbc27a` |
| `acquisition-combined-final/RECEIPT.json` | `7c1f59505754144894dfb89c01109005a85c827820459ad91e341502a7700a5b` |
| `sigchld-combined-final/RECEIPT.json` | `a8cd99e3623ea2c93ee5fe4066a6a9e313762c40febaa38332bd5221d382940e` |
| `lifecycle-parser-helper-final/RECEIPT.json` | `2d9ccca6a2e48f548e7bf8e7598d7f382d4b8a933a1982d7df38219e129d793e` |
| `sigchld-abi/RECEIPT.json` | `1ddc1ff9f02e2bedf054959f729b69e2ce813a32eaf030c8663350d015d331eb` |

The full test log records 113 unit plus 2 Receipt V2 integration tests passing. Focused disposition, fmt, clippy with warnings denied and release build records exit zero. These are inspected producer records, not reviewer executions. Hash correspondence establishes preserved identities, not independent reproducible-build proof.

## Lifecycle decision

The prior ignored-SIGCHLD counterexample established that successful exit can remove the child identity before the collector waits. This successor queries its own disposition without changing it and refuses command collection before spawning unless the handler is default and SA_NOCLDWAIT is absent. Other target ABIs refuse collection rather than guessing. An observed wait error during acquisition now returns directly with explicit unconfirmed ownership, bypassing numeric group signaling. This addresses the demonstrated inherited-disposition path and the identified unsafe ordering after a wait error.

The external CLI default control returns CLEAR and creates the fixture command marker. The ignored-disposition launcher returns UNKNOWN and leaves the marker absent. Its receipt explicitly states unsupported SIGCHLD disposition and command not spawned. The focused unit test runs the SIG_IGN and SA_NOCLDWAIT controls in separate single-test subprocesses, avoiding process-global mutation in the parallel test runner. Both pass. No PID-reuse or unrelated-group signal trial is needed to establish pre-spawn refusal; none was performed or claimed.

The C ABI probe's source hash matches its receipt. Its successful static assertions/output independently corroborate installed glibc x86_64 struct size 152, mask offset 8, flags offset 136, restorer offset 144, SIGCHLD 17 and SA_NOCLDWAIT 2. The Rust repr(C) field layout corresponds on this target. This evidence supports the named target and installed headers, not arbitrary Linux architectures/libcs. The narrower target support is explicitly disclosed in design/envelope documents; it must remain visible in any release envelope.

The remaining identity invariant is conditional: library callers cannot concurrently reap collector children or change SIGCHLD disposition. Under the stated default/no-auto-reap/stable-disposition/exclusive-reaper assumptions, the unreaped leader pins the numeric group identity until the failure signal. Preflight alone cannot establish those conditions for arbitrary concurrent library callers. A cleanup poll can encounter an error after a signal already issued; the change prevents signaling after an acquisition wait error, not retroactive cancellation of an earlier valid signal. No universal descendant retirement, scheduler bound or arbitrary embedding guarantee is accepted.

The nine acquisition regressions remain intact on this exact executable: malformed/invalid/truncated/capped inputs and retained stdout/stderr produce UNKNOWN; both structured formatting controls CLEAR; retained positive with malformed neighbor DEGRADED and partial evidence. This shows the lifecycle refusal does not force ordinary default-disposition acquisition to UNKNOWN in these controls. The earlier two-kernel evidence belongs to ca2a, not automatically to this c84a successor.

## Parser decision

The source rejects df available bytes or free inodes greater than their corresponding totals. It does not invent a Used+Available=Total invariant, change reserved-space semantics or require positive totals. Existing independent-dimension handling is preserved. Original total 100/free 101 now returns an error rather than accepted negative evidence; a collector-level regression verifies malformed/absent source value when both dimensions fail.

Mount parsing requires one coherent access mode from ro/rw, rather than defaulting missing mode to writable. Both absent and contradictory modes are rejected. The octal decoder works on bounded byte slices, rejects incomplete/non-octal/out-of-byte-range/NUL encodings, and performs strict decoded UTF-8 conversion. This eliminates the original character-boundary slice panic while preserving a valid Unicode-plus-octal-space control. Existing public `unescape` remains total with literal fallback, but the acquisition parser calls the checked function and cannot silently use that fallback as observed mount identity.

The preserved original helper inputs now show errors for contradictory capacity and invalid mount options, and `panic=false` for the non-ASCII escape. The source/tests independently corroborate that the latter is rejection, not silent acceptance. Tests exercise byte and inode bounds, contradictory modes, malformed escapes and valid Unicode. These establish the demonstrated parser repairs; they do not establish full df/procfs grammar, mount generation identity, complete filesystem support or normal-kernel emission of malformed rows. Legal non-UTF-8 pathnames are explicitly unsupported and become UNKNOWN, not lossy substituted identities.

## Limits, formalization and custody

No new accusation, receipt schema, external crate or remediation behavior is introduced. Original counterexamples remain preserved. The repaired propositions are bounded: unsupported child-reaping state refuses before spawn; acquisition wait ownership error cannot initiate numeric group signaling; the tested malformed capacity/access-mode/escape classes cannot become complete negative evidence or the original panic.

Decision owner `acceptance_review` accepts source correspondence, independent original counterexamples, finite state reasoning and regression evidence for this local repair scope. The lifecycle argument depends on the explicitly named OS/libc and no-concurrent-reaper assumptions; no model of arbitrary scheduler behavior, signal mutation or mount replacement is claimed. A separate formal model would not establish those external correspondence assumptions, and is not required for this bounded decision. Remaining whole-campaign acceptance must independently adjudicate actual guest fault evidence, usefulness, authority limits and holdouts.

Only this small review and a new immutable lane notice are created here. Campaign root owns custody through successor qualification/release review and the 2026-10-18 evidence review event. No VM, compiler output, credential or temporary test substrate was created by this reviewer. Lane notice announces completion; this separate record states the limited acceptance.
