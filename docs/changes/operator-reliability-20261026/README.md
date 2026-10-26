# Operator reliability changes, 2026-10-26

This change serves two consumers: a human investigating a machine and an agent
consuming a receipt. It improves evidence handling and interpretation; it does
not establish general production readiness or measured incident-time savings.
Execution tracking: [issue #22](https://github.com/unpingable/alibi/issues/22)
in the existing Alibi Project.

The initial conversation inspected stale local main `6f8ad5f`. Implementation
starts from freshly fetched `2da0107`, which already includes the human envelope,
generic UNKNOWN explanations, and retained-log temporal warnings. Those existing
features were preserved rather than presented as new implementations.

## Delivered scope

| Requested improvement | Result |
| --- | --- |
| Useful UNKNOWN explanations | Conditional authorized-access guidance for unavailable kernel logs; absent-EDAC guidance that explains why privilege cannot create hardware evidence; filesystem-native investigation for unsupported roots. Incomplete sources remain visible alongside DEGRADED indications. |
| Terminal coverage | Existing default human source envelope is preserved, including required/supporting roles, acquisition completeness, methods, details, and sampling times. Added scope notes mention only selected sources. |
| Historical versus current evidence | Explain mount snapshots, cumulative EDAC counts with unknown event ages, bounded network deltas, and retained logs whose age/continuity/current activity is not established. No age filtering or acquisition reordering. |
| Ordinary configurations | An absent IPv6 route table is acceptable only with affirmative integer `disable=1` module evidence. Permission failures, malformed tables, and unproven absence stay incomplete. Bond master/slave counter behavior is pinned by fixtures; this is not bonded-host qualification. |
| Partial mount evidence | Preserve valid rows around malformed neighbors. A missing writable sibling cannot be inferred from incomplete enumeration, so ambiguous read-only observations remain evidence rather than indications. Filesystem usage inherits the partial mount domain while retaining valid capacity measurements. |

For agents, [the interface guide](../../agent-interface.md) and the optional
standard-library [receipt reader](../../../scripts/read-receipt.py) explain and
check selection, source identity, enums, completeness, verdict consistency,
MTTI/innocence consistency, and optional original process exit status. Malformed
or deeply nested input fails without a successful digest. Its own status is
separate from Alibi's. It is a bounded validator, not a complete schema validator,
authenticity check, or evaluator implementation. Python is not an Alibi runtime
dependency.

Receipt V2 keys, types, golden bytes, and exit codes are unchanged. Explanation
strings remain prose. In particular, V2 still lacks typed permission-denial,
event-age, and IPv6-applicability fields. Consumers must not infer those through
string matching or interpret acquisition completeness as historical coverage.

## Review and correction

Implementation commits:

- `79a48c841413977a932ac1590a309aee384a84f4`: first tested implementation,
  including coordinating-review corrections.
- `6db99068a8f006d9985310872de52611ac8cec90`: dependent-source propagation and
  additional independent-review corrections.

[Opus's initial review](opus-review-initial.md) is retained verbatim as an
early-snapshot review, not a description of the final source. Its inputs are
identified by [file hashes](initial-input-hashes.json). The review was static:
Opus did not run commands. The first API attempt returned no written review;
[execution metadata](review-execution.json) records that outcome separately.

[The final Opus review](opus-review-final.md) accepts `6db9906` with no remaining
blockers and includes its operator/agent reliability note. The coordinator's
separate [acceptance record](ACCEPTANCE.json) names the reviewed source and the
independently executed evidence. Completion notices under `.lanes/sol-build`
are implementation reports, not acceptance.

The verbatim review's draft note says the reader "never prints a traceback";
that is broader than the implementation promises. Its tested malformed-input
cases fail without tracebacks. External interruption and output-pipe failures
remain ordinary process failures, and cannot be treated as successful receipt
consumption. Uncapped malformed-mount detail, partial-root explanatory ambiguity,
and generic no-default-route wording remain non-blocking presentation limits.

| Finding | Disposition |
| --- | --- |
| B1: partial RO inference could hide a writable sibling | Accepted and repaired. Tests preserve ambiguous observations, cover malformed rows before/after, and retain independent capacity/log positives. |
| B2: IPv6 applicability only tested as a string predicate | Accepted and repaired with a pure read-result seam and evaluator checks. The disable parameter is read only on NotFound; present, denied, and malformed tables never use it. |
| B3: reference reader accepted contradictions or crashed on malformed containers | Accepted and repaired, including duplicate identifiers/JSON keys, enums, exact expected selection, aggregate consistency, exit-status matching, and deep-nesting CLI cases. |
| B4: stale design text and overstated correspondence | Corrected normative partial-mount behavior, parser test names, linkage wording, and fixture-only bond scope. Golden prose is explicitly illustrative. |
| I1: complete capacity acquisition from an incomplete mount domain | Accepted and repaired. Dependent usage is partial, with the original values and failure detail retained. |
| I2: inactive bond backup traffic routinely increments the collected drop counter | The specific mechanism is not established. Linux documents inactive-backup delivery under `rx_nohandler`, which Alibi does not collect. The general ambiguity of drop-only indications is documented; no counter suppression or bond-health claim was added. |
| I3: no total execution deadline | Existing limitation retained; agent guidance calls for an outer timeout and treats missing output as unresolved. |
| I4: kernels compiled without IPv6 may remain UNKNOWN | Deliberately conservative: missing paths alone do not prove family absence. No unsupported exemption was added. |

The source correspondence checks use the primary
[IPv6 parameter documentation](https://docs.kernel.org/networking/ipv6.html)
and [interface statistics documentation](https://docs.kernel.org/6.9/networking/statistics.html).
The disable witness is an integer parameter; accepting `Y`/`y` was removed.
These documents support the bounded interpretation, not qualification across
all kernels, namespaces, devices, or drivers.

## Verification and limits

The coordinator independently executed all required checks at `6db9906`:

- `cargo fmt --check`;
- `cargo test --locked`: 141 unit, 1 golden, 3 CLI tests;
- `cargo clippy --all-targets -- -D warnings`;
- `cargo build --release --locked`;
- `python3 tests/read_receipt_test.py`: 6 tests with negative subcases;
- `git diff --check`.

All passed. The unchanged golden was checked without regeneration. CI now runs
the Python consumer tests. Command results, hashes, and the exact executable
identity are retained in [validation](validation/checks.json) and the
[live smoke summary](validation/host-smoke-summary.json).

Live smoke observations exercise the built CLI and consumer on the development
host. They do not induce faults, demonstrate a real incident resolution, or
qualify a disabled-IPv6 or bonded host. Private raw receipts are not included in
the public review inputs or change record. Human and agent outputs from separate
invocations are separate observations.

Historical guest qualification is unchanged and does not transfer to this
modified executable. [Issue #3](https://github.com/unpingable/alibi/issues/3)
remains the place for actual investigation usefulness, including agent-led
investigations. The separately gated acquisition-order proposal
[issue #20](https://github.com/unpingable/alibi/issues/20) was not implemented.
No release, monitoring, remediation, active network probing, new accusation,
or integration with another operational system is added.

The exact successor executable was packaged locally and its archive checksum,
binary member hash, version, and help command were verified. Its metadata says
`CHANGED_SOURCE`; it is not the historically qualified binary. This package was
not published. The [package record](validation/package.json) identifies it.

[Storage closeout](STORAGE-CLOSEOUT.json) records the retained package and private
review-input/receipt archives under the ignored `dist/` directory. Temporary
build and review working files were retired after preserving those dependencies:
181,055,488 allocated bytes reclaimed, approximately 721 KB retained in the named
evidence paths at measurement time. Preexisting build output was untouched.
The original API credential and supplied workspace file were not modified.

## Formalization consideration

Decision owner: coordinating reviewer. Scope: the product sources at `6db9906`
and the fixtures named above.

The required-source guard establishes a necessary condition: CLEAR implies all
declared required sources are `ok/complete`. It does not prove that declarations,
parsers, kernel observations, or the accusation model cover every actual fault.

The new partial-mount rule addresses a concrete non-monotone inference: an
unobserved RW sibling can appear when missing rows become available. Partial RO
observations therefore cannot alone imply damage. Capacity/log positives remain
available, and usage completeness cannot exceed its mount-domain completeness.
IPv6 non-applicability requires both NotFound and an affirmative kernel witness;
other read outcomes retain their failures.

Focused parser/assembly/evaluator fixtures and source review are sufficient for
these bounded changes. A separate formal model would assume the same kernel
correspondence, and is not required here. No theorem of total runtime, atomic
snapshot, whole-boot continuity, universal health, or incident-time reduction is
claimed. The earlier counterexample and residual assumptions remain documented.
