# Sealed H04 message provenance

This case uses an explicitly labeled command substitution because the VM exposes no real EDAC memory controller. It does not claim actual controller registration or memory-error injection.

Primary Linux source shows the informational message template "Giving out device to module %s controller %s: DEV %s (%s)" on the successful registration path of `edac_mc_add_mc_with_groups`, after device construction and before return zero. It describes setup, not a corrected/uncorrected memory error. See [Linux EDAC controller source](https://github.com/torvalds/linux/blob/master/drivers/edac/edac_mc.c) and [pinned source snapshot](https://code.googlesource.com/linux/torvalds/linux/+/aac4de465af08ccec90ef47bdcc13435e48a7223/drivers/edac/edac_mc.c).

Source inspection used indexed primary-source text, 2026-10-11. Exact v6.5 raw-page retrieval was unavailable; do not claim this proves a specific guest kernel emitted the message. The fixture substitutes conspicuous fixture names into the authentic template and uses selected dmesg framing. The independent semantic oracle is that informational registration alone does not establish memory-integrity degradation; absent EDAC counters remain inadequate negative evidence. Root must preserve this source-version limitation in final adjudication.
