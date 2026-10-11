# Release provenance and evidence availability

The release candidate descends directly from the original `main` (`6f8ad5fba4ed6f3fb9f32c9930377a0a48e06931`) through both qualification branches. It preserves their commits, historical HOLD decision, failures and later `QUALIFIED_WITH_LIMITATIONS` decision. Product files under src/tests and Cargo manifests are unchanged from qualified product commit `7e04d3cb076a48f2e131d6284a16f6bbb836ff58`. The final campaign record is `5e41b1834c280f804e775535238c18f6909d1934`.

The exact guest-tested executable SHA-256 is `0760ea3529ac3c3d831038a71c46f82b6bd3d750fd4dbc52866399962e593b06`. The prepared experimental archive wraps these same bytes without stripping or patching. A GitHub CI rebuild validates the corresponding source and has its own binary identity; it is not presented as the original guest-tested executable. Every archive carries SOURCE.json, and its outer checksum is provided separately. Checksums establish byte correspondence, not a cryptographic publisher signature.

## Included in Git

- Source, Receipt V2 golden tests, license and attribution.
- Both campaign reports, original HOLD, repaired acceptance, explicit corrections, useful-exoneration denominators and exclusions.
- Guest fixture/harness sources, disclosed former holdouts, fresh-set manifest/provenance, execution protocol and storage/custody reports.
- Bounded release history/privacy audit and packaging script.

## Retained separately

Large kernel/module inputs, compressed guest archives, raw serial/capture evidence, exact executables and machine-readable local acceptance records remain in campaign custody. Compiler target trees and expanded guest roots were retired after dependency checks. They are not committed or automatically published with the release. Historical reports contain absolute paths and hashes identifying those records; a public clone does not make those paths accessible and cannot independently inspect every raw historical observation.

The harness sources permit construction of new disclosed replay occurrences with the stated Linux inputs, tools and storage admission. Reproducing old paths verbatim is not required; preserving input identity and reporting differences is. The former holdouts are now disclosed and cannot serve as unseen acceptance for future changes. No new QEMU campaign was performed for release preparation.

The bounded public recommendation relies on the cited independent reasoning-lane reviews and retained evidence, not on an externally accredited audit. No operator time-savings study has been conducted. Review or sharing of additional raw evidence requires its owner to check host details and custody before disclosure.

## Public-history metadata

The history intentionally preserves author/license attribution, GitHub identities, build/package metadata, development-host labels and absolute local campaign paths. The bounded audit found no credential material requiring history rewriting. These metadata are listed in [AUDIT.md](AUDIT.md) for the owner's visibility decision. No private keys, authentication tokens, VM disks or bulk raw host evidence are added as release assets.

## Release-only claims

The packaging proposition is narrow: unchanged qualified product source plus an identified executable, installation files and checksums. Byte/hash comparison, archive extraction and CLI version/help checks are sufficient practical validation for this packaging change; no new diagnostic guarantee or formal model is introduced. CI permissions grant repository-content read access only. A CI pass is build/test evidence, not authorization to change visibility, publish a release or trust arbitrary host health.
