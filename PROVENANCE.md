# Provenance

This project is human-directed and AI-assisted. Final design authority,
acceptance criteria, and editorial control rest with the human author.
AI contributions were material and are categorized below by function.

## Human authorship

The author defined the project direction, requirements, and design intent.
AI systems contributed proposals, drafts, implementation, and critique under
author supervision; they did not independently determine project goals or
deployment decisions. The author reviewed, revised, or rejected AI-generated
output throughout development.

## AI-assisted collaboration

### Concept and product shape

Working-session development of the core concept (Mean Time To Innocence, the
failure-class taxonomy, and the "named accusations → evidence → verdict →
receipt" grammar), recorded in `notes.md`. The V0 seed prompt and the
repository-closeout scope were human-authored.

### Design and implementation

Lead collaboration: Kimi Code (Moonshot AI). Architecture and design note,
verdict and MTTI semantics, Rust implementation, test suites, receipt
contract stabilization, and repository closeout, under the seed prompt and
human review. Real-host runs during development exposed and corrected
false-positive classes (EDAC banner lines, read-only bind mounts) before
closeout.

## Provenance basis and limits

This document is a functional attribution record based on commit history,
project notes, and documented working sessions. It is not a complete forensic
account of all contributions.

Some AI contributions (especially design critique, rejected alternatives,
and footguns avoided) may not appear in repository artifacts or commit
metadata.

Model names/tools are recorded at the platform level; exact model versions
may vary across sessions and are not exhaustively reconstructed here.

## What this document does not claim

- No exact proportional attribution. Contributions are categorized by
  function, not quantified by token count or lines of code.
- Design and implementation were not cleanly sequential. Architecture
  informed code, code revealed design gaps, and the feedback loop was
  continuous.
- "Footguns avoided" and "ideas that didn't ship" are real contributions
  that leave no artifact. This document cannot fully account for them.

---

This document reflects the project state as of 2026-09-16 and may be revised.
