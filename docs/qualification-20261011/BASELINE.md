# Baseline — 2026-10-11

Source: `6f8ad5fba4ed6f3fb9f32c9930377a0a48e06931` (clean `/data/git/alibi`). Dedicated branch `campaign/host-qualification-20261011`, worktree `/data/git/alibi-qualification-20261011`. Original worktree is untouched. Evidence root `/data/git/alibi-qualification-evidence-20261011`; its CHECKPOINT.json is the recovery pointer, not an execution-status tracker.

Host crow; toolchain rustc 1.98.0 (88d9e12ae 2026-08-18), cargo 1.98.0. Baseline `cargo fmt --check`, `cargo test`, `cargo clippy --all-targets -- -D warnings`, `cargo build --release` all exit 0. Tests: 102 unit + 2 receipt integration. Raw logs and per-command times in evidence-root baseline.json. These tests establish baseline regression status, not Linux fault qualification.

Exact baseline executable SHA256: `6e05e61cde2c092148bf2788f3dd9560a5d5dc05c83f7685fc5020d1bf1bc859`, retained as evidence-root/alibi-baseline. The ordinary release build is dynamically linked to glibc/libgcc despite README's static-binary claim. Capture the executable and libraries for guest reproducibility; do not silently substitute another build.

## Live interface and contract

The CLI implements `check [accusations] [--json|--verbose]`, `explain <accusation>`, and `--version`. Receipt V2 is live in `src/receipt`, CLI, and golden tests. Four actual accusations:

| Accusation | Required negative evidence | Positive indications / limits |
|---|---|---|
| kernel-scream | kernel log | Curated kernel/hardware patterns; absence of recognized patterns is not exhaustive hardware diagnosis. |
| silent-killer | mounts, filesystem usage, kernel log | Material filesystem byte/inode scarcity, applicable read-only filesystems, curated filesystem events. Unsupported root blocks CLEAR. Read-only bind siblings are specially excluded. |
| memrot | EDAC and kernel log | EDAC counts and curated memory-error patterns; VM absence of EDAC limits exoneration. |
| network-choke-point | interface samples, route tables, kernel log; conntrack when present | Error/drop deltas, absent default route, recognized log events, conntrack pressure. Current carrier state, DNS, remote service availability and workload function are not separately accused. |

Source and applicability details must be reviewed alongside `docs/design.md`, `docs/envelope.md`, `src/explain.rs`, and independent TEST-MATRIX.md. No service-health, resource-starvation or storage-pathology accusation exists. Prior proposals explicitly deferred the latter two. We will not create them through qualification.

## Prior evidence and assumptions

`docs/next-stage-campaign.md` and `docs/next-stage-qualification.md` record historical evidence-engine and V2 work. Existing examples are explicitly fictional. Prior claims and tests are hypotheses, not independent evidence of this campaign's faults. No prior VM evidence has yet been accepted into this run.

Potential assumptions needing empirical challenge: well-formed external command output; route semantic interpretation; filesystem identity across sequential observations; retained log coverage; command timeout and output bounds. A named accusation may legitimately CLEAR while an unrelated service fails. UNKNOWN can be correct and reduces neither soundness nor completeness by itself; useful exoneration will use an independently established sufficient-evidence denominator.

## Environment and isolation

KVM device absent. QEMU 8.2.2 includes TCG. Host `/boot/vmlinuz-6.5.0-44-generic` is not readable as campaign user, including outside sandbox. Matching modules and static BusyBox are available. `/tank/nfs` is read-only. Guest destination decision pending; no guest fault run yet. User systemd is reachable outside sandbox (manager reports degraded); durable units will require exact identity, file logs, terminal record and supervisor-independent lifetime verification before a prolonged wait.

Read-only ordinary CLI controls run against the development environment are separately labeled operator controls; they are not injected-fault guest experiments and cannot establish hardware or network-namespace correspondence. No development-host fault injection is authorized.

## Custody and formalization consideration

Root campaign agent owns aggregate storage and minimal evidence. Initial available bytes: `/` 385745440768; `/data` 630935212032; hard reserve 64424509440 each. Initial small source/build allowance 512 MiB. Other shared tenants exist; their future allocations are unknown. Large guest allocation is not admitted yet. Baseline target is reproducible; retain exact executable and necessary replay identity, retire target after final checks and consumer reconciliation. Source/reproducer/evidence review due 2026-10-18 with operator as next custodian.

Proposition under test: each emitted CLEAR has sufficient applicable collected evidence for its named accusation. Scope is this exact baseline and later separately identified candidates, not all Linux implementations. Counterexamples and temporal assumptions are recorded per experiment. Root is decision owner; independent oracle and acceptance agents challenge the claim. Empirical kernel observations and minimized regressions are required because a facts-level model cannot establish collector correspondence to a changing Linux host. A small verdict-algebra model may be useful later, but is not a substitute for this campaign and is not currently admitted as product work.
