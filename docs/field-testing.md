# External field trial: one real investigation

Use a machine you administer and an actual symptom you already need to
investigate. Alibi does not modify the host, probe a service or remediate.
Do not create faults on a shared or production host for this exercise. A quiet
baseline run is useful compatibility evidence but not a solved incident.

## Before seeing the results

Record the build version/source/hash, OS/kernel, symptom and approximate start
time, intended manual checks and initial hypotheses. Record what you think the
chosen accusation can and cannot exclude. Use `alibi guide` where available,
then `alibi explain <accusation>`; do not study evaluator internals first.

Run your chosen accusation initially without additional privileges. Capture the
default human output and exit status. If you also run `--json`, label it as a
second observation at a different time; do not assume the receipts are an atomic
pair. If a source is unavailable, record whether the explanation helps you decide
what evidence is missing. Do not automatically rerun as root.

## After the result

Before continuing the investigation, write in your own words:

- What exactly did CLEAR exclude, or what remains unknown?
- Which planned checks will you skip, and why? Which will you still perform?
- Did a source restriction or unsupported environment stop useful work?
- Was installation, accusation selection or interpretation confusing?

Continue your normal investigation. Record the independently established cause
if known, whether Alibi's bounded findings remained useful, and any tempting
but incorrect broader interpretation. A kernel-related cause can coexist with
kernel-scream CLEAR if no recognized event was present in the acquired log.
“No cause established” is a valid final answer. Tell us whether you would use
Alibi again and whether its receipt helped explain or challenge your conclusion.

## Independent usability evaluation

Recruit a consenting administrator unfamiliar with Alibi's implementation.
Let them use a real investigation and ordinary documentation; the implementer
should not coach accusation selection or interpret verdicts during the task.
A separate observer records selection, uncertainty comprehension, CLEAR scope
and later receipt usefulness. The operator decides whether any recommendation
is relevant; Alibi grants no authority to perform effects.

Record observations and quotations with consent, distinguishing what was said
from evaluator interpretation. Do not grade correctness using Alibi's own output
as ground truth. If measuring time, record actual human investigation time and
manual checks, including interruptions; Alibi's invocation timers are not that
measurement. A single uncontrolled story can establish usefulness to that person,
not causal average time savings. A comparative study needs independently matched
cases/order control; running the same solved incident twice has learning bias.

No participant has been recruited or evaluated by this document. Issue #3 stays
open until actual operator evidence exists. Participant access and any external
contact require the corresponding operator authorization; this protocol sends
nothing and introduces no telemetry.

## Share the minimum necessary

Use the [report template](field-report-template.md). Review hostnames, addresses,
interface names, mount paths and log excerpts before posting. Share a sanitized
summary first; retain original receipts privately for your own review. Mark
redactions and missing data, and never label a redacted receipt complete. Do not
post secrets or proprietary logs to a public issue. Automatic safe export remains
a separately gated idea (#8), not a promise that arbitrary logs can be anonymized.
