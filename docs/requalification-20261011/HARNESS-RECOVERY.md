# Disclosed batch infrastructure recovery

The original 51-case producer stopped after 13 terminal guest attempts, before
launching G09. Its exact user-systemd unit was observed failed, MainPID 0 and
ExecMainStatus 1, with InvocationID matching the sealed occurrence. Producer PID
2434761 was absent; no visible QEMU referenced this successor campaign. Original
PLAN, protocol, captures and INDETERMINATE record remain unchanged.

G09 failed because the runner created its output directory and then rejected the
long QMP socket pathname before writing START or invoking QEMU. The output
directory is empty. The runner successor uses an explicitly selected short
`/tmp/alibi-qmp-<UUID>/q` socket, independent of bulk storage location. START names
that exact socket before directory creation. An owner-only 0700 directory is
created, and CONTROL-CUSTODY records its device/inode/owner before QEMU starts.
After the owned child is reaped, cleanup verifies that directory identity and
removes only the exact socket and empty directory. Changed/unexpected identity
is retained and reported. Supervisor-loss recovery uses those receipts and never
removes a directory while its original QEMU remains live. This is a runtime
socket location, not a fallback for images, captures or bulk allocation.

The 6.8 acquisition attempt booted the kernel but emitted MODULE_KERNEL_MISMATCH
and powered off before running the scenario or invoking Alibi. The extended
archive declared 6.5 modules. This is F7 infrastructure evidence, not a product
verdict. The recovery constructor builds a minimal module-free acquisition
archive using the identical frozen candidate and original scenario/capture
protocol; the existing builder already supports this configuration.

`campaign/prepare-harness-recovery.py` verifies the original plan, thirteen
terminal hashes, all retained archive/kernel hashes, frozen executable and exact
registered detached construction identity. It creates a new 39-case serial plan:
38 previously unexecuted cases from G09 onward, followed by a separately named
6.8 infrastructure retry. Every run gets a new UUID. It never edits the original
occurrence, restarts it, launches a guest, or reads fresh sealed holdouts.

Recovery evidence and proposed exact checkpoint/launch arguments are under:
`guest-lab/requalification-20261011/harness-recovery-3c800926-a8b2-44b1-bfb2-034741a4a909/`.
RECOVERY.json records observed identity and preserved artifact hashes;
LAUNCH-PROPOSAL.json is a proposal for root, not execution or authorization by
itself. Root owns checkpoint edits, construction and subsequent serial launch.

Validation at preparation: Python syntax, conservative socket length, exact
original terminal hashes and unchanged source plan cardinality. Actual successor
construction/QMP execution remains to be observed. No product source changed.
Root retains exact replay inputs and small recovery receipts through independent
acceptance, with custody review by 2026-10-18; expanded roots retire only after
sealing and consumer checks.

Formalization consideration: the bounded propositions are no replay of completed
cases, explicit identification of a bootstrap-only retry, and exact owned socket
retirement after child reaping. The finite plan transformation and path/inode
checks are directly reviewable; receipt-backed real execution remains required.
No global process absence or supervisor/provider recovery theorem is claimed.
