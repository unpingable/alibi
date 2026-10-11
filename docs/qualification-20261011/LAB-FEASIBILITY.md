# Isolated Linux laboratory feasibility

Current disposition: the isolated Linux guest and corrected external-tool/module
configuration have executed successfully. See [LAB-CORRECTIONS.md](LAB-CORRECTIONS.md)
for demonstrated initial F7 gaps, their bounded repairs, the independent-kernel
proof and exact custody handoff. Earlier sections below retain the initial
read-only observations and staged construction history.

Read-only investigation, 2026-10-11 16:47 UTC. Source inspected:
`6f8ad5fba4ed6f3fb9f32c9930377a0a48e06931`. Owner: Alibi campaign root;
reviewer lane: lab_feasibility. This is an environment assessment, not a
host qualification result. No guest, download, fault, or background producer
was launched by this lane. No other campaign artifact was modified.

## Execution path

`/usr/bin/qemu-system-x86_64` is QEMU 8.2.2 (Ubuntu package
1:8.2.2+ds-0ubuntu1.18), SHA-256
`8a35ccba41582fc6c38b9df85fc9e35fa1d42f414d2d7d8090ee9b2f5e7c0854`.
`/dev/kvm` is absent in the current execution environment. TCG is the candidate
for a disposable VM with its own Linux kernel; actual boot remains untested.
This is not a reason to substitute a container for kernel qualification.

`/usr/bin/busybox` is a static x86-64 BusyBox 1.36.1 executable, SHA-256
`dbac288c29ba568459550a2da9e7ae0ded6b1fc728ee9fad3044c44e62d6ac14`.
Its df applet advertises -P, -B and -i, but exact output/parser compatibility
must be established in the guest before treating it as supported evidence.
GNU `/usr/bin/df` and util-linux `/usr/bin/dmesg` are alternative real tools;
copy their dynamic loader/libraries if selected. `cpio`, `unmkinitramfs`,
`qemu-img`, `timeout`, `ip`, `mkfs.ext4`, and `mkfs.xfs` are available.

Alibi docs/envelope.md explicitly supports Linux without systemd when dmesg
works. A minimal real-kernel rootfs can therefore exercise bounded claims.
Kernel modules/features, runtime libraries, df behavior, root filesystem type,
and exact source/binary/tool hashes must be captured. An initramfs root alone
may remain outside the filesystem model; switch to a guest-local supported
root filesystem before claiming silent-killer healthy controls.

Smallest candidate: one 256–512 MiB RAM TCG VM, kernel plus small initramfs,
serial-only output, no shared writable host mounts, no host devices, no network
backend that contacts outside systems. Virtio networking can expose a private
NIC while isolated from external traffic. Guest-local ext4/tmpfs mounts and
loop devices can supply independent filesystem conditions. Independently
verify each fault before Alibi execution. This is a proposal, not a boot result
or storage allocation authorization.

## Kernel asset boundary

`/boot/vmlinuz-6.5.0-44-generic` exists (14,304,328 bytes) but is unreadable
from this environment (0600, displayed owner nobody). The symlink /boot/vmlinuz
points there. `/boot/initrd.img-6.5.0-44-generic` is readable (66,034,045 bytes).
`/boot/config-6.5.0-44-generic` and matching
`/usr/lib/modules/6.5.0-44-generic` are readable. Config confirms built-in
loop, virtio block/network, ext4 and tmpfs; dummy, e1000 and XFS are modules.
No readable replacement kernel was found in bounded inspection of /boot,
module-directory top levels, named ABSD physical assets, or the named
/tank/nfs/vms directory top levels. This is not an exhaustive filesystem claim.
The old vms entries inspected had no boot directory; they are not established
boot assets. ABSD physical ISOs are a different OS and are unsuitable here.

A privileged authorized read of the installed kernel, or acquisition of a
verified Linux distribution kernel package, is still needed. This lane did not
attempt either. Existing assets from other campaigns retain their ownership
and recovery dependencies; finding a file does not authorize modification,
retirement, or a writable attachment. A fresh kernel acquisition needs exact
package identity, integrity verification, allocation, and retirement records.

## Durable supervision

Existing local campaign practice is detached user-systemd producers with null
stdin, recorded InvocationID, source/protocol/input hashes, phase/status,
terminal record and RECOVERY.txt. Concrete documentary precedent:
`/data/git/absd-kernel-lane-smp-measured-borrow/docs/smp-common-path/BASELINE.md`
at source `921e5bd90c2ae493dacf78aa0b9919f4ece96b22`, plus its
`docs/smp-preemptive-ap/CHECKPOINTS.md`. The similarly named
`scripts/durable-boot.sh` concerns repeated boot of retained disk state; it is
not itself proof of supervisor-loss survival and must not be confused with the
user-systemd mechanism.

The visible user bus is /run/user/1000/bus. Direct restricted-environment
`systemctl --user show --property=Version --property=SystemState` failed with
Operation not permitted. Campaign root independently reported that the same
read-only check succeeded through approved escalated execution, showing a
degraded user manager. Its raw observation belongs in campaign evidence;
this lane did not independently execute that escalation. An actual admitted
producer still needs an identity/recovery checkpoint and a bounded
supervisor-disconnection rehearsal before unattended qualification. No claim
about survival follows solely from the existence of the bus or binary.

Fresh supervision must inspect the original unit InvocationID, result/status,
phase, logs and immutable terminal records without restarting the producer.
Keep QEMU and evidence collection in the same admitted lifecycle; use bounded
resource/time limits and terminal records even when a guest fails to boot.
No SSH identities are necessary for a serial-driven minimal guest.

## Storage admission and disposition

/tank/nfs is currently NFSv4.2 mounted read-only in this environment. It is
unsuitable as a writable campaign destination here. Campaign root owns the
pending destination decision and aggregate admission; this lane made no
alternative large allocation. Read-only sample at 16:47 UTC: / available
385,733,566,464 bytes and 57,710,129 inodes; /data available 630,827,114,496
bytes and 120,384,512 inodes. Both exceed the 64,424,509,440-byte reserve,
but active tenant allocations and unknown consumers were not reconciled by
this lane. These samples do not independently admit allocation.

Only this small result document and an immutable completion notice were
created by this lane. Retain them as campaign decision evidence through
independent acceptance and the release disposition review; root owns their
subsequent repository/custody disposition. No build output, image, cache,
credential, generated corpus, or large log was created. Next review event:
campaign storage admission/closeout.

## Formalization consideration

Proposition: a QEMU guest can isolate Linux fault state from the diagnosed
host, and a detached user-systemd producer may survive supervisor loss.
Both remain conditional on actual invocation configuration and observed
execution; no model or empirical claim is asserted here. Practical configuration
review, a minimal boot, and a supervisor-disconnection rehearsal are sufficient
for this bounded lab-admission decision. Root is decision owner. Unknowns:
TCG boot compatibility, guest feature/tool behavior, actual durable producer
lifecycle, writable destination, and kernel access. None may be converted to
PASS without evidence.

## Reusable construction scripts (follow-on lane)

`campaign/build-initramfs.py` constructs only after the caller names an existing
storage admission record, owner, retirement condition, exact Alibi executable
and a fresh output directory. It inspects trusted executable dependencies with
ldd, copies their exact bytes, includes GNU df/dmesg/ip plus static BusyBox,
and emits BUILD.json and initramfs.cpio.gz. The guest mounts proc, sysfs and
devtmpfs, explicitly reopens /dev/console, then switches to a tmpfs root. A
caller-provided scenario executes only in that guest. Optional tools and module
files are explicitly supplied; the module kernel-release check must match.
Module dependency ordering/loading is the scenario's responsibility.

`campaign/run-guest.py` runs one supplied kernel/initramfs under TCG with
one CPU and 512 MiB RAM by default. Its argv includes no shared host directory,
host block device, SSH identity, or external network backend. Optional virtio
NICs attach to isolated QEMU hubs. START.json binds source/input/protocol
hashes, checkpoint, owner and retirement. EXECUTION.json records the QEMU PID
and /proc start identity; TERMINAL.json records exact execution status, elapsed
and resource measurements, log hashes, and whether the matching guest terminal
marker appeared. Files are created in an exclusive new run directory. A
terminal marker is not oracle acceptance. The default 180-second timeout and
64 MiB per-output file ceiling bound an unsuccessful guest. The caller must
review that these bounds fit each admitted experiment.

Run the runner as the command of an admitted detached user-systemd service
with null stdin and a named InvocationID/recovery checkpoint. It does not
establish the service lifecycle itself. Do not launch it as an interactive
agent-owned long-running process. Record the service identity before prolonged
waiting and reconcile that same identity after supervision interruption.

Validation so far: Python AST parsing for both scripts, both --help interfaces,
and /bin/sh syntax checking of all three constant shell-script fragments.
No archive construction, kernel boot, tool-output compatibility trial, or
supervisor-loss rehearsal has been performed by this lane. Subsequent execution
must report failures as qualification gaps rather than infer a working lab
from syntax checks. The code and small records are reproducible campaign source;
root owns acceptance and closeout disposition. Next action remains root's
admission followed by one minimal kernel-scream guest trial.

Formalization consideration for scripts: isolation is scoped to their fixed
QEMU argv (no host-share/device/external-network options) and guest-only scenario
execution. Durability relies on the established user-systemd lifecycle, not on
a new formal protocol. Practical argv review, exact input hashes, exclusive
receipts, a minimal boot and an interruption rehearsal are the chosen evidence;
root owns acceptance. Inputs are campaign-controlled files and executables;
content mutation between hashing and QEMU consumption is not modeled or
prevented by these scripts and must be excluded by sealed run custody.

Construction review correction: the builder now inserts a synthetic newc
`dev/console` character-device header (major 5, minor 1, mode 0600) before the
archive trailer, without creating any host device node. A memory-only archive
containing the exact helper-generated header was listed successfully by GNU
cpio, confirming character-device type and numbers. This permits the kernel's
initial console open before /init mounts devtmpfs; /init also reopens the
console explicitly. Both builders now check available blocks/inodes on the
actual destination parent as well as the independent / and /data hard floors.
Construction copies are capped at 60 MiB inside the 128 MiB construction
envelope; capture requires a 160 MiB / 16-inode envelope, covering both bounded
QEMU output files and small receipts. Aggregate tenant admission still belongs
to root; these local checks cannot replace it.

Root reported a completed durable-producer probe at
`/data/git/alibi-qualification-evidence-20261011/durability/59150f02-d5cc-438f-86e4-206b3fb3d007`,
service InvocationID `3b0bb9f27a73465b8db2f238421a9678`, PID 2075869,
parent PID 2379: a terminal record was written after the launching tool exited.
This establishes that bounded probe, not whole-manager/provider recovery or a
QEMU producer's completion. This lane has not independently accepted the probe.

## Admitted minimal laboratory support revision

Campaign root reported operator admission of the 4 GiB local
`/data/git/alibi-qualification-evidence-20261011/guest-lab` envelope and a verified
Ubuntu 6.5.0-44 kernel acquisition. Root owns those receipts and all allocations;
this lane did not launch a guest or copy runtime assets. A read-only inspection
of root's smoke archive
`bae947c2ebdfa40eac807e58bed36783ea0bfc4808e953206b66474dbb0eedfc`
confirmed executable root-owned init/stage2/scenario and Alibi, BusyBox /bin/sh
symlink, helper presence and synthetic /dev/console major5/minor1. Archive size
3,752,472 bytes; source guest tree 8,189,272 bytes. Root's BUILD.json retains
actual input identities. This proves archive properties, not Linux boot.

Further harness review corrected archive ownership to uid/gid0, confirmed
stage2 remains executable when copied into tmpfs, and identified compressed
module loading as a gap. Explicit module inputs now require matching vermagic,
are decompressed to /modules/*.ko during construction, and retain both original
compressed-source and resulting-byte hashes. Root's kernel and installed module
release must match. `campaign/linux-6.5.0-44-assets.json` records exact available
module identities and dependency order; it performs no module insertion.
Loop/ext4/jbd2/mbcache/crc16 and virtio block/network are built in. Dummy/veth,
conntrack, overlay, XFS and two device-mapper fault modules are optional assets,
not newly qualified behavior. tc dynamic plugins remain uninventoried.

`campaign/guest-capture.sh` provides guest-only `lab_run CASE COMMAND...`.
It records unique-case argv, stdout, stderr, GNU time resource measurements,
exit status and monotonic uptime endpoints under /run/alibi-captures/CASE.
A GNU timeout bounds each command to 30 seconds with a 2-second kill grace.
Diagnostic nonzero exit is retained as data and does not abort a shell using
`set -e`; helper transport success returns0, helper failure125. Unique labels
are required; files remain until guest teardown. No expected verdict enters the
helper. The builder includes GNU time/timeout and the helper by default.

Each captured file becomes a byte-length/SHA-256-checked base64 serial frame.
`campaign/capture_frames.py` reconstructs files, rejects duplicate/corrupt/
truncated/wrong-run frames, and reports missing components. Runner captures
these errors separately from QEMU/guest termination and does not equate either
with oracle acceptance. The fixed `--case-id` token becomes alibi_case in the
kernel command line; it cannot add arbitrary QEMU or host mount options. The
runner now admits 224 MiB of capture headroom to cover its two64 MiB bounded
QEMU files plus decoded evidence and receipts. Parent campaign still controls
aggregate allocation and serialization.

Validation: AST checks for three Python files; shell syntax check for capture
helper; memory-only parser controls for complete frames, altered bytes,
duplicate frames, missing trailer and wrong run identity. No kernel fault
result is claimed by these helper checks. The parser's nontrivial proposition
is bounded transport integrity and incomplete-evidence detection, assuming
SHA-256 integrity and campaign-controlled input identities. Exact source
identities are linked from the latest immutable lab lane notice. Practical
negative controls and independent campaign review are sufficient for this
small transport; no model of kernel behavior or provider recovery is implied.

## Extended matrix support and observed smoke execution

This lane subsequently read the actual smoke terminal and captured JSON:
run `f714748f-92bd-451c-b303-0dd5b5e70bb6` exited QEMU0 with guest terminal0,
4.474616730003618 seconds, five valid capture files and no capture errors.
The captured kernel-scream receipt said CLEAR. This confirms the minimal guest
and transport executed; independent oracle acceptance remains root/reviewer
work, and this observation does not qualify additional accusations.

Root admitted the extended G12/G16/G22/G24/G25/G26/G35 laboratory work within
its existing4 GiB destination. `campaign/extended-builder-args.json` supplies
an exact reusable argv list of ten additional tools and eleven distinct
matching module files. It includes nftables/conntrack hooks, XFS and overlay;
device-mapper error target is part of builtin dm_mod. `dmsetup --noudevsync`
avoids relying on an absent udev daemon. GNU nft, XFS tools, dmsetup and Bash
all have available shared-library closures handled by the builder. No Python
userspace bundle is needed for the bounded Bash UDP generator. Tool/module
availability still requires actual per-case verification.

The new builder `--overlay-case G22` selects an actual boot-root transition:
for that kernel command-line case alone, /init creates a private tmpfs substrate
with lower/upper/work directories, copies its constructed userspace into lower,
loads the exact supplied overlay module, mounts overlay on /newroot and performs
the same switch_root. Other cases retain tmpfs root. This is not a chroot-only
substitution. Its actual /proc root mount identity must be established by the
G22 oracle after boot. Python AST and the generated conditional shell fragment
passed syntax checking; this lane did not launch the overlay case. The property
is bounded guest root construction with an explicit case selector, under the
already-reviewed QEMU isolation argv. Concrete post-transition mount observations
and independent case evaluation are the chosen qualification; no new abstract
state model is needed for this small fixture. Root owns the decision.

Second-environment candidate metadata is retained in
`campaign/linux-6.8-kernel-candidate.json`: Ubuntu signed
linux-image-6.8.0-90-generic version6.8.0-90.91~22.04.1,
14,736,848-byte package, SHA-256
`26566b977d86105903d8f4ca8e9d807b4cc7791f2ad9065e634a6dcb687fe129`,
selected from root's existing verified Packages.xz. This lane downloaded
nothing. No readable matching6.8 configuration was found at the named local
header/config paths, so builtin ext4/virtio behavior requires an actual boot
check. Host6.5 modules cannot be reused. A second kernel with the same constructed
userspace is a second kernel environment, not qualification of two complete
Linux distributions. Root owns acquisition, admission, execution and retirement.
