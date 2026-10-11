# Successor repair design

`R` is `/data/git/alibi-qualification-evidence-20261011/guest-lab/requalification-20261011`.

Owner: implementation_review; root owns scope, guest execution, storage and freeze. This successor starts from `6a7b5012494b090726c4184644cdaccbaba7837e`; original qualification and holdout records remain historical. H04, H05 and H06 are now disclosed regressions. Fresh holdouts are inaccessible to this implementation lane.

## H05: same-object counter samples

Names, MAC addresses and ifindices are reusable and do not identify a device generation. The proposed collector opens one interface directory, opens all counter files relative to that held directory through `/proc/self/fd`, and retains each counter descriptor until both readings finish. It seeks each descriptor to offset zero before each bounded read. It does not reopen a failed counter by name. Optional endpoint state is also read relative to the held directory. A check against the final visible directory detects endpoint name replacement or loss; it supplements, rather than supplies, counter-object continuity.

The Linux mechanism matters: a removed sysfs attribute's kernfs node becomes inactive, and a fresh read cannot obtain the active reference needed to invoke its callback. Holding the old file does not attach it to a newly created device even if names and numeric indices are reused. Seeking zero resets seq-file traversal before a new read. All initial opens are directory-relative, preventing separate pathname resolutions from joining different device generations during acquisition. Failure of one required descriptor blocks CLEAR but does not discard independently comparable positive deltas from another descriptor or source.

Independent primary-source review currently covers Linux v6.6 `fs/kernfs/file.c`, `fs/seq_file.c`, `net/core/net-sysfs.c`, and Linux v6.5 namespace-transition code. Browser excerpts are preserved by the reviewer under `R/review-kernel-sources`; these are excerpts, not complete source archives. Actual guest deletion/reuse/rename controls must establish correspondence on the qualified kernel. Ordinary temporary-file tests establish descriptor mechanics only, not sysfs removal behavior.

Namespace migration can retain the same `net_device` and sysfs attributes. A same-object move away and back therefore need not make a sampled delta invalid. This repair promises same-object counter endpoints, not uninterrupted namespace residence, an atomic topology snapshot, or detection of every transient interface. It assumes genuine sysfs attributes and conventional coherent host proc/sys namespace views. Individual bind-mounted substituted counter files, concurrent source-view replacement and hidden driver counter resets are outside the continuity proposition. There is no new monitoring or network tracking facility.

## H04: supported positive EDAC reports

Successful registration and startup vocabulary are not memory errors. Replace the broad EDAC substring rule with an explicit counted CE/UE event prefix for EDAC memory-controller messages. Informational controller registration, known version/detection banners and zero-count reports produce no indication. Unknown EDAC forms must not become an assumed clean negative: memrot records a limitation and remains UNKNOWN without another supported positive. Recognition applies before generic signatures to EDAC-origin messages so a controller label containing a signature cannot create a false attribution. Counter-based corrected/uncorrected error evidence remains independent.

Independent review found that legal journal continuations are joined into one
record by the collector. A benign EDAC first line therefore cannot classify an
entire multiline record as clean. The successor classifies such a record as
unrecognized unless its first physical line independently establishes a positive;
continuation tokens never complete a truncated positive grammar. The first local
reproduction had an indentation error and is retained as failed fixture induction.
The corrected accepted-record reproduction demonstrated CLEAR versus required
UNKNOWN before the repair (`R/continuation-before-corrected`).

This is a finite report grammar, not a claim to understand every EDAC driver. Source-backed log fixtures establish semantics, not actual hardware fault induction.

## H06: human envelope fidelity

The default human renderer should consume the same receipt records as JSON: all collected source identities, status, completeness, methods, details, elapsed and completion offsets, sample windows, boot time and per-finding source roles. Required limitations must be visible without `--verbose`, including when an independent indication already makes the verdict DEGRADED. Verbose remains useful for expanded evidence/indications. JSON Receipt V2 does not change.

## Formalization consideration

Proposition: under genuine supported sysfs semantics, each accepted first/second counter pair refers to one retained counter object; disappearance never causes pathname fallback. A second proposition is that every canonical source record is rendered in default human output without suppressing failures because of verdict precedence. EDAC classification is a finite prefix decision, with unknown forms refusing exoneration.

These are bounded implementation claims. Tests cover descriptor state transitions, source-record fidelity and finite grammar cases; independent Linux source review plus actual guest transitions establishes the syscall/kernel correspondence the local tests cannot prove. A standalone abstract model would not establish that correspondence and is deferred rather than substituted for guest evidence. Exact final source, test and executable identities will be recorded at freeze; untested environments and source-view integrity remain explicit assumptions. Root and the independent reviewer decide acceptance, not this design note.

Custody: source, small regression records and exact frozen executable have an active repair-review/replay dependency. Root owns target retirement after freeze/custody verification and campaign closeout; next review is closeout or 2026-10-18. No source commitment or acceptance is implied by this design.
