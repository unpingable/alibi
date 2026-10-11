# Packaging claim correction

Baseline and candidate ELF inspection established that ordinary `cargo build
--release` links glibc/libgcc dynamically. The README's static-binary claim was
therefore not supported by the delivered build. This campaign changes the claim,
not the linker or supported diagnostic families. The isolated archives carry the
exact executable and runtime libraries, each identified in BUILD.json. No static
packaging qualification or second complete distribution is inferred.

Formalization consideration: root owns this descriptive claim. Exact executable
and library hashes plus ELF dependency inspection suffice; a formal model would
not establish the actual linking performed by the toolchain. The source guide's
static-binary aspiration remains a follow-on packaging obligation, not evidence
that this build is static. Product source identity remains74e4143844c0535d3837dc67df625f8251197dad;
this documentation correction does not rebuild or replace the frozen executable.
