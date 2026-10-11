# Findings and disposition

Evidence root `/data/git/alibi-qualification-evidence-20261011`; exact baseline/candidate identities and source pointers are in BASELINE.md, IMPLEMENTATION-REVIEW.md, and RESULTS.md. Independent ORACLE-REVIEW.md governs the distinctions below. No finding is represented as real guest fault qualification.

| Priority | Finding | Demonstration / category | Disposition |
|---|---|---|---|
| 1 | Inadequate log bytes accepted as complete negative evidence | LE04 invalid UTF-8 and LE08 known non-log substitution produce kernel-scream CLEAR, exit 0, innocence true. F0/F2/F3 within explicit command-fixture boundary. | Unresolved. Trusted acquisition/encoding and completeness contract requires bounded design; timestamp regex would not establish authenticity or retention. |
| 1 | Command acquisition deadline does not bound inherited pipes | 100 ms helper returns success after ~1001 ms; ordinary CLI LE06 returns CLEAR after ~4s despite 3s dmesg deadline. F4 independently demonstrated. | Unresolved. Must bound command-output collection and account for descendants/readers; no indefinite fixture needed. |
| 2 | Zero-address narrow routes accepted as defaults | Legal /8 and /32 parser rows return true at baseline. F2; possible CLI false CLEAR not yet established on a guest. | Locally repaired in 0427a7c; real affected guest experiment pending. |
| 2 | Contradictory capacity accepted | df parser accepts total100/free101. F2 parser boundary. | Unresolved. No full CLI false CLEAR claimed from this test. |
| 2 | Missing mount access mode becomes writable | ext4 root options `nonsense` accepted as ro=false. F2 parser boundary. | Unresolved. Actual kernel normally emits access mode; malformed evidence is the tested domain. |
| 2 | Malformed mount escape panics | Valid UTF-8 text containing backslash followed by non-ASCII characters slices across a character boundary; caught helper panic. F4 parser boundary. | Unresolved. No real procfs/CLI crash established. |
| 3 | Ordinary documented build is dynamically linked | file/ldd on sealed baseline show glibc/libgcc loader dependencies; README says static binary. | Packaging/documentation discrepancy; recorded, not broadly repaired. |

Additional unexecuted hypotheses: per-path df can sample replacement filesystem identity after mount capture; command output unbounded allocation; EDAC aggregation/overflow; unsupported mount and temporal edge cases. They are not promoted to demonstrated failures. Existing narrow positive and negative evidence must be independently checked before changing policy.

Known limits are not new defects: down/unused interface alone, DNS, application endpoint reachability, service functionality and resource starvation are not current named positive conditions. Kernel log retention is not all-uptime coverage. Guest EDAC absence cannot establish host hardware memory integrity.

The selected repair changes only IPv4 mask validation/default recognition; Receipt V2 unchanged, no new accusation or external crate. Baseline failing binary/library and receipts retained. Minimized reproducer, controls and required regression checks rerun. Original affected guest and final holdouts remain unexecuted, so repair acceptance cannot be stated as completed host qualification.

Formalization decision: source-identity and result hashes in acceptance/manifest bind these finite counterexamples. Root owns scope decisions. Simple route conjunction is locally tested; no facts-only model can validate collection against a changing kernel. A bounded acquisition-lifetime or mount-identity transition model may inform a later admitted repair. No such model or broader implementation is claimed here.
