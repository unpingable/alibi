# Bounded command acquisition repair proposal

Owner: implementation_review; scope authorized by campaign root after continuation admission. Status: implemented and locally checked after root preserved the original guest baseline. Source before this work is the frozen route-mask candidate; baseline failures remain in original custody. No hidden holdouts inspected.

## Required behavior

A successful command result requires direct-child successful exit, EOF on stdout and stderr, finite in-budget output, successful reads, and strict UTF-8 stdout before the acquisition deadline. Neither child exit without pipe EOF nor a valid prefix before truncation establishes complete evidence. Timeout/output cap/read/encoding failure becomes inadequate required evidence through existing collectors. Receipt V2 needs no schema change.

Replace blocking reader threads with two standard-library UnixStream pairs. Their parent ends are nonblocking; child ends become stdout/stderr descriptors. Drain bounded chunks from each stream per turn, checking the same monotonic deadline between chunks; cap each stream at 8 MiB. No read thread or join exists. Check direct-child status only after both streams reach EOF; until then retain the unreaped child identity. Success must be checked against the deadline after all observations, not before a potentially long read.

Use a dedicated command process group. On acquisition failure signal only that child-owned group before reaping its leader; the leader's unreaped identity prevents PID reuse from redirecting the group signal. Poll direct-child status during a separately declared bounded cleanup allowance (100 ms proposed), never a blocking wait. Closing parent streams prevents retained acquisition resources. If the kernel cannot retire/reap a child in the cleanup allowance, return an explicit cleanup-incomplete error; do not claim successful retirement. Descendants that deliberately leave the owned group are outside this cleanup guarantee. No host service or ambient process group is in scope.

This is a userspace acquisition deadline with bounded polling/cleanup, not a real-time scheduling guarantee. Spawn, system-call scheduling, and kernel uninterruptible waits cannot be made strictly wall-clock bounded by this code. Remove any literal universal hard-timeout claim inconsistent with these limits. There must be no detached stuck reader thread to accumulate across filesystem queries.

## Explicit kernel-log representation

Select dmesg `--raw --force-prefix`; the installed util-linux help supports those options. Do not use a nonexistent `--time-format=raw`. Require numeric priority and numeric relative bracket timestamp plus a message separator on each physical record (empty framed messages are legal). Journal fallback selects `-o short-monotonic --no-hostname --all` in addition to existing kernel/current-boot/no-pager/quiet flags; require bracket monotonic timestamp and identifier. Confirm both actual representations in the admitted guest before claiming support; unsupported commands/options yield fallback or UNKNOWN.

Successful command bytes must be valid UTF-8. Selected representations must end in a full newline-delimited record, not an unterminated fragment. Validate numeric prefix syntax, priority range, message separator, and forbidden framing control characters. Do not interpret arbitrary message bodies as authenticated kernel events. Valid records may be retained from mixed valid/malformed output with partial/malformed status, so positive evidence survives while CLEAR remains forbidden. If no valid records remain, retain failure detail and try fallback. A successful complete fallback can supply the bounded retained-log source; acquisition details must preserve known limitations where needed.

An output cap or transport failure currently returns only an error, discarding its untrusted prefix. This deliberately prevents false CLEAR but does not establish retained-positive semantics for transport failure. State that limitation. Representation validation cannot establish command provenance, freshness, complete since-boot history, or absence of overwritten records. Valid-looking substituted text can still fool it; that remains an explicit authority/evidence limitation.

## Predeclared regression categories

- Exit-success-before-stdout-EOF and before-stderr-EOF exceed neither collection deadline plus cleanup allowance nor outer fixture bound, and return error.
- Active child timeout terminates the admitted owned group; finite fixture children do not outlive their custody.
- Valid stdout with no stderr succeeds; nonzero exit with stdout is not accepted as complete.
- Invalid UTF-8 fails, including an otherwise benign prefix.
- Finite output cap is detected before unbounded allocation; no success on a truncated prefix.
- Strict dmesg and journal positive-format controls; empty, non-log, missing-final-newline, invalid timestamp/priority, and mixed valid-positive plus malformed record controls.
- Relevant existing regressions, golden Receipt V2, fmt, clippy and release build.

Fixtures are bounded command/parser evidence, not real-host faults. Original guest fixtures and repaired guest repeats remain separate root-owned evidence. No hidden holdout case may shape this implementation.

## Formalization consideration

Proposition: only the terminal state `(both EOF, child success, no observed failure, deadline unexpired)` produces successful acquisition; all observed incomplete states refuse complete evidence. The loop has finite per-turn read work and finite retained bytes, and creates no reader threads. Group signaling must occur before reaping the owned leader on a failure path. The bounded state table below records the acquisition gate and its correspondence to actual loop ordering. The model cannot prove kernel syscall timing, scheduler fairness, escaped descendants, command authenticity or real log format. Decision owner implementation_review; root approves scope and independent oracle/acceptance reviews correspondence. Exact source/result identities follow in the completion notice after implementation, not before.

## Custody

This proposal is minimal campaign documentation, owned by root for pending release review. No binaries, credentials, guests, or background producers created by this design step. Reassess at candidate freeze/closeout; preserve original failing evidence separately.

## Final implementation and observed correspondence

Product changes are confined to `src/collect/mod.rs` and `src/collect/kernellog.rs`; the receipt schema and accusation inventory are unchanged. `docs/design.md` and `docs/envelope.md` now state actual acquisition and authority limits. The standard-library UnixStream design uses sockets for child stdout/stderr, a dedicated child group, nonblocking bounded reads, and no reader threads. The negative-PID signal is restricted to that owned group before leader reaping on failure paths. Cleanup does not prove retirement of descendants that leave the group.

| Observed state | Permitted action/result | Implementation correspondence |
|---|---|---|
| Either stream not EOF | Drain fairly or fail; never accept | One chunk per stream per turn; no child reap yet |
| Both EOF, child still running | Poll while budget remains | Nonblocking try_wait, no blocking wait |
| Both EOF, child success, budget remains | Strict UTF-8 conversion can succeed | Success checked after EOF/status and deadline observation |
| Deadline, read or cap failure before reaping | Refuse acquisition, signal owned group, bounded cleanup | terminate_command; no retained prefix interpreted complete |
| Failed exit or invalid UTF-8 | Refuse complete evidence | Error propagates through required-source handling |
| Structured log plus malformed record | Preserve valid records as partial | Existing evaluator gate blocks CLEAR but permits supported positive |

This finite state table is the selected bounded formalization. It has four EOF combinations, three observed child states (running/success/failure), and deadline/failure flags; only both-EOF/success/unexpired/no-error may accept. Correspondence depends on single-threaded ordering, standard-library nonblocking semantics, and unreaped child identity, not a model of the scheduler or kernel. Practical regression tests exercise the risky transition histories (exit-before-EOF, stdout versus stderr retention, active-child timeout, cap, invalid encoding). A separate model checker would duplicate this small gate without proving the kernel assumptions; no universal liveness proof is claimed.

The initial representation proposal was challenged with actual host output before freeze. Local util-linux rejects `--raw --color=never`; final dmesg command is `dmesg --raw --force-prefix`. Real `journalctl -k` output includes early-userspace identifiers such as `systemd[1]`, empty messages, and aligned multiline continuations. Final parsing accepts a framed nonempty no-whitespace identifier token, legal empty messages, and aligned continuations only after a valid framed predecessor. Invalid standalone/post-malformed continuations remain inadequate. This is representation support, not authenticity.

Genuine Linux 6.5 guest capture `acquisition-baseline-batch/guest/captures/kernel-format/stdout` was replayed through the candidate external CLI and produced K CLEAR with complete required dmesg evidence. This establishes representation correspondence to that captured output, not a new guest fault trial. Genuine read-only local journal fallback was also exercised: initial narrower format returned UNKNOWN with 113 then 3 malformed records; corrected complete acquisition returned K CLEAR in approximately 191 ms. All these earlier receipts remain preserved, rather than erased as unsuccessful prototypes.

Final verification: `cargo fmt --check`, `cargo test` (109 unit + 2 Receipt V2 integration), `cargo clippy --all-targets -- -D warnings`, and `cargo build --release` pass. Exact logs/timings are in external `guest-lab/acquisition-final-checks/RECEIPT.json`. Reusable `campaign/acquisition-reproducers.py` executes nine finite substitution cases: invalid UTF-8, known non-log, adequate dmesg, adequate journal, mixed valid-positive/malformed, truncated record, inherited stdout, inherited stderr, output cap. Local preliminary run matched all expected verdicts; both inherited-pipe cases returned UNKNOWN around 3013 ms and their recorded fixture child identities were absent after collection. Final replay is separately retained under `guest-lab/acquisition-local-final`. These are not nine real-host experiments.

Original failing guest evidence remains root-owned at `guest-lab/acquisition-baseline-batch`. Candidate guest reruns, independent acceptance and final holdouts are root-owned next actions after source/executable freeze. No hidden holdout was accessed. Other parser/capacity/mount findings from the previous campaign remain outside this repair.

## Artifact custody update

The compiler target is root-admitted `guest-lab/build-target`, ceiling 200 MiB. It is reproducible build output; retire it after candidate executable/library custody and pending regression/replay dependencies end. Root owns final measured storage disposition. The frozen executable and source/result identities are release-review evidence; retain until root's campaign-closeout review. Local fixture scripts are retained without executable mode after use; their child identities are checked, and no long-lived producer is started by this lane. Exact logical/allocated sizes and final source/executable hashes are recorded in the successor completion notice and candidate receipt.
