# The interrogation envelope

Operator-facing notes on what Alibi assumes, what it can see from where you
stand, and what its verdicts do and do not mean. Read this before trusting a
receipt, and before sharing one.

## The core semantic statement

> `CLEAR` means that within the explicitly recorded interrogation envelope,
> Alibi found no evidence supporting that named accusation. It does not mean
> the machine is universally healthy.

`CLEAR` is bounded exoneration, not health attestation. Every receipt records
its own envelope — evidence sources, acquisition methods, the boot window, and
everything that could not be checked — so the bound is always visible on the
receipt itself. If a receipt does not show the envelope, it is not an Alibi
receipt.

## Platform assumptions

- Linux only. Facts come from `/proc`, `/sys`, and a small set of standard
  tools (`dmesg`, `journalctl`, `df`).
- No systemd is required, but the kernel-log fallback prefers it: Alibi tries
  `dmesg` first, then `journalctl -k -b`. On non-systemd hosts without dmesg
  access, kernel-log evidence is unavailable and the affected accusations
  report `UNKNOWN`.
- The network evidence window is a 2-second counter sample. It is part of the
  envelope: Alibi detects error/drop deltas across that window, not trends
  over uptime.
- V0 performs **no remediation** (strictly read-only) and **no active network
  probes** — no pings, no DNS lookups, no synthetic traffic of any kind.

## Privilege behavior

Alibi runs unprivileged and degrades honestly. It never guesses around a
permission boundary; it records it.

| Evidence | Unprivileged | As root (or equivalent) |
|---|---|---|
| Kernel log | Often restricted (`kernel.dmesg_restrict=1`, journal group membership required). Alibi falls back from `dmesg` to `journalctl`; if both are denied, kernel-log evidence is unavailable. | Direct `dmesg` access; the largest single improvement in envelope coverage. |
| Mounts, space, inodes | Fully available. | Same. |
| Network counters, routes, conntrack | Fully available. | Same. |
| EDAC counters | Available when the platform exposes them (world-readable sysfs), but most VMs expose no EDAC at all — the hypervisor owns hardware memory integrity. | Same; privilege does not create EDAC on a VM. |

Concretely: an unprivileged run on a locked-down host will typically report
`kernel-scream` and the log-dependent parts of `silent-killer` as `UNKNOWN`.
That is the tool working as designed, not malfunctioning. Run with sufficient
privilege when you need the full envelope; treat an unprivileged receipt as a
receipt about a smaller envelope.

## What UNKNOWN means

`UNKNOWN` means innocence was **not established**: evidence required to
exonerate the host was unavailable, unsupported on this platform, inaccessible,
or malformed. It is not a soft pass and not a failure; it is the explicit
boundary of what Alibi could check. A receipt full of `UNKNOWN` is a statement
about the envelope, not about the host.

`UNKNOWN` still ends the MTTI clock (the interrogation reached a bounded
conclusion), but a run containing any `UNKNOWN` never reports
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

Each finding carries `time_to_verdict` — seconds from interrogation start
until that accusation reached a terminal verdict. The receipt's `mtti` is when
**every** requested accusation terminated. Any terminal verdict (`CLEAR`,
`DEGRADED`, `UNKNOWN`, `NOT_APPLICABLE`) ends the clock; `UNKNOWN` ends it
without establishing innocence. Whole-host MTTI is therefore "time to a
bounded conclusion", meaningful only alongside the envelope and
`innocence_established`. See [design.md](design.md) §4 for the normative
semantics.
