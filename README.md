# Alibi

> **Monitoring tells you why a host may be guilty. Alibi asks how quickly you can establish that it isn't.**

Alibi is an on-demand Linux host interrogation tool for the moment an operator
asks: *"is this machine actually okay?"*

Most operational tooling is built around positive evidence of failure: alerts,
incidents, threshold violations. The complementary question is under-served —
the host looks suspicious, or simply hasn't complained lately, and you want to
know whether you can **prove it is presently sane**, how far you can prove it,
and how long that proof takes.

Alibi interrogates the host directly and answers in a fixed grammar:

> **named accusations → evidence → verdict → receipt**

It does not print `PASS`. It prints what it accused the host of, what evidence it
found, what it could not check, and a bounded conclusion — with the clock running.
That clock is the point: **MTTI, Mean Time To Innocence**. How long does it take
to stop suspecting the machine?

## Verdicts

Nothing collapses to pass/fail:

- `CLEAR` — every required evidence source was collected; nothing supports the accusation.
- `DEGRADED` — collected evidence positively indicates the failure class.
- `UNKNOWN` — required evidence was unavailable, unsupported, or malformed. Innocence was **not** established. Collection failure is never reported as health.
- `NOT_APPLICABLE` — the accusation is semantically void on this host.

Alibi never claims "this machine is definitely healthy." It claims: *within this
explicitly recorded interrogation envelope, no evidence supports these named
accusations.* The envelope — sources, methods, evidence window, everything that
could not be checked — is part of every receipt.

## The V0 accusations

| Accusation | Question |
|---|---|
| `kernel-scream` | Has the kernel/hardware been screaming (oops, MCE, watchdog, PCIe AER, I/O errors) while monitoring stayed quiet? |
| `silent-killer` | Is the host up but functionally damaged (read-only remounts, fs/journal errors, inode exhaustion, critical space)? |
| `memrot` | Is there memory-integrity degradation (EDAC corrected/uncorrected, kernel-reported memory errors)? |
| `network-choke-point` | Is there host-local network pathology (error/drop deltas, link flaps, missing default route, conntrack pressure)? |

## Build and use

Requires a Rust toolchain. No external crates; single static binary.

```sh
cargo build --release
```

```text
alibi check                       # interrogate everything
alibi check kernel-scream         # one accusation
alibi check silent-killer memrot  # several
alibi check --json                # stable machine-readable receipt
alibi check --verbose             # evidence detail
alibi explain kernel-scream       # what an accusation means and cannot mean
```

```text
ALIBI host=crow elapsed=2.31s mtti=2.31s

CLEAR     kernel-scream        no serious kernel/hardware events found
CLEAR     silent-killer        mounted filesystems writable; no journal/fs errors found
UNKNOWN   memrot               EDAC counters unavailable on this hardware/kernel
DEGRADED  network-choke-point  eno1 reports increasing RX errors

verdict: DEGRADED (innocence not established)
```

Exit codes: `0` innocence established · `1` degraded · `2` unknown · `64` usage.

Some evidence (kernel log, EDAC) needs sufficient privilege on many systems.
Run unprivileged and Alibi does not guess — the affected accusations come back
`UNKNOWN` with the reason on the receipt.

## What Alibi is not

Not a monitoring daemon, agent platform, observability stack, or healthcheck
framework. Not a repair tool — V0 is strictly read-only. No GUI, server,
database, scheduler, fleet manager, remediation engine, or plugin marketplace.
Deferred on purpose: SSH/fleet interrogation, persistent history, alerting
integrations, Prometheus/OpenTelemetry export, active network probes,
Windows/macOS, remote attestation.

## Design

See [docs/design.md](docs/design.md): verdict algebra, precise MTTI semantics,
per-class evidence models, and the category errors Alibi refuses to commit
(e.g. collection failure ≠ innocence; cumulative counters are not trends;
a successful ping proves nothing).
