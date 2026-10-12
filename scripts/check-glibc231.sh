#!/bin/sh
# Run only inside the pinned Debian 11 test runtime. No diagnostic qualification claim.
set -eu
archive=${1:?usage: check-glibc231.sh ARCHIVE NEW_OUTPUT_DIRECTORY}
output=${2:?usage: check-glibc231.sh ARCHIVE NEW_OUTPUT_DIRECTORY}
[ "$(getconf GNU_LIBC_VERSION)" = 'glibc 2.31' ] || { echo 'requires actual glibc 2.31 runtime' >&2; exit 1; }
case "$archive:$output" in /*:/*) ;; *) echo 'paths must be absolute' >&2; exit 64;; esac
mkdir "$output"
{
  cat /etc/os-release
  uname -srmo
  getconf GNU_LIBC_VERSION
  ldd --version
  sha256sum /lib/x86_64-linux-gnu/libc-2.31.so /lib/x86_64-linux-gnu/ld-2.31.so
} > "$output/environment.txt" 2>&1
cd "$(dirname "$archive")"
sha256sum --check SHA256SUMS > "$output/checksum.txt"
tar -xzf "$archive" -C "$output"
cd "$output"
binary=$(find . -mindepth 2 -maxdepth 2 -type f -name alibi)
[ -n "$binary" ]
sha256sum "$binary" > binary.sha256
/lib64/ld-linux-x86-64.so.2 --list "$binary" > loader.txt
"$binary" --version > version.txt 2> version.stderr
"$binary" --help > help.txt 2> help.stderr
set +e
timeout -k 2 30 "$binary" check > human.txt 2> human.stderr
human_rc=$?
timeout -k 2 30 "$binary" check --json > receipt.json 2> json.stderr
json_rc=$?
set -e
printf '{"human_exit":%s,"json_exit":%s}\n' "$human_rc" "$json_rc" > exits.json
[ "$human_rc" -le 2 ] && [ "$json_rc" -le 2 ]
printf '{"completed":true,"runtime":"glibc 2.31"}\n' > COMPLETE.json
