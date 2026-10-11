# Independent implementation review

Source revision: `6f8ad5fba4ed6f3fb9f32c9930377a0a48e06931`.
Reviewer/decision owner: campaign lane `implementation_review`.
Status: source review followed by authorized bounded local reproductions. **No guest experiment was executed by this lane.** The ranked section preserves the original hypotheses; executed outcomes and limits are recorded below. No holdout or private oracle files were inspected. No product source changes are included.

## Ranked qualification targets

### IR-01: IPv4 zero network mistaken for default — candidate F0/F2

`src/collect/net.rs::parse_v4_default` checks the destination string for eight zeroes but never checks field 7, the netmask. A legal route to `0.0.0.0/8` (or `/32`) is not a default route yet takes the same `found=true` branch. This violates the explicitly named default-route evidence requirement, independently of any opinion about remote reachability.

Minimal parser input:

```text
Iface Destination Gateway Flags RefCnt Use Metric Mask MTU Window IRTT
eth0 00000000 00000000 0001 0 0 0 000000FF 0 0 0
```

Predicted current parser result: `Ok(true)`; oracle result: no `/0` route. Disposable guest experiment: remove true defaults, install `ip route add 0.0.0.0/8 dev eth0`, verify `ip -4 route show table main`, `ip -6 route show table main`, and raw proc tables; acquire clean applicable log/counters, invoke supported CLI. Expected allowable verdict is DEGRADED for missing default or UNKNOWN for inadequate required evidence, never CLEAR based on this route. Add a genuine default control and a compound narrow-zero-route plus unrelated healthy interface case.

### IR-02: command timeout does not bound pipe-reader joins — candidate F4

`src/collect/mod.rs::run_command` joins stdout/stderr reader threads after direct-child exit and after killing the direct child. A descendant retaining either pipe keeps `read_to_end` waiting after the nominal timeout. A stalled direct child in an uninterruptible kernel wait can also delay `child.wait`; that is a separate kernel limitation. The current design claims every command has a hard timeout.

Minimal bounded local substitution fixture: an exit-zero command spawns a child that sleeps for 6 seconds retaining stdout, while the parent emits a benign line and exits; call with a 100 ms timeout. Record monotonic duration and an independent outer bound, with every fixture child terminating itself. Predicted runtime exceeds 100 ms despite immediate parent exit. Do not leave a perpetual writer or orphan. Guest stalled-storage qualification remains distinct and cannot be replaced by this fixture.

### IR-03: nonempty text is accepted as complete kernel-log evidence — candidate F0/F2/F3

`src/collect/kernellog.rs::usable_lines` accepts every nonempty dmesg exit-zero output. Only blank output and journal's exact all-lines no-entries sentinel are rejected. Invalid UTF-8 is already converted lossily by `run_command`. A command substitution returning `not a kernel log` therefore becomes status ok/completeness complete; the kernel-scream pattern evaluator finds no match and predicts CLEAR. Empty output has a test, nonempty malformed output does not.

Run the supported CLI with a campaign-local PATH command fixture, explicitly label it a command-substitution boundary case, capture exact fixture and receipt. This does not establish real Linux log truncation behavior. Real ring wrap is separately disclosed by explain output and should not be called an undisclosed since-boot guarantee. There is no robust universal kernel message grammar; a bounded repair must state what it validates without pretending it can authenticate arbitrary text.

### IR-04: contradictory capacity and unrelated df rows accepted — candidate F2/F3, possible F0

Both df parsers check only prefix `Filesystem`, >=6 whitespace fields, and numeric total/free. They accept free > total, ignore Used/percentage consistency, ignore extra rows, and never verify returned device or mountpoint. A fixture with `Filesystem...` then `/dev/other 100 0 101 0% /other` predicts complete evidence attached to the requested filesystem and no low-capacity indication. Zero totals are silently skipped by the evaluator; some genuine filesystems legitimately have no inode total, so do not forbid zero without environment-specific evidence.

Separate malformed fixture checks from actual temporal experiment: capture mount table, overmount the requested path with another filesystem before df, and independently identify both superblocks and capacities. The collector can associate capacity from the replacement with the earlier mount identity. Requires a deterministic guest barrier and observation-order record; ordinary receipt timestamps alone do not establish the substitution occurred.

### IR-05: mount options treated as writable by default — candidate F2, possible F0

`parse_mounts` checks field count but treats absence of exact `ro` as writable. `/dev/vda / ext4 nonsense 0 0` is accepted as a material writable root. A replacement mount-table fixture therefore can turn malformed required evidence into complete writable evidence. Contradictory ro/rw tokens are not rejected either. This is a malformed-source boundary case, not evidence that genuine procfs emits such rows.

### IR-06: malformed non-ASCII mount escape can panic — candidate F4

`mounts::unescape` indexes UTF-8 `&s[i+1..i+4]` after seeing a backslash. For the string consisting of backslash followed by two `é` characters, the end offset falls inside the second character. A direct parser reproducer can call `parse_mounts` on a six-field row using this mountpoint. Expected behavior is rejection or preserved literal bytes, not panic. Genuine procfs escapes literal backslashes octally, so this is explicitly a malformed substitution-input case.

### IR-07: finite command output is not bounded in memory — candidate F4

The two `read_to_end` buffers have no cap; all per-filesystem command outputs are retained before assembly. Filesystem/proc reads also read entire files. Any test must use a small finite fixture with independent resource limits; do not generate unbounded output on a shared host. Source review proves absence of a bound, not an observed resource failure. Decide an admitted maximum and measure only after storage owner admission.

### IR-08: aggregate EDAC sum overflows — candidate F4 debug / F0 release

`memrot` sums u64 UE and CE counts. Two individually accepted values `u64::MAX` and `1` cause checked debug arithmetic failure or release wrapping to zero. With both sums zero and a clean complete kernel log, the evaluator can predict CLEAR despite positive controller counts. This is a parser-legal synthetic counter case, not a realistic claim about current Linux EDAC widths. Minimal regression should assert any positive retained counter prevents CLEAR, independently of aggregate presentation. No runtime execution by this lane.

### IR-09: interface names are substring-matched in flap association — candidate F1

The network evaluator uses `line.contains(iface.name)` before matching link-down phrases. Three records concerning `eth10` also match `eth1`; unrelated message substrings can match as well. If both interfaces remain present a degraded overall result may still be justified, but the named per-interface indication is wrong. More discriminating oracle: `eth1` is present, `eth10` is absent, and retained log has three old eth10 events. Whether historical removed-interface events should degrade is not fully specified; do not label a definite F1 without resolving that scope.

### IR-10: broad pattern matches can attribute benign kernel text — candidate F1

`JBD2:` always means journal error to the table; `EDAC` means memory error except three banner exclusions; `Oops` has no token/context validation. Obtain genuine benign messages from an isolated guest before declaring unsupported attribution. Handwritten strings alone demonstrate matcher behavior, not that a supported kernel emits those strings. Preserve actual kernel version and configuration with any example.

## Deliberately rejected or bounded hypotheses

- A down/unused interface is **not** an accusation: design §6/§7 and CLI explain explicitly record operstate only as context. Thus steady down state alone is F6, not a missing indication repair.
- Remote DNS/server reachability, service function, resource pressure and generic slow storage are not newly admitted accusations. Their use as compound faults tests bounded exoneration, not new feature coverage.
- Ring-buffer wrap is explicitly disclosed. It limits retained-log evidence and is not by itself a demonstrated defect.
- Read-only mounts sharing the same device with a read-write mount are intentionally excluded by current design. Device-string equality does not prove operator intent, but changing that policy requires a minimized actual counterexample and oracle agreement.
- Central `enforce_required_sources` prevents CLEAR/NOT_APPLICABLE when a required source is partial or failed. Main risks are collectors incorrectly calling inadequate data complete, scope exceptions, or positive indications lacking support.
- Receipt aggregation ranks DEGRADED above UNKNOWN above CLEAR, and suppresses aggregate innocence/MTTI when any finding is unresolved or degraded. No concrete aggregation inconsistency found by this review. This is not receipt qualification.
- Collection is sequential and has a bounded network sample but no atomic host snapshot. Do not demand an impossible instantaneous snapshot; qualify each actual evidence combination and identify stale identity joins.

## Formalization consideration

Propositions: (a) no complete-looking malformed required evidence yields CLEAR; (b) no non-default route satisfies a default-route requirement; (c) command collection terminates within its claimed bound; (d) positive EDAC evidence cannot disappear through aggregation; (e) a receipt reflects actual acquisition identity and limitation.

Scope: the exact baseline source identities below, standard-library collectors, pure evaluators and Receipt V2. Current source review identifies counterexample candidates rather than proving propositions. A small bounded model could establish verdict monotonicity over required-source completeness and positive evidence, but would assume collector truthfulness and therefore miss the highest-priority findings here. Decision owner `implementation_review` recommends practical minimized parser/command tests plus independent guest oracles for this campaign, with prose documenting temporal and kernel assumptions. No new formal model was built. Future bounded transition modeling is warranted only if source generation/identity checks become a durable mechanism; not admitted here.

## Custody and next action

This document is minimal review evidence owned by the campaign root; retain with campaign source/results. The follow-up helper executable was created and retired as recorded below. No VM images, credentials, caches, or large retained evidence were created by this lane. Root should reproduce IR-01 and IR-02 first, adjudicate IR-03's log contract, and select bounded repairs only after preserving failing evidence. Independent acceptance must inspect the cited results; this lane's completion notice is not acceptance.

## Exact inspected source identities

- `src/collect/mod.rs` SHA-256 `2a8a77fb6758fe2308bdc88b7b32c5d2f2f3f372b092116b52e21f2dab29d4e4`
- `src/collect/net.rs` SHA-256 `8d3117fd1896cf3a8b62835003f90a9f1e1dc0774a5a0b86aa558a3e37a3431f`
- `src/collect/kernellog.rs` SHA-256 `807f27828c522bfa3ccadb283e45412c38be04a2bb8236efb46f9f6ddb2a38bd`
- `src/collect/fsusage.rs` SHA-256 `3b393f9f1f32faec89025cc96dc88ec8cd8e1594bb2562fc1dfaca776576fabf`
- `src/collect/mounts.rs` SHA-256 `8476baa2675b4ec6a62c7b77abb835e527ac03e044d71954a9cf71392bab542f`
- `src/eval/network_choke_point.rs` SHA-256 `4799c2a1d82e8a3be85b5bdfa46d585f6d9097e219afcc69ef46fb5cc99b86ab`
- `src/eval/memrot.rs` SHA-256 `b3a697d51e7c38343c5a720223e91c0e66f249baff458f4fdc1aab8f978bb112`
- `src/eval/silent_killer.rs` SHA-256 `4b1bc1f9e119f029062d1cc8a117d0d893d4ad74dfd5e6770cd6807c7221925e`
- `src/eval/patterns.rs` SHA-256 `c8784df12af8e30bbdc2ef5e872f4d4d5962c74992a6ccae409087204f376817`
- `src/eval/mod.rs` SHA-256 `8965b3f1ddc9fd3679f2900e8b27e89f1a48f025f9c4aed07ea841e41df1be0c`
- `src/receipt/mod.rs` SHA-256 `6cc36fe890f7ed0d908919a97ae8c50e2e87c85199db23b31841873ee9b21289`
- `docs/design.md` SHA-256 `2f9312c808c4be6f1712328c2cb465b0d126b32fb994a8a1ca49c06c4589a4ad`
- `src/explain.rs` SHA-256 `7faebb5cd06c73fd21402f85c47e7a31f558f3c5c74ffef9afa2e4d069bf7bfd`

## Executed local follow-up (2026-10-11T16:48:24Z)

Root authorized bounded local reproduction after baseline build completion. Used the existing release library and sealed baseline CLI; did not rebuild Alibi. The original source-review notice remains immutable and is superseded by a new completion notice. These are parser/substitution experiments, **not real-host fault qualification**.

| Case | Actual baseline outcome | Independent expected property | Classification |
|---|---|---|---|
| IR-01 zero `/8` and `/32` | Both `Ok(true)`; genuine `/0` control also true | Nonzero netmask is not a default | Demonstrated F2 parser defect; CLI F0 requires guest reproduction |
| IR-02 inherited stdout pipe | 100 ms deadline; returned success after 1001 ms | Deadline should bound completion under documented hard-timeout claim | Demonstrated local F4 |
| IR-03 malformed log CLI | `not a kernel log` yielded CLEAR, complete required source, innocence true, exit 0 | Fixture supplies no kernel evidence; may not exonerate | Demonstrated substitution F0/F2/F3, subject to log-evidence contract acceptance |
| Log healthy control | Timestamped benign line yielded CLEAR, exit 0 | Benign admitted log can exonerate bounded matcher accusation | Passing substitution control |
| Log empty control | UNKNOWN, incomplete source, no innocence, exit 2 | No required evidence cannot exonerate | Passing substitution control |
| IR-04 capacity contradiction | `Ok((100, 101))` | Available greater than total is inconsistent capacity evidence | Demonstrated local F2 |
| IR-05 invalid mount options | Accepted writable material ext4 root | Unrecognized access-mode evidence is not observed rw state | Demonstrated local F2 |
| IR-06 invalid non-ASCII escape | Panic caught by helper | Malformed input should reject without panic | Demonstrated local F4 substitution boundary |

Every process exit, stdout, stderr, elapsed time and outer-timeout result is retained in `/data/git/alibi-qualification-evidence-20261011/local-reproducers-baseline/RECEIPT.json` and adjacent files. All outer timeouts remained false. IR-02's fixture child terminated naturally after one second; no indefinite process was launched. These outcomes do not establish stalled-block-device behavior, real malformed procfs, or real dmesg truncation. IR-08/09/10 remain unexecuted hypotheses.

Replay: `python3 campaign/local-reproducers.py BASELINE_BINARY BASELINE_RLIB NEW_OUTPUT_DIRECTORY`. The runner invokes rustc only for a small helper linked to the already-built library, runs each helper under a five-second process-group bound, uses campaign-local PATH commands only for three kernel-scream CLI invocations, records the helper hash, and deletes that reproducible helper plus transient command scripts. Exact dmesg fixture contents remain as minimal replay inputs.

Storage follow-up: helper logical size 4,697,184 bytes created then reclaimed. Retained result evidence 12,244 logical bytes plus the two tracked replay source files and this report. Root owns aggregate filesystem measurements and final campaign custody. The baseline CLI/library remain root-owned evidence; this lane did not alter them.

Formalization update: actual outcomes now supply concrete counterexamples for the bounded propositions (b) and (c), and malformed-source examples for (a)/(e). They do not establish all behavior over real Linux hosts. Practical minimized tests remain sufficient for these counterexamples; no proof claim or source repair is made.

## Bounded route-mask repair and candidate freeze

Only IR-01 was repaired under explicit campaign-root authorization. IPv4 route parsing now requires an eight-hex-digit mask, and recognizes zero destination as default only when the mask is also zero. Invalid mask width/hex causes malformed evidence even if an earlier row supplied a default. Existing rejection and no-device semantics are preserved. Added two regressions distinguish genuine `/0` from legal zero-address `/8` and `/32`, including narrow routes coexisting with a genuine default, and malformed rows after a valid default.

Before rebuilding, the original release library was copied to `/data/git/alibi-qualification-evidence-20261011/alibi-baseline.rlib`; its hash matches the original local-reproducer receipt. The root-retained baseline executable was unchanged. `cargo fmt --check`, `cargo test` (104 unit + 2 integration), `cargo clippy --all-targets -- -D warnings`, and `cargo build --release` all exited zero. Logs and exact timings are in `route-mask-repair-checks/RECEIPT.json` under the evidence root.

Replayed the same local reproducer against the new release into `local-reproducers-route-mask-repair`: `/8` and `/32` now return `Ok(false)`; `/0` remains `Ok(true)`. The other demonstrated defects remain unchanged and unresolved. No real guest affected experiment or holdout was run by this lane, so this is a parser repair with local regression evidence, **not host qualification**.

Formalization decision (owner implementation_review; root authorizes scope): proposition is that the parser never classifies a nonzero-netmask IPv4 route as default and never silently accepts a mask with invalid width/hex. Source/result identities are recorded in the successor lane notice and replay receipt. Simple conjunction and bounded boundary/regression examples suffice here; a state model would add no coverage of kernel table correspondence. This does not prove all route fields, netmask contiguity, policy routing, temporal consistency, or guest behavior. No expanded compatibility or receipt contract claim is made.

Repair replay retained 12,270 logical bytes; its 4,697,464-byte helper was retired immediately. Baseline library is unique replay custody until root's campaign closeout review, owned by root. No remaining helper executable or fixture child is retained.
