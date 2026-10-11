# Acceptance environment correction

Reviewer: `root/acceptance_review`; time: 2026-10-11T21:13:35.244778+00:00.

The second-kernel identifier in sealed `ACCEPTANCE-FINAL.md` and `FINAL-ACCEPTANCE.json` is **6.8.0-90-generic**, not the mistakenly transcribed `6.8.0-31-generic`. This addendum is authoritative for that identifier. Original records remain immutable.

I independently re-read the retained acquisition retry's boot banner and `uname`, which both identify `6.8.0-90-generic`, and verified START/serial hashes against its terminal record. The second kernel still covers only the acquisition subset. Disposition **QUALIFIED_WITH_LIMITATIONS**, product/binary identity, findings, denominators and exclusions are unchanged; no experiment was rerun.

Machine correction: `R/FINAL-ACCEPTANCE-CORRECTION.json`, SHA256 `330fe5d71b974c779df9fe413e9c7762011218334d2f2a296fc30291a8126d3b`. It binds the original acceptance hashes and exact supporting retry artifacts. Root identified the error during final report reconciliation. Root owns combined acceptance/correction custody through the release decision, with review at closeout or 2026-10-18.
