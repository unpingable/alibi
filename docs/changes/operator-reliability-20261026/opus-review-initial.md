# Alibi review: `work/operator-reliability-20261026` against `2da0107`

**Scope and provenance.** This is a static review of the supplied source snapshot and diff. I executed nothing. The passing fmt, test, clippy and release results come from Sol's report and have not been independently reproduced, by me or by the parent.

Several referenced artifacts were not supplied, so I could not review them:
- `tests/read_receipt_test.py`
- the 3 CLI tests
- `PROVENANCE.md`

Historical field qualification of earlier executables does not transfer to this modified build. Issue #20 (acquisition reordering) is out of scope and not required here.

## Verdict summary

| ID | Class | Item |
|---|---|---|
| B1 | **Change blocker** | A partial mount table can produce a false DEGRADED: "read-only with no RW sibling" is inferred from absence, and a new test pins that behavior. |
| B2 | **Change blocker (test)** | The IPv6-disabled exception is untested where it matters (`collect_routes`); only the string predicate is tested. |
| B3 | **Change blocker (reference consumer)** | `read-receipt.py` can exit 1 (Alibi's DEGRADED code) on bad input, hides duplicate sources, ignores unknown roles and does not flag contradictions. |
| B4 | **Change blocker (docs)** | Source correspondence: `design.md §10` still says partial mount parsing is deferred; the bond paragraph overstates what was addressed. |
| I1 | Important, introduced | `fs-usage` can report `ok/complete` over a domain derived from a partial mount table. |
| I2 | Important, preexisting | Drop deltas produce false DEGRADED on ordinary active-backup bonds and for unknown-protocol drops. |
| I3 | Important, preexisting | There is no total runtime bound. The docs are currently correct; the reliability note must stay correct. |
| I4 | Important, preexisting | `CONFIG_IPV6=n` kernels (no `/sys/module/ipv6`) give permanent UNKNOWN for `network-choke-point` unless an IPv4 issue already gives DEGRADED. |
| N1–N4 | Nits | Unconditional scope line, unbounded mount detail, golden prose drift, guidance ordering. |

No false-CLEAR path was found in the change. `enforce_required_sources` structurally prevents CLEAR whenever any required source is not `ok/complete`, and partial mounts and routes are both non-complete.

The defects are false positives, missing tests, a non-conservative reference consumer, and documentation that claims more than the code does.

---

## B1. Partial mount evidence: a malformed row can hide the RW view (false DEGRADED)

**Mechanism.** `SilentKiller` raises "mounted read-only" for a write-capable RO row only when no RW row with the same device exists in the table.

That makes it a *relational, absence-based* inference, not a per-row positive. Before this change, any malformed row failed the whole mount source, so the inference only ever ran over a complete table.

Now `source_from_mounts` keeps valid rows and drops malformed ones. If the dropped row was the RW view of the same device, a deliberate RO bind view (snap-style) becomes an indication. DEGRADED outranks the root-missing and partial-source UNKNOWN, so the false positive reaches the verdict and exit code 1.

**Why the change made this reachable.** Two edits combine:
- The new `fields.len() != 6` check is stricter than the previous `< 6`.
- Per-line salvage means a malformed line no longer fails the whole source.

Kernel-produced `/proc/self/mounts` rarely contains malformed rows. The guard exists for unexpected or substituted input, and that is exactly the case where an absence-based inference is unsound.

**Reproduction.** Add this to `src/collect/mounts.rs` tests:

```rust
#[test]
fn malformed_row_cannot_turn_bind_view_into_degraded() {
    use crate::eval::{Evaluator, Verdict};
    let valid = "/dev/vda / ext4 rw 0 0\n/dev/vda /snap/x ext4 ro 0 0\n";
    for bad_rw in ["/dev/vda / ext4 rw 0 0 extra", "/dev/vda /\\9 ext4 rw 0 0"] {
        let mut facts = crate::eval::fixtures::clean_facts();
        // Control: complete table → bind view, CLEAR.
        facts.mounts = source_from_mounts(valid, std::time::Duration::ZERO);
        assert_eq!(crate::eval::silent_killer::SilentKiller.evaluate(&facts).verdict, Verdict::Clear);
        // Same table with the RW view malformed → currently DEGRADED; must be UNKNOWN.
        facts.mounts = source_from_mounts(
            &format!("{bad_rw}\n/dev/vda /snap/x ext4 ro 0 0\n"), std::time::Duration::ZERO);
        assert_eq!(crate::eval::silent_killer::SilentKiller.evaluate(&facts).verdict, Verdict::Unknown);
    }
}
```

The new test `malformed_neighbors_preserve_positive_and_negative_mount_observations` (the `ro` case expecting `Degraded`) currently enshrines the unsound behavior. It must be revised with the fix.

**Corrective direction (small).** In `SilentKiller`:
- When `!facts.mounts.is_complete()`, an RO write-capable row with no *observed* RW sibling goes to `required_failures` and `skipped`, not to indications. Suggested wording: "observed read-only; a malformed mount row may hold a read-write view of the same device".
- Capacity indications (independent df rows) and kernel-log indications remain valid DEGRADED positives.
- The existing `routes.is_complete()` guard on "no default route" is the right model to copy.

An optional refinement, not required: have the collector record the best-effort raw device token of each malformed row. RO inference could then stay positive when every malformed row's device is parseable and differs. Do not add this in this change unless it is cheap.

**Doc fix.** `envelope.md` says "Valid read-only mounts and capacity observations can still establish DEGRADED". Change it to: "Capacity observations and log reports can still establish DEGRADED; a read-only observation cannot, because a malformed row may be the read-write view of the same device."

The receipt's embedded `semantics.degraded` string ("retained evidence from a partial source") stays true only after the fix.

## B2. IPv6 missing-vs-disabled: the logic looks right but is untested at the collector

**Logic reviewed.** The new exception covers only this case:
- `/proc/net/ipv6_route` returns `ErrorKind::NotFound`, and
- `/sys/module/ipv6/parameters/disable` trims to `1`, `Y` or `y`.

In that case IPv6 becomes `Some(false)` and the source can be `ok/complete` with an explanatory detail. Denied or malformed tables do not qualify, and a present table is parsed. This matches the documentation.

It is also semantically reasonable. `ipv6.disable=1` leaves the module present but unregistered. The per-interface sysctl `disable_ipv6` leaves the table present, so it is correctly not treated as absence.

**The gap.** `collect_routes` hard-codes paths, so none of these claims are exercised. Only `ipv6_explicitly_disabled` is unit-tested.

**Required seam (small).** Extract a pure assembly function and keep `collect_routes` as a thin wrapper:

```rust
fn routes_from(v4: io::Result<String>, v6: io::Result<String>,
               disable: impl FnOnce() -> io::Result<String>, elapsed: Duration) -> Source<RouteFacts>
```

**Test matrix** (each case should also be evaluated through `NetworkChokePoint`):

| v4 | v6 | disable | Expect |
|---|---|---|---|
| default | NotFound | `"1\n"` | ok/complete, detail mentions disable; network CLEAR |
| default | NotFound | `"0"`, NotFound, PermissionDenied | partial/Unavailable; UNKNOWN |
| default | PermissionDenied | `"1"` | partial; UNKNOWN (no exception) |
| default | Ok(malformed) | `"1"` | partial/Malformed; UNKNOWN |
| no default | Ok(gateway default) | `"1"` | ok; IPv6 default honored, CLEAR |
| NotFound | NotFound | `"1"` | partial (v6 `Some(false)` makes it partial, not failed); UNKNOWN, never CLEAR |
| no default | NotFound | `"1"` | complete; DEGRADED "no default route" |

The `disable` closure must not be invoked unless v6 is NotFound. Test that by passing a panicking closure.

**Minor issues:**
- The DEGRADED wording "no default route in IPv4 or IPv6 tables" is inaccurate when the IPv6 table is absent by design. Consider "no usable IPv4 default route; IPv6 disabled".
- When v4 has an issue and IPv6 is disabled, the partial detail omits the IPv6 not-applicable reason.

## B3. Reference Python consumer

The script runs no commands and has no network, eval or subprocess calls. That is good: it is a stdin-to-stdout filter. It does print `host`, so digests remain sensitive receipts.

The defects matter because agents will copy the script.

1. **Exit-status collision.** `echo '[]' | python3 scripts/read-receipt.py` raises `AttributeError` (`list.get`), which is uncaught. Python then exits 1, which is Alibi's DEGRADED code.
   - The same happens for `"x"`, for `{"findings": 5, ...}` shapes that reach attribute access, and for `RecursionError` from deeply nested JSON.
   - Fix: validate `isinstance(receipt, dict)` and the type of every container. Catch `Exception` at top level and exit 2.
   - Consider a distinct code (e.g. 3) so "consumer could not read" is never confusable with Alibi's codes.
2. **Duplicate envelope sources, last wins.** The dict comprehension lets a later `kernel-log ok/complete` entry mask an earlier failed one. Reject duplicate names, and duplicate accusations too.
3. **Unknown roles are silently treated as supporting.** `role != "required"` simply skips. Reject any role not in `{required, supporting}`. Also validate each finding's `verdict` against the enum; today only the top-level verdict is checked.
4. **No contradiction checks.** A conservative reader should reject:
   - a finding with verdict `CLEAR` whose `inadequate_required_sources` is non-empty;
   - a top-level `verdict` that is not the maximum by rank (DEGRADED > UNKNOWN > CLEAR > NOT_APPLICABLE) over the findings;
   - `innocence_established` inconsistent with the findings;
   - `mtti_ms` non-null while any finding is DEGRADED or UNKNOWN.
5. `receipt_version == 2` also accepts `2.0`. This is harmless but worth an `isinstance(..., int)` check.
6. **Optional, not required:** `--expect-accusations` and `--alibi-exit N` flags, so the documented cross-checks ("check selected accusations", "preserve exit status") are mechanized rather than prose-only.

Each fix needs a test in `tests/read_receipt_test.py`, which I have not seen:
- `[]`
- `"x"`
- duplicate source names
- `role: "Required"`
- CLEAR plus a failed required source
- top-level verdict mismatch
- deep nesting

Each must exit 2 with no traceback.

## B4. Documentation and source correspondence

- **`design.md §10` is stale.** It still says "partial mount parsing remain[s] explicitly deferred". `§6 silent-killer` should state the partial-table rule from B1.
- **Test names are now misleading.** `malformed_line_fails_whole_parse` and `malformed_root_row_fails_whole_parse` remain true of `parse_mounts` but not of the collector. Rename them, e.g. `parse_mounts_rejects_*`.
- **The deleted root-retention comment in `parse_mounts` carried a real invariant.** It explained why non-material root rows are kept. Restore it.
- **The bond paragraph overstates the change.** The bond "fix" is a regression pin with no code change. Its fixture clones `eno1`, and `is_virtual` does not affect evaluation.
  - The text "checks observed errors and drops" is accurate.
  - Presenting this as addressing "ordinary bond configurations" is not, see I2.
  - Reword it as a statement of current behavior and add the drop-counter caveat.
- **`agent-interface.md` overstates human coverage.** It says human output suggests next observations for "unsupported root filesystems". That guidance lives in the finding summary and `skipped` text, which also appear in JSON, not in the `next investigation:` mechanism.
  - Source-specific guidance exists only for `edac/unsupported` and `kernel-log/unavailable`.
  - Other cases fall back to generic categories: kernel-log malformed or partial, EDAC unavailable or malformed, mounts malformed, routes partial.
  - That is acceptable scope; just state it accurately.
- **`design.md §9` conflicts with `AGENTS.md`.** "Single static binary" contradicts "dynamically linked". This is preexisting; fix it opportunistically.

## I1. `fs-usage` completeness over a partial domain (introduced)

`fsusage::collect` takes `mounts.get()`, which now returns `Some` for a partial table. It can therefore return `ok/complete` while the filesystem in the malformed row was never measured. CLEAR is still blocked by the mounts source.

The risk is that a per-source digest, like the reference consumer, reports `fs-usage` adequate.

Fix: when the input mount source is not complete, mark `fs-usage` as `partial` with the detail "domain derived from partial mount table". Add one assertion to B1's test.

## I2. Drop counters on ordinary configurations (preexisting false DEGRADED)

`rx_dropped` routinely increases in benign cases:
- on active-backup bond masters and inactive slaves, for frames arriving on the backup;
- on many NICs, for unknown-protocol or VLAN frames.

The two-second delta then gives DEGRADED. This change does not alter that behavior, so it is not a blocker for the change. However, the docs and the reliability note must say that a drop-only DEGRADED needs interpretation.

A behavior change (separating drops from errors, or excluding inactive slaves) is new scope requiring a design note.

## I3. Total timeout (preexisting; docs are currently accurate)

There is no aggregate deadline:
- kernel log: up to 3 s (dmesg) plus 5 s (journal), plus cleanup allowance;
- df: 2 × 3 s *per material filesystem*, run sequentially;
- network: a 2 s sleep;
- `/proc` and `/sys` reads: no deadline at all.

Twenty hung df targets alone add about 120 s.

`design.md` and `envelope.md` correctly disclaim a universal bound. Nothing in this change claims a total timeout, and the reliability note must not either. Agents should apply an outer timeout and treat being killed as "no receipt", never CLEAR.

## Nits

- **N1.** The human `observation scope:` line mentions mounts, EDAC and network even in a `kernel-scream`-only run, which can suggest those sources were collected. Consider "where collected".
- **N2.** The mount `detail` embeds every malformed raw line with no bound. Cap it at the first N lines plus a count.
- **N3.** Golden receipt prose differs from current evaluator strings (memrot summary, kernel-scream summary, the net `method`). This is harmless for a serialization golden, but `agent-interface.md` points readers at it. Add a sentence saying its strings are illustrative, which reinforces "do not match prose".
- **N4.** The `next investigation:` line prints before the `uncertainty:` line it refines. This is cosmetic.

## Formalization consideration

**P1. No false CLEAR.**
- *Proposition:* CLEAR ⇒ every required role's source is `ok ∧ complete` ∧ no indications.
- *Holds by construction:* `enforce_required_sources` runs after every evaluator, and the NOT_APPLICABLE early return is also subject to it.
- *Assumptions:*
  - roles are declared correctly;
  - conntrack `NotFound` on the count file means the subsystem is absent;
  - IPv6 `NotFound` plus the disable parameter means the family is absent.
- *Counterexamples found:* none. Focused tests suffice; a generic test can assert P1 over all evaluators by mutating each required source to each non-complete state.

**P2. DEGRADED monotonicity (soundness of positives).**
- *Proposition:* for every indication derived from a partial source S, the indication persists under every completion S′ ⊇ S. In plain terms: adding the missing rows never removes it.
- *Satisfied by:*
  - df capacity rows (per-dimension and independent);
  - log matches (per-line);
  - counter deltas (per-interface);
  - "no default route", which is guarded by `is_complete`.
- *Counterexample:* B1. RO-without-RW-sibling is an existential-negation over the whole table, so it is non-monotone.
- *Do focused tests suffice?* Yes. A small exhaustive, std-only model check is cheap: enumerate mount tables of ≤3 rows over 2 devices × {ro, rw}; for each table, drop each row (simulating malformed); assert that the indications of the partial table are a subset of the indications of the full table. That covers the class B1 belongs to and catches future relational indications. A formal tool is not warranted.

**P3. Route applicability.**
- *Proposition:* `v6` is not-applicable ⇔ `NotFound(ipv6_route) ∧ disable ∈ {1, Y, y}`.
- The input space is finite, so B2's matrix is exhaustive over error kinds that matter.

**P4. Bounded runtime.**
- No such proposition is offered. Per-command deadlines are bounded only modulo scheduling and D-state, and file reads are unbounded. Do not formalize this; document it as a non-guarantee.

## Required before merge

1. Fix B1 and replace the `ro → Degraded` test case with the reproduction above.
2. Add the B2 seam and matrix.
3. Fix B3 items 1–4, with consumer tests.
4. Make the B4 doc edits (design §10, §6; envelope partial-mount and bond wording; agent-interface guidance claim).
5. Recommended in the same change: I1.

Then rerun fmt, test, clippy and release. The parent should independently execute before acceptance. Suggested commit subject, without attribution: `mounts: retain valid rows beside malformed ones without absence-based RO inference`. Reviewer attribution belongs in `PROVENANCE.md` or review artifacts only.

---

# Draft: Operator and agent reliability note

**What a verdict means**
- **CLEAR:** for the *selected* accusation, every required source was acquired completely and nothing Alibi recognizes indicated the failure class. It is not a health attestation, not a statement about remote reachability, and not a claim about history before the retained log.
- **DEGRADED:** at least one positive observation was found. Positives can survive incomplete neighboring evidence. Inspect every indication; some need judgment:
  - a deliberate read-only mount;
  - an absent default route on an appliance;
  - drop counters on active-backup bonds or for unknown-protocol frames.
- **UNKNOWN:** innocence was not established. Look at the required source's status and completeness to find out why. Additional privilege helps only for access denials, chiefly the kernel log. It cannot create absent EDAC, and it cannot extend the mount model to ZFS, overlay or NFS roots.
- **NOT_APPLICABLE:** semantically void (no non-loopback interfaces). It is not exoneration.

**For agents**
1. Run `alibi check --json` under your own outer timeout. Alibi has per-command deadlines but **no total runtime bound**. Many filesystems or hung devices can extend a run substantially.
2. Never treat any of these as CLEAR: kill, timeout, missing or invalid JSON, unexpected `receipt_version`, or missing findings.
3. Keep Alibi's exit status (0, 1, 2, 64) separate from any consumer's exit status.
4. Confirm the findings match the accusations you requested.
5. Join `findings[].sources` to `envelope.sources` by name. Reject receipts with duplicate source names or unknown roles.
6. `completeness` describes acquisition, not historical coverage. `completed_at_ms` and `window` are collection timing, not event ages. Retained log reports and cumulative EDAC counts do not establish that a fault is current, recent, or resolved.
7. `detail`, `skipped`, `evidence` and `summary` are explanations, not an API. Do not branch on their text. Receipt V2 has no typed fields for permission denial, event age, or IPv6 not-applicable.
8. Alibi is read-only and never probes the network. Its suggestions are observations to consider, not authorization for follow-up actions.

**Known limitations to carry forward**
- A malformed mount table yields at best partial silent-killer evidence. After the pending fix, read-only observations from a partial table do not establish DEGRADED by themselves.
- A missing IPv6 route table counts as not applicable only when `/sys/module/ipv6/parameters/disable` is `1`. Kernels built without IPv6 therefore keep `network-choke-point` UNKNOWN unless an IPv4 issue already gives DEGRADED.
- On most VMs (no EDAC), a run of all accusations cannot reach aggregate innocence.
- Results from field qualification of earlier builds do not apply to this build until it is re-qualified.
