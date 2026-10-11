# Reproduction, isolation and recovery

This laboratory has executed independent Linux kernels in QEMU TCG. It uses campaign-owned initramfs archives and guest RAM filesystems, no host disk/share, no external network backend and no SSH credentials. Root owns storage under `/data/git/alibi-qualification-evidence-20261011/guest-lab` (L), admitted by the user for at most 4 GiB on local ext4. `/tank/nfs` was not used. The independent 60 GiB free-space floors on `/` and `/data` remain mandatory. See `L/ADMISSION.json`, the root `CHECKPOINT.json` and STORAGE-CLOSEOUT.md.

## Exact environment

Full disclosed matrix: Ubuntu-distributed kernel 6.5.0-44-generic, SHA-256 `8f7c4c1243d276419b7368c1ec4409cb567f65c73c59f612b4334076dc3ee39a`. Acquisition subset also uses kernel 6.8.0-90-generic, SHA-256 `4df4ff923c15c36c591f53b309c1471aefca72a93080aef7c7f89a7585a1ada3`. Signed archive metadata, package hashes, configs and matching modules are retained. The same constructed GNU/Dash/BusyBox userspace is used on both kernels: this is not two complete distribution installations or a systemd qualification. Supported tmpfs root plus dedicated guest ext4/XFS mounts are actually observed; the matrix's initially proposed ext4-root environment was not silently claimed.

QEMU has one virtual CPU, usually 512 MiB guest RAM; XFS G24 uses 1024 MiB. Only one campaign guest runs at a time. Fault scripts verify guest identity/boot markers before mutation. Filesystem images, loop devices, device-mapper transitions, network configuration and private evidence views exist only inside those guests. Software device errors do not establish physical hardware equivalence.

## Replay inputs and ordinary interface

Tracked `campaign/build-initramfs.py` copies exact trusted tools/libraries, scenario, Alibi and matching kernel modules; `BUILD.json` hashes every file and cpio archive. Final executable `L/candidate-g35-final` is source `74e4143844c0535d3837dc67df625f8251197dad`, SHA-256 `eb1ef3e9ac00462707e8e416d1e9496c916d074b7af06ec03e1673387a64f1fe`. Archive hashes in each sealed batch PLAN bind the actual replay, independent of later host tool changes. Retained archives are preferred for exact replay.

`guest-matrix.sh` implements the disclosed cases; dedicated route/acquisition/carrier/TUN/filesystem-control scripts cover their corresponding sequences. `guest-capture.sh` invokes ordinary `alibi check ... --json`, preserving argv, stdout, stderr, exit status, guest monotonic times and GNU time resource output. Independent utilities capture state separately. Command substitutions G27/G28/G35/G37 and acquisition cases are labeled; a real guest does not turn substituted command bytes into a kernel fault.

For a new replay, allocate a new exclusive directory/run UUID, copy the retained PLAN and protocol snapshot, change only output/run identities and record the new plan hash. Never overwrite or rerun an uncertain original producer. The minimal invocation is:

```text
python3 campaign/run-guest.py --kernel VERIFIED_KERNEL --initramfs VERIFIED_ARCHIVE
  --output-dir NEW_RUN_DIRECTORY --checkpoint CAMPAIGN_CHECKPOINT
  --source-revision EXACT_BINARY_SOURCE --owner root --retirement REVIEW_CONDITION
  --run-id NEW_UUID --case-id CASE --timeout 180 --memory-mib 512 --isolated-nics 1
```

Use the case's sealed parameters, including G02 zero NICs and G24 memory. G09/G15 require `--carrier-control --nic-model e1000` and the exact carrier archive. The owned private QMP socket controls only the fixed guest NIC; it is retired after QEMU termination. TUN successors use the guest-only compiled helper, independently sampled kernel rx_dropped counters and actual CLI sampling-window overlap; no host TUN is opened. See GUEST-MATRIX-HARNESS.md, CARRIER-HARNESS.md and TUN-COUNTER-HARNESS.md for fault mechanics and oracle boundaries.

## Durable execution

`run-batch.py` serializes a sealed PLAN, rechecks allocation, and records START/PROGRESS/COMPLETE. `run-guest.py` records exact inputs/QEMU/process identity, bounded serial captures, guest terminal marker and TERMINAL. SHA/base64 framing transports stdout/stderr independently of guest filesystem faults. A normal guest exit or valid transport hash does not imply a semantic PASS.

Launch the batch with the established detached user-systemd mechanism, null stdin, file-backed logs, MemoryMax=2G, TasksMax=128, Nice=10 and finite RuntimeMaxSec appropriate to the plan. Before launch persist exact unit/run/source/protocol/input identities, expected terminals and next action in CHECKPOINT.json; afterwards record InvocationID. The original durability probe completed after its launching tool returned, and all guest batches use that mechanism. No interactive tool session owns producer lifetime.

After supervisor interruption, read the checkpoint, compare original InvocationID, START and terminal seals, inspect useful progress and resume supervision. Never duplicate a run merely because the conversation ended. A collected terminal systemd unit is reconciled from durable producer records. Provider recovery and whole-host power-loss recovery are not proven.

## Earlier substrate failures

Host module vermagic alone was insufficient: CRC mismatch prevented loads. Exact matching distribution modules repaired the fixture. BusyBox ash selected internal applets despite external tools; Dash and explicit GNU tools repaired that fixture. Original failed boots/inductions remain F7 records. Final ext4/overlay, nft counter, XFS EIO, device-mapper direct-read EIO, QMP carrier and TUN counter proofs are separate observed successors.

Local minimized reproducers remain in `campaign/local-reproducers.py` and `run-local-evidence.py`, with preserved original/final binaries and libraries. They establish parser/command boundaries, not real-host fault equivalence. Final holdouts are released only after `E/FINAL-FREEZE.json`; their original sealed manifest and exact execution inputs remain in evidence custody.

## Retirement and custody

Expanded build roots and compiler caches are reproducible substrate; retire after exact archives/binaries and receipt custody are verified and live consumers excluded. Retain minimal result/receipt/source identities and current replay inputs under root custody for operator review on 2026-10-18. STORAGE-CLOSEOUT.md records actual removals and remaining obligations. No generated SSH identity or ambient-agent mutation occurred.
