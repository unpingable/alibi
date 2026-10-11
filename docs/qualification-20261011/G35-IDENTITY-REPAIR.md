# G35 filesystem-name correspondence repair

Owner: implementation_review, with oracle_design's independent oracle agreement and root's real-guest evidence. The original executable, receipt and guest artifacts remain unchanged. This repair follows the sealed G35 run; it is not a claim that an inferred full-filesystem fault occurred.

## Independently observed condition

Original occurrence `1797c588-b36b-4363-9fb6-b09b9a832549`, Linux 6.5.0-44 guest. The transition oracle records mount 29, device 7:0, ext4 `/dev/loop0` at `/run/alibi-matrix/mnt`, at uptime 9.32. It was healthy: 7,145 total blocks and 6,567 available. Unmount returned zero; at uptime 9.39 the same path resolved to parent tmpfs, 80,365 total blocks and 74,677 available. Independent mountinfo and statfs distinguish the objects. The wrapper then runs real GNU df; it does not fabricate output.

Alibi had collected the earlier mount set, yet reports fs-usage complete and silent-killer CLEAR with two material mounts and no skipped evidence. The demonstrated defect is wrong-identity evidence combined with the earlier mount snapshot (F2/F3 and inadequate-evidence F0), not proven ENOSPC, blocked writes, or an actually full original filesystem.

## Minimized regression and bounded change

The before-repair test supplies a requested `/dev/loop0` mount and successful df-shaped parent `guest-root` `/` output. It fails because the collector calls this complete; `guest-lab/g35-regression-before` preserves exit 101 and the test-only source hash. This is a minimized assembly fixture derived from the guest condition, not a raw df stdout capture from the guest.

Each byte/inode acquisition now validates textual source and mountpoint correspondence to its requested mount-snapshot pair. It matches the exact source prefix and mountpoint suffix with whitespace boundaries and exactly four intervening capacity columns. Literal spaces in either name are preserved. A single-target command must return exactly one nonempty data row; additional rows are inadequate evidence. Valid capacity requires numeric total/used/free, percentage framing (or the existing zero-total dash representation), and free no greater than total. This does not impose Used+Available=Total, which is invalid for reserved blocks.

A mismatch invalidates only that dimension. Other valid dimensions and mounts survive as partial evidence: valid low capacity still permits DEGRADED, while otherwise benign partial evidence yields UNKNOWN. Both expected source/target and the observed mismatching row are retained in failure detail. Receipt V2 is unchanged. Existing deduplication remains unchanged, including selecting the original mount for ordinary original-plus-read-only-bind snapshots.

This is textual name correspondence, **not** stable mount/superblock identity or an atomic generation protocol. Device aliases, df's choice of another bind view, quoting differences or unsupported name representations can conservatively cause UNKNOWN. Matching strings cannot detect every same-name replacement, remount, or intervening generation. Literal-space support is a parser test until a genuine GNU df guest control confirms it. General header grammar, final-newline validation and all conceivable malformed df representations are outside this bounded repair.

## Validation and custody

Twelve focused fsusage tests pass, including G35, source-only/target-only differences, prefix/suffix collisions, additional columns/rows, literal spaces, and dimension-local positive retention. Final durable checks pass 117 unit tests, 2 Receipt V2 integration tests, fmt, clippy and release. Source and recovery checkpoint, stage logs and COMPLETE.json are under `guest-lab/g35-final-checks`; service `alibi-g35-checks-53535f71-beea-4c15-a305-9922f4c86b5e`, invocation `8e95015920fc4caa8094e834efccf553`, ran independently of the supervising tool session. Final executable/source hashes are recorded in the candidate receipt after commit.

Root must replay the actual affected guest occurrence and passing controls against that exact frozen executable, then independently accept evidence and run final holdouts. This lane did not inspect holdouts or conduct new guest experiments. No source/library test alone closes G35's real-guest repair qualification. Root owns final executable/library custody and compiler-output retirement at campaign closeout.

## Formalization consideration

Proposition: within the selected single-row df textual representation, a dimension is accepted only when it can be associated with the requested textual source/target pair and has valid capacity fields. Counterexample G35 refutes the earlier association based solely on successful command completion. The finite dimension states are valid (retain), command failure (inadequate), malformed/mismatched (inadequate); aggregation retains valid positives and prevents CLEAR when any required dimension is inadequate. Tests establish these bounded transitions and correspondence to evaluator policy. They cannot prove mounted-object continuity from names. A generation/identity protocol would require separate architecture and qualification and is not introduced here. Decision owner implementation_review, oracle owner oracle_design, acceptance independent.

## Original evidence identities

- `/data/git/alibi-qualification-evidence-20261011/guest-lab/corrected-baseline-batch/G35/serial.log` SHA-256 `fc4ec2578cc2abae7d6204e2ad6bd4f6446db2c95f4d7f8dc9c70f41891ebf28`
- `/data/git/alibi-qualification-evidence-20261011/guest-lab/corrected-baseline-batch/G35/TERMINAL.json` SHA-256 `19023a87ec996b395401fede849da91939ae4ab94fb1f250e41bf9ee7d66b5df`
- `/data/git/alibi-qualification-evidence-20261011/guest-lab/corrected-baseline-batch/G35/captures/G35/stdout` SHA-256 `460c387728e09e06748eceb1ab80ae7b226f95a1b2856d4f1cee6fa170b44ee8`
- `/data/git/alibi-qualification-evidence-20261011/guest-lab/corrected-baseline-batch/G35/captures/G35/metadata` SHA-256 `f5c44ec6d1ce1371f7fea0151f6c25e3d4a348cd2036fdfeb9c6cdfb3fbe1b2b`

## Independent padding review and successor

Independent acceptance identified that expected source `volume ` can consume a
space in GNU df's column padding and match printed source `volume`. The first
G35 candidate (`7cb0d1bcef7b2ddc34f59b277030eb3d77594c3a`, executable `7e2bc8a3f3b7e7c6f43927c083b304a37dae0e7e60ed6ed587da6c9517a71356`) remains preserved.
`guest-lab/g35-padding-before` records a failing test before the successor repair.
The successor refuses empty or boundary-whitespace expected source/mountpoint
names; internal spaces remain supported. Its final checks and exact identities
are recorded separately rather than overwriting the first freeze.

Even this refusal cannot authenticate every actual source label: GNU df's
printed trailing source spaces may be indistinguishable from column padding for
a normal expected name. The supported property is expected-name textual
correspondence under the admitted GNU df representation, not byte-for-byte
authentication of arbitrary kernel labels or proof of mount generation. This
representation limitation is explicit; no broader identity protocol is claimed.
The successor adds one regression (118 unit tests total, 13 fsusage tests); exact
completed verification is in its final-check receipt.
