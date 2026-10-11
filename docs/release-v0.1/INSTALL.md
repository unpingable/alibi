# Install experimental Alibi 0.1.0

The supplied archive is for Linux x86_64 with the GNU runtime: glibc 2.39 or newer, libgcc_s.so.1, and the standard x86_64 loader. It is dynamically linked, not a static binary. The exact guest-qualified executable has SHA-256 `0760ea3529ac3c3d831038a71c46f82b6bd3d750fd4dbc52866399962e593b06`. SOURCE.json identifies whether your archive contains that executable or a CI rebuild of the corresponding source.

Alibi reads procfs/sysfs and invokes `df` and `dmesg`; `journalctl` is an optional log fallback. Supported command options and evidence limits are documented in the repository's docs/envelope.md. The glibc requirement is a binary-loader requirement, not qualification of every distribution with that runtime.

Download the archive and SHA256SUMS from the same release or CI artifact. From that directory:

```sh
sha256sum --check SHA256SUMS
tar -xzf alibi-0.1.0-x86_64-unknown-linux-gnu.tar.gz
cd alibi-0.1.0-x86_64-unknown-linux-gnu
./alibi --version
mkdir -p "$HOME/.local/bin"
install -m 0755 alibi "$HOME/.local/bin/alibi"
"$HOME/.local/bin/alibi" check
```

Checksums detect mismatched/corrupted downloads; they are not a signature. Use the repository's official release/artifact source. No installer daemon, background service or privileged installation is required.

On an older GNU Linux runtime, build on the target system with Rust/Cargo:

```sh
git clone --branch release/v0.1.0-rc1 https://github.com/unpingable/alibi.git
cd alibi
cargo build --release --locked
mkdir -p "$HOME/.local/bin"
install -m 0755 target/release/alibi "$HOME/.local/bin/alibi"
```

A rebuild is not byte-identical qualification of a new environment. There are no external Rust crates. macOS, Windows, ARM and musl/static builds are not qualified in this release.

Start unprivileged. If a required source is restricted, Alibi reports UNKNOWN. An administrator may choose `sudo /absolute/path/to/alibi check` when access to the kernel log is needed. Alibi does not remediate, modify network configuration or send active probes. Its command runner may terminate its own acquisition subprocesses at a deadline.

```sh
alibi check
alibi check network-choke-point
alibi check --json
alibi explain silent-killer
```

Exit status: 0 all selected accusations CLEAR/NOT_APPLICABLE; 1 at least one DEGRADED; 2 UNKNOWN without DEGRADED; 64 usage error. A nonzero diagnostic status is not necessarily an execution failure. CLEAR never means the whole host is healthy.

Receipts can contain hostnames, interface names, mountpoints and kernel-log excerpts. Review/redact them before sharing. To uninstall, remove the exact binary you installed; there is no service or persistent telemetry to remove.
