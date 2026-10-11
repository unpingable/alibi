# Waitable child lifecycle repair

Owner: implementation_review. Independent acceptance identified a missing precondition in frozen acquisition revision `f908852c7c8e85bbf94ba959f5af855d65bad4e5`, executable SHA-256 `ca2a6b9ef258e3643d0feaabc46c530908699db28843f823f3207077d85245b1`. That executable and its receipts remain unchanged.

## Preserved negative control

A disposable launcher sets SIGCHLD to SIG_IGN, then execs the frozen CLI. No PID-reuse scenario or unrelated process signaling was attempted. The same structured-log fixture with default disposition returns CLEAR; ignored disposition returns UNKNOWN with ECHILD from both command waits and cleanup. The source path then attempts numeric process-group signaling despite already losing waitable-leader evidence. This establishes the failed ownership precondition, not a demonstrated signal delivered to an unrelated process or F5 result.

Evidence: `guest-lab/sigchld-baseline-direct/RECEIPT.json`. An earlier strace attempt was refused by the ptrace permission boundary and is preserved under `sigchld-baseline-negative`; it did not produce a syscall trace. Group-signal ordering is therefore established from source inspection, not claimed as a traced observation.

## Bounded repair

Before creating a command, the collector performs a read-only sigaction query. It requires the default SIGCHLD handler and absence of SA_NOCLDWAIT. Ignored/custom handlers, auto-reaping, query failure, or an unsupported ABI refuse collection before spawn. No global signal disposition is normalized or changed. The ABI is explicitly qualified for Linux x86_64 GNU only; other targets fail closed rather than guessing a C layout.

Any try_wait error returns an ownership-unconfirmed error without numeric group signaling. Successful try_wait reaping is never followed by group cleanup. Ordinary failure cleanup still signals the dedicated group before its leader is reaped. The standalone CLI has no concurrent child reaper or signal mutation. Library callers must exclusively own collector-child reaping and keep SIGCHLD disposition stable throughout collection; a read-only preflight cannot prove these properties of arbitrary concurrent callers.

The chosen ABI was independently checked against the installed GNU C headers with compile-time C assertions: sigaction size 152, mask offset 8, flags offset 136, restorer offset 144, SIGCHLD 17 and SA_NOCLDWAIT 2. Source, compiler/run statuses and output are under `guest-lab/sigchld-abi`; its reproducible executable was retired after completion.

## Validation and source integration

The new focused test creates isolated single-test subprocesses for SIG_IGN and SA_NOCLDWAIT, then verifies refusal before even attempting a deliberately nonexistent program. It never changes signal handling in the parallel test runner. The external CLI replay script `campaign/sigchld-reproducers.py` independently records whether command fixtures executed: default control CLEAR with marker present; ignored disposition UNKNOWN with marker absent. Exact final output is under `guest-lab/sigchld-combined-final`.

Combined checks also include root-owned bounded parser repairs in mounts.rs and fsusage.rs, described in PARSER-REPAIR.md. All 113 unit tests, 2 Receipt V2 integration tests, fmt, clippy and release build pass. Checks ran as durable user service `alibi-lifecycle-parser-9c2d46b1-9113-4df8-951c-5eb9b0aade66`, invocation `1a611e951cb5416e87f6a56d078699db`, independent of the interactive supervisor. Recovery/source checkpoint, per-stage logs and terminal COMPLETE.json are under `guest-lab/lifecycle-parser-final-checks`. Earlier failed/provisional attempts remain separate; the first focused regression caught an omitted preflight call during editing, which was corrected before final verification.

The unchanged original helper was linked to the exact final release library: excessive df capacity now errors, missing mount access mode now errors, malformed non-ASCII escape no longer panics, narrow zero-prefix routes remain non-default, and a 100 ms inherited-pipe test returns timeout at approximately 100 ms. Evidence is under `guest-lab/lifecycle-parser-helper-final`; its helper executable was retired after the terminal receipt. The nine supported-CLI acquisition controls are replayed separately under `guest-lab/acquisition-combined-final`. These local fixtures do not establish guest-kernel behavior or close final holdouts.

## Formalization consideration

Proposition: numeric group cleanup is authorized only while the originally spawned leader remains waitable and unreaped. The earlier acquisition state table omitted automatic reaping as an alternate transition history. This revision adds an admission predicate (qualified ABI, default disposition, no SA_NOCLDWAIT) and a refusal transition for any wait ownership error. Under stable disposition and exclusive child-reaping assumptions, exit leaves the leader waitable until this collector reaps it; group signaling precedes that reap on cleanup paths. Checking and reaping the leader before group signaling would destroy the needed identity pin and is deliberately avoided.

This is a bounded lifecycle argument, not a proof over arbitrary concurrent library callers. The focused subprocess controls and external spawn markers demonstrate refusal for the two known auto-reap conditions; C ABI checks support correspondence on the qualified target. The model excludes external reapers, concurrent disposition changes, descendants leaving the group, scheduler guarantees, and exact wall-clock return deadlines. Decision owner implementation_review; independent acceptance evaluates the final source/receipt identities. No new execution platform or signal-management service is introduced.

## Custody

Root owns the original frozen executable, new final candidate and minimal logs/source receipts for pending independent acceptance and guest repeats. Compiler output remains under the admitted 200 MiB target ceiling and is reproducible substrate until replay dependencies end. Temporary C and Rust helper executables have been retired; script fixtures retain source without executable permission after use. Review remaining custody at candidate acceptance/campaign closeout. No hidden holdout was inspected.
