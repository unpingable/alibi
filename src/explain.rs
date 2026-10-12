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
supporting evidence: none in V0.
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
required evidence: /proc/self/mounts (with the root filesystem inside the
  mount model), df space/inode facts, kernel log.
supporting evidence: none in V0.
CLEAR: the root filesystem is a type the model reasons about, material
  mounts are writable, no journal/fs errors, no critical space or inode
  exhaustion.
DEGRADED: at least one positive indication; a deliberately read-only
  mount is reported with wording to that effect — the operator judges.
UNKNOWN: mounts, df output, or the kernel log could not be collected, or
  the root filesystem is of a type outside the mount model (ZFS, overlay,
  network filesystems) — its functional state cannot be excluded.
limitations: alibi cannot know operator intent; squashfs/iso9660 and
  pseudo filesystems are excluded as read-only-by-design; ZFS, overlay
  and network filesystems are out of V0's mount model and a root of that
  kind yields UNKNOWN, never CLEAR.
";

const MEMROT: &str = "\
memrot — is there memory/hardware integrity degradation?

accuses: memory integrity failure — EDAC uncorrected errors (severe),
  corrected errors (any nonzero count matters when exonerating), and
  kernel-log EDAC / \"Hardware Error\" / memory-failure patterns.
required evidence: EDAC sysfs (/sys/devices/system/edac/mc/mc*/ue_count
  and ce_count), kernel log.
supporting evidence: none in V0.
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
  and /proc/net/ipv6_route, the kernel log (link flaps, table-full), and
  conntrack counts whenever connection tracking is present on the host.
supporting evidence: none. When the conntrack subsystem is absent (its
  /proc/sys files do not exist) that failure class is not applicable; the
  finding records conntrack as not present and it does not block CLEAR.
CLEAR: no error/drop deltas, no flap storm, a usable (non-rejecting)
  default route exists, and conntrack (when present) is below threshold.
DEGRADED: at least one positive indication; counters are cumulative, so
  only bounded deltas verdict — lifetime totals never do.
UNKNOWN: interface facts, route tables, or the kernel log could not be
  collected, or conntrack is present but unreadable or malformed. Losing
  access to evidence reduces what can be excluded; it never clears.
NOT_APPLICABLE: no non-loopback interface exists on this host.
limitations: passive local evidence only — V0 runs no active probes; a
  successful or failed ping proves nothing about host state. Down/unused
  interfaces are recorded as evidence, not accusations.
";

/// Static scope descriptions, not an additional diagnostic evaluator.
pub fn clear_boundary(accusation: &str) -> &'static str {
    match accusation {
        "kernel-scream" => "no recognized serious-event patterns in the acquired kernel log; excludes neither unlogged kernel faults nor lost log history",
        "silent-killer" => "supported material mounts and acquired filesystem evidence only; does not exclude stalled I/O, every storage fault or later mount changes",
        "memrot" => "available EDAC counters and recognized acquired log reports only; not a hardware memory test or a statement about memory pressure",
        "network-choke-point" => "sampled local counters, routes, applicable conntrack and acquired log only; not DNS, remote-service reachability or continuous network availability",
        _ => "selected accusation and recorded applicable evidence only; not whole-host health",
    }
}

pub const GUIDE: &str = "\
Choose an accusation from a symptom — this guide does not diagnose the symptom.

Process disappeared, machine stalled, or suspected kernel trouble:
  alibi check kernel-scream
  Looks for recognized serious kernel-log events. A clear log does not rule out
  unlogged kernel faults, application exits, resource limits or all OOM causes.

Writes fail or storage seems unusable:
  alibi check silent-killer
  Checks supported mount state, critical space/inode levels and filesystem logs.
  It does not establish storage latency or application correctness.

Concern about reported memory integrity errors:
  alibi check memrot
  Checks available EDAC counters and recognized memory-error logs.
  High RAM usage, swapping and workload pressure are not this accusation.

Connections fail or the network seems intermittent:
  alibi check network-choke-point
  Checks bounded host-local observations. DNS, firewall policy, remote services
  and end-to-end reachability are separate questions.

Use alibi explain <accusation> for evidence requirements and exclusions.
Use alibi check to evaluate all four; none covers every cause of a symptom.
CLEAR requires sufficient applicable evidence for the named accusation.
UNKNOWN is a limit on the evidence, not proof of health or proof of a fault.
";
