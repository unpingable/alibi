# Storage and runtime closeout — 2026-10-11

Root campaign agent owns aggregate allocation. This is closeout of the executed local phase; guest admission remains pending. Evidence receipts: `/data/git/alibi-qualification-evidence-20261011/PRE-CLOSEOUT.json`, `CLOSEOUT.json`, `ARTIFACT-CUSTODY.json`, `runtime/CUSTODY.json`, and CHECKPOINT.json. No other tenant was cleaned or modified.

Initial free bytes: / 385745440768; /data 630935212032. Before retirement: / 385422782464; /data 630359130112. After retirement: / 385422192640; /data 630781517824. Both exceed the hard 64424509440-byte floor. Final free inodes at that sample: / 57709047; /data 120382549. Shared host changes are concurrent and cannot be attributed to this campaign's cleanup. No reserve was lowered.

| Object | Classification | Disposition |
|---|---|---|
| Dedicated worktree /data/git/alibi-qualification-20261011 | Source/unique commits and reviewable harness | Retain branch and source; 573440 allocated bytes at closeout sample, before final documentation/commit metadata. |
| Exact campaign target tree | Reproducible build output | Removed after preserved binary/library custody, no named live build, no visible process references, no mounts, and all 383 hard-link groups resolved wholly inside tree. 138731520 allocated bytes reclaimed. |
| Two helper executables | Reproducible fixture outputs | Immediately retired by original runners: 4697184 and 4697464 logical bytes; their allocated-byte baselines were not measured. Do not add logical size to allocated-byte reclamation totals. |
| Evidence root | Durable minimal results, exact replay inputs, held final oracle | 7012352 allocated bytes at closeout sample; includes subsequent PRE-CLOSEOUT/CLOSEOUT records. Small manifest/checkpoint additions follow. Retained purpose/review below. |
| Baseline/candidate executables and rlibs | Exact replay custody | Four distinct artifacts; baseline rlib has two named hard-link aliases inside evidence root. Hashes, modes and link counts recorded. |
| Three shared dynamic runtime inputs | Exact executable replay dependency | 2549064 logical bytes retained; hashes in runtime/CUSTODY.json. |
| Kernel downloads, guest disks/ISOs/initramfs, VM caches/corpora | None created | Zero; guest storage destination not admitted. Harness construction is unexecuted. |
| Other user-owned targets/worktrees/VMs | Outside cleanup authority | Original Alibi target ~228966400 allocated bytes observed read-only and left untouched. Shared other-tenant allocation totals/future plans unknown. |

Known created/retired material is enumerated above. A complete cumulative byte-write or peak-allocation total was not measured; compiler overwrites and intermediate files prevent reconstructing one honestly. At least the measured final target plus retained source/evidence were created by this phase (roughly 146 MB allocated); this is a lower bound, not total write volume. No new clone, package/download cache or multi-GB log was created. No estate-wide scan was performed; remaining shared large consumers are not exhaustively attributed.

Consumer inspection outside the sandbox found no visible reference to the exact target. 334 inaccessible/disappearing unrelated process entries limit that observation; all named campaign build/test and detached fixture producers were terminal. No guest backing relationship exists because no guest/image was created. This limitation does not imply unrelated tenants were inspected or changed. Captured hardlink details explain ownership; all target links were internal before retirement.

Minimal results, scripts and exact pre/post repair replay inputs remain under campaign root custody for review and the admitted guest continuation. Next custodian: operator reviewing this handoff. Concrete retirement condition: final guest round and result review end the exact-replay dependency, or operator closes the interrupted campaign and selects the minimal archived evidence. Review due 2026-10-18 if no continuation occurs. Six hidden holdouts remain sealed for frozen-candidate evaluation; reveal/incorporate their replay contents after that evaluation. Retention is not indefinite by default. No automatic deletion or scheduled cleanup was created.

Both detached user services finished; exact finite fixture descendant groups were reconciled empty. No persistent campaign process remains. No SSH identity was generated, enrolled, loaded or forwarded; no inherited agent was inspected or altered; no revocation claim is needed. No container or host cgroup changes were made.

No storage-pressure follow-up is required from these capacity samples. The outstanding storage action is choosing an authorized writable guest destination; owner/operator reply, then root remeasures admission immediately before allocation. The 4 GiB proposal remains a pending authorization, not consumed storage.
