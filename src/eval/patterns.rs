pub struct Pattern {
    pub needle: &'static str,
    pub label: &'static str,
    pub tags: &'static [&'static str],
    pub excludes: &'static [&'static str],
}

pub const MATCH_CAP: usize = 10;

pub const PATTERNS: &[Pattern] = &[
    Pattern {
        needle: "Kernel panic",
        label: "kernel panic",
        tags: &["kernel-scream"],
        excludes: &[],
    },
    Pattern {
        needle: "Oops",
        label: "kernel oops",
        tags: &["kernel-scream"],
        excludes: &[],
    },
    Pattern {
        needle: "BUG:",
        label: "kernel BUG",
        tags: &["kernel-scream"],
        excludes: &[],
    },
    Pattern {
        needle: "general protection fault",
        label: "general protection fault",
        tags: &["kernel-scream"],
        excludes: &[],
    },
    Pattern {
        needle: "soft lockup",
        label: "CPU soft lockup",
        tags: &["kernel-scream"],
        excludes: &[],
    },
    Pattern {
        needle: "hard LOCKUP",
        label: "CPU hard lockup",
        tags: &["kernel-scream"],
        excludes: &[],
    },
    Pattern {
        needle: "detected stall",
        label: "RCU stall",
        tags: &["kernel-scream"],
        excludes: &[],
    },
    Pattern {
        needle: "Hardware Error",
        label: "machine check / hardware error",
        tags: &["kernel-scream", "memrot"],
        excludes: &[],
    },
    Pattern {
        needle: "AER: PCIe Bus Error",
        label: "PCIe AER error",
        tags: &["kernel-scream"],
        excludes: &[],
    },
    Pattern {
        needle: "AER: Corrected error",
        label: "PCIe AER corrected error",
        tags: &["kernel-scream"],
        excludes: &[],
    },
    Pattern {
        needle: "I/O error",
        label: "block I/O error",
        tags: &["kernel-scream"],
        excludes: &[],
    },
    Pattern {
        needle: "EXT4-fs error",
        label: "ext4 error",
        tags: &["kernel-scream", "silent-killer"],
        excludes: &[],
    },
    Pattern {
        needle: "Remounting filesystem read-only",
        label: "filesystem remounted read-only",
        tags: &["silent-killer"],
        excludes: &[],
    },
    Pattern {
        needle: "JBD2:",
        label: "ext journal error",
        tags: &["silent-killer"],
        excludes: &[],
    },
    Pattern {
        needle: "metadata I/O error",
        label: "XFS metadata I/O error",
        tags: &["kernel-scream", "silent-killer"],
        excludes: &[],
    },
    Pattern {
        // xfs_do_force_shutdown: "<why> (0x..) detected at ... Shutting down
        // filesystem." The mount stays rw, so only the log witnesses it.
        needle: "Shutting down filesystem",
        label: "XFS forced shutdown",
        tags: &["kernel-scream", "silent-killer"],
        excludes: &[],
    },
    Pattern {
        needle: "BTRFS: error",
        label: "btrfs error",
        tags: &["silent-killer"],
        excludes: &[],
    },
    Pattern {
        needle: "BTRFS error",
        label: "btrfs error",
        tags: &["silent-killer"],
        excludes: &[],
    },
    Pattern {
        needle: "Memory failure",
        label: "kernel memory failure",
        tags: &["memrot"],
        excludes: &[],
    },
    Pattern {
        needle: "NIC Link is Down",
        label: "link down event",
        tags: &["net-down"],
        excludes: &[],
    },
    Pattern {
        // phylib (r8169, stmmac, ...): "Link is Down"
        needle: "Link is Down",
        label: "link down event",
        tags: &["net-down"],
        excludes: &[],
    },
    Pattern {
        // tg3, bnx2x: "Link is down"
        needle: "Link is down",
        label: "link down event",
        tags: &["net-down"],
        excludes: &[],
    },
    Pattern {
        needle: "link is down",
        label: "link down event",
        tags: &["net-down"],
        excludes: &[],
    },
    Pattern {
        // mlx4_en: "Link Down"
        needle: "Link Down",
        label: "link down event",
        tags: &["net-down"],
        excludes: &[],
    },
    Pattern {
        // mlx5_core: "Link down"
        needle: "Link down",
        label: "link down event",
        tags: &["net-down"],
        excludes: &[],
    },
    Pattern {
        needle: "link down",
        label: "link down event",
        tags: &["net-down"],
        excludes: &[],
    },
    Pattern {
        needle: "nf_conntrack: table full",
        label: "conntrack table full",
        tags: &["network-choke-point"],
        excludes: &[],
    },
];

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum EdacMessage {
    Other,
    Informational,
    Error,
    Unrecognized,
}

pub(crate) fn edac_message(line: &str) -> EdacMessage {
    // Match event position, never a CE/UE token embedded in a device label.
    // Acquired records retain their dmesg/journal framing; plain messages are
    // also used by the pure evaluator fixtures.
    let mut message = line.trim_start();
    if message.starts_with('<') {
        if let Some(end) = message.find('>') {
            message = &message[end + 1..];
        }
    }
    if message.starts_with('[') {
        if let Some(end) = message.find(']') {
            message = message[end + 1..].trim_start();
        }
    }
    message = message.strip_prefix("kernel: ").unwrap_or(message);
    let Some(edac) = message.strip_prefix("EDAC ") else {
        return EdacMessage::Other;
    };
    // A journal continuation belongs to this record but is not a separately
    // framed event. It cannot be assumed benign or supply missing fields in
    // the first line's positive grammar.
    let (first, multiline) = edac
        .split_once('\n')
        .map_or((edac, false), |(first, _)| (first, true));
    let classification = classify_edac_body(first);
    if multiline && classification != EdacMessage::Error {
        EdacMessage::Unrecognized
    } else {
        classification
    }
}

fn classify_edac_body(edac: &str) -> EdacMessage {
    if edac.starts_with("MC: Ver: ")
        || (edac.starts_with("amd64: ") && edac.contains(" detected ("))
    {
        return EdacMessage::Informational;
    }
    let Some(controller) = edac.strip_prefix("MC") else {
        return EdacMessage::Unrecognized;
    };
    let Some((index, report)) = controller.split_once(": ") else {
        return EdacMessage::Unrecognized;
    };
    if index.is_empty() || !index.bytes().all(|byte| byte.is_ascii_digit()) {
        return EdacMessage::Unrecognized;
    }
    if report.starts_with("Giving out device to module ") {
        return EdacMessage::Informational;
    }
    let mut fields = report.split_whitespace();
    let Some(count) = fields
        .next()
        .filter(|field| !field.is_empty() && field.bytes().all(|byte| byte.is_ascii_digit()))
        .and_then(|field| field.parse::<u64>().ok())
    else {
        return EdacMessage::Unrecognized;
    };
    if !matches!(fields.next(), Some("CE" | "UE")) || fields.next().is_none() {
        return EdacMessage::Unrecognized;
    }
    if count == 0 {
        EdacMessage::Informational
    } else {
        EdacMessage::Error
    }
}

pub fn line_label_for(line: &str, tag: &str) -> Option<&'static str> {
    match edac_message(line) {
        EdacMessage::Error => return (tag == "memrot").then_some("EDAC memory error report"),
        EdacMessage::Informational | EdacMessage::Unrecognized => return None,
        EdacMessage::Other => {}
    }
    PATTERNS
        .iter()
        .find(|p| {
            p.tags.contains(&tag)
                && line.contains(p.needle)
                && p.excludes.iter().all(|e| !line.contains(*e))
        })
        .map(|p| p.label)
}

pub fn matches_for(lines: &[String], tag: &str) -> (Vec<(String, &'static str)>, usize) {
    let mut capped = Vec::new();
    let mut total = 0;
    for line in lines {
        if let Some(label) = line_label_for(line, tag) {
            total += 1;
            if capped.len() < MATCH_CAP {
                capped.push((line.clone(), label));
            }
        }
    }
    (capped, total)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dmesg_sample() -> Vec<String> {
        vec![
            "Kernel panic - not syncing: VFS: Unable to mount root fs".to_string(),
            "EXT4-fs error (device sda1): ext4_journal_start_sb: Detected aborted journal"
                .to_string(),
            "EDAC MC0: 1 CE error on CPU_SrcID#0".to_string(),
            "nf_conntrack: table full, dropping packet".to_string(),
            "ixgbe 0000:01:00.0 eno1: NIC Link is Down".to_string(),
        ]
    }

    #[test]
    fn dmesg_flavored_lines_map_to_expected_tags() {
        let lines = dmesg_sample();
        assert_eq!(matches_for(&lines, "kernel-scream").1, 2);
        assert_eq!(matches_for(&lines, "silent-killer").1, 1);
        assert_eq!(matches_for(&lines, "memrot").1, 1);
        assert_eq!(matches_for(&lines, "network-choke-point").1, 1);
        assert_eq!(matches_for(&lines, "net-down").1, 1);
    }

    #[test]
    fn boring_boot_lines_produce_no_matches() {
        let boring = vec![
            "Linux version 6.8.0".to_string(),
            "pci 0000:00:02.0: [mem 0xfed00000-0xfed003ff]".to_string(),
            "clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x2c95790, mode: 32-bit"
                .to_string(),
            "systemd[1]: Reached target Multi-User System.".to_string(),
        ];
        for tag in [
            "kernel-scream",
            "silent-killer",
            "memrot",
            "network-choke-point",
            "net-down",
        ] {
            assert_eq!(matches_for(&boring, tag).1, 0, "tag {}", tag);
        }
    }

    #[test]
    fn edac_banner_lines_are_excluded() {
        let lines = vec![
            "EDAC MC: Ver: 3.0.0".to_string(),
            "EDAC amd64: F17h_M10h detected (node 0)".to_string(),
        ];
        assert_eq!(matches_for(&lines, "memrot").1, 0);
    }

    #[test]
    fn edac_error_lines_still_match() {
        let lines = vec!["EDAC MC0: 1 CE memory read error on DIMM_A1".to_string()];
        let (capped, total) = matches_for(&lines, "memrot");
        assert_eq!(total, 1);
        assert_eq!(capped[0].1, "EDAC memory error report");
    }

    #[test]
    fn edac_classification_uses_event_prefix_not_label_vocabulary() {
        for line in [
            "EDAC MC0: Giving out device to module Hardware Error controller Memory failure: DEV 9 UE fault (POLLED)",
            "<6>[ 1.0] EDAC MC0: Giving out device to module sample controller sample: DEV sample (POLLED)",
            "[ 1.0] kernel: EDAC MC1: 0 CE memory read error on DIMM0",
        ] {
            assert_eq!(edac_message(line), EdacMessage::Informational);
            for tag in ["memrot", "kernel-scream"] {
                assert_eq!(line_label_for(line, tag), None);
            }
        }
        for report in ["1 CE memory read error on DIMM0", "2 UE error on DIMM1"] {
            assert_eq!(
                edac_message(&format!("EDAC MC12: {report}")),
                EdacMessage::Error
            );
        }
        for report in [
            "module label 9 CE error",
            "-1 UE error",
            "CE",
            "3 CE",
            "18446744073709551616 UE error",
        ] {
            assert_eq!(
                edac_message(&format!("EDAC MC0: {report}")),
                EdacMessage::Unrecognized
            );
        }
        assert_eq!(
            line_label_for(
                "Kernel panic caused by EDAC initialization",
                "kernel-scream"
            ),
            Some("kernel panic")
        );
    }

    #[test]
    fn xfs_forced_shutdown_matches_both_filesystem_accusations() {
        let lines = vec![
            "XFS (sdb1): Corruption of in-memory data (0x8) detected at xfs_trans_cancel+0x1a0/0x1b0 (fs/xfs/xfs_trans.c:1097).  Shutting down filesystem.".to_string(),
            "XFS (sdb1): Please unmount the filesystem and rectify the problem(s)".to_string(),
        ];
        let (capped, total) = matches_for(&lines, "silent-killer");
        assert_eq!(total, 1);
        assert_eq!(capped[0].1, "XFS forced shutdown");
        assert_eq!(matches_for(&lines, "kernel-scream").1, 1);
        assert_eq!(matches_for(&lines, "memrot").1, 0);
    }

    #[test]
    fn driver_link_down_forms_match_net_down() {
        for line in [
            "r8169 0000:07:00.0 enp7s0: Link is Down",
            "tg3 0000:02:00.0 eth0: Link is down.",
            "mlx4_en: eth2: Link Down",
            "mlx5_core 0000:03:00.0 eth3: Link down",
            "ixgbe 0000:01:00.0 eno1: NIC Link is Down",
        ] {
            assert_eq!(
                line_label_for(line, "net-down"),
                Some("link down event"),
                "{}",
                line
            );
        }
        for line in [
            "r8169 0000:07:00.0 enp7s0: Link is Up - 1Gbps/Full - flow control rx/tx",
            "mlx4_en: eth2: Link Up",
        ] {
            assert_eq!(line_label_for(line, "net-down"), None, "{}", line);
        }
    }

    #[test]
    fn matches_are_capped_but_total_is_full_count() {
        let lines: Vec<String> = (0..25).map(|_| "Kernel panic again".to_string()).collect();
        let (capped, total) = matches_for(&lines, "kernel-scream");
        assert_eq!(capped.len(), MATCH_CAP);
        assert_eq!(total, 25);
    }
}
