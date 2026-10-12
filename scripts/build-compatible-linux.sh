#!/bin/sh
# GNU ABI compatibility build. Docker supplies build userspace, not host qualification.
set -eu
output=${1:?usage: scripts/build-compatible-linux.sh ABSOLUTE_OUTPUT_DIRECTORY}
case "$output" in /*) ;; *) echo 'output must be absolute' >&2; exit 64;; esac
mkdir -p "$output"
source_dir=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
docker run --rm --network none --read-only --cap-drop ALL \
  --security-opt no-new-privileges --pids-limit 128 --memory 2g --cpus 2 \
  --user "$(id -u):$(id -g)" --tmpfs /tmp:rw,nosuid,nodev,size=128m \
  --mount "type=bind,src=$source_dir,dst=/src,readonly" \
  --mount "type=bind,src=$output,dst=/out" \
  rust:1.94.0-bullseye@sha256:16950191527a4cb9e0762d9d48b705a6315158e4035e64f7a93ce8656a1b053c \
  sh -ec 'cd /src; export CARGO_TARGET_DIR=/out/target; rustc -vV; cat /etc/os-release; cargo test --locked --offline; cargo build --release --locked --offline; cp /out/target/release/alibi /out/alibi'
python3 "$source_dir/scripts/package-release.py" --binary "$output/alibi" --output "$output/dist" --glibc-baseline 2.31
