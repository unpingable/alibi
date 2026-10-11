# Corrected laboratory executable and module identities

Owner: root campaign agent. Implementer: lab_feasibility lane. Original sealed
baseline records remain unchanged. These are F7 laboratory corrections, not
product fixes or replacement evidence for failed original experiments.

## Demonstrated initial limitations

The original route-baseline serial record shows BusyBox ip usage despite
/bin/ip containing the intended iproute2 executable. A separate read-only
host command demonstrated the mechanism: BusyBox ash resolves unqualified ip
as its internal applet, while Dash resolves the external executable. Original
G18 specifically reported `mke2fs: invalid option -- 't'` followed by BusyBox
mke2fs usage before its F7 record. Required external tool bytes were present;
the shell command-resolution behavior prevented the intended tools from running.

Original G08/G12/G16 logs contain module symbol-version disagreements, and G22
contains overlay symbol-version disagreements. Host modules and the downloaded
Ubuntu Jammy HWE kernel report the same release/vermagic, but that is insufficient
for symbol-CRC compatibility. The original overlay bootstrap then entered an
interactive shell and consumed its bounded timeout. No force loading was used.

## Bounded corrections

The builder now copies Dash plus its library closure as /bin/sh. BusyBox remains
available for its explicit applets. Oracle-owned matrix corrections additionally
pin required external tools with absolute paths. The matrix loads dummy with
numdummies=0, preserving a genuine zero-interface control before an explicitly
created dummy interface. Bootstrap command failures now print
ALIBI_GUEST_BOOT_FAILED and power off instead of entering an interactive shell.

The exact module package is linux-modules-6.5.0-44-generic version
6.5.0-44.44~22.04.1, SHA-256
`92dc300885d624f758dd1ea003c2958e54b2d2fd01d42fd80f6a1a1a7e681d53`.
Its version matches the independently acquired kernel package. Its stanza was
selected from the retained, already signature-verified Packages.xz and the index
hash was checked against KERNEL.json. Acquisition verified exact package length
and SHA-256, selected only eleven required module files, checked dependency
closure against its builtin list, and recorded source package/member/module
hashes. The larger modules-extra package was unnecessary and was not downloaded.
No package was installed into the host and no host module was inserted.

Acquisition ran under existing detached user-systemd:

- occurrence: matching-modules-51279060-4abd-4eaf-8f50-3ac57507374a;
- unit: alibi-modules-51279060-4abd-4eaf-8f50-3ac57507374a;
- InvocationID: e92569ee4a724213899226813dd6fa13;
- producer PID:2187107, parent2379;
- terminal: VERIFIED_SELECTIVE_MODULE_CUSTODY after3.010 seconds.

RECOVERY.json, INPUT.json, EXECUTION.json, PROGRESS.json and TERMINAL.json remain
under that occurrence in guest-lab. Root granted this lane a256 MiB aggregate
acquisition/proof envelope within the shared4 GiB guest-lab admission. Root
remained the sole guest-runtime launcher; this lane launched no VM.

## Independent-kernel proof of the corrections

A new root-launched proof used the original baseline Alibi executable, the exact
kernel and package-matched modules, and the corrected builder. Archive SHA-256:
`37f1eccc9991a3840875721d3fcc46d002b90952e77269d8f2cb54929e12d561`.
Run44d30945-aba5-44fe-9cc0-78698e8dc9c6 completed QEMU0/scenario0 in
9.885183482983848 seconds with five intact capture files and zero capture errors.
The terminal SHA-256 is
`ed13f37d77ac40a493b00747f81aa42cadf285c85f5facc9b33cc45f0c47466a`.

The inspected serial evidence establishes:

- iproute2 version output through unqualified ip under the new shell, plus GNU
  df and util-linux dmesg/losetup interfaces;
- zero non-loopback interfaces before and after dummy module insertion with
  numdummies=0; explicit dummy creation, detailed inspection and deletion;
- successful veth creation, detailed inspection and deletion;
- successful package-matched netfilter/conntrack module insertion, a real nft
  output hook with ct-state rule, independent rule listing and count0/max4096,
  then table deletion;
- XFS module insertion and its /proc/filesystems entry;
- actual ext4 creation in a32 MiB guest-local image, loop attachment, mount,
  write/read equality, independent statfs/df, unmount and loop detach;
- supported external Alibi CLI execution with captured argv/stdout/stderr,
  resource measurement, exit status and receipt bytes.

Proof evidence is guest-lab/harness-proof-batch/guest. The proof does not yet
establish full-filesystem behavior, XFS shutdown, conntrack pressure, overlay
root behavior or any final holdout result. Those require their own corrected
experiments. Its larger guest resource footprint is not an Alibi performance
regression measurement because the guest image and exercised setup changed.

## Custody and storage handoff

New lane-owned allocations at handoff:

- exact module acquisition occurrence:33,218,560 allocated bytes;
- harness-proof-build-cf0c776e-cbe4-4a90-aae6-d6c23b816f21:32,210,944 allocated
  bytes, including an8,928,877-byte archive and23,043,210-byte logical guest tree;
- combined65,429,504 allocated bytes, within the granted256 MiB envelope.

At measurement the entire guest-lab used380,317,696 allocated bytes. Available
space was385,399,455,744 bytes on / and629,827,342,336 bytes on /data. Concurrent
host changes are not attributed to this lane. No bytes were reclaimed by this
lane during active campaign execution.

Root is the named successor custodian and closeout owner. Exact selected module
bytes and required archive inputs have a current replay/acceptance dependency.
Package copies and the expanded proof build tree are reproducible; retire them
at campaign closeout after verifying exact retained replay custody and absence
of live consumers. Preserve original mismatched-module archives as necessary
failed-occurrence replay evidence until independent review determines the
minimal custody set. Next review is final campaign acceptance/closeout, no later
than2026-10-18. No other tenant cleanup is authorized.

## Formalization consideration

The failed assumption was that matching release/vermagic implied compatible
module symbols, and that PATH placement implied the intended external command
executed. Both have concrete counterexamples above. The replacement claim is
narrower: exact package provenance plus actual successful insertion, and an
external-command shell plus observed command identities, establish only the
particular laboratory configuration exercised. Exact source/artifact identities
are linked by the lane completion notice, BUILD and occurrence records. Root
owns acceptance. Practical independent-kernel negative and positive controls
are sufficient for these construction choices; no general compatibility or
shell-resolution theorem is claimed. Future package/kernel combinations must
repeat correspondence checks instead of relying on this result.

## Successor dispatch boundary

After the corrected harness proof, root launched the separate39-case corrected
baseline producer under unit
`alibi-corrected-baseline-90816699-b0c4-4958-9273-d38abb709639`, InvocationID
`04acb1bb8a78408d969737f26a7c0aa1`. Its matrix/archive identities and results
belong to the root/oracle dispatch records. G24 was admitted at1024 MiB RAM;
other guests at512 MiB, serialized. This lane performs no further guest launch
and makes no acceptance claim about that still-separate producer. Its named
next action is root's original-identity supervision and independent evaluation.
