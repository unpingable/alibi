# Bounded guest carrier transition fixture

Owner and guest launcher: root campaign agent. Lab lane owns the optional
runner/controller and separate guest-carrier-cases.sh; no product changes.
Admission: at most64 MiB new carrier inputs/build output inside the existing
4 GiB guest-lab envelope and root's pending reserve. One guest at a time.

## Mechanism and authority boundary

The existing runner optionally accepts --carrier-control for G09/G15 only,
requires exactly one isolated NIC, and can select the QEMU e1000 device model.
The NIC stays attached to an internal QEMU hub with no external network backend,
host link, shared mount or host device. The in-process controller addresses only
the fixed device labnic0. It is not a daemon or an Alibi capability.

The guest emits ALIBI_CARRIER_REQUEST with the exact run ID, increasing sequence
number and alternating down/up state. The controller permits two events for G09
and six for G15, rejects wrong identities, duplicates, gaps and wrong ordering,
and records each QMP request/response with monotonic host timing. Socket records
are bounded to64 KiB and replies to32 per command, with two-second socket I/O
and an overall guest timeout. The private socket directory is mode0700 and the
QEMU socket is checked for owner-only mode and matching UID. After QEMU is reaped,
the exact socket identity is retired and the empty directory removed. Unexpected
identity/retirement failures remain explicit terminal gaps for root review.

QMP set_link changes the emulated adapter's link state. The official
[QEMU QMP set_link specification](https://www.qemu.org/docs/master/interop/qemu-qmp-ref.html#command-set-link)
warns that command success alone does not establish guest link notification.
Accordingly the guest independently waits for sysfs carrier changes and captures
operstate, counters, actual route tables and raw kernel logs before/after.
A QMP reply is never the oracle for the resulting Linux condition.

## Scope and oracle

G09 first establishes a healthy control, preserves a usable default route, then
captures carrier0 and executes the supported external network accusation. The
current accusation does not positively accuse a down/unused interface by itself.
CLEAR is bounded and potentially justified if all required evidence is adequate,
error/drop deltas are zero and no other in-scope indication exists. It says
nothing about destination reachability.

G15 performs three independently observed down/up cycles and compares actual
interface-attributed recognized link-down log events against the baseline count.
Attribution uses an exact interface token boundary, not a bare substring. Raw
logs and observation times are retained. Three newly observed matching events
establish the specified flap-message condition; absence of those messages is
explicit F7 for that experiment, even if physical carrier transitions happened.
The receiver still captures Alibi's actual receipt when this coverage gap occurs.

All invocation outputs use the existing hash-checked capture protocol. Control,
transition raw logs, fault oracle, fault receipt and restored-state oracle have
unique labels. No expected verdict is sent to the Alibi process.

## Exact construction inputs and initial validation

The e1000 module was selectively extracted, without downloading or installing,
from the previously verified exact kernel-matching package
linux-modules-6.5.0-44-generic version6.5.0-44.44~22.04.1. Module SHA-256:
`63559ae7fa359f35f56a8f1dfc3216c26b95761d699a6cb30de4e39d5534a957`.
Its dependency list is empty. INPUT.json and the exact371,833-byte module are in
`guest-lab/carrier-inputs-cc5d42ed-3283-409e-a11d-15cbe63905e4`.

Baseline archive:
`guest-lab/carrier-build-69f46538-a452-43cd-8f86-1cc7f4240938/initramfs.cpio.gz`,
SHA-256 `5bf652ad3cc1682306a5c6f50f9b85248a8a9a3c626d1851e5e338e592252be8`.
The expanded build/ archive occupies12,759,040 allocated bytes. Combined with the
small module input, this stays below the64 MiB grant. Root owns final closeout;
expanded build output is reproducible, while exact module/archive custody lasts
through named replay/acceptance review, no later than2026-10-18.

Root must snapshot carrier_control.py beside run-guest.py and capture_frames.py
in each carrier producer. Use caseG09 orG15, --carrier-control --nic-model e1000
--isolated-nics1 --memory-mib512 --timeout90 and the exact6.5 kernel. The lab lane
has not launched this VM. Initial checks cover Python/shell syntax and pure
controller valid-sequence/wrong-run/duplicate/order/boundary negative controls.
Those checks are not substitutes for an actual QMP/kernel trial.

## Formalization consideration

Proposition: the optional control path can change only the fixed NIC of its
owned QEMU occurrence through a finite ordered set of run-bound requests, and
no successful control reply alone qualifies a Linux fault. The scope assumes
campaign-owned immutable scripts and private socket custody. Source/input
identities are linked by the lane completion notice and actual producer START.
The controller's small finite request gate is exercised directly by deterministic
negative controls; runtime argv, socket ownership, raw responses and guest state
must then be inspected independently. These practical checks are sufficient for
the admitted fixture; no generic QMP control plane or new lifecycle model is
introduced. Root owns acceptance. Provider recovery, arbitrary guest control,
and correctness of unrelated QEMU commands are outside the claim.
