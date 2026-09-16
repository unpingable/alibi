pub fn explain(accusation: &str) -> Option<&'static str> {
    match accusation {
        "kernel-scream" => Some(KERNEL_SCREAM),
        "silent-killer" => Some(SILENT_KILLER),
        "memrot" => Some(MEMROT),
        "network-choke-point" => Some(NETWORK_CHOKE_POINT),
        _ => None,
    }
}

const KERNEL_SCREAM: &str = "\
kernel-scream — has the kernel/hardware been screaming while monitoring stayed quiet?

accuses: serious kernel/hardware events in the kernel log: panic, oops,
  BUG, general protection fault, soft/hard CPU lockups, RCU stalls,
  machine checks (\"Hardware Error\"), PCIe AER errors, block I/O errors,
  and filesystem errors (shared with silent-killer).
required evidence: kernel log (dmesg, falling back to journalctl -k -b).
optional evidence: none in V0.
CLEAR: the kernel log was read and no serious-event patterns matched.
DEGRADED: at least one serious kernel/hardware event was found.
UNKNOWN: the kernel log could not be read (e.g. kernel.dmesg_restrict=1
  without journal access); a log you cannot read is not a quiet kernel.
limitations: the dmesg ring buffer may be wrapped on long-uptime hosts;
  the receipt envelope records which acquisition method was used.
";

const SILENT_KILLER: &str = "\
silent-killer — is the host superficially up but functionally damaged?

accuses: write-capable filesystems (ext2/3/4, xfs, btrfs, f2fs, jfs,
  reiserfs) remounted read-only; journal/filesystem error patterns in
  the kernel log; space or inode free at or below 5% on a material
  mount (write-capable or tmpfs).
required evidence: /proc/self/mounts, df space/inode facts, kernel log.
optional evidence: none in V0.
CLEAR: material mounts are writable, no journal/fs errors, no critical
  space or inode exhaustion.
DEGRADED: at least one positive indication; a deliberately read-only
  mount is reported with wording to that effect — the operator judges.
UNKNOWN: mounts, df output, or the kernel log could not be collected.
limitations: alibi cannot know operator intent; squashfs/iso9660 and
  pseudo filesystems are excluded as read-only-by-design; ZFS and
  network filesystems are out of V0's mount model.
";

const MEMROT: &str = "\
memrot — is there memory/hardware integrity degradation?

accuses: memory integrity failure — EDAC uncorrected errors (severe),
  corrected errors (any nonzero count matters when exonerating), and
  kernel-log EDAC / \"Hardware Error\" / memory-failure patterns.
required evidence: EDAC sysfs (/sys/devices/system/edac/mc/mc*/ue_count
  and ce_count), kernel log.
optional evidence: none in V0.
CLEAR: EDAC counters are zero; no kernel-reported memory errors.
DEGRADED: uncorrected or corrected errors present, or matching log lines.
UNKNOWN: the platform exposes no EDAC (most VMs) — the hypervisor owns
  hardware memory integrity and the guest cannot establish it.
limitations: \"RAM is busy\" is not memrot — usage, swapping and PSI are
  workload signals and are excluded from the integrity accusation.
";

const NETWORK_CHOKE_POINT: &str = "\
network-choke-point — is there obvious host-local network pathology?

accuses: host-local network pathology — RX/TX error or drop deltas over
  a 2 s counter sample window; >= 3 link-down events for one interface
  in the kernel-log window; no default route in IPv4 or IPv6; conntrack
  usage at or above 90% of max.
required evidence: /sys/class/net (two samples 2 s apart), /proc/net/route
  and /proc/net/ipv6_route.
optional evidence: conntrack counts, kernel log (link flaps, table-full).
CLEAR: no error/drop deltas, no flap storm, a default route exists, and
  conntrack (when present) is below threshold.
DEGRADED: at least one positive indication; counters are cumulative, so
  only bounded deltas verdict — lifetime totals never do.
UNKNOWN: interface facts or route tables could not be collected.
NOT_APPLICABLE: no non-loopback interface exists on this host.
limitations: passive local evidence only — V0 runs no active probes; a
  successful or failed ping proves nothing about host state. Down/unused
  interfaces are recorded as evidence, not accusations.
";
