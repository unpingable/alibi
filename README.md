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

- `CLEAR` — all required evidence sources were complete and no collected evidence indicates the failure class.
- `DEGRADED` — collected evidence contains at least one positive indication of the failure class, including retained evidence from a partial source.
- `UNKNOWN` — no indication was found but at least one required evidence source was unavailable, unsupported, inaccessible, malformed, or incomplete; innocence was **not** established. Collection failure is never reported as health.
- `NOT_APPLICABLE` — the accusation is semantically void on this host.

Alibi never claims "this machine is definitely healthy." It claims: *within this
explicitly recorded interrogation envelope, no evidence supports these named
accusations.* The envelope — source completeness, methods, collection completion
and sample windows, and everything that could not be checked — is part of every
receipt. Each finding identifies which sources are required to clear it and which
are supporting only. Partial evidence is retained: it can establish `DEGRADED`,
but an incomplete required source can never authorize `CLEAR`.

## The V0 accusations

| Accusation | Question |
|---|---|
| `kernel-scream` | Has the kernel/hardware been screaming (oops, MCE, watchdog, PCIe AER, I/O errors) while monitoring stayed quiet? |
| `silent-killer` | Is the host up but functionally damaged (read-only remounts, fs/journal errors, inode exhaustion, critical space)? |
| `memrot` | Is there memory-integrity degradation (EDAC corrected/uncorrected, kernel-reported memory errors)? |
| `network-choke-point` | Is there host-local network pathology (error/drop deltas, link flaps, missing default route, conntrack pressure)? |

## Build and use

Requires a Rust toolchain. There are no external Rust crates. The ordinary
Linux GNU release build produces one executable dynamically linked to the
system runtime; a standalone static distribution has not been qualified.
Command collection currently supports the Linux x86_64 GNU signal ABI. See
[the interrogation envelope](docs/envelope.md) for command dependencies and
evidence limitations.

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
alibi --version                   # implementation version
```

```text
ALIBI host=crow decision=2.31s mtti=n/a complete=2.31s

CLEAR     kernel-scream        no serious kernel/hardware events found
CLEAR     silent-killer        mounted filesystems writable; no journal/fs errors found
UNKNOWN   memrot               EDAC counters unavailable on this hardware/kernel
DEGRADED  network-choke-point  eno1 reports increasing RX errors

verdict: DEGRADED (innocence not established)
```

Exit codes: `0` innocence established · `1` degraded · `2` unknown · `64` usage.

Some evidence (kernel log, EDAC) needs sufficient privilege on many systems.
Run unprivileged and Alibi does not guess — the affected accusations come back
`UNKNOWN` with the reason on the receipt. See
[docs/envelope.md](docs/envelope.md) for exactly what improves with privilege
and what remains meaningful without it.

## The receipt contract

`--json` receipts carry `"receipt_version": 2` and the Alibi `version` that
produced them. V2 separates decision latency from time to innocence:
`decision_latency_ms` is always numeric, while `mtti_ms` is `null` unless every
requested accusation is `CLEAR` or `NOT_APPLICABLE`. The format is guarded by a
golden contract test (`tests/golden/receipt_v2.json`): field removals, renames,
and reorderings fail the build. This is not a long-term compatibility promise —
contract changes are intentional, not accidental. Sanitized, fictional example
receipts live in [examples/](examples/); use those in documentation and bug
reports. The V1 examples remain as historical contract illustrations.

## Receipt sensitivity

Read-only does not mean harmless to share. A receipt is host evidence —
hostname, interfaces, mountpoints, filesystem fill levels, kernel-log
excerpts, timings. Treat receipts like `dmesg` output: do not paste them into
public channels or upload them to third-party tooling without review. See
[docs/envelope.md](docs/envelope.md#receipt-sensitivity).

## What Alibi is not

Not a monitoring daemon, agent platform, observability stack, or healthcheck
framework. Not a repair tool — V0 is strictly read-only. No GUI, server,
database, scheduler, fleet manager, remediation engine, or plugin marketplace.
Deferred on purpose: SSH/fleet interrogation, persistent history, alerting
integrations, Prometheus/OpenTelemetry export, active network probes,
Windows/macOS, remote attestation.

One non-goal stands above the others: **`CLEAR` is bounded exoneration, not
health attestation.** Alibi will never grow a mode, flag, or summary that
claims a machine is universally healthy.

## Design

See [docs/design.md](docs/design.md): verdict algebra, precise MTTI semantics,
per-class evidence models, and the category errors Alibi refuses to commit
(e.g. collection failure ≠ innocence; cumulative counters are not trends;
a successful ping proves nothing). See [docs/envelope.md](docs/envelope.md)
for the operator-facing interrogation envelope: platform assumptions,
privilege behavior, what `UNKNOWN` means, and receipt sensitivity.

## License

Licensed under Apache-2.0. See [LICENSE](LICENSE) and [NOTICE](NOTICE).
