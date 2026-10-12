# Initial field observations, 2026-10-12

These are authorized machine observations, not a timed human study. No actual
incident symptom was supplied: these runs are reconnaissance, not proof that
Alibi solved an incident. Raw host receipts remain in private local custody.

The released executable on the development host reported increasing RX drops.
An independent later sysfs sample also recorded increasing drops. That supports
recurrence of the observable, not application impact or cause. Kernel-log and
filesystem accusations cleared within their observed evidence; EDAC was
unavailable and memory integrity remained UNKNOWN. Restricted dmesg access
used a journal fallback explicitly recorded in the receipt.

The older-GNU binary ran on a user-owned Ubuntu 22.04.5 VM with glibc 2.35 and
Linux 5.15.0-194. Human and JSON interrogations completed in about two seconds:
three bounded CLEARs and EDAC UNKNOWN. Independent mount, capacity, interface,
route and conntrack observations were captured. This does not establish service
health, DNS functionality or absence of historical/transient faults. The scoped
temporary executable was removed. No faults or service changes were made.

In Debian 12 userspace, a restricted read-only container loaded the executable,
produced JSON, identified a read-only filesystem and reported missing kernel
logs as UNKNOWN. This establishes userspace compatibility, not independent-kernel
host qualification.

For actual operator investigations record: symptom/time; hypothesis selected;
manual checks planned before Alibi; bounded conclusion and evidence limits;
checks avoided and still required; independently established diagnosis; any
CLEAR interpreted too broadly; whether UNKNOWN made sense; and willingness
to use it again. Do not equate invocation latency with human diagnostic time.

[Issue #3](https://github.com/unpingable/alibi/issues/3) remains open for incident
and human feedback. No new accusation family is justified by these runs alone.

## Disposable synthetic filesystem observations

One isolated QEMU boot with Linux 6.5.0-44 and constructed GNU userspace ran
three observations using the compatibility binary. A guest-local ext4 loop
filesystem was writable with free space (CLEAR), filled until dd reported
ENOSPC and df showed zero available blocks (DEGRADED), then emptied and
remounted read-only (DEGRADED). Mount records independently confirmed the state.
No host disks, directories or network backend were attached to the guest.

This is a small disclosed functional check, not a fresh holdout campaign or
Debian/Ubuntu distribution qualification. The retained parser reported an
unrecognized oracle frame kind; all three raw frames were recovered separately
from serial and checked against their recorded lengths and SHA-256 hashes.
The original warning and execution receipt remain intact.
