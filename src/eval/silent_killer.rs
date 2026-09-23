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
                    finding.indications.push(format!("{}: {}", label, line));
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
                    "mounted filesystems writable; no journal/fs errors found".to_string();
                let rw = mounts.iter().filter(|m| !m.ro).count();
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
            "mounted filesystems writable; no journal/fs errors found"
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
