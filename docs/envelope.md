# The interrogation envelope

Operator-facing notes on what Alibi assumes, what it can see from where you
stand, and what its verdicts do and do not mean. Read this before trusting a
receipt, and before sharing one.

## The core semantic statement

> `CLEAR` means that within the explicitly recorded interrogation envelope,
> Alibi obtained sufficient applicable evidence to exonerate that specific,
> bounded accusation. It does not mean the machine is universally healthy.

`CLEAR` is bounded exoneration, not health attestation. Every receipt records
its own envelope — evidence-source roles and completeness, acquisition methods,
collection completion offsets, bounded sample windows, the boot window, and
everything that could not be checked — so the bound is always visible on the
receipt itself. If a receipt does not show the envelope, it is not an Alibi
receipt.

Sources are planned from the requested accusations. A receipt for only
`kernel-scream`, for example, does not run the unrelated two-second network
sample and does not list network sources. Within a finding, a required source
must be complete before `CLEAR`; a supporting source adds context or can provide
positive evidence, but its absence does not block `CLEAR`.

Default human output includes the collected source envelope and per-finding source
roles, including failures when other evidence already establishes DEGRADED.
All DEGRADED indications are visible by default; `--verbose` adds expanded
evidence. UNKNOWN retains its evidence details by default. JSON Receipt V2 is unchanged.

## Platform assumptions

- Linux only. Facts come from `/proc`, `/sys`, and a small set of standard
  tools (`dmesg`, `journalctl`, `df`).
- No systemd is required, but the kernel-log fallback prefers it: Alibi tries
  `dmesg --raw --force-prefix` first, then
  `journalctl -k -b --no-pager --quiet -o short-monotonic --no-hostname --all`. On non-systemd
  hosts without dmesg access, kernel-log evidence is unavailable and the
  affected accusations report `UNKNOWN`.
- The network evidence window is a bounded counter sample whose actual monotonic
  start, end, and duration are part of the envelope. Alibi detects error/drop
  deltas across that window, not trends over uptime. Each pair uses the same
  retained sysfs counter descriptor, opened relative to one held interface
  directory. Missing, removed or unreadable required counters prevent CLEAR;
  reopening a reused name never repairs missing evidence. This requires genuine
  sysfs attributes and coherent host proc/sys namespace views. Same-object
  namespace round trips can remain comparable; uninterrupted namespace residence,
  every transient interface and hidden driver counter resets are not established.
- V0 performs **no remediation** (strictly read-only) and **no active network
  probes** — no pings, no DNS lookups, no synthetic traffic of any kind.

## Privilege behavior

Alibi runs unprivileged and degrades honestly. It never guesses around a
permission boundary; it records it.

| Evidence | Unprivileged | As root (or equivalent) |
|---|---|---|
| Kernel log | Often restricted (`kernel.dmesg_restrict=1`, journal group membership required). Alibi falls back from `dmesg` to `journalctl`; if both are denied, kernel-log evidence is unavailable. | Direct `dmesg` access; the largest single improvement in envelope coverage. |
| Mounts, space, inodes | Fully available. | Same. |
| Network counters, routes, conntrack | Normally readable. Conntrack is required whenever connection tracking is present; if its files exist but cannot be read, `network-choke-point` is `UNKNOWN`. If the subsystem is absent, that failure class is not applicable and does not block `CLEAR`. | Same. |
| EDAC counters | Available when the platform exposes them (world-readable sysfs), but most VMs expose no EDAC at all — the hypervisor owns hardware memory integrity. | Same; privilege does not create EDAC on a VM. |

Concretely: an unprivileged run on a locked-down host will typically report
every accusation that requires the kernel log — `kernel-scream`,
`silent-killer`, `memrot`, and `network-choke-point` (link flaps and
conntrack-full events are log evidence) — as `UNKNOWN`.
That is the tool working as designed, not malfunctioning. Run with sufficient
privilege when you need the full envelope; treat an unprivileged receipt as a
receipt about a smaller envelope.

## What UNKNOWN means

`UNKNOWN` means innocence was **not established**: evidence required to
exonerate the host was unavailable, unsupported on this platform, inaccessible,
malformed, or incomplete. It is not a soft pass and not a failure; it is the explicit
boundary of what Alibi could check. A receipt full of `UNKNOWN` is a statement
about the envelope, not about the host.

A partial source is not discarded. Its retained values can establish
`DEGRADED` when they contain positive evidence. If they do not, a partial
required source produces `UNKNOWN`, never `CLEAR`. The envelope distinguishes
`complete`, `partial`, and `none` so consumers do not have to infer this boundary
from prose.

`UNKNOWN` ends the decision clock, but not the innocence clock. A run containing
any `UNKNOWN` reports `mtti_ms: null` and never reports
`innocence_established: true`.

## Receipt sensitivity

Read-only does not mean harmless to share. A receipt is host evidence: it
contains the hostname, interface names and states, mountpoints and devices,
filesystem fill levels, kernel-log excerpts, and timing — operationally
sensitive infrastructure detail. Treat receipts like you treat `dmesg` output
or `ip -s link` dumps:

- do not paste them into public issues, chats, or ticket systems without
  review;
- do not upload them to third-party tooling casually;
- the sanitized, fictional examples under `examples/` exist precisely so
  documentation and bug reports never need real receipts.

## MTTI in one paragraph

Each finding carries `decided_at_ms`, measured from one monotonic interrogation
start until that accusation reached a terminal verdict. A `CLEAR` finding also
has the same value in `innocence_at_ms`; other verdicts use `null` there. The
receipt's `decision_latency_ms` is when every requested accusation terminated.
Its `mtti_ms` is numeric only when every finding is `CLEAR` or
`NOT_APPLICABLE`; otherwise it is `null`. `interrogation_complete_ms` records
when all collection and evaluation finished, before receipt construction and
excluding serialization/output. See [design.md](design.md) §4 for the normative
semantics.

## Acquisition limits

Commands must finish and close both output streams within their acquisition
deadline. Each stream is limited to 8 MiB; a cap, read failure, timeout or invalid
UTF-8 produces inadequate evidence, never a complete negative result from the
retained prefix. Transport failures currently discard that command's output,
including any positive prefix. Structured log records survive malformed
neighboring records as partial evidence. Selected log formats require supported
command options; unsupported formats fall back or produce UNKNOWN.

Numeric log framing validates representation only. It cannot authenticate a
substituted command, detect all valid-looking corruption, establish freshness,
or recover overwritten history. Complete acquisition means the currently
retained requested output was acquired without known loss, not a complete boot
history. Cleanup uses the command's dedicated process group and a separate
100 ms polling allowance; inability to reap is explicitly reported. Kernel
scheduling and uninterruptible waits prevent a universal elapsed-time bound.

Command collection currently qualifies the Linux x86_64 GNU signal ABI. Before
spawning, Alibi requires default SIGCHLD handling without SA_NOCLDWAIT; ignored
or custom handlers and other target ABIs produce inadequate evidence. It does
not normalize or modify inherited signal handling. A wait error prevents numeric
process-group cleanup because child ownership is then unconfirmed. Reusing the
collector as a library requires exclusive ownership of its child reaping and
stable SIGCHLD disposition throughout collection; concurrent external reapers
or disposition mutation are outside this contract.

Filesystem capacity is accepted only when df reports the requested textual
source and mountpoint with one unambiguous capacity row. If a path resolves to a
different mount after the mount snapshot, mismatched capacity becomes incomplete
evidence; any separately valid positive remains available. Device aliases, bind
views or unsupported quoting can conservatively cause UNKNOWN. Matching names
is not proof of stable superblock identity or absence of same-name replacement.

Empty or boundary-whitespace expected df source/mountpoint names are unsupported.
Even ordinary expected names cannot authenticate all actual labels: trailing
source-name spaces in GNU df output may look like column padding. The check
therefore establishes correspondence to the admitted printed representation,
not arbitrary kernel-label identity.

## EDAC report coverage

A loaded or registered EDAC controller is not a memory-error indication. Supported
positive log evidence is an EDAC memory-controller event beginning with a positive
count and CE or UE classification. Known informational registration and startup
forms are ignored as positives. Other EDAC-origin forms are explicitly unclassified
and block memrot CLEAR unless another supported indication warrants DEGRADED.
Driver formats beyond this finite grammar are not silently treated as clean.
For multiline EDAC journal records, a supported positive must be complete on the
first physical line. Otherwise the whole record is unclassified; a benign prefix
does not make its continuations clean evidence.
EDAC counters remain independent evidence; absent controllers on a VM still do not
establish hardware memory integrity. Log representation does not authenticate its
contents or establish complete history.
