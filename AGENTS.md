# Agent guide

Alibi is a small, read-only Linux host interrogation CLI. Rust, zero external
crates, standard library only, single static binary. Keep it that way.

## Quick start

```bash
cargo build
cargo test
./target/debug/alibi check
```

## Before claiming a change is ready

Run all of these and mean it:

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets -- -D warnings
cargo build --release
```

Never claim tests pass without running them.

## Repository layout

- `src/collect/` — evidence collection from `/proc`, `/sys`, and standard
  tools into typed facts with per-source status. No interpretation.
- `src/eval/` — pure `facts → Finding` evaluators, one per failure class, plus
  the curated kernel-log pattern table (`patterns.rs`). No I/O.
- `src/receipt/` — receipt model, verdict aggregation, MTTI, human and JSON
  renderers.
- `docs/design.md` — normative architecture, verdict algebra, MTTI semantics.
- `docs/envelope.md` — operator-facing interrogation envelope and sensitivity.
- `tests/golden/` — pinned receipt contract (see below).
- `examples/` — sanitized fictional receipts for documentation use.

## Invariants

1. **Collection failure is `UNKNOWN`, never `CLEAR`.** This is enforced by a
   mandated test; do not weaken it.
2. **`CLEAR` is bounded exoneration, not health attestation.** No mode, flag,
   summary, or wording may imply universal health.
3. **Read-only.** No remediation, no writes to host state, no active network
   probes.
4. **Receipt contract changes are deliberate.** If
   `tests/receipt_golden.rs` fails, the contract changed. Bump
   `RECEIPT_VERSION` for breaking changes and regenerate the golden only via
   `ALIBI_UPDATE_GOLDEN=1 cargo test receipt_v1_matches_golden`, reviewed as
   its own diff.
5. Verdict logic never parses; parsing never verdicts. Evaluators stay pure
   functions over `Facts` and fixture-testable without host access.

## What this is not

- Not a monitoring daemon, agent platform, or healthcheck framework.
- Not a repair tool.
- Not fleet tooling: no SSH, no history database, no metrics export.
- No new failure classes without a design.md update that survives the
  category-error review in §7.

## Commit hygiene

Do not add AI/vendor attribution trailers, signatures, promotional language,
or model branding to commits. If provenance matters, record it in
PROVENANCE.md, not in git history.
