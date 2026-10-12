use crate::collect::{Facts, FsUsage, Mount, SourceId};
use crate::eval::{
    base_verdict, first_plus_more, patterns, Evaluator, Finding, SourceRole, Verdict,
};

const SOURCES: &[SourceRole] = &[
    SourceRole::required(SourceId::Mounts),
    SourceRole::required(SourceId::FsUsage),
    SourceRole::required(SourceId::KernelLog),
];

pub struct SilentKiller;

impl Evaluator for SilentKiller {
    fn accusation(&self) -> &'static str {
        "silent-killer"
    }

    fn source_roles(&self) -> &'static [SourceRole] {
        SOURCES
    }

    fn evaluate_evidence(&self, facts: &Facts) -> Finding {
        let mut finding = Finding::new(self.accusation());
        let mut required_failures: Vec<String> = Vec::new();

        let mounts = match facts.mounts.get() {
            Some(m) => m.as_slice(),
            None => {
                let msg = format!("mount table unavailable: {}", facts.mounts.failure_reason());
                required_failures.push(msg.clone());
                finding.skipped.push(msg);
                &[]
            }
        };

        // The accusation is about the host's filesystems being functionally
        // usable; the root filesystem is the one that must be in the model.
        // A root of a type the model does not reason about (zfs, overlay,
        // nfs, ...) or a root missing from the table is incomplete required
        // evidence, not innocence.
        if facts.mounts.get().is_some() {
            match root_mount(mounts) {
                Some(root) if root.material => {}
                Some(root) => {
                    let msg = format!(
                        "root filesystem type {} is outside the mount model; functional damage cannot be excluded",
                        root.fstype
                    );
                    required_failures.push(msg.clone());
                    finding.skipped.push(msg);
                }
                None => {
                    let msg = "root filesystem missing from mount table".to_string();
                    required_failures.push(msg.clone());
                    finding.skipped.push(msg);
                }
            }
        }

        let domain: Vec<&Mount> = mounts
            .iter()
            .filter(|m| m.material && m.write_capable)
            .collect();
        let mut bind_views = 0usize;
        for m in domain.iter().filter(|m| m.ro) {
            let has_rw_sibling = domain
                .iter()
                .any(|other| other.device == m.device && !other.ro);
            if has_rw_sibling {
                bind_views += 1;
            } else {
                finding.indications.push(format!(
                    "{} ({}, {}) mounted read-only; if intentional, treat as expected",
                    m.mountpoint, m.fstype, m.device
                ));
            }
        }
        if bind_views > 0 {
            finding.evidence.push(format!(
                "{} read-only bind mount(s) on otherwise-writable filesystems (treated as intentional)",
                bind_views
            ));
        }

        match facts.kernel_log.get() {
            Some(log) => {
                let (matches, _) = patterns::matches_for(&log.lines, self.accusation());
                for (line, label) in matches {
                    finding
                        .indications
                        .push(format!("retained kernel-log report: {}: {}", label, line));
                }
            }
            None => {
                let msg = format!(
                    "kernel log unavailable: {}",
                    facts.kernel_log.failure_reason()
                );
                required_failures.push(msg.clone());
                finding.skipped.push(msg);
            }
        }

        let usage = match facts.fs_usage.get() {
            Some(u) => u.as_slice(),
            None => {
                let msg = format!(
                    "filesystem usage unavailable: {}",
                    facts.fs_usage.failure_reason()
                );
                required_failures.push(msg.clone());
                finding.skipped.push(msg);
                &[]
            }
        };

        for u in usage {
            if let Some(bytes) = u.bytes {
                if bytes.total > 0 && (bytes.free as u128) * 20 <= bytes.total as u128 {
                    finding.indications.push(format!(
                        "{} ({}) critically low on space ({}% free)",
                        u.mountpoint,
                        u.device,
                        pct_free(bytes.free, bytes.total)
                    ));
                }
            }
            if let Some(inodes) = u.inodes {
                if inodes.total > 0 && (inodes.free as u128) * 20 <= inodes.total as u128 {
                    finding.indications.push(format!(
                        "{} ({}) inode exhaustion ({}% free)",
                        u.mountpoint,
                        u.device,
                        pct_free(inodes.free, inodes.total)
                    ));
                }
            }
        }

        finding.verdict = base_verdict(&finding.indications, &required_failures);
        match finding.verdict {
            Verdict::Clear => {
                finding.summary =
                    "material mount observations writable; no recognized journal/fs errors in acquired retained log".to_string();
                let rw = mounts.iter().filter(|m| m.material && !m.ro).count();
                finding
                    .evidence
                    .push(format!("{} material mounts read-write", rw));
                if let Some((mp, pct)) = min_free(usage, true) {
                    finding
                        .evidence
                        .push(format!("min free space observed: {}% ({})", pct, mp));
                }
                if let Some((mp, pct)) = min_free(usage, false) {
                    finding
                        .evidence
                        .push(format!("min free inodes observed: {}% ({})", pct, mp));
                }
            }
            Verdict::Degraded => {
                finding.summary = first_plus_more(&finding.indications);
            }
            Verdict::Unknown => {
                finding.summary = required_failures.join("; ");
            }
            Verdict::NotApplicable => unreachable!(),
        }
        finding
    }
}

/// The mount that currently backs `/`: the last root row wins, since later
/// rows in `/proc/self/mounts` over-mount earlier ones.
fn root_mount(mounts: &[Mount]) -> Option<&Mount> {
    mounts.iter().rev().find(|m| m.mountpoint == "/")
}

fn pct_free(free: u64, total: u64) -> u64 {
    if total == 0 {
        return 0;
    }
    ((free as u128) * 100 / total as u128) as u64
}

fn min_free(usage: &[FsUsage], bytes: bool) -> Option<(String, u64)> {
    usage
        .iter()
        .filter_map(|u| {
            let capacity = if bytes { u.bytes } else { u.inodes }?;
            let (free, total) = (capacity.free, capacity.total);
            if total == 0 {
                None
            } else {
                Some((u.mountpoint.clone(), pct_free(free, total)))
            }
        })
        .min_by_key(|(_, pct)| *pct)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collect::{FsCapacity, FsUsage, KernelLog, Mount, Source, Status};
    use crate::eval::fixtures::{self, src_ok};
    use std::time::Duration;

    #[test]
    fn clear_on_clean_fixture() {
        let finding = SilentKiller.evaluate(&fixtures::clean_facts());
        assert_eq!(finding.verdict, Verdict::Clear);
        assert_eq!(
            finding.summary,
            "material mount observations writable; no recognized journal/fs errors in acquired retained log"
        );
        assert!(!finding.evidence.is_empty());
    }

    #[test]
    fn degraded_on_read_only_ext4_mount() {
        let mut facts = fixtures::clean_facts();
        if let Some(mounts) = facts.mounts.value.as_mut() {
            mounts[0].ro = true;
        }
        let finding = SilentKiller.evaluate(&facts);
        assert_eq!(finding.verdict, Verdict::Degraded);
        assert!(finding.indications[0].contains("mounted read-only"));
        assert!(finding.indications[0].contains("if intentional"));
    }

    #[test]
    fn read_only_bind_view_of_writable_device_is_not_an_indication() {
        let mut facts = fixtures::clean_facts();
        if let Some(mounts) = facts.mounts.value.as_mut() {
            mounts.push(Mount {
                device: "/dev/sda1".to_string(),
                mountpoint: "/snap/x".to_string(),
                fstype: "ext4".to_string(),
                ro: true,
                write_capable: true,
                material: true,
            });
        }
        let finding = SilentKiller.evaluate(&facts);
        assert_eq!(finding.verdict, Verdict::Clear);
        assert!(!finding.indications.iter().any(|i| i.contains("/snap/x")));
        assert!(finding
            .evidence
            .iter()
            .any(|e| e.contains("1 read-only bind mount(s)")));
    }

    #[test]
    fn device_mounted_read_only_everywhere_is_an_indication() {
        let mut facts = fixtures::clean_facts();
        if let Some(mounts) = facts.mounts.value.as_mut() {
            mounts.push(Mount {
                device: "/dev/sdb1".to_string(),
                mountpoint: "/snap/y".to_string(),
                fstype: "ext4".to_string(),
                ro: true,
                write_capable: true,
                material: true,
            });
        }
        let finding = SilentKiller.evaluate(&facts);
        assert_eq!(finding.verdict, Verdict::Degraded);
        assert!(finding
            .indications
            .iter()
            .any(|i| i.contains("/snap/y") && i.contains("mounted read-only")));
    }

    #[test]
    fn unknown_when_kernel_log_unavailable() {
        let mut facts = fixtures::clean_facts();
        facts.kernel_log = Source::failed(
            Status::Unavailable,
            "dmesg: operation not permitted",
            Duration::from_millis(1),
        );
        let finding = SilentKiller.evaluate(&facts);
        assert_eq!(finding.verdict, Verdict::Unknown);
    }

    #[test]
    fn degraded_on_critical_space() {
        let mut facts = fixtures::clean_facts();
        facts.fs_usage = src_ok(vec![FsUsage {
            device: "/dev/sda1".to_string(),
            mountpoint: "/".to_string(),
            bytes: Some(FsCapacity {
                total: 100,
                free: 3,
            }),
            inodes: Some(FsCapacity {
                total: 100,
                free: 90,
            }),
        }]);
        let finding = SilentKiller.evaluate(&facts);
        assert_eq!(finding.verdict, Verdict::Degraded);
        assert!(finding
            .indications
            .iter()
            .any(|i| i.contains("critically low on space")));
    }

    #[test]
    fn partial_usage_preserves_positive_evidence() {
        let mut facts = fixtures::clean_facts();
        facts.fs_usage = Source::partial(
            vec![FsUsage {
                device: "/dev/sda1".to_string(),
                mountpoint: "/".to_string(),
                bytes: Some(FsCapacity {
                    total: 100,
                    free: 3,
                }),
                inodes: None,
            }],
            Status::Unavailable,
            "/data: df timed out",
            "df fixture",
            Duration::from_millis(1),
        );
        let finding = SilentKiller.evaluate(&facts);
        assert_eq!(finding.verdict, Verdict::Degraded);
        assert!(finding
            .indications
            .iter()
            .any(|i| i.contains("critically low on space")));
    }

    #[test]
    fn partial_benign_usage_cannot_clear() {
        let mut facts = fixtures::clean_facts();
        facts.fs_usage = Source::partial(
            vec![FsUsage {
                device: "/dev/sda1".to_string(),
                mountpoint: "/".to_string(),
                bytes: Some(FsCapacity {
                    total: 100,
                    free: 50,
                }),
                inodes: None,
            }],
            Status::Unavailable,
            "/data: df timed out",
            "df fixture",
            Duration::from_millis(1),
        );
        let finding = SilentKiller.evaluate(&facts);
        assert_eq!(finding.verdict, Verdict::Unknown);
    }

    #[test]
    fn degraded_on_inode_exhaustion() {
        let mut facts = fixtures::clean_facts();
        facts.fs_usage = src_ok(vec![FsUsage {
            device: "/dev/sda1".to_string(),
            mountpoint: "/".to_string(),
            bytes: Some(FsCapacity {
                total: 100,
                free: 50,
            }),
            inodes: Some(FsCapacity {
                total: 100,
                free: 4,
            }),
        }]);
        let finding = SilentKiller.evaluate(&facts);
        assert_eq!(finding.verdict, Verdict::Degraded);
        assert!(finding
            .indications
            .iter()
            .any(|i| i.contains("inode exhaustion")));
    }

    fn root(device: &str, fstype: &str, write_capable: bool, material: bool) -> Mount {
        Mount {
            device: device.to_string(),
            mountpoint: "/".to_string(),
            fstype: fstype.to_string(),
            ro: false,
            write_capable,
            material,
        }
    }

    #[test]
    fn unsupported_root_filesystem_cannot_clear() {
        for (device, fstype) in [
            ("rpool/ROOT/ubuntu", "zfs"),
            ("overlay", "overlay"),
            ("10.0.0.5:/export/root", "nfs4"),
        ] {
            let mut facts = fixtures::clean_facts();
            facts.mounts = src_ok(vec![
                root(device, fstype, false, false),
                Mount {
                    device: "shm".to_string(),
                    mountpoint: "/dev/shm".to_string(),
                    fstype: "tmpfs".to_string(),
                    ro: false,
                    write_capable: false,
                    material: true,
                },
            ]);
            facts.fs_usage = src_ok(vec![]);
            let finding = SilentKiller.evaluate(&facts);
            assert_eq!(finding.verdict, Verdict::Unknown, "{}", fstype);
            assert!(finding.summary.contains(fstype), "{}", finding.summary);
            assert!(finding.summary.contains("outside the mount model"));
            assert!(!finding.evidence.iter().any(|e| e.contains("read-write")));
        }
    }

    #[test]
    fn root_missing_from_mount_table_cannot_clear() {
        let mut facts = fixtures::clean_facts();
        facts.mounts = src_ok(vec![Mount {
            device: "tmpfs".to_string(),
            mountpoint: "/tmp".to_string(),
            fstype: "tmpfs".to_string(),
            ro: false,
            write_capable: false,
            material: true,
        }]);
        let finding = SilentKiller.evaluate(&facts);
        assert_eq!(finding.verdict, Verdict::Unknown);
        assert!(finding.summary.contains("root filesystem missing"));
    }

    #[test]
    fn malformed_root_mount_evidence_cannot_clear() {
        let mut facts = fixtures::clean_facts();
        facts.mounts = Source::failed(
            Status::Malformed,
            "malformed mounts line: \"/dev/sda1 / ext4\"",
            Duration::from_millis(1),
        );
        facts.fs_usage = Source::failed(
            Status::Unavailable,
            "mounts unavailable",
            Duration::from_millis(1),
        );
        let finding = SilentKiller.evaluate(&facts);
        assert_eq!(finding.verdict, Verdict::Unknown);
    }

    #[test]
    fn unsupported_root_still_degrades_on_positive_evidence() {
        let mut facts = fixtures::clean_facts();
        facts.mounts = src_ok(vec![root("overlay", "overlay", false, false)]);
        facts.fs_usage = src_ok(vec![]);
        facts.kernel_log = src_ok(KernelLog {
            lines: vec![
                "EXT4-fs error (device dm-3): ext4_find_entry: reading directory lblock 0"
                    .to_string(),
            ],
            method: "dmesg".to_string(),
        });
        let finding = SilentKiller.evaluate(&facts);
        assert_eq!(finding.verdict, Verdict::Degraded);
    }

    #[test]
    fn supported_roots_preserve_existing_behavior() {
        // ext4 root (the clean fixture) and a tmpfs root are both inside the
        // model: tmpfs cannot be remounted read-only by failure, and its
        // space/inode facts are collected like any other material mount.
        let finding = SilentKiller.evaluate(&fixtures::clean_facts());
        assert_eq!(finding.verdict, Verdict::Clear);

        let mut facts = fixtures::clean_facts();
        facts.mounts = src_ok(vec![root("rootfs", "tmpfs", false, true)]);
        let finding = SilentKiller.evaluate(&facts);
        assert_eq!(finding.verdict, Verdict::Clear);
    }

    #[test]
    fn last_root_row_wins_when_root_is_over_mounted() {
        let mut facts = fixtures::clean_facts();
        facts.mounts = src_ok(vec![
            root("rootfs", "rootfs", false, false),
            root("/dev/sda1", "ext4", true, true),
        ]);
        let finding = SilentKiller.evaluate(&facts);
        assert_eq!(finding.verdict, Verdict::Clear);
    }

    #[test]
    fn degraded_on_xfs_forced_shutdown_log_line() {
        let mut facts = fixtures::clean_facts();
        facts.kernel_log = src_ok(KernelLog {
            lines: vec![
                "XFS (sdb1): Log I/O Error (0x2) detected at xlog_ioend_work+0x7d/0x90 (fs/xfs/xfs_log.c:1378).  Shutting down filesystem.".to_string(),
            ],
            method: "dmesg".to_string(),
        });
        let finding = SilentKiller.evaluate(&facts);
        assert_eq!(finding.verdict, Verdict::Degraded);
        assert!(finding
            .indications
            .iter()
            .any(|i| i.contains("XFS forced shutdown")));
    }

    #[test]
    fn degraded_on_remount_read_only_log_line() {
        let mut facts = fixtures::clean_facts();
        facts.kernel_log = src_ok(KernelLog {
            lines: vec!["EXT4-fs (sda1): Remounting filesystem read-only".to_string()],
            method: "dmesg".to_string(),
        });
        let finding = SilentKiller.evaluate(&facts);
        assert_eq!(finding.verdict, Verdict::Degraded);
    }
}
