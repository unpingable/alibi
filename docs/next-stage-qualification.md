# Next-stage campaign qualification report

Date: 2026-09-23

## Outcome

This campaign made Alibi narrower in what it claims and stronger in how it
handles incomplete evidence. It did not add either proposed accusation.

| Change | Qualification | Reason |
|---|---|---|
| Evidence-engine hardening | **QUALIFIED WITH LIMITATIONS** | Required/supporting source roles are explicit; partial evidence is retained; incomplete required evidence cannot authorize `CLEAR`; known network, route, EDAC, filesystem-usage, and empty-log failure paths now fail closed. General I/O and clock injection and partial mount parsing remain deferred. |
| `resource-starvation` | **DEFERRED** | Host-local PSI measures contention and stall time, not “useful progress.” CPU pressure can coexist with productive work, and no cross-host pathological threshold survived review. |
| `storage-pathology` | **DEFERRED** | Strong positive storage events exist, but no portable negative evidence matrix can exonerate the relevant NVMe, SATA/SCSI, USB, virtio, device-mapper, MD, partition, and virtualized paths. |
| MTTI / timing semantics | **QUALIFIED** | Receipt V2 separates terminal decision latency from time to innocence and makes MTTI absent when aggregate innocence was not established. |

The explicit evidence contracts and deferral gates are recorded in
`docs/next-stage-campaign.md`.

## Repository state and commits

- Starting commit: `48774753350fe321d288ea26411a1fabadcccb9f`
- Design-contract commit: `3689ed3` (`Document next-stage accusation and MTTI contracts`)
- Qualified implementation commit: `39cdea5` (`Harden evidence handling and correct MTTI semantics`)
- No push was performed.

The final report-only commit, if present, is intentionally reported by the
campaign handoff rather than self-referenced here.

## Admitted implementation

The evidence model now records status separately from completeness
(`complete`, `partial`, or `none`). Partial typed values remain available to
evaluators, so known positive evidence can establish `DEGRADED`; a centralized
required-source guard prevents partial or absent required evidence from
establishing `CLEAR`.

Each evaluator declares required and supporting source roles. The CLI builds a
source plan from only the selected accusations, and the receipt envelope is
derived from that actual plan. A selected `kernel-scream` check therefore does
not run or wait for the two-second network sampler.

The admitted hostile-path repairs include:

- missing, malformed, decreasing, and disappearing network counter samples;
- interface topology changes during the bounded sample;
- independent retention of positive counter deltas when another counter fails;
- truthful `unavailable`/`reset` wording for contextual packet counters;
- independent IPv4/IPv6 route-table completeness and malformed-row detection;
- EDAC absence versus access failure and partial per-controller counters;
- filesystem byte/inode dimensions retained independently across every target;
- conntrack absence distinguished from access failure;
- empty kernel-log output and journal “no entries” output rejected as negative
  evidence;
- source completion offsets and actual bounded sampling windows serialized in
  the receipt.

## MTTI and receipt decision

Receipt V2 is a deliberate breaking contract change:

- `time_to_verdict_ms` becomes per-finding `decided_at_ms`;
- `innocence_at_ms` is numeric only for `CLEAR`;
- `decision_latency_ms` always reports the last terminal decision;
- `mtti_ms` is numeric only when every selected accusation is `CLEAR` or
  `NOT_APPLICABLE`, and is otherwise `null`;
- `interrogation_complete_ms` replaces the ambiguous top-level `elapsed_ms`;
- findings declare source roles;
- envelope sources declare completeness, completion offset, and optional
  sample-window offsets.

`RECEIPT_VERSION` changed from 1 to 2. The V1 golden and fictional examples
remain as historical artifacts; V2 has a separately reviewed golden and
fictional examples. Consumers that require numeric `mtti_ms`,
`time_to_verdict_ms`, or top-level `elapsed_ms` are not backward compatible and
must branch on `receipt_version`.

## Verification

Baseline at the starting commit:

- `cargo test`: 57 passed (55 unit, 2 golden integration), 0 failed.

Final qualification at implementation commit content:

- `cargo fmt --check`: passed;
- `cargo test`: 82 passed (80 unit, 2 golden integration), 0 failed;
- `cargo clippy --all-targets -- -D warnings`: passed;
- `cargo build --release`: passed;
- `git diff --check`: passed;
- deliberate golden regeneration:
  `ALIBI_UPDATE_GOLDEN=1 cargo test receipt_v2_matches_golden`: passed.

An independent semantic review rejected the first implementation pass for
remaining partial-evidence losses and a journal no-entry false-`CLEAR` path.
Those findings were repaired and the second review found no remaining
false-`CLEAR` blocker.

## Post-review repairs (same day)

An independent review of the qualified commit found five accusation-level
paths where `CLEAR` could be issued although a named indication class was
undetectable. None involved the centralized required-source guard, which held.
All five were repaired narrowly, without a receipt-contract change:

1. Route parsers counted rejecting default routes (`RTF_REJECT`), including
   the kernel's always-present IPv6 null entry, as usable default routes, so
   the no-default-route indication could never fire on an IPv6-enabled host.
   Both parsers now ignore rejecting rows, IPv4 ignores device-less rows
   (`blackhole default`), and malformed flags fail closed.
2. `network-choke-point` treated the kernel log and conntrack as supporting,
   so a denied log or denied/malformed conntrack turned a would-be `DEGRADED`
   (flap storm, table-full, table pressure) into `CLEAR`. Both are now
   required; conntrack is required only while the subsystem is present, and
   an absent subsystem is recorded as not applicable (`supporting`, not
   present) rather than blocking `CLEAR`.
3. `silent-killer` issued a vacuous `CLEAR` when the root filesystem was of a
   type outside the mount model (ZFS, overlay, network filesystems) or absent
   from the table. The collector now retains the root row regardless of type,
   and the evaluator requires the root to be inside the model; otherwise
   `UNKNOWN`. This is an epistemic repair, not filesystem-support expansion.
4. The XFS forced-shutdown message (`Shutting down filesystem.`) matched no
   pattern although the mount stays `rw`; it is now a silent-killer and
   kernel-scream indication.
5. Link-down messages from phylib/r8169 (`Link is Down`), tg3/bnx2x
   (`Link is down`), mlx4 (`Link Down`) and mlx5 (`Link down`) did not match
   the case-sensitive flap needles; those exact forms were added.

Test count after repair: 104 (102 unit, 2 golden). The V2 golden is unchanged.
Lower-priority review observations (human receipt omits the envelope,
`mtti_ms` equals `decision_latency_ms` under sequential collection, duplicated
`skipped` entries after a guard flip, non-hard post-kill timeout, permanent
`UNKNOWN` on `ipv6.disable=1` and bonding hosts, documentation drift) remain
deferred and are not part of this repair.

## Unresolved epistemic limitations

- Kernel-log acquisition still cannot prove that the ring or journal covers the
  entire boot. Method and boot time bound the claim, but freshness and earliest
  retained event are not established.
- `/proc/self/mounts` parsing remains all-or-nothing. A malformed later row can
  discard an earlier positive mount fact and produce `UNKNOWN`; it cannot
  produce `CLEAR`.
- Filesystem paths, commands, sleep, and monotonic time do not yet have one
  general injectable platform seam. Focused pure helpers cover the hostile
  assembly cases admitted in this campaign, but collector timeout and arbitrary
  permission/disappearance sequences are not all deterministic unit fixtures.
- Command timeout is represented as unavailable detail rather than a distinct
  structured status, and stale evidence has no dedicated status.
- Collection and evaluation are sequential. `decided_at_ms` is the actual time
  the evaluator returned, not the earliest theoretical time its evidence might
  have sufficed; another selected accusation can delay it.
- Existing V0 policy choices remain bounded rather than universal: no default
  route may be legitimate for specialized hosts, and EDAC corrected counts are
  lifetime observations rather than trends.

## Exact deferred work

1. Revisit `resource-starvation` only with a renamed/narrowed accusation and a
   defensible sustained-stall boundary. Required PSI records, sample windows,
   reset behavior, and supporting-only memory/swap/task facts must be specified
   before code.
2. Revisit `storage-pathology` only after mountinfo major:minor topology,
   recursive device-mapper/MD/partition resolution, transport-specific required
   evidence, log attribution, and shared evidence identity have a credible
   `CLEAR` contract.
3. Add a small internal filesystem/command/clock test seam for deterministic
   permission, disappearance, timeout, and timing fault injection. This must not
   become a plugin or configuration framework.
4. Preserve partial mount facts when later mount rows are malformed.
5. Consider structured issue kinds for permission denied, timeout, disappeared,
   counter reset, and stale/indeterminate coverage without weakening the
   existing fail-closed rule.

No daemon, remote execution, persistence, remediation, plugin framework,
generic command execution, application-health logic, or additional Linux
telemetry was admitted.
