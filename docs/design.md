# Alibi — design note (V0)

Status: normative for V0. Where code and this note disagree, fix one of them deliberately.

## 1. What Alibi is

Alibi is an on-demand Linux host interrogation tool. It exists for the moment an
operator asks: *"is this machine actually okay?"* — especially when monitoring has
been quiet and that quiet is itself suspicious.

Monitoring looks for positive evidence of failure. Alibi looks for important failure
classes that may have slipped through alert gaps, and produces an evidence-bearing
receipt of what it found, what it could not check, and how long a bounded conclusion
took. The product grammar is:

> **named accusations → evidence → verdict → receipt**

Alibi is not a monitoring daemon, agent platform, observability stack, or healthcheck
framework. It runs when invoked, interrogates, prints a receipt, exits.

**MTTI — Mean Time To Innocence.** Traditional ops metrics measure failure and
restoration (MTTD → MTTA → MTTR). Alibi adds the complementary primitive: how long
it takes to stop suspecting a machine. V0 makes that primitive measurable; it does
not build aggregate SLO machinery.

## 2. Non-goals (V0 scope fence)

Explicitly deferred: SSH/fleet interrogation, persistent history, automatic
remediation, alerting integrations, Prometheus/OpenTelemetry export, web UI,
Windows/macOS, remote attestation, arbitrary third-party plugins. Also: no GUI,
server, database, daemon, scheduler, or plugin marketplace. V0 is read-only:
diagnose, never repair. No active network probes (see §7).

## 3. Verdict algebra

Each accusation (failure class) produces exactly one terminal verdict:

| Verdict | Meaning |
|---|---|
| `CLEAR` | Every **required** evidence source for the class was collected successfully, and no collected evidence indicates the failure class. |
| `DEGRADED` | Collected evidence contains at least one positive indication of the failure class. |
| `UNKNOWN` | No indication was found, but at least one **required** evidence source was unavailable, unsupported, inaccessible, or malformed. Innocence was not established. |
| `NOT_APPLICABLE` | The accusation is semantically void on this host (V0 uses this only for `network-choke-point` on hosts with zero non-loopback interfaces). |

Rules:

- Sources are **required** or **optional** per class (§6). Optional-source failure is
  recorded in the finding's `skipped` list and never blocks `CLEAR`; required-source
  failure forces `UNKNOWN` unless an indication forces `DEGRADED`.
- `DEGRADED` outranks source failure: if evidence shows the failure, say so, and
  still record what could not be checked.
- "No evidence of failure" never silently becomes "healthy". Collection failure is
  `UNKNOWN`, not `CLEAR`. This is the tool's core epistemic rule and is enforced by
  a mandated test.
- Overall verdict precedence: `DEGRADED` > `UNKNOWN` > `CLEAR` > `NOT_APPLICABLE`.
- `innocence_established` on the receipt is true iff every finding is `CLEAR` or
  `NOT_APPLICABLE`.
- Exit codes: `0` innocence established; `1` any `DEGRADED`; `2` otherwise any
  `UNKNOWN`; `64` CLI usage error.

## 4. MTTI semantics

- `t0` = interrogation start (immediately after CLI parsing).
- Each finding carries `time_to_verdict`: duration from `t0` until the evaluator
  returned a terminal verdict for that accusation. Collection is shared across
  classes and runs first, so per-class times include amortized collection time.
- Receipt `mtti` = duration from `t0` until **every requested accusation** reached a
  terminal verdict (equivalently: max of per-finding times). Any of `CLEAR`,
  `DEGRADED`, `UNKNOWN`, `NOT_APPLICABLE` ends the clock for that class.
- **`UNKNOWN` ends the clock but does not establish innocence.** Whole-host MTTI is
  therefore "time to a bounded conclusion", not "time to a clean bill of health".
  A receipt full of `UNKNOWN` is fast and useless; the envelope (§5) makes that
  visible instead of hiding it in the number.
- The receipt embeds these semantics as machine-readable strings so later
  fleet/incident tooling consumes the same definitions.

## 5. Architecture

Deliberately boring:

```
collector → normalized facts (+ per-source status) → failure-class evaluator → finding → receipt
```

- **Collection** (`collect`): reads `/proc`, `/sys`, and a small set of standard
  tools (`dmesg`/`journalctl`, `df`) into typed facts. Every source yields
  `Source<T> { status, value, detail, method, elapsed }` with
  `status ∈ ok | unavailable | unsupported | malformed`. Collectors never interpret.
- **Evaluation** (`eval`): pure functions `facts → Finding`. No I/O, no command
  parsing. Fully fixture-testable. A shared, explicit substring-pattern table tags
  kernel-log lines to failure classes; no regex engine. Patterns carry per-pattern
  exclusion substrings so benign banners do not match (e.g. the `EDAC` needle
  excludes `Ver:`/`detected (`/version lines — a loaded EDAC core is not a memory
  error). The table is curated and versioned in one place; that is the mechanism,
  not a bug.
- **Presentation** (`receipt`): human renderer, zero-dependency JSON emitter.
  Verdict logic never contains parsing; parsing never contains verdicts.

The **interrogation envelope** is recorded on every receipt: each source's name,
status, method, elapsed time, and the evidence window (boot time from
`/proc/stat` btime; kernel-log acquisition method). Alibi never says "this machine
is healthy"; it says "within this explicitly described envelope, no evidence
supports these named accusations."

Kernel-log acquisition: try `dmesg`, fall back to `journalctl -k -b --no-pager`,
else `unavailable` (e.g. `kernel.dmesg_restrict=1` and no journal access — common
for unprivileged users; run with sufficient privilege for a full envelope). Every
external command runs under a hard timeout; the whole interrogation stays
interactive-scale (target ≤ ~3 s, dominated by the network delta window).

## 6. Failure classes (V0)

Registry order: `kernel-scream`, `silent-killer`, `memrot`, `network-choke-point`.

### kernel-scream
*Has the kernel/hardware been screaming while monitoring stayed quiet?*

- Required: kernel log. Optional: none in V0 (machine-check sysfs noted as future).
- Indications (log patterns): kernel panic / Oops / BUG / general protection fault;
  soft/hard lockup and stall reports; MCE / "Hardware Error"; PCIe AER errors;
  serious block I/O errors; filesystem errors (shared with silent-killer).
- Limitation: dmesg ring buffer may be wrapped on long-uptime hosts; the envelope
  records which acquisition method was used. V0 does not assume `mcelog` exists;
  modern kernels report MCEs through the kernel log.

### silent-killer
*Is the host superficially up but functionally damaged?*

- Required: mounts (`/proc/self/mounts`), space/inode facts (targeted `df` — see
  below), kernel log.
- Indications: write-capable block-backed filesystem (ext2/3/4, xfs, btrfs, f2fs,
  jfs, reiserfs) mounted `ro` — with a precision rule: a kernel `remount-ro` flips
  the whole filesystem, so an `ro` mount whose device is also mounted `rw`
  elsewhere is a deliberate read-only bind view (snap-style content sharing) and
  is recorded as evidence, **not** an indication. Only a write-capable device with
  no `rw` mount anywhere is an indication. squashfs/iso9660 and other
  read-only-by-design fstypes are excluded. Also: kernel-log remount-ro /
  fs-journal error patterns; space or inode free ≤ 5% on a material filesystem
  (critically low, not merely "high usage").
- Space/inode facts come from targeted `df -P` calls against the material
  filesystems only (block-backed filesystems deduped by device; each tmpfs is
  distinct). Whole-table `df` is deliberately not used: one inaccessible unrelated
  mount (e.g. a permission-restricted FUSE mount) must not destroy all space
  evidence for the host.
- Limitation: Alibi cannot know operator intent. A deliberately read-only root is
  reported with wording that says so; evidence is shown, the operator judges.
  ZFS and network filesystems are out of V0's mount model.

### memrot
*Is there memory/hardware integrity degradation?*

- Required: EDAC sysfs (`/sys/devices/system/edac/mc/mc*/{ue,ce}_count`), kernel log.
- Indications: uncorrected errors > 0 (severe); corrected errors > 0 (reported with
  trend caveat — any nonzero count matters when exonerating); kernel-log
  EDAC / "Hardware Error" / memory-failure patterns.
- `UNKNOWN` is the honest verdict on platforms exposing no EDAC (most VMs): the
  hypervisor owns hardware memory integrity and the guest cannot establish it.
- Out of scope by design: "RAM is busy". High usage, swapping, and PSI are workload
  signals, not integrity evidence (§7).

### network-choke-point
*Is there obvious host-local network pathology?*

- Required: interface facts (`/sys/class/net`, two samples 2 s apart), route tables
  (`/proc/net/route`, `/proc/net/ipv6_route`). Optional: conntrack
  (`/proc/sys/net/netfilter/nf_conntrack_{count,max}`), kernel log (link flaps,
  conntrack-full).
- Indications: RX/TX error counter delta > 0 across the sample window; RX/TX drop
  delta > 0; ≥ 3 link-down events for one interface in the log window; no default
  route in either address family (worded as interpretation — legitimate on some
  appliances); conntrack usage ≥ 90%.
- Counters are cumulative since boot/driver load, so **absolute values never
  verdict**; only the bounded delta does. The 2 s window is part of the envelope.
- Down/unused interfaces are normal and are evidence, not accusations.
- `NOT_APPLICABLE` when no non-loopback interface exists.

## 7. Category errors and epistemic traps

Checks that cannot support their claimed verdict are worse than no check. V0 rules:

1. **Collection failure ≠ innocence.** Unavailable/unsupported/malformed required
   evidence yields `UNKNOWN`. (Mandated test.)
2. **Absence of a subsystem ≠ absence of failure.** No EDAC does not mean "no memory
   errors"; it means the accusation is unresolved → `UNKNOWN`.
3. **Cumulative counters are not trends.** Lifetime error counts flag nothing;
   only bounded deltas do.
4. **A successful ping proves nothing; a failed one proves nothing.** V0 performs
   **no active probes**. Ping/DNS tests measure one synthetic path, not host state,
   and ICMP absence is not failure. Passive local evidence only. (Bounded active
   checks are a possible later, opt-in direction.)
5. **Low disk space is not automatically failure.** Only critically low (≤ 5% free)
   space or inodes on a write-relevant mount is an indication, worded as such.
6. **"RAM is busy" is not memrot.** Usage/pressure/PSI are workload telemetry and
   are excluded from the integrity accusation.
7. **Read-only-by-design is not read-only-by-failure.** squashfs, iso9660, pseudo
   filesystems, and container bind-mount conventions are excluded from the ro check.
8. **A quiet kernel log you cannot read is not a quiet kernel.** dmesg restricted →
   `UNKNOWN`, with the reason on the receipt.
9. **Down interface ≠ failure.** Operstate is recorded as evidence only; repeated
   link-state transitions are the signal.
10. **Default-route absence is an interpretation, not a fact of failure.** It is an
    indication with explicit wording, because most general-purpose hosts have one.
11. **Whole-host MTTI is bounded-conclusion time, not health certification** (§4).

## 8. CLI and output contract

```
alibi check [flags] [accusation...]   # default: all four, registry order
alibi check --json                    # stable machine-readable receipt (stdout)
alibi check --verbose                 # human receipt with evidence detail
alibi explain <accusation>            # meaning, evidence, verdict semantics, limits
```

Human default is one line per accusation plus an overall verdict line, per the
project seed. The JSON receipt is the stable interface: additive changes only,
field removals/renames are breaking. Top level: `tool`, `version`, `host`,
`started_at` (RFC 3339 UTC), `elapsed_ms`, `mtti_ms`, `innocence_established`,
`verdict`, `findings[]` (accusation, verdict, summary, indications, evidence,
skipped, time_to_verdict_ms), `envelope` (boot_time, sources[] with status /
method / detail / elapsed_ms), `semantics` (machine-readable verdict/MTTI
definitions).

## 9. Implementation choice

**Rust, zero external crates.** Both Rust and Go fit a small portable Linux systems
CLI; this environment ships Rust 1.98 and no Go toolchain, so Rust produces the
simpler artifact and testing story here. Standard library only: facts come from
`/proc`/`sysfs` and standard tools; the receipt JSON is a small hand-rolled emitter;
kernel-log matching is a literal-substring table. Single static binary, `cargo
test` for everything, no dependency supply chain. Decision made; moving on.

## 10. Testing strategy

Evaluators are pure functions over facts — exercised with in-code fixtures:
clean evidence, obvious degradation, ambiguous evidence, unavailable/unsupported
subsystems, malformed collector input. Parsers are tested against recorded
command/file samples (including pathological spacing and junk). The JSON emitter
and RFC 3339 clock conversion are unit-tested. **Mandated test:** kernel-log
collection failure yields `kernel-scream: UNKNOWN`, never `CLEAR`.

## 11. Later directions (not V0)

Fleet/SSH transport with MTTI aggregation, persistent receipt history, opt-in
bounded active probes, ZFS/network-filesystem evidence, machine-check sysfs detail,
alerting integrations. Each must earn its place against §7 before entering the tool.
