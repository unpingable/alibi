# Consuming Alibi as an agent

Run `alibi check --json` and retain both stdout and the process exit status.
Exit 0 means CLEAR or NOT_APPLICABLE for the selected accusations, 1 means
DEGRADED, 2 means UNKNOWN, and 64 means CLI usage error. Inspect each finding;
exit 0 alone does not distinguish bounded exoneration from non-applicability.
A missing receipt, missing finding, invalid JSON, unexpected version, or process
failure is not CLEAR. Check the actual selected accusations against the request.

The standard-library Python consumer in [read-receipt.py](../scripts/read-receipt.py)
provides a scoped digest without matching explanation strings:

```bash
python3 scripts/read-receipt.py --expected kernel-scream,silent-killer,memrot,network-choke-point --exit-status 1 < tests/golden/receipt_v2.json
python3 tests/read_receipt_test.py
```

The consumer's exit status means parsing succeeded (0) or failed (2); it is not
Alibi's exit status. Preserve the original process status separately. Pass the exact requested accusation list through `--expected` and, when available,
the original status through `--exit-status`. The consumer rejects duplicate IDs,
unknown enums, contradictory acquisition/exoneration, inconsistent aggregate verdicts,
and selection mismatches. It validates a bounded subset of V2, not provenance,
all schema fields, or evaluator correctness. CI runs `python3 tests/read_receipt_test.py`.
This example does not launch Alibi or authorize any host action.

Join each finding's `sources` array (`name`, `role`) to the `envelope.sources`
array by `name`. Required sources with status other than `ok` or completeness
other than `complete` are inadequate acquisition evidence. A supporting source
may be absent without blocking the accusation. The evaluator remains authoritative:
complete acquisition can still be insufficient or outside the model, and
NOT_APPLICABLE is not a claim of innocence. DEGRADED takes precedence over UNKNOWN,
so inspect every finding and its required sources even when the overall verdict
is DEGRADED. A positive observation may survive incomplete neighboring evidence.

`completeness` describes acquisition, never complete historical coverage.
`completed_at_ms` and `window` describe collection timing, not event ages or
continuity. CLEAR covers only the selected accusations and their documented
boundaries. Retained-log reports and cumulative EDAC counts do not establish a
currently active fault, a recent event, or recovery. No typed event-age or
permission-denial field exists in V2: preserve `detail`, `skipped`, `indications`,
and `evidence` for explanation, and do not turn prose matching into a stable API.

Human output suggests next observations for unavailable logs, absent EDAC, and
unsupported root filesystems. These suggestions are not remediation instructions
or permission grants. An agent must obtain any authorization needed for its own
follow-up; Alibi itself remains read-only and performs no active network probes.
