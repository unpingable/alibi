# Alibi 0.1.0 — experimental release candidate

Prepared for operator feedback. Publication and public repository visibility still require the owner's approval. No production-readiness or universal Linux claim is made.

Alibi is a standalone, on-demand, read-only Linux host diagnostic CLI. It reports named accusations, evidence, bounded verdicts and receipts. It sends no active probes, performs no remediation and runs no background service. JSON Receipt V2 is unchanged.

## Qualified candidate

Disposition: **QUALIFIED_WITH_LIMITATIONS**. Product source `7e04d3cb076a48f2e131d6284a16f6bbb836ff58`; completed campaign record `5e41b1834c280f804e775535238c18f6909d1934`. The release branch preserves these commits and does not change product source.

Repairs prevent mixed interface generations from supporting counter exoneration, narrow EDAC attribution, and expose material required-source limitations in default human output. Earlier campaign repairs addressed route semantics, evidence parsing, acquisition deadlines and filesystem-name correspondence.

The two campaigns executed 214 QEMU boots including controls, repeats, failed induction and infrastructure outcomes. For the repaired evaluated primary set, independent review observed no false CLEAR among 72 findings and 25/25 useful exonerations from real required evidence, plus one separately labeled benign-log fixture. Ten fresh cases followed implementation freeze; nine full case oracles were supported, while one preserves an explicit overconstrained-oracle subclaim. These are finite selected observations, not production accuracy estimates.

## Installation

See [INSTALL.md](INSTALL.md). The exact guest-qualified binary requires Linux x86_64, glibc 2.39+, libgcc_s and the GNU loader; it is not static. The prepared archive contains that unmodified executable, license, notice, installation instructions and source identity. CI also builds a corresponding-source archive; SOURCE.json distinguishes its binary from the original guest-tested one. Source builds remain available without a packaging ecosystem.

## Limitations that matter

- CLEAR concerns only the selected accusation and recorded applicable evidence. It is never permission to perform an effect or a healthy-host certificate.
- Full guest evidence uses Linux 6.5.0-44-generic with constructed GNU userspace. Linux 6.8.0-90-generic has acquisition-subset evidence only. No broad distribution, physical-hardware, ARM or static/musl qualification.
- No actual healthy/error EDAC hardware qualification; VMs without EDAC usually return UNKNOWN. Benign/malformed message fixtures are labeled as fixtures.
- Network findings do not establish DNS, service reachability, filtering policy or continuous availability. Hidden resets and unobserved history can escape endpoint sampling.
- Filesystem name correspondence is not proof of arbitrary mount-generation continuity. Stalled storage, starvation and general systemd/service diagnosis remain outside established scope.
- Retained logs can omit history. Command-format checks do not authenticate source contents. Genuine coherent proc/sys views are assumed.
- H03's intended induction and G30/G36 conditions remain qualification gaps. Q04's malformed-unused-ifindex expectation remains a recorded oracle limitation, not a blanket pass.
- Receipts can expose host details; review them before sharing. Default human rendering is checked for fidelity, not measured for operator comprehension.

Reduced human diagnostic time is not established. That does not block this experimental release: feedback from actual operators is welcome. Tell us whether it helped, whether UNKNOWN made sense, and whether you would use it again. A formal comparison can follow if useful.

[Qualification](../requalification-20261011/QUALIFICATION.md) · [Results](../requalification-20261011/RESULTS.md) · [Acceptance correction](../requalification-20261011/ACCEPTANCE-CORRECTION.md) · [Evidence availability](EVIDENCE.md)
