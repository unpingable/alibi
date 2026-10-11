# Storage and runtime closeout — guest campaign successor

Root owns campaign storage. The guest continuation and six final holdouts are
terminal. This record supersedes the earlier local-only closeout narrative;
`CLOSEOUT.json` and `PRE-CLOSEOUT.json` remain unchanged historical evidence.
Current exact receipt: `/data/git/alibi-qualification-evidence-20261011/GUEST-CLOSEOUT.json`.
The preceding read-only census is `storage-closeout-inventory.json` in that root.

Root authorized exactly 22 census candidates, the final holdout expanded root,
and the inventory-generation scratch script. The execution lane retired those
24 paths: 21 expanded initramfs roots, the campaign compiler target, worktree
Python cache, and `/tmp/alibi-storage-inventory.py`. No archives, BUILD receipts,
kernels, downloaded modules/packages, raw captures, retained executables/libraries,
source commits, or other tenants' artifacts were removed.

Before retirement, 345 retained-input hash checks matched BUILD/PLAN/START and
executable/library custody receipts. Every named batch had its terminal record,
every planned invocation had a terminal record, and checkpoint active producers
was empty. No mounts or accessible process references pointed into cleanup paths.
All 415 candidate hard-link groups were reconciled against campaign evidence and
worktree names; retained aliases were preserved. Process inspection covers only
the visible process namespace and readable descriptors, not a universal absence
claim. Audit continues against retained archives and receipts.

| Measurement | Allocated bytes |
|---|---:|
| Retired path totals, including directory blocks | 424,366,080 |
| Reclaimed unique regular-file blocks after accounting for retained hard links | 422,412,288 |
| Retained evidence root immediately after cleanup | 268,275,712 |
| Retained source worktree immediately after cleanup | 995,328 |

Small receipt, documentation and commit additions follow these samples. Complete
campaign bytes-created and cumulative writes were not measured; compiler rewrites
and intermediate files prevent reconstructing them from final totals. Filesystem
free-space changes include concurrent tenants and are not attributed solely to
this cleanup. The earlier local-phase retirement is separately recorded and is
not silently folded into these successor figures.

After retirement, available bytes were 385,370,923,008 on `/` and 630,043,521,024
on `/data`; free inodes were 57,709,189 and 120,367,683 respectively. Both exceed
the independent 64,424,509,440-byte reserve. No reserve was lowered. No storage
pressure follow-up is indicated by these measurements.

Remaining largest custody objects include exact matching-module acquisition
(33,218,560 bytes), extracted 6.8 kernel (15,020,032), its verified package
(14,737,408), extracted 6.5 kernel (14,405,632), its package (14,401,536), and final
matrix captures (12,521,472). The receipt enumerates further objects. Initramfs
archives, baseline/final executables and libraries, exact fixture inputs, signed
package-verification inputs, captured raw observations, terminal receipts and
source commits retain independent acceptance/replay dependencies.

Custodian and next action: root/operator reviews the remaining exact replay
requirements at acceptance, no later than 2026-10-18, and selects minimal durable
custody. Download/extraction redundancy is retained under the explicit current
replay instruction; it is not indefinite retention. Any later retirement needs
exact consumer/dependency checks and its applicable authority. No new background
cleanup or retention framework was introduced.

No SSH identity was generated, enrolled, loaded or forwarded. No inherited agent
was inspected or modified. No guest disks or writable host-backed devices were
created; QEMU used isolated kernel/initramfs inputs. No other tenant's VM, container,
service cgroups or data were changed by closeout.

Formalization consideration: exact-path retirement after retained hash validation
and hard-link reconciliation is the bounded proposition. The receipt records all
paths, hashes and observed links; direct finite checks suffice here. Process
visibility and concurrent filesystem allocation remain explicit empirical limits.
Decision owner is root; execution lane is lab_feasibility. This establishes no
stronger global storage or process absence theorem.
