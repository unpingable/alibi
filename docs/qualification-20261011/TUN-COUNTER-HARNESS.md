# Guest TUN counter qualification fixture

This is a bounded successor to G13/G14 veth fixtures that did not establish a
counter increase. Those original induction failures remain F7. This construction
record does not claim the new fixture has run or changed any kernel counter.

`campaign/tun-drop-fixture.c` is compiled on the development host but must never
execute there. Before opening TUN or creating fixture state it requires effective
UID 0, hostname `alibi-disposable-guest`, a nonempty `alibi_run` kernel command-line
token, and case G13 or G14. These checks reduce accidental execution; the primary
authority boundary remains the campaign-owned QEMU guest and root's launch policy.

The helper creates guest-only `aqtun0` using TUNSETIFF with IFF_TUN|IFF_NO_PI. It
writes eight zero bytes (invalid protocol-version nibble) every 5 ms for a bounded
interval, then keeps the interface alive without writes. Write success or failure
is not evidence of a drop. The guest script independently reads actual sysfs
`rx_dropped` counters and captures a 50 ms counter/uptime trace across Alibi's
ordinary CLI invocation. It does not replace proc/sys sources or alter counters.

G13 requires actual increasing counters during the collection window. Whole-process
bracketing alone is insufficient if temporal overlap is ambiguous. G14 requires a
nonzero historical counter, stopped emission, three seconds of stable samples,
and stable bracketing observations through collection. The independent evaluator
must inspect the trace and captured receipt. No increase, incomplete observations,
or ambiguous overlap is F7 for the intended fixture, not a pass. A bounded CLEAR
also requires all other required evidence and no applicable positive condition.
Software TUN receive drops establish no hardware-fault or transport-loss equivalence.

The guest emits initial control, ordinary case invocation, helper execution, and
counter trace through the existing five-part capture protocol. It records routes,
interface details, raw dmesg and verifies interface retirement after closing the
TUN descriptor. Helper generation and hold intervals are each at most 20 seconds;
the capture timeout and guest timeout bound the overall experiment.

Baseline proof inputs:

- Evidence root: `/data/git/alibi-qualification-evidence-20261011/guest-lab`.
- Compile receipt: `tun-inputs-88d2b6a9-6ae6-47fb-87ca-cdd2bbe4be5a/INPUT.json`.
- Archive receipt: `tun-build-88d2b6a9-6ae6-47fb-87ca-cdd2bbe4be5a/BUILD.json`.
- Archive SHA256: `683d177a134e97d32d9a68d974093281cf03d736f78a660cf3c223ae72bfdd93`.
- Exact existing 6.5.0-44 guest kernel has CONFIG_TUN=y; no extra module is needed.
- Runner: case G13 or G14, one isolated NIC, 512 MiB, 60-second timeout.

Compilation with `-std=c11 -O2 -Wall -Wextra -Werror` and shell syntax checks passed.
No host execution of the compiled helper occurred. Root owns guest scheduling and
subsequent result acceptance. The original baseline executable is in this proof
archive; a candidate replay is a separately identified occurrence.

Storage owner is root; fixture creator is lab_feasibility. The authorized new
allocation envelope is 64 MiB inside root's existing pending reserve. Admission
measured destination and both required filesystem reserves before construction.
At acceptance/closeout, no later than 2026-10-18, root reviews exact replay
requirements: retain source, compile/build receipts and minimal result evidence;
retire reproducible build substrate after consumer/dependency checks. An archive
needed for a named replay remains custody until that replay dependency ends.

Formalization consideration: the proposition is only that a finite guest-local
write sequence can produce independently measurable kernel receive-drop state,
and that sampled state can justify the documented bounded diagnosis. Kernel
implementation behavior and scheduling are empirical unknowns here. Guards and
finite bounds are reviewable directly; real-guest counter traces and independent
oracle review are more informative than a model that assumes the counter effect.
Decision owner: lab_feasibility, with root/oracle acceptance separate. A future
sampling-order model could clarify overlap assumptions; it is outside this fixture.
