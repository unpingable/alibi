use crate::collect::{run_command, Mount, Source, Status};
use std::time::{Duration, Instant};

const DF_TIMEOUT: Duration = Duration::from_secs(3);
const DF_METHOD: &str = "df -P -B1 / df -P -i per material filesystem";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FsCapacity {
    pub total: u64,
    pub free: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FsUsage {
    pub device: String,
    pub mountpoint: String,
    pub bytes: Option<FsCapacity>,
    pub inodes: Option<FsCapacity>,
}

#[derive(Debug)]
struct DfOutputs {
    device: String,
    mountpoint: String,
    bytes: Result<String, String>,
    inodes: Result<String, String>,
}

pub fn collect(mounts: &Source<Vec<Mount>>) -> Source<Vec<FsUsage>> {
    let start = Instant::now();
    let mounts = match mounts.get() {
        Some(m) => m,
        None => {
            return Source::failed(Status::Unavailable, "mounts unavailable", start.elapsed());
        }
    };
    let outputs = dedupe_filesystems(mounts)
        .into_iter()
        .map(|(device, mountpoint)| {
            let bytes = run_command("df", &["-P", "-B1", "--", &mountpoint], DF_TIMEOUT);
            let inodes = run_command("df", &["-P", "-i", "--", &mountpoint], DF_TIMEOUT);
            DfOutputs {
                device,
                mountpoint,
                bytes,
                inodes,
            }
        })
        .collect();

    assemble(outputs, start.elapsed())
}

fn assemble(outputs: Vec<DfOutputs>, elapsed: Duration) -> Source<Vec<FsUsage>> {
    let mut usages = Vec::new();
    let mut failures = Vec::new();

    for output in outputs {
        let bytes = parse_dimension(
            output.bytes,
            &output.device,
            &output.mountpoint,
            "byte usage",
            &mut failures,
        );
        let inodes = parse_dimension(
            output.inodes,
            &output.device,
            &output.mountpoint,
            "inode usage",
            &mut failures,
        );

        if bytes.is_some() || inodes.is_some() {
            usages.push(FsUsage {
                device: output.device,
                mountpoint: output.mountpoint,
                bytes,
                inodes,
            });
        }
    }

    if failures.is_empty() {
        Source::ok(usages, DF_METHOD, elapsed)
    } else {
        let status = failures[0].0;
        let detail = failures
            .into_iter()
            .map(|(_, detail)| detail)
            .collect::<Vec<_>>()
            .join("; ");
        if usages.is_empty() {
            Source::failed(status, detail, elapsed)
        } else {
            Source::partial(usages, status, detail, DF_METHOD, elapsed)
        }
    }
}

fn parse_dimension(
    output: Result<String, String>,
    device: &str,
    mountpoint: &str,
    dimension: &str,
    failures: &mut Vec<(Status, String)>,
) -> Option<FsCapacity> {
    match output {
        Ok(output) => match parse_df_for_mount(&output, device, mountpoint) {
            Ok((total, free)) => Some(FsCapacity { total, free }),
            Err(error) => {
                failures.push((
                    Status::Malformed,
                    format!("{} {}: {}", mountpoint, dimension, error),
                ));
                None
            }
        },
        Err(error) => {
            failures.push((
                Status::Unavailable,
                format!("{} {}: {}", mountpoint, dimension, error),
            ));
            None
        }
    }
}

fn parse_df_for_mount(output: &str, device: &str, mountpoint: &str) -> Result<(u64, u64), String> {
    let line = data_line(output)?;
    let mismatch = || {
        format!(
            "df identity mismatch: expected source {:?} mounted on {:?}; observed row {:?}",
            device, mountpoint, line
        )
    };
    // Match the exact snapshot names around the four capacity columns. This
    // supports literal spaces without guessing their token boundaries. A
    // different source or target (including an unmounted path's parent) cannot
    // be attached to the earlier mount merely because df exited successfully.
    let middle = line
        .strip_prefix(device)
        .filter(|rest| rest.starts_with(char::is_whitespace))
        .and_then(|rest| rest.strip_suffix(mountpoint))
        .filter(|rest| rest.ends_with(char::is_whitespace))
        .ok_or_else(mismatch)?;
    let fields: Vec<&str> = middle.split_whitespace().collect();
    if fields.len() != 4 {
        return Err(format!(
            "df identity/capacity columns are ambiguous: {:?}",
            line
        ));
    }
    let total = parse_u64(fields[0], line)?;
    let _used = parse_u64(fields[1], line)?;
    let free = parse_u64(fields[2], line)?;
    if fields[3] != "-"
        && fields[3]
            .strip_suffix('%')
            .is_none_or(|value| value.is_empty() || !value.bytes().all(|b| b.is_ascii_digit()))
    {
        return Err(format!("malformed df capacity percentage: {:?}", line));
    }
    if free > total {
        return Err(format!("df available capacity exceeds total: {:?}", line));
    }
    Ok((total, free))
}

pub fn dedupe_filesystems(mounts: &[Mount]) -> Vec<(String, String)> {
    let mut seen: Vec<String> = Vec::new();
    let mut filesystems = Vec::new();
    for m in mounts.iter().filter(|m| m.material) {
        let key = if m.device.starts_with("/dev/") {
            &m.device
        } else {
            &m.mountpoint
        };
        if seen.contains(key) {
            continue;
        }
        seen.push(key.clone());
        filesystems.push((m.device.clone(), m.mountpoint.clone()));
    }
    filesystems
}

pub fn parse_df_bytes(output: &str) -> Result<(u64, u64), String> {
    let line = data_line(output)?;
    let fields: Vec<&str> = line.split_whitespace().collect();
    if fields.len() < 6 {
        return Err(format!("malformed df line: {:?}", line));
    }
    let total = parse_u64(fields[1], line)?;
    let avail = parse_u64(fields[3], line)?;
    if avail > total {
        return Err(format!("df available capacity exceeds total: {:?}", line));
    }
    Ok((total, avail))
}

pub fn parse_df_inodes(output: &str) -> Result<(u64, u64), String> {
    let line = data_line(output)?;
    let fields: Vec<&str> = line.split_whitespace().collect();
    if fields.len() < 6 {
        return Err(format!("malformed df line: {:?}", line));
    }
    let total = parse_u64(fields[1], line)?;
    let free = parse_u64(fields[3], line)?;
    if free > total {
        return Err(format!("df free inodes exceed total: {:?}", line));
    }
    Ok((total, free))
}

fn data_line(output: &str) -> Result<&str, String> {
    let mut lines = output.lines();
    let header = lines
        .next()
        .ok_or_else(|| "df produced no output".to_string())?;
    if !header.starts_with("Filesystem") {
        return Err(format!("unexpected df header: {:?}", header));
    }
    let mut rows = lines.filter(|line| !line.trim().is_empty());
    let row = rows
        .next()
        .ok_or_else(|| "df produced no data line".to_string())?;
    if rows.next().is_some() {
        return Err("df produced multiple data rows for one requested mount".to_string());
    }
    Ok(row)
}

fn parse_u64(field: &str, line: &str) -> Result<u64, String> {
    field
        .parse()
        .map_err(|_| format!("malformed df line: {:?}", line))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collect::Completeness;

    const DF_BYTES: &str = "\
Filesystem                        1B-blocks        Used    Available Capacity Mounted on
/dev/mapper/vgubuntu-root     490946592768 40515952640 42527576064       9% /
";

    const DF_INODES: &str = "\
Filesystem                         Inodes   IUsed     IFree IUse% Mounted on
/dev/mapper/vgubuntu-root       30531584  654321  29877263    3% /
";

    #[test]
    fn parses_single_mount_df_output() {
        let (total, avail) = parse_df_bytes(DF_BYTES).unwrap();
        assert_eq!(total, 490946592768);
        assert_eq!(avail, 42527576064);
        let (total_inodes, free_inodes) = parse_df_inodes(DF_INODES).unwrap();
        assert_eq!(total_inodes, 30531584);
        assert_eq!(free_inodes, 29877263);
    }

    #[test]
    fn garbage_output_is_malformed() {
        assert!(parse_df_bytes("not df output at all\n").is_err());
        assert!(parse_df_bytes(
            "Filesystem 1B-blocks Used Available Capacity Mounted on\ncomplete junk\n"
        )
        .is_err());
        assert!(
            parse_df_bytes("Filesystem 1B-blocks Used Available Capacity Mounted on\n").is_err()
        );
        assert!(parse_df_inodes("garbage\n").is_err());
    }

    #[test]
    fn disappearing_mount_cannot_acquire_parent_capacity_as_complete() {
        // G35: the requested ext4 loop mount is removed before real df runs;
        // df successfully reports its now-containing parent tmpfs instead.
        let source = assemble(
            vec![outputs(
                "/dev/loop0",
                "/run/alibi-matrix/mnt",
                Ok("Filesystem 1B-blocks Used Available Capacity Mounted on\nguest-root 329175040 23298048 305876992 8% /\n".to_string()),
                Ok("Filesystem Inodes IUsed IFree IUse% Mounted on\nguest-root 57403 366 57037 1% /\n".to_string()),
            )], Duration::ZERO,
        );
        assert!(!source.is_complete());
        assert!(source.value.is_none());
        assert!(source.failure_reason().contains("identity mismatch"));
    }

    #[test]
    fn df_identity_requires_both_exact_names_and_one_unambiguous_row() {
        let normal = bytes_output("/dev/loop0", 100, 70, "/mnt/data");
        assert_eq!(
            parse_df_for_mount(&normal, "/dev/loop0", "/mnt/data"),
            Ok((100, 70))
        );
        for output in [
            bytes_output("/dev/loop0", 100, 70, "/"),
            bytes_output("/dev/alias", 100, 70, "/mnt/data"),
            bytes_output("/dev/loop00", 100, 70, "/mnt/data"),
            bytes_output("/dev/loop0", 100, 70, "/other/mnt/data"),
            normal.replace(" 100 0 70 1% ", " 100 0 70 1% extra "),
            format!("{}{}", normal, "/dev/loop0 100 0 70 1% /mnt/data\n"),
        ] {
            assert!(
                parse_df_for_mount(&output, "/dev/loop0", "/mnt/data").is_err(),
                "{output}"
            );
        }
    }

    #[test]
    fn literal_spaces_in_requested_names_are_preserved() {
        let output = bytes_output("volume with spaces", 100, 70, "/mnt/my data");
        assert_eq!(
            parse_df_for_mount(&output, "volume with spaces", "/mnt/my data"),
            Ok((100, 70))
        );
    }

    #[test]
    fn dimension_local_identity_loss_retains_supported_positive() {
        use crate::eval::{Evaluator, Verdict};
        let source = assemble(
            vec![outputs(
                "/dev/loop0",
                "/mnt/data",
                Ok(bytes_output("/dev/loop0", 100, 2, "/mnt/data")),
                Ok(inodes_output("guest-root", 100, 90, "/")),
            )],
            Duration::ZERO,
        );
        assert_eq!(source.completeness, Completeness::Partial);
        assert_eq!(
            source.get().unwrap()[0].bytes,
            Some(FsCapacity {
                total: 100,
                free: 2
            })
        );
        assert_eq!(source.get().unwrap()[0].inodes, None);
        let mut facts = crate::eval::fixtures::clean_facts();
        facts.fs_usage = source;
        let finding = crate::eval::silent_killer::SilentKiller.evaluate(&facts);
        assert_eq!(finding.verdict, Verdict::Degraded);
        assert!(finding
            .skipped
            .iter()
            .any(|s| s.contains("identity mismatch")));

        facts.fs_usage = assemble(
            vec![outputs(
                "/dev/loop0",
                "/mnt/data",
                Ok(bytes_output("/dev/loop0", 100, 70, "/mnt/data")),
                Ok(inodes_output("guest-root", 100, 90, "/")),
            )],
            Duration::ZERO,
        );
        assert_eq!(
            crate::eval::silent_killer::SilentKiller
                .evaluate(&facts)
                .verdict,
            Verdict::Unknown
        );
    }

    #[test]
    fn contradictory_capacity_is_not_evidence_of_free_space() {
        let output =
            "Filesystem 1B-blocks Used Available Capacity Mounted on\n/dev/test 100 0 101 0% /\n";
        assert!(parse_df_bytes(output).is_err());
        assert!(parse_df_inodes(output).is_err());
        let source = assemble(
            vec![outputs(
                "/dev/test",
                "/",
                Ok(output.into()),
                Ok(output.into()),
            )],
            Duration::ZERO,
        );
        assert_eq!(source.status, Status::Malformed);
        assert!(source.value.is_none());
    }

    fn bytes_output(device: &str, total: u64, free: u64, mountpoint: &str) -> String {
        format!(
            "Filesystem 1B-blocks Used Available Capacity Mounted on\n{} {} 0 {} 1% {}\n",
            device, total, free, mountpoint
        )
    }

    fn inodes_output(device: &str, total: u64, free: u64, mountpoint: &str) -> String {
        format!(
            "Filesystem Inodes IUsed IFree IUse% Mounted on\n{} {} 0 {} 1% {}\n",
            device, total, free, mountpoint
        )
    }

    fn outputs(
        device: &str,
        mountpoint: &str,
        bytes: Result<String, String>,
        inodes: Result<String, String>,
    ) -> DfOutputs {
        DfOutputs {
            device: device.to_string(),
            mountpoint: mountpoint.to_string(),
            bytes,
            inodes,
        }
    }

    #[test]
    fn assembly_preserves_bytes_when_inode_collection_or_parsing_fails() {
        let inode_failures = [
            Err("df timed out".to_string()),
            Ok("not df output".to_string()),
        ];

        for inode_failure in inode_failures {
            let source = assemble(
                vec![outputs(
                    "/dev/sda1",
                    "/",
                    Ok(bytes_output("/dev/sda1", 100, 3, "/")),
                    inode_failure,
                )],
                Duration::ZERO,
            );

            assert_eq!(source.completeness, Completeness::Partial);
            let usage = &source.value.as_ref().unwrap()[0];
            assert_eq!(
                usage.bytes,
                Some(FsCapacity {
                    total: 100,
                    free: 3
                })
            );
            assert_eq!(usage.inodes, None);
        }
    }

    #[test]
    fn assembly_continues_after_a_mount_fails_completely() {
        let source = assemble(
            vec![
                outputs(
                    "/dev/sda1",
                    "/broken",
                    Err("byte df failed".to_string()),
                    Err("inode df failed".to_string()),
                ),
                outputs(
                    "/dev/sdb1",
                    "/later",
                    Ok(bytes_output("/dev/sdb1", 200, 4, "/later")),
                    Ok(inodes_output("/dev/sdb1", 300, 250, "/later")),
                ),
            ],
            Duration::ZERO,
        );

        assert_eq!(source.completeness, Completeness::Partial);
        let usages = source.value.as_ref().unwrap();
        assert_eq!(usages.len(), 1);
        assert_eq!(usages[0].mountpoint, "/later");
        assert_eq!(
            usages[0].bytes,
            Some(FsCapacity {
                total: 200,
                free: 4,
            })
        );
        assert_eq!(
            usages[0].inodes,
            Some(FsCapacity {
                total: 300,
                free: 250,
            })
        );
    }

    fn mount(device: &str, mountpoint: &str, fstype: &str, ro: bool) -> Mount {
        Mount {
            device: device.to_string(),
            mountpoint: mountpoint.to_string(),
            fstype: fstype.to_string(),
            ro,
            write_capable: true,
            material: true,
        }
    }

    #[test]
    fn bind_mounts_of_one_device_dedupe_to_one_filesystem() {
        let mounts = vec![
            mount("/dev/sda1", "/", "ext4", false),
            mount("/dev/sda1", "/snap/x", "ext4", true),
            Mount {
                write_capable: false,
                ..mount("tmpfs", "/tmp", "tmpfs", false)
            },
        ];
        let filesystems = dedupe_filesystems(&mounts);
        assert_eq!(filesystems.len(), 2);
        assert_eq!(filesystems[0], ("/dev/sda1".to_string(), "/".to_string()));
        assert_eq!(filesystems[1], ("tmpfs".to_string(), "/tmp".to_string()));
    }

    #[test]
    fn non_material_root_row_is_not_a_df_target() {
        let mounts = vec![
            Mount {
                write_capable: false,
                material: false,
                ..mount("overlay", "/", "overlay", false)
            },
            Mount {
                write_capable: false,
                ..mount("shm", "/dev/shm", "tmpfs", false)
            },
        ];
        let filesystems = dedupe_filesystems(&mounts);
        assert_eq!(
            filesystems,
            vec![("shm".to_string(), "/dev/shm".to_string())]
        );
    }

    #[test]
    fn distinct_tmpfs_mounts_are_distinct_filesystems() {
        let mounts = vec![
            Mount {
                write_capable: false,
                ..mount("tmpfs", "/run", "tmpfs", false)
            },
            Mount {
                write_capable: false,
                ..mount("tmpfs", "/dev/shm", "tmpfs", false)
            },
        ];
        assert_eq!(dedupe_filesystems(&mounts).len(), 2);
    }
}
