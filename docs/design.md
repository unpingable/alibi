# Alibi — design note

Status: normative. Where code and this note disagree, fix one of them deliberately.

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
| `CLEAR` | All **required** evidence sources were complete and no collected evidence indicates the failure class. |
| `DEGRADED` | Collected evidence contains at least one positive indication of the failure class, including retained evidence from a partial source. |
| `UNKNOWN` | No indication was found but at least one **required** evidence source was unavailable, unsupported, inaccessible, malformed, or incomplete; innocence was not established. |
| `NOT_APPLICABLE` | The accusation is semantically void on this host (V0 uses this only for `network-choke-point` on hosts with zero non-loopback interfaces). |

Rules:

- Every finding declares each source's role as **required** or **supporting** (§6).
  Supporting-source failure is recorded in the finding's `skipped` list and never
  blocks `CLEAR`; required-source failure or incompleteness forces `UNKNOWN` unless
  an indication forces `DEGRADED`.
- Partial source values are retained. Positive retained evidence can establish
  `DEGRADED`; partial required evidence cannot establish `CLEAR`.
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

## 4. Timing and MTTI semantics

- `t0` is one monotonic interrogation start, immediately after CLI parsing. Wall
  time is used only for the receipt's RFC 3339 `started_at`.
- Each finding carries `decided_at_ms`: duration from `t0` until its evaluator
  returned a terminal verdict. It is present for every verdict.
- Each finding also carries `innocence_at_ms`. It equals `decided_at_ms` for a
  `CLEAR` finding and is `null` for `DEGRADED`, `UNKNOWN`, and
  `NOT_APPLICABLE`. Semantic non-applicability is not itself an exoneration event.
- `decision_latency_ms` is the maximum `decided_at_ms` across requested
  accusations. It is always present; a terminal `UNKNOWN` or `DEGRADED` is a
  decision even though it is not innocence.
- `mtti_ms` is numeric only if every requested accusation is `CLEAR` or
  `NOT_APPLICABLE`; otherwise it is `null`. When innocence is established, it is
  the time when the last accusation was cleared or established semantically void:
  the maximum `decided_at_ms` across those findings.
- `interrogation_complete_ms` covers collection and evaluation immediately before
  receipt construction. Serialization and output I/O are excluded. It can be
  later than the last decision only if later work is contextual and incapable of
  changing an already recorded verdict.
- Evidence capable of changing a verdict must finish before that verdict. This
  prevents an early `CLEAR` from being overturned by a late required source.
- The receipt embeds these definitions as machine-readable strings so downstream
  tooling consumes the same semantics.

## 5. Architecture

Deliberately boring:

```
collector → normalized facts (+ per-source status) → failure-class evaluator → finding → receipt
```

- **Collection** (`collect`): plans the minimum source set needed for the selected
  accusations, then reads `/proc`, `/sys`, and a small set of standard tools
  (`dmesg`/`journalctl`, `df`) into typed facts. Unrequested collectors neither run
  nor appear in the receipt. A source records status
  (`ok | unavailable | unsupported | malformed`), completeness
  (`complete | partial | none`), retained typed values, detail, method, elapsed
  time, monotonic completion offset, and a sampling window when applicable.
  Collectors never interpret. Dependencies are explicit: for example, filesystem
  usage collection consumes the mount set rather than rediscovering it.
- **Evaluation** (`eval`): pure functions `facts → Finding`. No I/O, no command
  parsing. Fully fixture-testable. A shared, explicit substring-pattern table tags
  kernel-log lines to failure classes; no regex engine. Patterns carry per-pattern
  exclusion substrings so benign banners do not match (e.g. the `EDAC` needle
  excludes `Ver:`/`detected (`/version lines — a loaded EDAC core is not a memory
  error). The table is curated and versioned in one place; that is the mechanism,
  not a bug.
- **Presentation** (`receipt`): human renderer, zero-dependency JSON emitter.
  Verdict logic never contains parsing; parsing never contains verdicts.

The **interrogation envelope** is recorded on every receipt: each planned source's
name, status, completeness, method, elapsed time, monotonic completion offset, and
bounded sample window where applicable (plus boot time from `/proc/stat` btime).
Each finding separately records its source names and their required or supporting
roles. Alibi never says "this machine is healthy"; it says "within this explicitly
described envelope, no evidence supports these named accusations."

Kernel-log acquisition requests `dmesg --raw --force-prefix`,
then `journalctl -k -b --no-pager --quiet -o short-monotonic --no-hostname --all`.
Unsupported options or unavailable commands are collection failures. Successful
command stdout must be valid UTF-8. Log records must have the selected numeric
priority/timestamp framing (dmesg) or monotonic timestamp/identifier
(journal), with a complete final newline. Malformed neighboring records make a
source partial while valid records remain available for positive evidence.
These checks validate representation, not provenance, freshness, or complete
since-boot retention; a substituted command can imitate valid framing.

Each external command has an acquisition deadline covering child exit and both
output streams, with an 8 MiB limit per stream. Timeout, read, encoding and cap
failures discard command output and cannot become complete evidence. The
collector uses nonblocking streams without reader threads. Failure signals only
the dedicated child process group while its leader identity is unreaped, then
allows at most 100 ms of userspace cleanup polling. Before spawning, a read-only
SIGCHLD disposition check requires the default handler without SA_NOCLDWAIT;
ignored/custom handlers or auto-reaping flags refuse collection. This ABI check
is qualified only for Linux x86_64 GNU targets; other targets refuse command
collection rather than guessing the signal layout. The check changes no signal
handler. Any wait error invalidates ownership evidence and skips numeric group
signaling. Library callers must not concurrently reap collector children or
change SIGCHLD disposition; preflight cannot establish that invariant for an
arbitrary concurrent caller. Failure to reap is reported;
this is not proof that every descendant retired. Scheduling, spawn, and kernel
uninterruptible waits prevent a universal wall-clock guarantee. The two-second
network delta window is paid only when `network-choke-point` is selected.

## 6. Failure classes (V0)

Registry order: `kernel-scream`, `silent-killer`, `memrot`, `network-choke-point`.

### kernel-scream
*Has the kernel/hardware been screaming while monitoring stayed quiet?*

- Required: kernel log. Supporting: none in V0 (machine-check sysfs noted as future).
- Indications (log patterns): kernel panic / Oops / BUG / general protection fault;
  soft/hard lockup and stall reports; MCE / "Hardware Error"; PCIe AER errors;
  serious block I/O errors; filesystem errors (shared with silent-killer).
- Limitation: dmesg ring buffer may be wrapped on long-uptime hosts; the envelope
  records which acquisition method was used. V0 does not assume `mcelog` exists;
  modern kernels report MCEs through the kernel log.

### silent-killer
*Is the host superficially up but functionally damaged?*

- Required: mounts (`/proc/self/mounts`), space/inode facts (targeted `df` — see
  below), kernel log. The root filesystem must be inside the mount model: a root
  of a type the model does not reason about (ZFS, overlay, network filesystems)
  or a root missing from the table is incomplete required evidence and yields
  `UNKNOWN`, never a vacuous `CLEAR` over the remaining mounts. The root row is
  retained by the collector for this purpose even when non-material.
- Indications: write-capable block-backed filesystem (ext2/3/4, xfs, btrfs, f2fs,
  jfs, reiserfs) mounted `ro` — with a precision rule: a kernel `remount-ro` flips
  the whole filesystem, so an `ro` mount whose device is also mounted `rw`
  elsewhere is a deliberate read-only bind view (snap-style content sharing) and
  is recorded as evidence, **not** an indication. Only a write-capable device with
  no `rw` mount anywhere is an indication. squashfs/iso9660 and other
  read-only-by-design fstypes are excluded. Also: kernel-log remount-ro /
  fs-journal error / XFS forced-shutdown patterns (an XFS shutdown leaves the
  mount `rw`, so only the log witnesses it); space or inode free ≤ 5% on a
  material filesystem (critically low, not merely "high usage").
- Space/inode facts come from targeted `df -P` calls against the material
  filesystems only (block-backed filesystems deduped by device; each tmpfs is
  distinct). Whole-table `df` is deliberately not used: one inaccessible unrelated
  mount (e.g. a permission-restricted FUSE mount) must not destroy all space
  evidence for the host.
- Filesystem usage requires each df row's textual source and mountpoint to match
  the pair requested from the mount snapshot. A mismatch, extra row or ambiguous
  capacity columns makes that dimension inadequate while preserving other valid
  dimensions. This is name correspondence, not mounted-object or generation
  continuity: same-name replacement can remain undetected, and aliases/bind-view
  reporting differences conservatively yield UNKNOWN. Literal spaces are matched
  around the capacity columns without silently normalizing names. Empty or
  boundary-whitespace expected names are unsupported. GNU df's source padding
  can still hide trailing whitespace in an actual label, so correspondence is
  to the admitted printed representation, not arbitrary kernel-label identity.
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
  (`/proc/net/route`, `/proc/net/ipv6_route`), kernel log (link flaps,
  conntrack-full), and conntrack (`/proc/sys/net/netfilter/nf_conntrack_{count,max}`)
  whenever connection tracking is present. Every indication class the accusation
  names must be excludable before `CLEAR`; a denied kernel log or denied/malformed
  conntrack therefore yields `UNKNOWN`. Conntrack is the one applicability-scoped
  source: when its files do not exist the subsystem is absent, that failure class
  is not applicable, and the finding records conntrack as `supporting` (not
  present) rather than blocking `CLEAR`. Supporting otherwise: none.
- Indications: RX/TX error counter delta > 0 across the sample window; RX/TX drop
  delta > 0; ≥ 3 link-down events for one interface in the log window; no usable
  default route in either address family (worded as interpretation — legitimate
  on some appliances); conntrack usage ≥ 90%.
- A default route is usable only if it does not reject traffic and has an output
  device: rows carrying `RTF_REJECT` (`unreachable`/`prohibit` defaults, and the
  kernel's always-present IPv6 null entry `::/0` via `lo`) and IPv4 rows with no
  device (`*`, e.g. `blackhole default`) are not default routes for this purpose.
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
11. **A terminal decision is not necessarily innocence.** Decision latency is
    always reported; MTTI is unavailable when any requested accusation is
    `DEGRADED` or `UNKNOWN` (§4).

## 8. CLI and output contract

```
alibi check [flags] [accusation...]   # default: all four, registry order
alibi check --json                    # stable machine-readable receipt (stdout)
alibi check --verbose                 # human receipt with evidence detail
alibi explain <accusation>            # meaning, evidence, verdict semantics, limits
alibi --version                       # implementation version (CARGO_PKG_VERSION)
```

Human default is one line per accusation plus an overall verdict line, per the
project seed. The JSON receipt is the stable interface: additive changes only;
field removals, renames, and type changes are breaking. Every receipt declares
`"receipt_version": 2` and the producing `version`. V2 deliberately replaces
V1's unconditional bounded-conclusion `mtti_ms`, finding
`time_to_verdict_ms`, flat source envelope, and top-level `elapsed_ms`.

The serialization is pinned by a golden contract test
(`tests/golden/receipt_v2.json`); a diff against the golden means the contract
changed and must be regenerated deliberately, never casually. This is
intentionality, not a long-term compatibility promise. Top level: `tool`,
`version`, `receipt_version`, `host`, `started_at` (RFC 3339 UTC),
`interrogation_complete_ms`, `decision_latency_ms`, nullable `mtti_ms`,
`innocence_established`, `verdict`, `findings[]`, `envelope`, and `semantics`.
Each finding includes accusation, verdict, summary, indications, evidence,
skipped, `decided_at_ms`, nullable `innocence_at_ms`, and `sources[]` entries
with a name and `required`/`supporting` role. Envelope source entries contain
status, completeness, method/detail where available, `elapsed_ms`,
`completed_at_ms`, and an optional monotonic `window` with `started_at_ms`,
`ended_at_ms`, and `duration_ms`.

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
subsystems, malformed and partial normalized facts, source disappearance, and
counter reset. Parsers are tested against recorded command/file samples
(including pathological spacing and junk). Focused pure assembly seams cover
evaluation and receipt construction. General filesystem, command, sleep, and
clock injection, along with partial mount parsing, remain explicitly deferred.
The JSON emitter and RFC 3339 clock conversion are unit-tested. **Mandated test:** kernel-log
collection failure yields `kernel-scream: UNKNOWN`, never `CLEAR`.

## 11. Later directions (not V0)

Fleet/SSH transport with MTTI aggregation, persistent receipt history, opt-in
bounded active probes, ZFS/network-filesystem evidence, machine-check sysfs detail,
alerting integrations. Each must earn its place against §7 before entering the tool.
