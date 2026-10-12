# Older GNU Linux binary

The compatibility archive is `alibi-0.1.1-rc.1-x86_64-unknown-linux-gnu-glibc2.31.tar.gz`.
It targets x86-64 Linux with glibc 2.31 or newer, libgcc_s and the standard GNU
loader. It is dynamically linked. Ubuntu 22.04 and Debian 12 meet that runtime
baseline; this is not universal distribution or kernel qualification.

Download this archive and its matching SHA256SUMS from the compatibility CI
artifact. The original v0.1.0 release archive remains unchanged and requires
glibc 2.39+. Until separately published, the compatibility build is a CI artifact.

```sh
sha256sum --check SHA256SUMS
tar -xzf alibi-0.1.1-rc.1-x86_64-unknown-linux-gnu-glibc2.31.tar.gz
cd alibi-0.1.1-rc.1-x86_64-unknown-linux-gnu-glibc2.31
./alibi --version
mkdir -p "$HOME/.local/bin"
install -m 0755 alibi "$HOME/.local/bin/alibi"
"$HOME/.local/bin/alibi" check
```

Start unprivileged. Missing evidence may yield UNKNOWN; access to every source
is not guaranteed by loader compatibility. CLEAR concerns a named accusation,
never whole-host health. No diagnostic-time improvement is established.

## Reproduction and observed limits

Run `scripts/build-compatible-linux.sh /absolute/output/directory` with Docker,
Python 3.11+ and binutils. The script pins Rust 1.94.0 and Debian 11 image content,
uses no build network, runs regression tests and verifies symbol requirements
before packaging. If the image is absent Docker first downloads that pinned image.
Source builds also work with `cargo build --release --locked` on the target.

The first build used unchanged product source from v0.1.0. Its highest required
glibc symbol was 2.30; 2.31 is the conservative build-userspace baseline.
The exact binary ran on an Ubuntu 22.04.5 Linode VM (Linux 5.15.0-194, glibc 2.35),
including human and JSON interrogation. Debian 12 userspace checks used a
restricted container: executable/acquisition compatibility, not a separate-kernel
qualification. Full historical qualification remains QUALIFIED_WITH_LIMITATIONS;
this new binary is not the original guest-qualified executable.

No musl ABI support, FreeBSD port or new diagnostic family is introduced.

## Evidence and formalization consideration

Product identity: v0.1.0 merge `0f76647c73a85bbdf202fa68a2834f7d71d2f9b4`;
product paths remain identical to qualified `7e04d3c`. First older-GNU binary
SHA-256: `d41ff1f06c413a42e1beae359ffdd46686f073e9a9d2dbc8accdb75bc8e895b3`.
Both native and build-container suites passed 127 unit and 3 receipt tests;
native formatting, Clippy and release build passed. Packaging rejects the
original glibc-2.39 binary when labeled baseline 2.31.

Proposition: lower the GNU runtime dependency without changing product source
or evidence semantics. Owner: release implementer. Git comparison, ELF version
inspection, pinned inputs and runtime checks suffice for this bounded packaging
change; a formal model would not establish loader behavior on untested systems.
No universal compatibility or transfer of exact-binary qualification is inferred.
Local custody: `alibi-portability-20261012`; raw host receipts are not public.

## Native glibc 2.31 runtime observation

Both retained compatibility and guidance CI archives were extracted and invoked
in the pinned Debian 11 runtime without newer libraries mounted. Version/help,
human and JSON commands completed; getconf reported glibc 2.31, and loader --list
resolved dependencies inside that Debian runtime. Independent review accepted
this bounded observation. The read-only container correctly exposed evidence
restrictions; it shares the host kernel and does not qualify Debian kernels.

The first harness attempt used an unsupported loader --version option and
stopped before invocation. That record is preserved; the corrected check uses
ldd for version and the loader's --list interface. Current CI repeats the check
on the exact new archive with scripts/check-glibc231.sh and uploads its runtime
records separately. A future publication must inspect that candidate's exact
records; the older binary test is not a blanket guarantee for successors.
