# Release history privacy and provenance audit

Reviewer: `root/acceptance_review`; time: 2026-10-11T22:50:57.244882+00:00. Audited revision: `5e41b1834c280f804e775535238c18f6909d1934`.

**No credential blocker identified.** Public visibility remains a separate user decision: the history contains local provenance and author metadata described below. This bounded audit is not a proof that every possible encoded secret is absent.

The audit enumerated all29 reachable commits,233 distinct historical blobs and140 tracked files. Unique blob content totals1,754,102 bytes; the largest blob is35,424 bytes. No binary blobs or blobs of at least1MiB were found; current tree modes are ordinary100644 files. Commit messages and author/committer metadata were included. Unrelated refs, unreachable objects, local configuration and external evidence archives were excluded.

No private-key markers, provider-token signatures or credential-bearing URLs were found. Two credential-assignment matches are `nsswitch.conf` lookup directives (`passwd: files`), not passwords. All76 contextual entropy candidates resolve to artifact paths, provenance identifiers or source expressions. Additional key/header/private-IPv6 context checks found no credential evidence. Potential credential values were not printed or copied into the audit report.

Historical campaign records expose local storage paths and host/account labels. Git metadata contains GitHub noreply author identities; package records contain maintainer mailing-list metadata. NOTICE intentionally records project authorship. Test addresses are fixtures, including isolated veth and NFS parser examples. These details would be public with the history and should be disclosed at the visibility decision; they are not automatically secrets and do not justify rewriting history. No history rewrite is recommended.

Product files under `src`, `tests`, `Cargo.toml` and `Cargo.lock` are unchanged from qualified commit `7e04d3cb076a48f2e131d6284a16f6bbb836ff58`. Original main `6f8ad5fba4ed6f3fb9f32c9930377a0a48e06931` is an ancestor of the audited release head. The tracked tree contains provenance manifests and evidence pointers, not the large external raw guest archives.

One current-status provenance issue was reported to root: the October paragraph in PROVENANCE.md still describes the early HOLD and unexecuted guests. Preserve that historical record, but add a dated successor correction/current-state pointer before release review. This is a documentation follow-up, not a secret or a reason to rewrite history.

Machine result: `E/AUDIT.json`, SHA256 `86f1c7ebad1951258102ac738e6122aa56252eca6ecd3fc30ead294da321f3ce`. E is `/data/git/alibi-release-evidence-20261011`. Its sanitized match inventory records category, path, line, object identity, hashed match and adjudication; the bounded scanner and complete object inventory are hash-bound there. Root owns final release-document diff review and public metadata disclosure. No build, VM, remote push, publication or visibility change was performed by this lane. Root retains audit custody through the release decision, with review at closeout or2026-10-18.
