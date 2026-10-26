# Alibi final static review: `6db99068` (successor to `79a48c8`)

## Disposition

**Accept the scoped change. No blockers remain.**

This is a static source review only; I executed nothing. The following were executed by the parent, and their results are taken from the supplied JSON log hashes, not reproduced by me:

- fmt
- 141 Rust unit tests, 1 golden test, 3 CLI tests
- clippy with all targets and `-D warnings`
- release build
- 6 Python tests
- `git diff --check`

Successor live smoke runs were not supplied to me, so I make no claim about them. The previous build's live runs do not qualify this revision. No field trial, guest-fault run, or bonded-host run backs any new behavior.

## Status of prior items

| ID | Status | Basis (source reading) |
|---|---|---|
| B1 | **Fixed** | Rule: a read-only (RO) write-capable row with no *observed* read-write (RW) sibling becomes an indication only when `facts.mounts.is_complete()`. Otherwise it becomes `evidence` ("observed read-only; mount enumeration incomplete…"). Effect: with no other positive, `base_verdict` yields CLEAR and `enforce_required_sources` then forces UNKNOWN. Test coverage: ro/rw rows, both malformed-row orderings, a retained RW sibling, and independent capacity and kernel-log positives that still reach DEGRADED. The old `ro → Degraded` pin has been replaced. |
| B2 | **Fixed** | `routes_from_reads` seam added. The matrix covers missing, denied, malformed, present and empty v6 tables, and disable values `1`, `0`, `Y`, missing and denied. Each case is evaluated through `NetworkChokePoint`. The `&&` short-circuit evaluates `disable` lazily, and a panicking closure proves it is not called for present, malformed or denied tables. Two further cases are covered: v4 missing with IPv6 disabled gives partial/UNKNOWN, and v4 without a default with IPv6 disabled gives DEGRADED. The v4 failure detail now carries the IPv6 not-applicable reason. |
| B3 | **Fixed** | **Input validation:** the reader requires a dict and type-checks containers. It rejects duplicate JSON keys, duplicate findings, sources and roles, unknown enums, `receipt_version` given as bool or float, and an empty or duplicate `--expected`. **Contradiction checks:** CLEAR or NOT_APPLICABLE with an inadequate required source; aggregate rank; the innocence flag; MTTI nullity; the indications-to-DEGRADED biconditional; and an optional `--exit-status` cross-check. **Failure exit:** `ValueError` (which includes `JSONDecodeError` and `UnicodeDecodeError`), `KeyError`, `TypeError` and `RecursionError` are caught and exit 2 with no traceback. A CLI test covers `[]`, `"x"`, 2000-deep nesting, and duplicate keys. |
| B4 | **Fixed** | §10 and §6 now state the complete-enumeration rule. The envelope's partial-mount paragraph is corrected. The bond paragraph is restated as a fixture regression with a drop-counter caveat. Tests are renamed to `parse_mounts_rejects_*`, and the root-retention comment is restored. The §9 "static" wording is corrected. `agent-interface.md` now states the actual extent of the next-step mechanism. |
| I1 | **Fixed** | `assemble_in_domain` downgrades an `Ok` status to the mount source's failing status (or `Unavailable`). It sets completeness to `Partial` when a value exists and appends "domain derived from partial mount table". Test: retained low capacity gives DEGRADED; benign capacity gives UNKNOWN. An empty retained domain gives `Malformed/partial` with an empty vector, which is never adequate. |
| I2 | **Specific claim withdrawn** | I accept the adjudication. I cannot cite a current-kernel source path showing that inactive-backup bond delivery increments `rx_dropped`; the cited statistics documentation assigns it to `rx_nohandler`. My recollection is that kernels predating `rx_nohandler` (roughly pre-4.6) behaved differently, but that is unverified here and requires no change. The general ambiguity of `rx_dropped` (unsupported protocols, filtering) is documented and preserved. |
| I3 | Docs correct | `agent-interface.md`, `envelope.md` and `design.md` all disclaim a universal runtime bound and direct agents to apply an outer timeout. |
| I4 | Accepted as documented limitation | The parameter is integer `0444`, and only `1` is accepted. A kernel without the parameter remains unresolved. This includes modular IPv6 not loaded and `CONFIG_IPV6=n`. |
| N1 | Fixed | The scope line is now gated on the selected findings' source roles, and a test asserts the mount and network lines are absent from runs that did not select them. |
| N3 | Fixed | The golden prose is labeled illustrative. |
| N2, N4 | Remain | Non-blocking; see below. |

## Remaining non-blocking observations

None of these affects verdict soundness:

1. **Consumer exit 2 shares the integer used by Alibi's UNKNOWN.** The docs state that the consumer's status is a separate channel, which is adequate. A distinct code (for example 3) would be more robust for copy-paste agents. `BrokenPipeError` on stdout and `KeyboardInterrupt` remain uncaught; this is an edge case.
2. **The mount `detail` embeds every malformed raw line without a cap (N2).** A hostile or huge table can bloat receipts.
3. **The DEGRADED wording "no default route in IPv4 or IPv6 tables" is imprecise when IPv6 is disabled by parameter.** The route `detail` records the actual reason.
4. **On a partial table, `root_mount` ("last root row wins") can select an earlier root row if the over-mounting row was malformed.** This can only add an UNKNOWN reason; it cannot cause CLEAR or DEGRADED. Its `skipped` text may then name the wrong filesystem type.
5. **`agent-interface.md` has a mild internal tension.** Paragraph 5 still lists "unsupported root filesystems" alongside the next-step suggestions, while the final paragraph correctly scopes the mechanism. Merging the two would remove it.
6. **The `next investigation:` line precedes its `uncertainty:` line (N4).** Cosmetic.

## Formal properties, re-evaluated

**P1 (guard): CLEAR ⇒ every source declared required in the finding's roles is `ok ∧ complete`.**

- It holds by construction. `enforce_required_sources` runs after every evaluator, including the NOT_APPLICABLE early return.
- It is a *necessary* guard property, not a proof that false CLEAR is absent. Complete acquisition can still miss real faults:
  - unrecognized log text;
  - pre-boot or overwritten history;
  - same-name df replacement;
  - out-of-model failure classes.
- Two applicability decisions sit outside the guard. Both rely on affirmative evidence and are tested:
  - **Conntrack:** the evaluator downgrades conntrack to `supporting` on `Unsupported`, which arises only from `NotFound` on the count file.
  - **IPv6:** the collector maps a missing IPv6 table to `Some(false)` only on `NotFound ∧ disable == "1"`.
- NOT_APPLICABLE needs only a complete `net` source. That is intentional and tested.
- No counterexample was found.

**P2 (monotonicity of positives under partial sources).**

The B1 counterexample is closed. I re-checked the remaining derivations for relational or absence-based inference:

- df capacity: per filesystem, per dimension;
- kernel-log matches: per line;
- link-down count ≥ 3: monotone in the retained lines;
- counter deltas: per interface, per counter;
- conntrack ratio: a single read;
- "no default route": guarded by completeness;
- `bind_views`: negative evidence only.

I found no further counterexample. The focused tests suffice for this change. The exhaustive small-model check remains an optional guard against future relational indications, not a requirement.

## Assumptions behind this disposition

- The supplied file contents are the complete diff surface of `6db99068` relevant to this change. Unshown modules (`collect/mod.rs`, the JSON emitter, `kernel_scream`, `memrot`, fixtures) are unchanged in ways that would matter. In particular, `Source::partial`, `is_complete` and `failure_reason` behave as their use implies.
- The parent's execution log hashes correspond to this revision.
- Kernel facts used:
  - `/proc/self/mounts` is normally well-formed;
  - the `ipv6` `disable` parameter prints `1` when booted with `ipv6.disable=1`;
  - `rx_dropped` semantics follow the cited kernel documentation.

---

# Operator and agent reliability note (Receipt V2, revision `6db99068`)

*Suitable for `docs/`. "Existing" means behavior that predates this change. "New" means introduced by this change. Fixture tests establish logic, not host qualification.*

## What verdicts mean (existing)

- **CLEAR:** for each *selected* accusation, every required source was acquired completely, and nothing Alibi recognizes indicated the failure class. It is bounded exoneration. It is not a health attestation, not a statement about remote reachability, and not a claim about history beyond the retained log.
- **DEGRADED:** at least one positive observation was found. Positives may come from a partial source and survive incomplete neighbors. Inspect every indication. These need judgment:
  - an intentional read-only mount;
  - no default route on an appliance;
  - a drop-only counter delta, since drops include unsupported-protocol or filtered frames, not only resource pressure.
- **UNKNOWN:** innocence was not established. Read the required source's status, completeness and `detail` to see why.
  - More privilege helps only for access denials, mainly the kernel log.
  - It cannot create absent EDAC.
  - It cannot extend the mount model to ZFS, overlay or NFS roots.
- **NOT_APPLICABLE:** the accusation is void on this host (no non-loopback interfaces). It is not exoneration.

## New in this change (fixture-tested only; no host qualification)

- **Partial mount tables.** Malformed `/proc/self/mounts` rows no longer discard valid neighbors. The source is `malformed/partial` and cannot support CLEAR.
  - A read-only row with no observed writable sibling becomes *ambiguous evidence*, not DEGRADED, because the malformed row might be the sibling.
  - Capacity and kernel-log positives still establish DEGRADED.
  - `fs-usage` collected over a partial mount table is itself marked partial, with the detail "domain derived from partial mount table".
- **Disabled IPv6.** A missing IPv6 route table is treated as not applicable only when `/sys/module/ipv6/parameters/disable` reads exactly `1`.
  - Missing, denied or other parameter values leave routes partial, giving UNKNOWN unless another positive applies.
  - A denied or malformed table never qualifies.
  - A present table is always parsed.
  - Kernels without the parameter remain unresolved; this includes the IPv6 module not loaded and kernels compiled without IPv6.
- **Bonds.** A regression fixture pins that bond masters and slaves stay in the per-interface counter domain, so clean slaves cannot hide a bond error or a missing counter. This is not redundancy, LACP or reachability assessment, and it has not been run on a bonded host. Inactive-backup delivery is counted by the kernel as `rx_nohandler`, which Alibi does not collect.
- **Human output.**
  - The observation-scope line lists only the sources actually selected.
  - Source-specific next steps exist only for absent EDAC and an unavailable kernel log. Other cases get generic categories.
  - Unsupported-root guidance appears in the finding summary and `skipped`.
  - JSON V2 and the golden fixture are unchanged.
- **Reference consumer (`scripts/read-receipt.py`).** It is a bounded, standard-library-only V2 validator:
  - It rejects duplicate keys and IDs, unknown enums, contradictory exoneration, aggregate, innocence or MTTI fields, and selection mismatches.
  - It optionally cross-checks Alibi's exit status.
  - It exits 0 when parsing succeeds and 2 when it fails, and never prints a traceback.
  - Its exit status is **not** Alibi's exit status, even though 2 is numerically the same.
  - It does not validate provenance, the full schema, or evaluator correctness. Its output contains the hostname and is sensitive.

## For agents

1. Run `alibi check --json` under your own outer timeout. Alibi has per-command deadlines but **no total runtime bound**: many filesystems, hung df targets, or uninterruptible I/O can extend a run.
2. Never treat any of these as CLEAR: being killed, a timeout, missing or invalid JSON, a `receipt_version` other than 2, or missing findings.
3. Preserve Alibi's exit code (0, 1, 2 or 64) separately from any consumer's exit code. Exit 0 covers both CLEAR and NOT_APPLICABLE, so inspect each finding.
4. Confirm the returned findings match exactly the accusations you requested. You can use `--expected` and `--exit-status`.
5. Join `findings[].sources` to `envelope.sources` by name. Any required source that is not `ok`/`complete` is inadequate acquisition. A DEGRADED aggregate can coexist with inadequate sources, so inspect every finding.
6. `completeness` describes acquisition, not historical coverage. `completed_at_ms` and `window` describe collection timing, not event ages. Retained log reports and cumulative EDAC counts do not show that a fault is current, recent, or resolved.
7. `detail`, `skipped`, `evidence` and `summary` are explanations, not an API. V2 has no typed fields for permission denial, event age, IPv6 non-applicability, or ambiguous read-only observations. Do not branch on their text.
8. Alibi is read-only and performs no network probes. Its suggestions are observations to consider, not authorization for follow-up actions.

## Evidence boundary for this revision

- **Logic established by:** unit, golden, CLI and Python fixture tests, executed by the parent. These are passing results, not host qualification.
- **Live evidence:** read-only runs of the *previous* source were accepted by the consumer. Successor smoke results are recorded separately. Neither constitutes field qualification of the new behaviors, which have not been exercised by any real malformed mount table, `ipv6.disable=1` host, or bonded host.
- **No claim is made of** a universal runtime bound, universal absence of false CLEAR, or health. The P1 guard means CLEAR requires complete required sources. It does not mean complete sources capture every fault.

## Known limitations carried forward

- On most VMs (no EDAC), an all-accusation run cannot reach aggregate innocence.
- Drop-only DEGRADED needs interpretation.
- Default-route absence is an interpretation, not a fault.
- The mount `detail` is uncapped.
- On a partial table, the root-type reason in `skipped` may name an earlier root row.
- Earlier builds' field qualification does not transfer to this build.
