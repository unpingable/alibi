# Independent qualification matrix — 2026-10-11

Status at specification: **all cases UNEXECUTED**. This inventory records proposed experiments, not qualification or acceptance. Source inspected: `6f8ad5fba4ed6f3fb9f32c9930377a0a48e06931`. Oracle/design owner: `oracle_design` lane; execution and storage owner: campaign root. Canonical execution ownership/status remains GitHub Projects; this matrix is a technical specification and evidence pointer only.

## Contract and independent oracle

K = `kernel-scream`; S = `silent-killer`; M = `memrot`; N = `network-choke-point`. C/D/U/NA = CLEAR/DEGRADED/UNKNOWN/NOT_APPLICABLE. Normative basis is `docs/design.md` §§3–7 and `docs/envelope.md`, read independently of test expectations. The implementation is a subject of comparison, not the oracle.

K requires readable adequate kernel logs. S requires supported root, all relevant mount/space/inode evidence, and kernel log. M requires EDAC plus kernel log; missing VM EDAC normally means U. N requires interface samples, both route-table observations, kernel log, and conntrack when present. A documented positive requires D even with another source missing. No positive plus inadequate required evidence requires U. Domain absence must itself be adequately observed before NA.

A real fault outside these four named indication sets is F6, not automatically a false CLEAR. In particular, the normative contract explicitly allows unused/down interfaces, does not probe DNS or application behavior, treats cumulative counters differently from sampled deltas, and excludes storage latency and resource contention as standalone accusations. Do not widen the contract after seeing a result. Ambiguous broad summary wording is separately reviewable.

## Environment and per-trial protocol

Environment A: admitted disposable Linux QEMU/KVM guest with ext4 root, real proc/sysfs and independently booted kernel. Environment B, if available: second distribution/kernel and standard-tool implementation; never claim a second environment based only on a new namespace. Exact image/kernel/binary hashes and versions must be supplied by execution receipt. No guest has access to host block devices, host service cgroups, or production networks. Mutations are guest-local. Private mount/PATH substitutions remain explicitly fixture qualifications even when run inside a real guest; do not count them as actual kernel fault induction.

Every row starts from an independently checked healthy control (G01, except deliberately different applicability/root conditions), fresh guest or documented restoration, readable quiet logs, stable route/counter state, ample supported filesystem capacity. Archive baseline first. Record fixture command, observed fault, condition proof, binary hash, exact invocation, stdout/stderr, exit status, JSON Receipt V2, monotonic chronology, resource/timeout data, and restoration outcome. Collect serial output outside the guest so full/read-only/stalled guest filesystems cannot erase evidence. Run ordinary `alibi check <accusation...> --json`; capture default human output in paired controls without pretending sequential runs have identical host state.

Allowed verdicts below are conditional on independent observations, never on injected-command success. Unexpected additional positives require adjudication from their raw evidence. Failure to establish baseline, fault, collection interval or restoration is F7/INCONCLUSIVE, not PASS. Clear eligibility requires all named evidence adequate. Repeat G13/G16/G24/G31/G32/G33/G35/G37 at least three times when admitted. Single-shot timing luck is insufficient.

| ID | Family | Accusations | Controlled change | Independent condition proof | Allowable conclusion and rationale |
|---|---|---|---|---|---|
| G01 | Healthy control | K,S,N | Fresh ext4 guest, working virtual NIC and default route, quiet real boot log, ample free blocks/inodes. | Capture raw log, mountinfo, statvfs and both route tables; bracket interface counters independently. | C for K/S/N if every required source is genuinely readable/complete; M U if EDAC absent. |
| G02 | Applicability control | N | Guest with loopback only; no emulated NIC. | Enumerate links via ip and sysfs; no non-loopback links at either sample. | NA; missing routes cannot manufacture a positive indication outside the interface domain. |
| G03 | Privilege boundary | K,S,N,M | Fresh guest; restrict kernel log and execute as unprivileged user without journal membership. | Record UID/groups, failed dmesg and journal access, absence of other positives. | U for all affected accusations; denied logs cannot authorize C. |
| G04 | Virtual hardware scope | M | Normal guest with no EDAC controllers. | Enumerate EDAC directory and kernel modules independently. | U unless real positive memory-log evidence yields D; no hardware integrity C. |
| G05 | Missing route | N | Delete IPv4 and IPv6 default routes with interface up. | ip route show table all plus raw proc route tables; preserve link/counter state. | D when both tables complete; U if inability to observe prevents absence proof. |
| G06 | Rejected route | N | Replace only default with unreachable route; no usable default in other family. | ip route get must reject; capture route type and raw flags. | D if complete route tables establish no usable default. |
| G07 | Blackhole route | N | Install blackhole default only. | ip route get and table output establish blackhole; inspect device-less proc row. | D on complete evidence; C is not allowed merely because destination prefix is default. |
| G08 | Unused down interface | N | Keep main NIC/default healthy; add unused dummy and set it down. | ip link proves administrative state; counters stable; no repeated matching down logs. | C allowed: down/unused alone is explicitly outside indication set; no unsupported D. |
| G09 | Carrier loss | N | Detach emulated NIC link while preserving route; stop before third log-down event. | QEMU link state plus guest carrier, routes, retained log and counters. | C allowed if required evidence complete and no defined indication; otherwise D/U as justified. Record F6 connectivity limit. |
| G10 | Bad next hop | N | Set default via an unused on-link guest-local gateway. | Route entry remains; independent guest probe shows unresolved neighbor; capture ARP state. | C allowed absent defined positives. Failed probe is independent observation, not an Alibi accusation. |
| G11 | DNS unavailable | N | Keep NIC/routes healthy; point resolver to guest-local non-listening DNS address. | getent timeout plus resolver bytes and route/counter/log evidence. | C allowed for bounded N, no inference of DNS health; F6 DNS coverage. |
| G12 | Packet filter | N | Guest-only nftables drops target traffic without interface counter delta. | Rule counters increment and peer fails; verify interface error/drop counters independently. | C allowed if no named positive; firewall-caused loss is not necessarily interface drop evidence. |
| G13 | Sampled drops | N | Use bounded guest-local packet flow and queue policy to induce interface drop counter increase. | Two independent timestamped sysfs/ip samples show actual drop delta within Alibi window; tc counters alone insufficient. | D only if sampled interface drop increase is observed; failed induction is F7. |
| G14 | Historical counters | N | Induce drops, stop traffic, then interrogate stable nonzero cumulative counters. | Bracket counters before/after Alibi and show equality despite nonzero values. | C allowed when other required sources complete; cumulative value alone cannot support D. |
| G15 | Repeated link transitions | N | Toggle emulated link sufficiently to generate at least three real interface-attributed down events. | Preserve raw guest kernel messages and independent link transition ledger. | D for recognized documented events; absence of actual logged transitions is F7, not PASS. |
| G16 | Conntrack pressure | N | Use guest-local traffic with a bounded reduced guest conntrack maximum to exceed 90%. | Read count/max before and after and preserve table entries; keep threshold exceeded during collection. | D when count/max establish threshold; no claim based only on generator success. |
| G17 | Conntrack absent | N | Guest kernel without loaded connection tracking and no conntrack sysctl files. | Kernel config/module list and exact ENOENT observations. | C allowed if other required sources complete; absence must be recorded as scoped applicability. |
| G18 | Critical block space | S | Fill a small guest-owned ext4 mount until independently measured free fraction <=5%. | statvfs and df before/after plus unsuccessful bounded write if ENOSPC intended. | D; distinguish threshold indication from actually exhausted allocation. |
| G19 | Inode exhaustion | S | Fill small-inode-count ext4 with zero-byte files while blocks remain available. | statvfs free inodes <=5%, substantial free blocks, create fails ENOSPC when fully exhausted. | D; receipt must identify inode condition rather than invent byte exhaustion. |
| G20 | Read-only filesystem | S | Remount dedicated ext4 filesystem read-only without another writable view. | mountinfo plus write attempt returning EROFS; source device identity retained. | D with intent caveat; intentional read-only is still declared policy indication. |
| G21 | Intentional bind view | S | Bind mount a directory then make only bind view read-only; backing filesystem stays rw. | mountinfo mount IDs/superblock identity; writes succeed backing and fail bind. | C allowed with explicit view treatment and sufficient other evidence; not proof all paths writable. |
| G22 | Unsupported root | S | Boot a guest with overlay root and readable kernel log; no positive S indication. | mountinfo verifies visible root type; guest kernel is still independent. | U; excluding unsupported root cannot authorize vacuous C. |
| G23 | Space across mounts | S | Mount ample ext4 root plus one critical tmpfs, keeping other mounts ample. | Independent statvfs for every material mount shows only target critical. | D; favorable root must not hide target. |
| G24 | XFS shutdown | S,K | Guest-owned XFS loop image; use supported guest-local shutdown mechanism if available. | Observe XFS shutdown log and guest file operation EIO while mount remains rw. | D on collected shutdown event. Tool/module absence or failed induction is F7. |
| G25 | Block I/O error | K,S | Guest-local device-mapper error target under disposable filesystem with bounded operations. | dm status, failed read/write errno and real kernel messages; independently establish which named log indication exists. | D only for accusations supported by actual emitted events; missing patterns require reviewed scope judgment. |
| G26 | Evidence view denied | N | In guest-private mount namespace, deny one required interface counter to an unprivileged Alibi process. | Outside observer reads real counter; inside same UID observes EACCES, no other positive. | U. This is real guest permission-boundary qualification, not hardware fault injection. |
| G27 | Malformed command evidence | S | Guest-private PATH substitution makes df emit truncated malformed output for one material target. | Save exact script/output; independent real df/statvfs establishes healthy target and required output corruption. | U; label command substitution fixture, not real filesystem pathology. |
| G28 | Empty log acquisition | K | Guest-private command substitution makes both log commands exit zero with empty output. | Save wrapper hashes and stdout/status; independent real log kept separately. | U; label command substitution fixture, not quiet-kernel proof. |
| G29 | DNS plus ample filesystems | N,S | Combine unavailable DNS with otherwise healthy network evidence and supported ample mounts. | Resolver failure independently verified; bracket N evidence and statvfs. | Bounded C allowed for N/S; no diagnostic may claim DNS or host health. |
| G30 | Full filesystem plus failed service | S,K | Critical ext4 mount plus independently failing guest-only systemd service. | statvfs and service result/status separately; do not infer causation between them. | S D; K based on actual logs; service failure F6, never asserted as diagnosed root cause. |
| G31 | Missing route plus log denied | N | Combine verified missing defaults with inaccessible kernel logs. | Both route tables complete and absent defaults; same-user log failure. | D from sufficient route positive; receipt must still disclose missing log evidence. |
| G32 | Counter topology transition | N | Create or remove guest veth interface inside network sampling interval. | Timestamp ip monitor and before/after sysfs listings; show interval overlaps collection. | U without retained positive, D with retained positive; unchanged endpoint names alone may hide replacement identity. |
| G33 | Fault immediately before | S | Fill dedicated filesystem below 5% before invocation and hold stable. | Timestamp threshold crossing before process start plus stable statvfs bracket. | D; repeat critical case at least three times. |
| G34 | Fault removed immediately before | S | Reclaim filled disposable files before invocation; ensure no retained relevant log positive. | Timestamp removal and sufficient free space before/after collection. | C eligible when required evidence complete; historical pressure is not present space evidence. |
| G35 | Mount disappears during collection | S | Unmount guest-private target after mount enumeration and before targeted df. | Trace open/exec order plus mount event ledger and actual df target identity. | U if required target missing/replaced; successful df against parent is not proof of original target capacity. |
| G36 | Route changes after observation | N | Introduce missing default after routes read while later sources still collect. | Trace proc read order and timestamp netlink transition; preserve both before/after routes. | C may be valid only for earlier observation bound; receipt cannot imply atomic current-state proof. If ordering unestablished, F7. |
| G37 | Evidence command stalls | K | Guest-private dmesg substitution stalls; journal fallback independently known. | Capture process tree, timeout/status and pipe ownership; external finite cutoff. | No unbounded hang; U if fallback unavailable, otherwise verdict solely from adequate fallback. F4 if declared hard timeout violated. |
| G38 | Failed application with healthy evidence | N,S,M | Run guest service that accepts TCP but returns deterministic errors; keep bounded evidence healthy. | Capture independent application response and stable named evidence. | C allowed for adequate N/S; M U absent EDAC. No claimed application exoneration. |

## Holdout boundary

Six separately specified final cases H01–H06 are sealed in campaign evidence custody outside this repository. Implementation lanes receive only their count and manifest hash until the candidate source and binary identities are frozen. They are additional to the 38 disclosed case specifications. No holdout has executed at specification time. The final evaluator reveals inputs only after freeze, runs the ordinary CLI, then records outcomes and any disclosure. A repair after holdout failure ends that holdout round; it is not an unlimited tuning cycle. Retain the failed round and qualify a successor only with a freshly separated acceptance decision.

## Measurement rules

Unit of soundness assessment is one accusation in one independently observed trial. Report actual trial count separately from unique scenarios, repeats and substitutions. Actual guest execution, command/path substitution, source-only analysis and unexecuted rows occupy different columns. No total should combine them as real fault trials.

Useful exoneration rate = correctly bounded CLEAR findings / findings whose independent oracle establishes (a) accusation applies, (b) every required evidence source is available, supported, complete and temporally applicable, and (c) no named positive exists in the bounded evidence. Predeclare eligibility before seeing the verdict. UNKNOWN on an eligible case reduces useful exoneration; UNKNOWN on an ineligible case is correct fail-closed behavior. NOT_APPLICABLE is reported separately, excluded from numerator and denominator. Undefined denominator is N/A, never 100%.

Report false CLEAR incidence both as count / all evaluated applicable findings and count / all emitted CLEAR findings, plus scenario/environment breakdown. Record F1/F2/F3/F4/F5 individually and allow multiple classes per finding. Do not exclude F7: publish inconclusive count, reason, and potential effect on release claims. Runtime median/p95/max requires sample count and timeout censoring; small samples should show individual durations. Distinguish diagnosis runtime from fixture preparation. Resource comparison requires identical workload/environment/binary build mode and a declared baseline; otherwise unavailable.

Receipt checks: source roles/completeness/methods agree with actual acquisition; limitations survive positive verdicts; selected-only source planning; per-finding and aggregate verdicts; exit status; innocence iff all C/NA; numeric MTTI only then; monotonic offsets and sample windows ordered consistently; human output communicates the relevant bounds. JSON field presence alone is not evidence fidelity.

## Temporal oracle limits

No atomic snapshot is required. Timestamp independent transitions and actual collection order, preferably guest monotonic trace plus controller serial ordering. A fault after a source read does not retroactively invalidate a narrowly stated observation. Cross-source identity or applicability changes that make negative observations incomparable cannot support C merely because each read succeeded. Route presence does not prove gateway reachability. Filesystem usage read from a replaced pathname does not establish usage of the earlier mounted object. If trace tooling perturbs timing, disclose it and separate traced from untraced repeat trials. Logs may contain retained historical events; repaired current state does not erase a legitimate logged positive.

## Formalization consideration

Decision owner: oracle_design lane, subject to independent campaign acceptance. Proposition: for a named accusation within the source/time/identity envelope, C requires adequate applicable evidence and no positive; D requires an independently supported positive; NA requires adequately established empty applicable domain. Baseline source identity above; result identity is the SHA-256 of this document in the lane completion notice. A four-state truth-table model is useful for receipt algebra but does not prove collectors, kernel behavior, identity correspondence or temporal completeness. No new model implementation is warranted in this documentation-only change: explicit conditional oracle rules, independent raw observations, and bounded temporal counterexamples are sufficient for the present specification. Unknowns include kernel-log retention/freshness, mount identity across df, driver-specific logs, guest EDAC availability, and trace scheduling. A small future model of acquisition identity/epoch would be appropriate if a repair introduces durable cross-observation identity claims; this consideration admits no new runtime or diagnostic family.

## Custody and retirement

This matrix and final minimal result/receipt/source identities are durable campaign documentation. Hidden case manifest owner is oracle_design, custodian campaign root; purpose is independent final evaluation. Current consumer is final evaluator; next review event is candidate freeze or campaign closeout. Retire hidden-only custody after revealing and incorporating the exact manifest into reproducible final evidence, retaining the content/hash required to replay the final round. This lane creates no VM, build tree, credentials, or large artifacts. Execution artifacts require the campaign's existing storage admission and closeout records.

## Executed successor matrix

The specification above is preserved as written before execution. Actual environment used supported tmpfs root plus dedicated ext4/XFS mounts, not the proposed ext4 root. See RESULTS.md for all145 occurrence counts and failed attempts. The final representative case table below adds fault-family/accusation outcomes without treating repeats as independent cases. `E/QUALIFICATION-MATRIX.json` binds exact receipts and oracle input hashes. All rows use independently booted Linux6.5.0-44 with constructed GNU userspace; the second-kernel acquisition subset is separately reported. C/D/U/NA retain their original meanings. G30/G36 remain no-primary-receipt F7; H03 has an actual receipt but failed intended induction.

| Case | Family | Accusation | Final actual | Evidence kind | Classes |
|---|---|---|---|---|---|
| G01 | Healthy control | kernel-scream | CLEAR | real-guest-condition | No demonstrated failure |
| G01 | Healthy control | network-choke-point | CLEAR | real-guest-condition | No demonstrated failure |
| G01 | Healthy control | silent-killer | CLEAR | real-guest-condition | No demonstrated failure |
| G02 | Applicability control | network-choke-point | NOT_APPLICABLE | real-guest-condition | No demonstrated failure |
| G03 | Privilege boundary | kernel-scream | UNKNOWN | real-guest-condition | No demonstrated failure |
| G03 | Privilege boundary | memrot | UNKNOWN | real-guest-condition | No demonstrated failure |
| G03 | Privilege boundary | network-choke-point | UNKNOWN | real-guest-condition | No demonstrated failure |
| G03 | Privilege boundary | silent-killer | UNKNOWN | real-guest-condition | No demonstrated failure |
| G04 | Virtual hardware scope | memrot | UNKNOWN | real-guest-condition | No demonstrated failure |
| G05 | Missing route | network-choke-point | DEGRADED | real-guest-condition | No demonstrated failure |
| G06 | Rejected route | network-choke-point | DEGRADED | real-guest-condition | No demonstrated failure |
| G07 | Blackhole route | network-choke-point | DEGRADED | real-guest-condition | No demonstrated failure |
| G08 | Unused down interface | network-choke-point | CLEAR | real-guest-condition | No demonstrated failure |
| G09 | Carrier loss | network-choke-point | CLEAR | real-guest-condition | No demonstrated failure |
| G10 | Bad next hop | network-choke-point | CLEAR | real-guest-condition | No demonstrated failure |
| G11 | DNS unavailable | network-choke-point | CLEAR | real-guest-condition | No demonstrated failure |
| G12 | Packet filter | network-choke-point | CLEAR | real-guest-condition | No demonstrated failure |
| G13 | Sampled drops | network-choke-point | DEGRADED | real-guest-condition | No demonstrated failure |
| G14 | Historical counters | network-choke-point | CLEAR | real-guest-condition | No demonstrated failure |
| G15 | Repeated link transitions | network-choke-point | DEGRADED | real-guest-condition | No demonstrated failure |
| G16 | Conntrack pressure | network-choke-point | DEGRADED | real-guest-condition | No demonstrated failure |
| G17 | Conntrack absent | network-choke-point | CLEAR | real-guest-condition | No demonstrated failure |
| G18 | Critical block space | silent-killer | DEGRADED | real-guest-condition | No demonstrated failure |
| G19 | Inode exhaustion | silent-killer | DEGRADED | real-guest-condition | No demonstrated failure |
| G20 | Read-only filesystem | silent-killer | DEGRADED | real-guest-condition | No demonstrated failure |
| G21 | Intentional bind view | silent-killer | CLEAR | real-guest-condition | No demonstrated failure |
| G22 | Unsupported root | silent-killer | UNKNOWN | real-guest-condition | No demonstrated failure |
| G23 | Space across mounts | silent-killer | DEGRADED | real-guest-condition | No demonstrated failure |
| G24 | XFS shutdown | kernel-scream | DEGRADED | real-guest-condition | No demonstrated failure |
| G24 | XFS shutdown | silent-killer | DEGRADED | real-guest-condition | No demonstrated failure |
| G25 | Block I/O error | kernel-scream | DEGRADED | real-guest-condition | No demonstrated failure |
| G25 | Block I/O error | silent-killer | DEGRADED | real-guest-condition | No demonstrated failure |
| G26 | Evidence view denied | network-choke-point | UNKNOWN | permission-view | No demonstrated failure |
| G27 | Malformed command evidence | silent-killer | UNKNOWN | command-substitution | No demonstrated failure |
| G28 | Empty log acquisition | kernel-scream | UNKNOWN | command-substitution | No demonstrated failure |
| G29 | DNS plus ample filesystems | network-choke-point | CLEAR | real-guest-condition | No demonstrated failure |
| G29 | DNS plus ample filesystems | silent-killer | CLEAR | real-guest-condition | No demonstrated failure |
| G31 | Missing route plus log denied | network-choke-point | DEGRADED | real-guest-condition | No demonstrated failure |
| G32 | Counter topology transition | network-choke-point | UNKNOWN | real-guest-condition | No demonstrated failure |
| G33 | Fault immediately before | silent-killer | DEGRADED | real-guest-condition | No demonstrated failure |
| G34 | Fault removed immediately before | silent-killer | CLEAR | real-guest-condition | No demonstrated failure |
| G35 | Mount disappears during collection | silent-killer | UNKNOWN | intervention-assisted-real-transition | No demonstrated failure |
| G37 | Evidence command stalls | kernel-scream | UNKNOWN | command-substitution | No demonstrated failure |
| G38 | Failed application with healthy evidence | memrot | UNKNOWN | real-guest-condition | No demonstrated failure |
| G38 | Failed application with healthy evidence | network-choke-point | CLEAR | real-guest-condition | No demonstrated failure |
| G38 | Failed application with healthy evidence | silent-killer | CLEAR | real-guest-condition | No demonstrated failure |
| H01 | Evidence pair inadequate | network-choke-point | UNKNOWN | guest-evidence-view | No demonstrated failure |
| H02 | Shadowed filesystem | silent-killer | UNKNOWN | real-guest-condition | No demonstrated failure |
| H03 | Drop plus denied counter | network-choke-point | UNKNOWN | guest-evidence-view | F7 |
| H04 | Benign log semantics | memrot | DEGRADED | command-substitution | F1 |
| H05 | Interface generation transition | network-choke-point | CLEAR | real-guest-condition | F0,F2,F3 |
| H06 | Human receipt limitation | network-choke-point | DEGRADED | real-guest-condition | F3 |
| G30 | Full filesystem plus failed systemd service | S,K | No primary receipt | Missing service-manager environment | F7 |
| G36 | Exact route-read transition | N | No primary receipt | No independently verified barrier | F7 |

H01 narrows the original intended oracle: missing max was established, but numeric count was not readable in the private view. H04 remains an explicit command fixture with independent informational-message provenance. H05 real replacement and H06 default-human detail loss remain unresolved; no holdout-driven repair was performed. Original failed G13/G14 attempts are retained alongside TUN successors. The aggregate accusation table in RESULTS.md reports C/D/U/NA and overlapping failure categories separately.
