# Native glibc 2.31 observation record

Observed 2026-10-12 using Debian 11 userspace from pinned
`rust@sha256:16950191527a4cb9e0762d9d48b705a6315158e4035e64f7a93ce8656a1b053c`.
`getconf GNU_LIBC_VERSION` returned `glibc 2.31`; libc and loader files were
hashed, and loader `--list` resolved dependencies within that runtime.
No newer host library directory was mounted. Both exact retained archives
passed checksum/extraction, version/help and human/JSON invocation checks.
Independent review accepted these bounded observations.

The container ran as an unprivileged UID with a read-only root, no network,
no capabilities and no host PID/cgroup namespace. It shares the host kernel:
this is a genuine native userspace/runtime test, not separate-kernel Debian
qualification. UNKNOWN/DEGRADED under restricted evidence are diagnostic
results, not loader failures. No host fault was injected.

The first harness attempt stopped on the older loader's unsupported --version
option. It was preserved, then a separately identified corrected run completed.

```json
[
  {
    "case": "compatibility",
    "archive_sha256": "7f58b3c11c80459adcc6685df717439dcffaee285b4d94d6b1d146951b10a7ac",
    "binary_sha256": "d41ff1f06c413a42e1beae359ffdd46686f073e9a9d2dbc8accdb75bc8e895b3",
    "exit": {
      "human_exit": 1,
      "json_exit": 1
    },
    "verdicts": {
      "kernel-scream": "UNKNOWN",
      "silent-killer": "DEGRADED",
      "memrot": "UNKNOWN",
      "network-choke-point": "NOT_APPLICABLE"
    }
  },
  {
    "case": "guidance",
    "archive_sha256": "1ea0e621859602af02c78b376bed2faa1c92d523ed351c62cae5246952c509f1",
    "binary_sha256": "fd9c0646c05066529fac4d6e26cf7a4e9f71f3322c9368dd0d718ee7c5e85be8",
    "exit": {
      "human_exit": 1,
      "json_exit": 1
    },
    "verdicts": {
      "kernel-scream": "UNKNOWN",
      "silent-killer": "DEGRADED",
      "memrot": "UNKNOWN",
      "network-choke-point": "NOT_APPLICABLE"
    }
  }
]
```

These are the older compatibility/guidance binaries, not the new versioned
candidate. The candidate workflow separately exercises its exact archive with
`scripts/check-glibc231.sh` and uploads runtime records. Inspect those identities
before any publication. Original local records and independent acceptance are
in campaign `alibi-next-20261012`; raw host data is not published here.

## Formalization consideration

Owner: implementing and independent review lanes. Proposition: these exact
executables load and complete the selected commands using the recorded glibc
2.31 libraries. File hashes, loader resolution, isolated mount inspection and
captured execution are sufficient empirical evidence for that narrow statement.
A formal model would not establish actual loader behavior on untested systems.
No verdict, evidence contract, Receipt V2 schema or broader qualification changes
are implied. Version metadata changes identify the candidate; the sole golden
change is its tool-version string, not its receipt version or diagnostic content.
