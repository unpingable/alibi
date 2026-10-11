# Harness, reproduction and recovery

Scripts are in `campaign/`. Local command fixtures have executed; guest construction/boot scripts are syntax-checked but **not boot-validated**. They do not themselves implement all 38 matrix faults. `guest-route-cases.sh` prepares the smallest baseline/non-default route sequence; it is also unexecuted. No guest fault evidence exists yet.

## Executed local reproduction

Preserved exact artifacts live at `/data/git/alibi-qualification-evidence-20261011`. `alibi-baseline` and `libalibi-baseline.rlib` correspond to source 6f8ad5f; `alibi-candidate` and `libalibi-candidate.rlib` correspond to 0427a7c. The original `alibi-baseline.rlib` name is a hard-link alias to `libalibi-baseline.rlib`, needed because rustc external-library input names must begin with lib. Hashes, not names, establish identity. Original receipts referencing the original target library remain historical; use preserved matching hashes for replay after target retirement.

From the dedicated worktree, choose **new exclusive** result directories:

```sh
python3 campaign/local-reproducers.py   /data/git/alibi-qualification-evidence-20261011/alibi-baseline   /data/git/alibi-qualification-evidence-20261011/libalibi-baseline.rlib   /data/git/alibi-qualification-evidence-20261011/REPLAY_NEW_ID
python3 campaign/run-local-evidence.py   /data/git/alibi-qualification-evidence-20261011/alibi-candidate   0427a7c0a81bb0b8212cc2e9a093453354121119   campaign/local-evidence-spec.json   /data/git/alibi-qualification-evidence-20261011/BOUNDARY_NEW_ID
```

These commands create small command substitution fixtures in the campaign directory and never replace system commands. Direct acquisition stdout/stderr, exit statuses, modes and oracle specification are retained. Finite inherited-pipe fixtures exit after one or four seconds; external process-group bounds prevent indefinite qualification waits. Do not describe library/PATH tests as actual kernel faults. The runner records allowable-verdict comparison; separate review is still required for the other receipt assertions.

## Guest admission and smallest experiment

Storage owner is root campaign agent. /tank/nfs is read-only; alternative destination authorization is pending. Do not silently use /data or /tmp for guest assets. Before allocation, update existing CHECKPOINT.json with destination, mount/capacity/inodes, total construction + capture + retained-artifact estimates, other known tenants and unknown allocations, and the hard 60 GiB independent reserve on / and /data. Serialize guests. Initial proposed alternative cap is 4 GiB, not an automatic grant to allocate that amount.

A readable distribution kernel is needed. Installed 6.5.0-44-generic kernel is unreadable; noninteractive sudo requires a password. No permission changes were attempted. The matching Ubuntu package is listed in the [official archive](https://archive.ubuntu.com/ubuntu/pool/main/l/linux-hwe-6.5/), but has **not been downloaded or verified**. Obtain package integrity evidence, exact kernel config and optional matching modules before admitting a boot. This is a local VM kernel; never substitute the ABSD kernel or share host block devices/cgroups. No SSH keys are required.

After storage and kernel admission, builder interface:

```text
python3 campaign/build-initramfs.py --alibi PRESERVED_BINARY
  --output-dir NEW_ADMITTED_BUILD_DIR --admission EXISTING_ADMISSION_RECEIPT
  --owner CAMPAIGN_OWNER --retirement CONCRETE_RETIREMENT_CONDITION
  [--scenario campaign/guest-route-cases.sh]
```

Builder copies static BusyBox, exact Alibi and dynamic libraries, GNU df/dmesg/ip; writes proc/sys/dev setup and switches to guest-local tmpfs root. It creates a synthetic cpio console node without privileged host mknod. Optional exact modules/tools require explicit inputs. BUILD.json hashes copies and archive. The minimal tmpfs environment is not the matrix's desired ext4-root environment: obtain separate ext4 baseline before treating storage matrix assumptions as satisfied. Module absence and failed induction are F7.

The default scenario captures uname and an ordinary kernel-scream JSON invocation. Begin there with no NIC, then use one isolated NIC for the disclosed route sequence. Runner interface:

```text
python3 campaign/run-guest.py --kernel VERIFIED_KERNEL --initramfs VERIFIED_ARCHIVE
  --output-dir NEW_ADMITTED_RUN_DIR --checkpoint EXISTING_CHECKPOINT
  --source-revision EXACT_BINARY_SOURCE --owner CAMPAIGN_OWNER
  --retirement CONCRETE_RETIREMENT_CONDITION --run-id UUID
  --timeout 180 --memory-mib 512 [--isolated-nics 1]
```

Runner uses software-emulated QEMU, one virtual CPU, no external network backend, shared filesystem or host block device. Its optional hub NIC is internal only. Per-file log limits and outer timeout bound capture. START/EXECUTION/TERMINAL.json describe identities, hashes, timing, QEMU exit and guest marker; **a guest completion marker is not acceptance**. Independently evaluate per-case fault proof and receipts before accepting results. The unexecuted guest route script rejects missing campaign boot marker/hostname/tmpfs-root prerequisites and performs route mutations only after these guest checks. It captures independent route/link/kernel state and before/after receipt markers; it does not infer outcomes from injection success.

## Durable supervision

Use the established detached user-systemd mechanism, outside the tool sandbox via applicable approval controls. Before starting, record exact unit/run/source/protocol/input hashes, cwd, log paths, expected terminal and next authorized action in CHECKPOINT.json. Use null stdin and file-backed stdout/stderr, bound service memory/tasks/runtime, and record returned InvocationID. Do not use an interactive tool session as producer lifetime. No container using host cgroups or nested service manager was launched.

The bounded probe `alibi-qualification-probe-59150f02-d5cc-438f-86e4-206b3fb3d007` emitted matching invocation identity and completion after its launching tool returned. The local fixture producer also completed in a detached service. This is evidence for that limited supervision boundary; no VM or provider recovery is thereby proven.

Fresh supervisor: read `/data/git/alibi-qualification-evidence-20261011/CHECKPOINT.json`; reconcile original unit, InvocationID, process start identity, output advancement and terminal hashes. Resume observation, never restart a producer simply because a chat ended. Terminal services can have been collected by systemd; reconcile their producer-written invocation/terminal evidence rather than inventing a replacement. No active producer remains at current closeout.

## Retirement

Source scripts, exact baseline/candidate executables/libraries, small raw receipts and oracle specifications have current replay/review custody. Reproducible target output is retired after exact artifact custody and consumer checks. No guest image, downloaded kernel, VM cache or SSH identity has been created. Holdouts remain sealed for a later frozen guest round; review dependency and custodian on 2026-10-18 if admission has not occurred. See STORAGE-CLOSEOUT.md and existing CHECKPOINT.json; no separate retention service is introduced.
