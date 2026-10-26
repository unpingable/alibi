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

## October 2026 qualification contribution — historical first checkpoint

The 2026-10-11 campaign used Codex coordination with separate implementation,
oracle-design, laboratory and acceptance reasoning lanes. Original source and
failing outputs were preserved before a bounded IPv4 route-mask repair.
Independent acceptance supports a HOLD and the local parser correction only.
Guest experiments and hidden final holdouts remain unexecuted because lab
admission is incomplete. No Claude/other external-provider execution is claimed.
See docs/qualification-20261011 for exact revisions, receipts, limits and custody.

## Completed qualification and experimental release preparation — 2026-10-11

The first-checkpoint paragraph above predates guest admission and is retained as
history, not current status. The first completed campaign executed 145 QEMU
boots and held release after its original six holdouts exposed defects. The
successor executed 69 additional boots, repaired the demonstrated defects,
froze product commit `7e04d3cb076a48f2e131d6284a16f6bbb836ff58`, and evaluated ten
new sealed cases without post-holdout implementation tuning. Independent
acceptance is `QUALIFIED_WITH_LIMITATIONS`; the explicit oracle gap and other
exclusions remain part of that decision. Counts include controls, repeats and
infrastructure outcomes, not only independent successful fault conditions.

Codex coordinated separate implementation, oracle, laboratory and acceptance
lanes. These were independent reasoning roles, not independent institutions or
a human operator study. No execution by an unrecorded provider is asserted.
The author's release decision remains separate from agent-recorded evidence.

The release-candidate branch preserves both campaign histories and the exact
qualified product source. Release preparation fixes GitHub checkout authority,
updates public documentation, audits the bounded repository/history, and wraps
the existing executable for installation. It adds no diagnostic feature or
Constellation integration. No reduction in operator diagnostic time has been
measured. See [current qualification](docs/requalification-20261011/QUALIFICATION.md),
[environment correction](docs/requalification-20261011/ACCEPTANCE-CORRECTION.md),
and [release evidence](docs/release-v0.1/EVIDENCE.md).

## Operator reliability work — 2026-10-26

The owner requested improvements for both human and agent production engineers,
with Sol implementing and Claude Opus challenging and documenting. A GPT-6.1 Sol
sub-agent implemented the changes; the coordinating agent reviewed the source,
requested corrections and independently ran the checks. Claude Opus 5.5 was
invoked through the Anthropic API on scoped source snapshots, without host
receipts or credentials in the review inputs. Its written reviews are retained
in [the change record](docs/changes/operator-reliability-20261026/README.md).
The first API response exhausted its output budget without a written review;
the subsequent review produced findings and a draft reliability note.

The independent review is static source reasoning, not independent execution,
a human field study, or qualification of the modified binary on other hosts.
Reviewer suggestions were checked against code and primary Linux documentation;
the change record distinguishes accepted findings from rejected claims.
