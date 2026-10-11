# Acquisition correspondence correction

Owner: implementation_review; independent acceptance identified this evidence-identity gap. This record supersedes any implication that the earlier genuine-journal or guest-capture replay receipts exercised the frozen executable. Original receipts are preserved unchanged.

The `acquisition-real-journal-final` and `acquisition-guest-format-correspondence` receipts both identify provisional binary SHA-256 `636e5de1864a38f4746de43adca55ceadb2d3238c449ffc16740b19bf097eb7b`. The frozen executable is SHA-256 `ca2a6b9ef258e3643d0feaabc46c530908699db28843f823f3207077d85245b1`, built from product revision `f908852c7c8e85bbf94ba959f5af855d65bad4e5`. The provisional receipts prove only provisional representation correspondence. They do not directly validate the frozen binary.

Between those observations and freeze, `valid_record` was tightened to require the explicit space after `]` and a regression for a missing separator was added. Other malformed-record behavior was preserved. The provisional uncommitted source was not separately sealed by hash, so its exact complete source identity is unavailable; this source-change account is the implementation lane's recorded work history, not a reconstructed immutable snapshot. Final source hashes are in `candidate-acquisition.receipt.json`. No product source or executable was changed for this correction.

## Exact frozen journal evidence

`guest-lab/acquisition-real-journal-frozen/RECEIPT.json` names and hashes the exact frozen executable and captures two fresh read-only invocations on crow: complete raw `journalctl -k -b --no-pager --quiet -o short-monotonic --no-hostname --all`, followed by the supported frozen CLI `check --json kernel-scream`. Both exit zero. The CLI yields CLEAR with kernel-log status ok/completeness complete, records the journal acquisition method and dmesg permission denial, and takes about 212 ms overall (207 ms source collection). The raw capture contains 275,523 bytes and is separately hashed. These are sequential observations, not a claim that the standalone raw command and Alibi saw an atomic identical snapshot.

The root-owned actual frozen-candidate guest run supplies fresh dmesg correspondence separately. This lane did not repeat a guest-capture replay or inspect any holdouts. Earlier provisional evidence remains useful history but cannot stand in for the frozen guest result.

## Deadline precision

The implementation checks its monotonic acquisition deadline after reading EOF and observing direct-child exit, before strict UTF-8 conversion. UTF-8 validation, result construction and return take additional finite userspace time; they are not followed by another deadline check. Failure cleanup has a separate 100 ms polling allowance. Therefore the qualified claim is deadline-governed acquisition and bounded retained output/cleanup polling, **not** exact wall-clock completion or a guarantee that validation finishes before the nominal deadline. Scheduler delay, spawn and kernel system calls add the already documented limits. No implementation expansion was made to address this wording distinction.

Formalization clarification: the finite state table covers acceptance at the last observed acquisition gate. It does not prove elapsed time at the later return, command provenance, or descendant retirement outside the owned group. Practical timing and lifecycle tests support the stated scope only. Decision owner implementation_review, with independent acceptance reviewing evidence correspondence.

## Custody

Root owns the new raw journal, stdout/stderr and receipt as pending release-review evidence; review at campaign closeout. No build output, guest, credential or background process was created. The new raw log is operational host evidence and remains external to tracked source. Original notices/receipts are not rewritten. Root's aggregate storage closeout includes the new directory.
