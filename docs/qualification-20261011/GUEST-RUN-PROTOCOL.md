# Executed guest protocol

Owner and storage coordinator: root campaign agent. The user authorized up to
4 GiB in `/data/git/alibi-qualification-evidence-20261011/guest-lab`; this is local
ext4, not NFS. Admission measurements, exact mounting, other-tenant uncertainty,
and retirement conditions are in `guest-lab/ADMISSION.json`. Both `/` and `/data`
retain a separate 60 GiB free-space floor. One campaign QEMU runs at a time.

The smallest successful kernel boot preceded the matrix. A later infrastructure
proof independently established tool selection, matching kernel-module insertion,
real dummy/veth interfaces, nftables/conntrack setup, and ext4 loop write/read and
restoration. `harness-proof-batch/ACCEPTANCE.json` accepts that substrate only.
Alibi's own receipt never establishes successful fault injection.

`build-initramfs.py` records every copied executable, library, module, script and
archive hash. The kernel comes from a signed Ubuntu package index; exact module
packages must match its build, not merely its release string. The guest uses Dash
as its scenario shell: the original BusyBox shell intercepted commands despite
correct external binaries being present. Original failed runs and immutable
archives are retained separately. See `LAB-FEASIBILITY.md` for this correction.

`run-guest.py` starts QEMU TCG with one virtual CPU, private RAM, synthetic devices,
no shared host filesystem, no host block device, and zero or one network interface
on a guest-only internal backend. No external network connectivity is configured.
QEMU lifetime and capture sizes are bounded. All filesystem and network changes
in the scenarios occur after checking guest boot identity and disposable root.

`guest-capture.sh` records exact command arguments, stdout, stderr, exit status,
monotonic uptime endpoints and GNU time resource output for each operator CLI
invocation. Streams travel through serial as SHA-256-checked base64 frames.
`capture_frames.py` verifies transport integrity; it does not assign verdict
correctness. Serial also carries independent utility and kernel observations.
`TERMINAL.json` distinguishes VM termination, scenario status, missing transport,
and timeouts. Exit zero is never an oracle PASS.

Every batch has a fresh UUID and an exclusive output directory. A sealed PLAN.json
binds exact kernels, archives, source identities and protocol snapshots. The
`run-batch.py` snapshot executes serially under a detached user-systemd service
with finite runtime, tasks and memory ceilings. It writes START, per-case PROGRESS,
and COMPLETE, or an explicit admission/indeterminate record. The root CHECKPOINT
binds the unit and systemd invocation. A new supervisor reads those original
records and observes the same producer; it must not restart an uncertain run.
The earlier durability probe verified continuation after the launching tool ended.

Reproduction uses the exact PLAN and archived inputs as source material, but a
new output directory and new run identities. Never rerun a producer into sealed
outputs. Build an archive using the selected exact Alibi binary, scenario, and
`extended-builder-args.json`; then use the existing runner options recorded in
START.json. Admission must be repeated before allocating a successor. Copy the
three protocol Python files into that successor's protocol directory and hash them
before launching. The final result remains unadjudicated until an independent
reviewer checks injected state, applicability, evidence adequacy and receipts.

These environments exercise genuine independent kernels with constructed minimal
userspace. Testing two kernels does not constitute two complete distribution or
systemd qualifications. PATH substitutions are explicitly marked as command
evidence-boundary cases even when executed inside a real VM. They are not storage,
network, or physical-hardware fault injections.

Formalization consideration: the transport claim is that decoded bytes match
producer-declared SHA-256 and stream lengths, under trusted local producer and
filesystem custody. This is integrity checking, not authentication of arbitrary
kernel claims. Finite fixtures, exact manifests, detached-process recovery records
and independent observed state suffice for this campaign; no atomic host snapshot
or crash-proof publication theorem is claimed. Receipt V2 remains unchanged.

Retirement: root retains exact replay archives, qualified executables and minimal
receipts until independent acceptance/operator review (next review 2026-10-18).
Expanded archive roots, duplicate package files, compiler targets and other
reproducible substrate are retired after custody and live-consumer checks. Final
storage closeout must enumerate what remains; this document alone is not cleanup.
