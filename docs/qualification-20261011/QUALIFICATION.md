# Release disposition: HELD_DEFECT

The v0.1 candidate is **not release-qualified**. The final frozen holdouts demonstrate a real-guest false CLEAR, a fixture-scoped unsupported positive attribution, and a default-human receipt limitation. No product tuning followed the holdouts; no push, merge, tag or publication occurred.

This disposition covers baseline `6f8ad5fba4ed6f3fb9f32c9930377a0a48e06931` and repaired, frozen product `74e4143844c0535d3837dc67df625f8251197dad`, executable SHA-256 `eb1ef3e9ac00462707e8e416d1e9496c916d074b7af06ec03e1673387a64f1fe`. `E/FINAL-FREEZE.json` sealed source, binary and passing checks before root saw the holdouts; the implementation lane never received their contents. E is `/data/git/alibi-qualification-evidence-20261011`. Source repairs and qualification records are committed only on the dedicated campaign branch.

## Empirical boundary

The laboratory ran 145 independent-kernel QEMU guest occurrences, including controls, repeats, failed substrate attempts and six final holdouts. This is not 145 independent fault scenarios. Of 38 disclosed specifications, 36 obtained representative adjudicated outcomes after independently reviewed fixture successors; G30 (systemd compound condition) and G36 (precise route-read race) remain F7. Holdouts add five adjudicable scenarios and one failed-induction F7. Evidence substitutions remain labeled separately from genuine kernel faults. See RESULTS.md for exact denominators and TEST-MATRIX.md for the predeclared oracle.

The disclosed representatives yielded useful exoneration **16/16**, with **0 false CLEAR /45 evaluated applicable findings** and **0/16 evaluated emitted CLEARs**. The sealed holdouts then found H05 false CLEAR and H04 unsupported attribution. Combined evaluated representatives therefore have **1 false CLEAR /50 applicable findings and1/17 emitted CLEARs**, alongside16/16 useful exonerations. Selected healthy eligibility in a small campaign is not population accuracy, and excludes no-evidence UNKNOWN cases for a stated reason. Failed trials remain recorded; holdout failures cannot be hidden by the favorable disclosed set.

Full matrix environment: x86_64 QEMU TCG, independently booted Ubuntu-distributed 6.5.0-44-generic kernel, constructed GNU/Dash/BusyBox userspace, supported tmpfs root with disposable ext4/XFS mounts. The acquisition subset was also repeated on 6.8.0-90-generic with the same userspace. This is not two full distributions, production workload qualification, physical-hardware qualification or systemd service qualification. Ordinary executable dynamically links the recorded GNU runtime; command-lifecycle preflight is specifically Linux x86_64 GNU ABI. No universal Linux support claim follows.

## Release gates

| Gate | Assessment |
|---|---|
| Zero unresolved demonstrated false CLEAR | **FAIL:** H05 joins different interface generations under the same name. |
| Zero unresolved unsupported positive attribution | **FAIL:** H04 treats benign EDAC registration as a memory-error report. |
| Faithful evidence limitations in receipts | **FAIL:** H05 overstates source completeness; H06 default human receipt omits the required-source envelope detail, though it warns of skipped checks and JSON is correct. |
| No critical reliability defect | Previously demonstrated inherited-pipe deadline failure repaired and exercised on both kernels; no unresolved critical execution failure observed in final candidate. Not a hard real-time guarantee. |
| Read-only / authority | No diagnostic host mutation or unauthorized operation demonstrated. Guest fixture mutations are separate actors. No exhaustive syscall/authority proof is claimed. |
| Reproducible evidence | Exact retained kernels, archives, binaries, plans, independent observations and receipt hashes; initial failed inductions/boot remain F7. |
| Explicit supported boundaries/exclusions | Recorded below and in HARNESS.md; no general release qualification. |
| Regression checks | 118 unit +2 receipt integration tests, fmt, clippy with warnings denied and release build pass at frozen source. |
| Final holdouts | Six executed once after freeze; failures retained and independently reviewed. |

## Which conclusions can an operator trust tomorrow?

Treat a verdict as a claim about its named indication set and collected evidence, never host health. The repaired candidate has concrete empirical support for these bounded behaviors:

- Missing, unreachable, blackhole and zero-address narrow routes do not masquerade as usable defaults. Real sampled TUN drops, conntrack pressure and repeated recognized link-down events produce supported network indications. **Network CLEAR is not trustworthy across same-name interface replacement**; even under stable identities it says nothing about DNS, gateway reachability, filtering or application health.
- Actual low blocks, inode exhaustion, read-only ext4, XFS shutdown and observed device-mapper I/O errors produce supported storage/kernel indications. Unsupported overlay root and mismatched filesystem identities become UNKNOWN; healthy spaced mounts and writable backing views can still clear. CLEAR assumes applicable supported mounts and adequate command/log evidence; matching textual names do not prove filesystem-generation continuity.
- Missing, denied, empty, invalidly encoded, malformed or late kernel-log acquisition cannot clear the tested accusations. A kernel-log CLEAR means no recognized indication in the acquired retained log under its documented evidence envelope. It cannot establish that earlier records were never lost or that a framed source is authentic/fresh.
- Absent EDAC does not clear memory integrity. **Do not trust every memrot DEGRADED as an actual memory error:** benign EDAC registration currently produces an unsupported attribution. No hardware-memory CLEAR was qualified here.

Prefer JSON receipts when inspecting evidence limitations: default human output currently loses the required-source detail in the demonstrated positive-plus-denied-log case. UNKNOWN is often the justified result and is not a regression without an independently sufficient-evidence denominator.

What can still fool it: interface name reuse across generations is demonstrated; benign EDAC vocabulary is demonstrated; lost/stale but well-framed logs, same-text filesystem replacement, concurrent external child reaping, unsupported ABIs and untested transition histories remain limits. Resource-starvation and slow/stalled storage are deferred capabilities, not silently covered by existing accusation names. Most critical cases did not reach the originally targeted three repeats; the subset of actual repeats is reported, not inflated.

## Decision and next action

Root decision: retain **HELD_DEFECT**. Independent oracle reports and ACCEPTANCE-FINAL.md corroborate the held disposition. A later bounded repair round should address H05, H04 and H06, retain these failing receipts, and establish new separated acceptance cases. The current final holdouts cannot be recycled as an unseen qualification claim. No release action is authorized or performed.

Formalization consideration: the durable proposition is sufficient applicable evidence before exoneration. Finite oracle rules, source identities and independent timing/identity traces expose counterexamples; they do not establish all interleavings or authenticity. A bounded interface-generation model could help a successor repair, but this campaign adds no new architecture or model-based guarantee. Root owns this decision; exact oracle/result identities are in RESULTS.md and the final completion notice.
