# Successor storage and credential closeout

Owner: root; successor artifacts only. Exact cleanup receipt R/STORAGE-CLOSEOUT.json, SHA-256 `b2804c500ff8d370c2e8d742b17ba1c0e99f1e7e3126dd29f879236b4589e527`. R is `/data/git/alibi-qualification-evidence-20261011/guest-lab/requalification-20261011`. The parent historical campaign and other tenants were outside cleanup scope.

Admission used the expressly approved local guest-lab allocation, capped at 4 GiB aggregate, with a 1 GiB expected successor peak. Mount inspection established local ext4, writable and not NFS. Root coordinated all lanes and serialized builds/guests. Initial available capacity was 385,358,249,984 bytes on `/` and 629,877,825,536 bytes on `/data`; initial free inodes were 57,709,174 and 120,366,774. The hard floor remained 64,424,509,440 bytes independently on both filesystems. Other shared tenants' future allocation was explicitly unknown; admission was rechecked before material allocations.

All producer chains were terminal or explicitly reconciled before retirement. The preflight and execution rechecked retained archive/binary/freeze/input hashes, exact proposed paths, hard-link names, visible process references, mounts and loop-device backing. No references or mounts were found; the final scan reported no unreadable process descriptor directories in its visible namespace. This is not a global absence proof across hidden namespaces. Both short QMP runtime directories were already retired by their owned runners, with custody and terminal records agreeing.

Thirteen exact campaign-owned objects were removed: two compiler target trees, the old-binary churn expanded guest root, eight disclosed expanded roots, the corrected acquisition expanded root and the fresh-holdout expanded root. Their exact paths are enumerated in the receipt. No source worktree, unique commit, binary, compressed replay archive, kernel, original failure, receipt, sealed fixture or parent historical artifact was removed.

| Measurement | Allocated bytes |
|---|---:|
| Successor R immediately before retirement | 611,446,784 |
| Exact retired path totals, including directory blocks | 495,525,888 |
| Unique regular-file blocks eligible for reclamation | 494,137,344 |
| Successor R immediately after retirement, including closeout receipt | 116,166,656 |
| Net observed R reduction | 495,280,128 |
| Entire parent guest-lab after closeout sample | 375,959,552 |
| Retained successor source worktree after sample | 1,343,488 |

Cumulative bytes ever created were not instrumented; the before-retirement inventory is an observed retained allocation, not cumulative write volume or a proven peak. Directory metadata, newly written closeout records and shared hard links explain why per-object and net figures differ. Small final reports/notices add bounded kilobytes after these timestamped samples. Host-wide changes during the campaign include concurrent tenants and are not attributed to this campaign.

After retirement `/` had 384,414,371,840 available bytes and 57,679,635 free inodes; `/data` had 628,564,959,232 available bytes and 120,331,883 free inodes. Both reserves remain satisfied. No storage-pressure follow-up is required by these samples. The receipt retains free/available block counts and block sizes as well as bytes/inodes.

Largest remaining successor consumers are the eight exact disclosed compressed archives (~72.1 MB), disclosed successor execution evidence (~11.4 MB), old-binary churn replay archive (~9.0 MB), fresh replay archive (~6.8 MB) and corrected acquisition archive (~3.8 MB). These are current replay dependencies for independent acceptance, defect comparison and operator review, not disposable expanded roots. Smaller items include exact binaries, sealed source fixtures, raw observations, original failed local reproductions and acceptance records. The branch/worktree retains unique source and review provenance. No new clone, ISO cache, persistent disk-backed VM, large corpus or download cache remains from this successor.

Root/operator is the named custodian through review on **2026-10-18**. Retirement condition: once exact replay/evidence custody is replicated and verified and the acceptance/defect-review dependency has ended, review exact remaining archive and duplicate-input ownership and retire only objects with no current consumer. This is a bounded handoff, not indefinite evidence retention or authority to delete the original campaign's shared kernel/module inputs.

No SSH identities were generated, enrolled, loaded or used by this successor; no agent was started, inspected or modified. Guests had no SSH access path. Accordingly there are no successor private keys or campaign agents to retire and no destination authorization/revocation action. This makes no claim about unrelated inherited identities.
