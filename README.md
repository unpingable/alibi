# Alibi

**A suspicious Linux host? Find out what you can rule out, what looks bad, and what the evidence cannot establish. With receipts.**

Alibi is an on-demand, read-only Linux diagnostic CLI. It evaluates four named accusations and explains each verdict. It does not monitor continuously, remediate, send active network probes or manage other hosts.

**Experimental v0.1 release candidate: `QUALIFIED_WITH_LIMITATIONS`.** The repaired candidate has independent Linux guest qualification. It is ready for bounded experimentation, not a claim that every Linux environment is supported or that a CLEAR host is healthy. Reduced operator diagnostic time has **not** been established.

## Install and try it

Linux x86_64 with a GNU runtime is the supported implementation target. The prepared binary archive requires **glibc 2.39 or newer** and `libgcc_s.so.1`; it is dynamically linked. Release assets await publication approval. Private-repository users can obtain a rebuilt archive from a successful release-branch GitHub Actions run. Its `SOURCE.json` distinguishes a CI rebuild from the exact guest-tested executable.

See [archive installation](docs/release-v0.1/INSTALL.md), including checksum verification and runtime dependencies. To build from the release candidate instead:

```sh
git clone --branch release/v0.1.0-rc1 https://github.com/unpingable/alibi.git
cd alibi
cargo build --release --locked
mkdir -p "$HOME/.local/bin"
install -m 0755 target/release/alibi "$HOME/.local/bin/alibi"
"$HOME/.local/bin/alibi" check
```

A Rust toolchain and ordinary linker are needed to build. There are no external Rust crates. Runtime collection uses `/proc`, `/sys`, `df` and `dmesg`, with optional `journalctl` fallback. A rebuild on an older GNU runtime may run there; that environment is not thereby qualified. macOS, Windows, ARM and musl/static distributions are not qualified.

```sh
alibi check                       # all four accusations
alibi check network-choke-point   # one accusation
alibi check silent-killer memrot  # selected accusations
alibi check --json                # machine-readable Receipt V2
alibi check --verbose             # expanded evidence
alibi explain kernel-scream       # scope and limitations
```

Start unprivileged. Restricted required sources produce UNKNOWN with an explanation. An administrator can choose to run the installed binary with sufficient privilege to read those sources. Privilege does not create EDAC hardware evidence that a VM lacks.

## Understand the result

| Verdict | Meaning |
|---|---|
| `CLEAR` | Sufficient applicable evidence exonerates this specific accusation within the recorded observation bounds. **Not a healthy-host certificate.** |
| `DEGRADED` | Collected evidence supports a named condition. It is not necessarily the cause of your application problem. |
| `UNKNOWN` | No supported positive conclusion, and required evidence is unavailable, unsupported, inaccessible, malformed or incomplete. Innocence was not established. |
| `NOT_APPLICABLE` | The accusation does not apply; currently used for networking with no non-loopback interfaces. |

A valid positive can coexist with incomplete evidence: DEGRADED does not hide missing required sources. Default human output includes source status, completeness, method, timing and material limitations; verbose mode is not required to discover them. JSON preserves the same canonical receipt semantics.

For example, a host can have `silent-killer: DEGRADED` because a material filesystem is full while `network-choke-point: CLEAR` legitimately exonerates the narrower network accusation. It can also have working DNS and `memrot: UNKNOWN` because no EDAC controller is exposed. These are compatible conclusions, not contradictory health scores. [Fictional example receipts](examples/) illustrate the format; older examples are labeled by receipt version and are not current qualification evidence.

Exit codes: **0** all selected findings CLEAR/NOT_APPLICABLE; **1** at least one DEGRADED; **2** UNKNOWN without DEGRADED; **64** usage error. In a shell script, preserve JSON even when the diagnostic status is nonzero.

## What it checks

| Accusation | Bounded scope |
|---|---|
| `kernel-scream` | Recognized serious kernel/hardware signatures in the acquired retained log. |
| `silent-killer` | Supported material filesystem space/inode exhaustion, read-only backing and recognized filesystem errors. |
| `memrot` | Applicable EDAC counters and supported memory-integrity log evidence. EDAC-less guests normally return UNKNOWN. |
| `network-choke-point` | Sampled interface error/drop deltas, recognized repeated link-down events, missing usable defaults and applicable conntrack pressure. |

Network CLEAR does not establish DNS, endpoint reachability, filtering policy, application health or continuous link availability. Storage CLEAR does not establish physical-device health or arbitrary mount-generation continuity. Logs can lose history; format validation does not authenticate a substituted source. Genuine sysfs and coherent proc/sys views are assumed. See [the interrogation envelope](docs/envelope.md) for exact dependencies and limitations.

## Qualification and remaining gaps

Two campaigns executed **214 QEMU guest boots** in total. That count includes controls, repeats, failures and infrastructure outcomes; it is not 214 independent successful fault tests. The first campaign held release and preserved counterexamples. The successor repaired demonstrated interface-identity, EDAC-attribution and human-receipt defects before freezing the candidate and opening ten new sealed cases.

For the repaired evaluated primary set: **zero false CLEARs across 72 findings**, with **25/25 useful exonerations from real required evidence**, plus **1/1 from a separately labeled benign-log fixture**. This deliberately selected finite sample is not a production error-rate estimate. Nine fresh case oracles were fully supported; a tenth retains an explicit overconstrained-oracle subclaim rather than being called a blanket pass.

The full campaign environment was QEMU x86_64, Linux **6.5.0-44-generic**, with constructed GNU/Dash/BusyBox userspace. **6.8.0-90-generic** covers only the acquisition subset, not a second fully qualified distribution. Real hardware EDAC exoneration, resource starvation, stalled storage, general service functionality and some precisely ordered races remain unqualified. The original H03/G30/G36 gaps and fresh Q04 subclaim remain visible.

Read the [current qualification](docs/requalification-20261011/QUALIFICATION.md), [results](docs/requalification-20261011/RESULTS.md), [independent acceptance and correction](docs/requalification-20261011/ACCEPTANCE-CORRECTION.md), and [release notes](docs/release-v0.1/RELEASE-NOTES.md). Historical records and replay scripts remain in the repository; large VM/build artifacts and raw local host evidence are not committed. [Evidence availability](docs/release-v0.1/EVIDENCE.md) explains what is independently reproducible from this checkout and what remains in local custody.

## Feedback from actual use

Try it on a Linux machine you administer, then tell us:

1. Did it tell you anything useful?
2. Did you understand what it could not establish?
3. Would you run it again when a host looked suspicious?

An experimental release does not wait on a formal operator study. We simply do not advertise improved human diagnostic time as a measured result. The receipt's decision/innocence timers measure this invocation, not the operator's investigation.

Receipts may include hostnames, interfaces, mountpoints and kernel-log excerpts. Review and redact them before sharing; Alibi does not upload them. Report issues with the version, selected accusations and relevant redacted evidence.

## Development and license

```sh
cargo fmt --check
cargo test --locked
cargo clippy --all-targets -- -D warnings
cargo build --release --locked
```

Read [design](docs/design.md), [provenance](PROVENANCE.md) and the [Apache-2.0 license](LICENSE). Alibi remains a standalone tool; possible consumers do not change its diagnostic or authority boundaries.

For Ubuntu 22.04 and Debian 12, see the [older-glibc compatibility build](docs/linux-compatibility.md). Its CI artifact is separate from the unchanged v0.1.0 release binary.
