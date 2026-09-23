# Next-stage defensibility campaign

Status: decision record and implementation ledger, 2026-09-23.

This campaign starts at commit
`48774753350fe321d288ea26411a1fabadcccb9f`. The starting worktree was clean
and the baseline suite passed 57 tests (55 unit and 2 receipt-golden tests).

The campaign is governed by one admission rule: additional telemetry is not a
reason to add an accusation. An accusation ships only when its positive and
negative claims are both defensible.

## Ground truth

Alibi is a read-only, on-demand, single-host interrogator. Collection produces
typed facts with per-source status; pure evaluators turn facts into findings;
receipt code aggregates and renders those findings. Collection failure must
never authorize `CLEAR`. `DEGRADED` outranks incomplete evidence when the
available evidence already establishes the accusation. `NOT_APPLICABLE` is a
semantic judgment, not a substitute for unavailable evidence.

Receipt V1 contains an unconditional numeric `mtti_ms`, per-finding
`time_to_verdict_ms`, and a flat source envelope. Its source states are `ok`,
`unavailable`, `unsupported`, and `malformed`. Required versus supporting
sources exist only in prose and evaluator control flow.

The useful existing test seams are pure evaluators and parsers for mounts,
`df`, routes, and kernel-log patterns. Filesystem roots, commands, sleep, and
the monotonic clock are not injectable. The most important observed contract
gaps are:

- missing or malformed interface counters, interface disappearance, and
  counter reset can be treated as a zero delta and permit `CLEAR`;
- an unreadable IPv6 route table is treated as a negative route observation;
- EDAC directory errors can be mistaken for an unsupported subsystem and
  directory-entry failures are discarded;
- a partial `df` collection discards positive evidence already collected;
- a successful but empty kernel-log command is accepted as negative evidence;
- source roles and sampling windows are not structured receipt provenance;
- every invocation collects every source, so an unrelated two-second network
  sample delays a selected `kernel-scream` check;
- current `mtti_ms` is maximum terminal-decision latency, including
  `UNKNOWN` and `DEGRADED`, rather than time to innocence.

The evidence-engine work admitted here is intentionally narrow: preserve
partial values, fail closed on incomplete required evidence, expose source
roles, record bounded sample provenance, and plan collection from the selected
accusations. A general plugin or configuration framework is out of scope.

## Proposed accusation: `resource-starvation`

### Proposed claim

> Is this host technically alive but unable to make useful progress because
> host-local CPU, memory, or I/O pressure is pathological?

### Evidence contract required for admission

The broad claim is **deferred**. “Useful progress” is workload-relative. Linux
PSI measures time lost to contention; it does not determine whether the work
being delayed is useful, whether a fully utilized CPU is pathological, or
whether an application SLO is being met.

If revisited, the minimum contract is:

| Element | Contract |
|---|---|
| Domain | Host-wide kernel-observed resource stalls, not workload latency or utilization. |
| Required for `CLEAR` | Complete, well-formed first and second samples of CPU, memory, and I/O PSI over one recorded monotonic window. Expected records and fields must all be present and cumulative totals must not decrease. |
| Supporting only | `MemAvailable`, runnable/blocked task snapshots, and bounded swap deltas. None may degrade the accusation alone. |
| `DEGRADED` | A future contract must define sustained pathological stall in the accusation itself. CPU `some`, nonzero PSI, low available memory, swap activity, load, and D-state tasks are insufficient individually. |
| `UNKNOWN` | PSI missing, denied, malformed, partial, disappearing, or reset; or a sample window that cannot be established. |
| `NOT_APPLICABLE` | Not permitted merely because PSI is absent. The failure class remains meaningful even when the kernel cannot expose it. |
| Provenance | Both sample offsets, actual window duration, every parsed PSI record, reset detection, and supporting-source status. |

Memory and I/O `full` PSI are the strongest candidates because they represent
intervals in which all non-idle work is stalled. No cross-host duration or
percentage threshold was justified in this campaign. CPU PSI `some` explicitly
allows productive CPU work and cannot establish host-wide inability to make
progress. Consequently no evaluator or receipt entry is admitted.

Hostile cases retained for a later contract are: PSI absent, permission denied,
malformed or partial; transient pressure; high utilization without stalls;
sustained full stalls; inactive versus thrashing swap; benign blocked tasks;
counter reset; source disappearance; timeout; and conflicting supporting
evidence.

## Proposed accusation: `storage-pathology`

### Proposed claim

> Is the kernel-visible local storage path backing an in-scope filesystem
> degrading before filesystem consequences make the host functionally unsafe?

This is distinct from `silent-killer`: storage-pathology would accuse the
underlying path; silent-killer accuses filesystem-level functional damage.

### Evidence contract required for admission

The claim is **deferred**. Linux exposes strong positive storage events but no
portable, universal negative telemetry that exonerates NVMe, SATA/SCSI, USB,
virtio, device-mapper, MD, partitions, and cloud-virtualized paths. Device
presence, a `running` state, block I/O completion statistics, or a vendor
summary health bit cannot independently authorize `CLEAR`.

If revisited, the minimum contract is:

| Element | Contract |
|---|---|
| Domain | Kernel-visible local block paths backing mounted, material, write-capable filesystems. The virtualization boundary must be explicit. |
| Topology | Start from mountinfo major:minor and recursively resolve partitions, device-mapper and MD dependencies to guest-visible leaves. RAM-backed filesystems are outside the domain; loop handling must be explicit. |
| Required for `CLEAR` | Complete topology, attributable kernel-log coverage, and a per-transport telemetry matrix that has survived review. A healthy summary bit is never sufficient. |
| `DEGRADED` | Attributable resets, transport/controller errors, medium errors, timeouts, offline/non-running path state, or array degradation. One shared kernel event may support multiple accusations but remains one observation. |
| `UNKNOWN` | Missing topology, missing applicable telemetry, inaccessible device data, indeterminate log coverage, unsupported transport, or disappearing device. |
| `NOT_APPLICABLE` | Only when no in-scope local block path backs a material write-capable filesystem. Missing telemetry is not non-applicability. |
| Provenance | Stable evidence identity, device major:minor, resolved dependency path, acquisition method, and log/window attribution. |

Hostile cases retained for later work are virtual and loop devices, partitions,
device-mapper and RAID fan-out, missing sysfs telemetry, unavailable NVMe data,
healthy vendor state alongside kernel resets, writable filesystems over a
degrading path, permissions, disappearance, and duplicated log evidence.

## MTTI decision record

The V1 definition is rejected for future receipts. A terminal `UNKNOWN` or
`DEGRADED` is a decision, not innocence. Renaming bounded-conclusion latency to
MTTI weakens the product's central claim even when the receipt discloses the
redefinition.

Receipt V2 uses these semantics:

- `decided_at_ms` is always present per finding and is measured from one
  monotonic interrogation start until its evaluator returns a terminal verdict.
- `innocence_at_ms` is present only for `CLEAR`. `NOT_APPLICABLE` has a decision
  time but is not itself an exoneration event.
- `decision_latency_ms` is the maximum `decided_at_ms` across requested
  accusations and is always present.
- `mtti_ms` is present as a number only when every requested accusation is
  `CLEAR` or `NOT_APPLICABLE`; otherwise it is `null`.
- When innocence is established, aggregate MTTI is the time when the last
  requested accusation was cleared or established semantically void.
- `interrogation_complete_ms` is the duration through collection and
  evaluation, immediately before receipt construction. Serialization and
  output I/O are not included.
- Wall time is used only for `started_at`; all durations use a monotonic clock.
- Evidence capable of changing a verdict must finish before that verdict.
  Evidence allowed to continue later is contextual only and cannot overturn a
  recorded `CLEAR`.
- Collection is planned from the selected accusations. Unrequested collectors
  do not delay a decision or appear in the envelope.

The nullable type and changed meaning of `mtti_ms`, renamed finding timing, new
decision latency, and explicit source roles are a deliberate breaking receipt
change. `RECEIPT_VERSION` therefore advances from 1 to 2. V1 fixtures remain in
history; V2 becomes the current golden contract.

## Admission summary

| Proposed change | Decision |
|---|---|
| Focused evidence-engine hardening | QUALIFIED WITH LIMITATIONS |
| `resource-starvation` | DEFERRED |
| `storage-pathology` | DEFERRED |
| MTTI semantic correction / receipt V2 | QUALIFIED |

The engine qualification does not promise complete filesystem/command/clock
injection in one step. It admits the minimum safety work needed to remove known
false-`CLEAR` paths and make later fault injection incremental and explicit.
