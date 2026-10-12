# Install the selected Alibi build

The published **v0.1.0** archive requires x86-64 Linux with **glibc 2.39+**,
libgcc_s and the GNU loader. Its exact binary and historical qualification are
unchanged. Download it from the official GitHub v0.1.0 release.

The unpublished **0.1.1-rc.1** candidate has a separately named
`alibi-0.1.1-rc.1-x86_64-unknown-linux-gnu-glibc2.31.tar.gz` archive for
**glibc 2.31+**, libgcc_s and the GNU loader. Ubuntu 22.04 and Debian 12 meet
that runtime baseline. Use the `glibc2.31` CI artifact, not the ordinary
newer-GNU build. CI downloads may require a GitHub login. No static/musl,
ARM, BSD or universal distribution compatibility is claimed.

Download the archive and SHA256SUMS from the same release or CI artifact,
into an otherwise empty directory. For the candidate:

```sh
sha256sum --check SHA256SUMS
tar -xzf alibi-0.1.1-rc.1-x86_64-unknown-linux-gnu-glibc2.31.tar.gz
cd alibi-0.1.1-rc.1-x86_64-unknown-linux-gnu-glibc2.31
./alibi --version
mkdir -p "$HOME/.local/bin"
install -m 0755 alibi "$HOME/.local/bin/alibi"
"$HOME/.local/bin/alibi" check
```

For v0.1.0, substitute its archive/directory name. Review SOURCE.json for
source revision, binary identity and historical source correspondence. Checksums
check byte correspondence, not authorship signatures. Start unprivileged;
permission failures should remain UNKNOWN, not trigger an automatic root rerun.

If loading fails with GLIBC_2.xx not found, check you selected the compatibility
artifact and compare `getconf GNU_LIBC_VERSION`. If the GNU loader or libgcc_s
is absent, use a supported GNU environment or build from source; do not copy
random shared libraries into a running server. Missing df/dmesg/journal access
is an evidence limitation distinct from failure to load the executable.

Build the published source with `git clone --branch v0.1.0
https://github.com/unpingable/alibi.git`, then `cargo build --release --locked`.
For a development checkout, record `git rev-parse HEAD` and the resulting binary
hash. A rebuild does not qualify an untested environment. There are no external
Rust crates. Reproduce the older-GNU candidate with
`scripts/build-compatible-linux.sh /absolute/output/directory` (Docker, Python
3.11+ and binutils required). That script pins the build image content.

The diagnostic uses procfs/sysfs, df and dmesg, with journalctl fallback.
Run `alibi guide` in newer builds or `alibi explain <accusation>` to understand
scope. Exit 1 means DEGRADED, 2 means UNKNOWN without DEGRADED; neither alone
means the executable failed. No service, background collector or host mutation
is installed. Remove the exact installed file to uninstall. Receipts may include
host identifiers and logs; review them before sharing.
