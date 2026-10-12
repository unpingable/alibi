# Synthetic first-use walkthrough, 2026-10-12

One fresh-context model acted as a Linux administrator unfamiliar with Alibi.
It received the public README, installation/compatibility/envelope documentation,
the exact unpublished candidate archive, and four fictional operator scenarios.
It received no implementation, qualification history, or evaluator rubric.
This was a procedural context boundary on a shared filesystem, not enforced
isolation. The evaluator wrote the rubric before dispatch. The participant
recorded its initial interpretation before invoking the executable.

This is **synthetic usability evidence**, not human feedback, independent host
qualification, or evidence of reduced operator diagnostic time. Documentation
primed the participant, which already had Linux expertise. A single model's
successful interpretation cannot establish how unfamiliar humans will respond.

## Identity and custody

- Source: `c3b493692ee0e9e300d657cc67140f08f2174c44`, version `0.1.1-rc.1`.
- Archive SHA-256: `f854f0b3ba3a879e2c7304f61a0d5389b42194a221df587a02e8694d8d7ae4b3`.
- Executable SHA-256: `62c7f178488dd8d94c264fd7f14db47fabb093bfe670a067dd851f8c2130b103`.
- Lane/run: `fresh_operator/2317f39d-e636-4c0f-ad43-7da6f701d880`.
- Owner: root campaign supervisor. Private custody:
  `/data/git/alibi-synthetic-usability-20261012`.
- `KIT-MANIFEST.json`, `DISPATCH.json`, `ORACLE.md`, the immutable lane notice,
  `BEFORE.md`, `SESSION.md`, `AFTER.md`, and `RECEIPT.json` preserve input and
  result identities. The evaluator verified four result and 26 capture hashes.
- Raw host receipts remain private; this report deliberately contains no host,
  interface, mount or log identifiers. Review custody when findings resolve or
  on 2026-10-19; retire disposable extraction after receipt verification.

The changed source/build does not inherit the historical exact-binary
`QUALIFIED_WITH_LIMITATIONS` finding. The trial created no new qualification claim.

## Actual execution and interpretation

Checksum verification, extraction, version, help, guide and explain commands
succeeded. Unprivileged local checks produced network DEGRADED (exit 1),
kernel-scream and silent-killer CLEAR (exit 0), and memrot UNKNOWN (exit 2).
A subsequent JSON network check returned DEGRADED. No loader failures or
stderr were recorded. There were no active probes, remote contacts, system
installation, permission changes or injected faults. The fictional incidents
were not reproduced on the workstation or solved by these commands.

Evaluation against the prewritten rubric:

| Scenario | Observed interpretation | Caveat |
| --- | --- | --- |
| Hostname requests fail | Selected network-choke-point; kept DNS, endpoint, application and policy questions open | Actual local DEGRADED did not establish the fictional incident's cause |
| Process disappears | Selected kernel-scream and adjacent filesystem check; retained unlogged kernel, application, service and resource-limit possibilities | Initially included silent-killer; later called kernel-scream the clearer start. The name alone suggested broader scope than the guide |
| VM stalls, high memory use | Separated memory integrity from utilization; absent EDAC meant UNKNOWN, not good or bad RAM | No VM or hardware integrity result was established |
| Share a receipt | Preserved diagnostic nonzero exits; identified sensitive fields and the need to mark redactions | A new JSON invocation is a new observation, not recovery of the earlier human receipt |

The participant corrected its expectation that denied dmesg access would
necessarily prevent useful findings after observing journal fallback. It did not
suggest that additional privilege could create absent EDAC hardware evidence.
It reported neither fictional manual checks avoided nor time saved.

## Demonstrated friction and bounded follow-up

1. Default DEGRADED output showed `(+1 more)` but omitted the indications list.
   The participant consulted a later JSON invocation. That later receipt cannot
   prove the exact omitted content of the earlier observation. The renderer
   independently confirms the omission; a same-receipt regression checks all
   canonical indications without relying on changing host state.
2. The envelope's opening CLEAR definition said only “found no evidence.”
   Later required-source language resolved the tension, but the opening was
   weaker than the established sufficient-applicable-evidence contract.
3. Network explain said a ping “proves nothing about host state.” Separate
   probes can provide bounded observations; they do not substitute for Alibi's
   required local evidence or extend this passive accusation to endpoint health.
4. Help described exit 0 as “innocence established” without stating the selected
   accusation boundary. Actual CLEAR output already included that boundary.

The repair in #16 exposes all canonical DEGRADED indications by default and
clarifies those three statements. It does not alter evaluators, collectors,
Receipt V2, accusation names or diagnostic scope. The existing guide addressed
the name ambiguity; no renaming is justified by this one trial. Published-release
README examples versus the unpublished supplied artifact also required a
lookup, but install instructions and SOURCE.json resolved the choice.

## Formalization consideration

Decision owner: root, with independent repair review. Proposition: rendering
preserves the canonical receipt and makes all DEGRADED indications accessible
in default output. Scope: this presentation layer only. A same-receipt fixture,
unchanged JSON golden checks and source review are sufficient for this bounded
change; no new state-transition mechanism is introduced. These tests cannot
establish human comprehension, source authenticity or host diagnostic truth.
A formal model of user understanding would not resolve the missing human evidence.

The next utility evidence remains an unfamiliar human investigating an actual
incident under the field-testing protocol (#3). Synthetic follow-up after these
repairs would be a disclosed regression exercise, not a fresh holdout.
