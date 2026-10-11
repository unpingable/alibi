# Independent G35 name-correspondence repair acceptance

Reviewer/decision owner: `acceptance_review`, 2026-10-11. Final source: `74e4143844c0535d3837dc67df625f8251197dad`; executable SHA-256 `eb1ef3e9ac00462707e8e416d1e9496c916d074b7af06ec03e1673387a64f1fe`; library SHA-256 `06ce0eb4a78c64e7b1f42d54fb00666ef826e35a862cffbeb4bb71baa001092b`.

**Accept the bounded source repair and its local regressions. Actual G35 repair qualification remains pending an exact-candidate guest repeat and control review.** This is neither overall release acceptance nor a claim of stable mount identity. Earlier acceptance records remain unchanged. No source was edited, build or guest run by this reviewer, and no holdouts were inspected.

## Original counterexample

Independently inspected occurrence `1797c588-b36b-4363-9fb6-b09b9a832549` under `guest-lab/corrected-baseline-batch/G35`, including its sealed serial transition observations, CLI receipt and all ten captured-file hashes. START, serial and stderr hashes match the terminal record. Evidence root is `/data/git/alibi-qualification-evidence-20261011`.

The transition record identifies mount 29, device 7:0, ext4 `/dev/loop0` at `/run/alibi-matrix/mnt` before the acquisition wrapper unmounts it. The unmount succeeds; subsequent mountinfo no longer contains that mount, and statfs at the same pathname reports parent tmpfs with different filesystem identity and capacity. The wrapper delegates output to real `/bin/df`; it does not synthesize df data. Alibi nevertheless reports two material mounts, complete filesystem-usage evidence and CLEAR. This demonstrates inadequate identity correspondence in required evidence (F0/F2/F3 under the declared evidence contract). It does not demonstrate a hidden full filesystem, ENOSPC or inability to write: the original ext4 observations actually show available capacity.

The exact df-shaped parent rows in the minimized regression are derived replay inputs, not raw df stdout captured from inside that original Alibi invocation. The kernel transition, wrapper order, real-df delegation and receipt establish the bad association; do not mislabel the derived rows as an independently captured original command output.

| Evidence beneath the evidence root | SHA-256 |
|---|---|
| `guest-lab/corrected-baseline-batch/G35/TERMINAL.json` | `19023a87ec996b395401fede849da91939ae4ab94fb1f250e41bf9ee7d66b5df` |
| `guest-lab/corrected-baseline-batch/G35/captures/CAPTURES.json` | `85b75b538b2374442736ac3784a67d2658411d3594c25cd2c14e93efecca73be` |
| `guest-lab/g35-regression-before/RECEIPT.json` | `b89d1633eabe1afd865b4f9da63ae0acc9c8f72220fc807d48ad5edde80c9736` |
| `guest-lab/g35-padding-before/RECEIPT.json` | `e3f84b055b03e44a8074338998fb98c5befd7f2c7f05e829c23e485ad7bd362e` |
| `guest-lab/candidate-g35-final.receipt.json` | `24a27f53bdec6425f03a8cdc78974631890fc3df699f9165f2a8102513e345ab` |
| `guest-lab/g35-padding-final-checks/COMPLETE.json` | `9df40777192bfbf32f74ab607018e0dfc82413de44f05baa7dbddb946e3c44b1` |

## Source review and challenge

The collector now checks each byte/inode output against the requested source and mountpoint names. It requires a single nonempty row, the expected prefix/suffix with whitespace separation and exactly four capacity columns between them. This rejects G35's parent source/target instead of attaching parent capacity to the earlier ext4 mount. Numeric total/used/free, percentage framing and free-at-most-total are checked. No Used+Available=Total rule is introduced, preserving reserved-block semantics.

Failure is dimension-local: usable capacity survives as partial evidence, and a supported positive may still yield DEGRADED. Otherwise partial required evidence yields UNKNOWN. The regression explicitly exercises both low valid byte capacity plus mismatching inode identity, and benign valid capacity plus the same mismatch. The former retains DEGRADED and its limitation; the latter becomes UNKNOWN. Both-dimension mismatch has no usable source value. The repair does not silently drop a positive simply because another dimension's identity failed.

Independent review identified an additional ambiguity in the first frozen repair: expected source `volume ` could consume GNU df separator padding after printed `volume`. The implementation preserved that first freeze and a failing regression, then refused empty/boundary-whitespace expected names. The successor source and regression implement that refusal while preserving internal spaces. The recorded before-repair padding assertion fails; the successor focused suite passes. This is a bounded response to a concrete parser ambiguity, not a redesign of mount identity.

The representation still cannot authenticate all actual source labels: actual trailing spaces may be indistinguishable from padding for an ordinary expected source name. That limitation is now explicit in design, envelope and repair documents. Matching source/target text also cannot establish superblock continuity or detect all same-name mount-generation replacements. Device aliases, df choosing another bind view, unsupported quoting or unsupported boundary-whitespace names may cause conservative UNKNOWN. A claim that arbitrary mounted-object identity is proved would remain unacceptable.

Existing filesystem deduplication and bind policy are unchanged. Internal-space names have parser controls; root separately declared genuine GNU df double-space pathname and read-only-bind controls for execution. Their future results are not assumed here. Conservative UNKNOWN must remain visible in useful-exoneration measurements when an independent oracle establishes that sufficient evidence was actually available.

## Verification and limits

Recomputed the final binary/library hashes, both source hashes and each linked receipt hash in the final candidate receipt; all match. Recomputed the original minimized-regression stdout/stderr and padding-counterexample stdout hashes; all match. Final logs record 13 focused fsusage tests, 118 unit tests and 2 Receipt V2 integration tests passing, with fmt, clippy and release commands exiting zero. These are inspected producer records, not independent executions or reproducible-build proof.

The change does not alter Receipt V2 or add an accusation. General df header grammar, final-newline validation, all possible malformed rows and generation tracking remain outside its claim. Original failed guest evidence and both intermediate/final candidate identities are preserved. Local regression acceptance does not close the original affected guest experiment; root must rerun G35 and adequate controls with the exact eb1e executable, reconcile receipt limitations and retain final holdout separation.

Formalization consideration: under the admitted printed-name representation, each acquisition dimension is either valid, command-failed or malformed/mismatched; invalid dimensions cannot supply complete required negative evidence, while valid positives survive. Source inspection plus original independent transition evidence and bounded regression states are sufficient for this scoped repair decision. The model assumes representation correspondence and does not prove mounted-object continuity. Reviewer `acceptance_review` owns this decision; a stronger identity protocol would require separately admitted architecture and qualification.

Only this small review and a new immutable lane notice were created by this lane. Root owns custody through actual-guest repair review, final acceptance and the 2026-10-18 evidence review event. No VM, build output, credential or temporary substrate was created here.
