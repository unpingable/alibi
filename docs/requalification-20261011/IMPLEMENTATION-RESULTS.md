# Implementation lane results

Base source: `6a7b5012494b090726c4184644cdaccbaba7837e`.
`R` is `/data/git/alibi-qualification-evidence-20261011/guest-lab/requalification-20261011`.
Owner: implementation_review; root owns source freeze, actual guest execution, storage and closeout. New sealed holdouts remain unopened.

The three disclosed findings received bounded repairs: same-descriptor network counter samples, explicit EDAC event classification and default human receipt envelope fidelity. See [design and assumptions](REPAIR-DESIGN.md). Existing Receipt V2 JSON is unchanged; no external crate, new accusation, active probe, daemon or remediation was introduced.

## Preserved negative evidence

- Root's `R/baseline-repro` preserves actual guest reproductions of H04/H05/H06 against the original executable. H04 is a labeled command fixture with no claimed hardware-memory event; H05 is an actual Linux interface replacement; H06 has actual denied log access and missing default routes.
- `R/regression-before` preserves the two intended local assertions: registration yielded DEGRADED rather than UNKNOWN with EDAC absent, and default human output omitted the canonical kernel-log source.
- Independent review found an accepted journal continuation could inherit its first line's informational EDAC classification. `R/continuation-before` is a failed fixture induction: indentation was wrong, so source completeness failed. It is not evidence of that semantic defect. The corrected `R/continuation-before-corrected` passes source completeness and then fails at CLEAR versus required UNKNOWN. Neither occurrence was overwritten.

## Local verification

`R/repair-checks-1` passed 125 unit tests, 3 receipt tests, formatting, Clippy with warnings denied, and release build. Its provisional executable `R/candidate-repair-1` has SHA-256 `14a59ad879347764f4ced7db39caab150afab753de73196aa57bbf0b1c6b922b`; it precedes the independently discovered continuation repair and is not a final candidate.

`R/repair-checks-2` passed 127 unit tests, 3 receipt tests, formatting, Clippy with warnings denied, and release build. Exact provisional successor `R/candidate-repair-2` is 856,464 bytes, SHA-256 `0760ea3529ac3c3d831038a71c46f82b6bd3d750fd4dbc52866399962e593b06`. Its receipt binds source/test hashes, the complete tracked patch, the full-check terminal and corrected pre-repair reproducer. These durable user-service runs record invocation identity, commands, outputs, exit statuses, hashes and terminal results. Original logs remain immutable.

The new descriptor tests exercise retained-file behavior under directory rename/name reuse, fresh seek/read, unavailable initial files with later replacements, and bounded-read truncation refusal. They use ordinary campaign-owned temporary files and do not simulate or prove kernfs deactivation. Actual guest removal/reuse/rename is still required. The EDAC continuation tests cross the actual collector/parser-to-evaluator boundary with synthetic log text and zero-counter facts. Human tests compare default output to the same canonical receipt used by the unchanged JSON golden.

## Boundaries and custody

The repaired collector assumes genuine sysfs attribute layout and coherent host proc/sys views; its claim is same-object endpoint pairing, not continuous namespace residence, stable counters through driver reset, or complete observation of every transient interface. EDAC support is a finite report grammar, and unsupported EDAC-origin records explicitly limit exoneration. Log framing does not authenticate content or complete history.

This record reports local implementation checks only. Independent source acceptance, actual repaired guest correspondence, final freeze and newly sealed evaluation are separate records owned by root/review lanes. No release recommendation follows from these tests alone.

Latest target allocation measured 156,585,984 bytes. Root received exact candidate and receipt custody. The compiler target is reproducible and eligible for retirement after final executable/source custody and dependent runs are verified; provisional binaries retain an explicit disclosed-replay/review dependency until root resolves it. Next custody review is campaign closeout or 2026-10-18. The implementation lane created no VM, mounted fault environment, container or credential.
