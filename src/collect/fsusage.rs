use crate::collect::{run_command, Mount, Source, Status};
use std::time::{Duration, Instant};

const DF_TIMEOUT: Duration = Duration::from_secs(3);

#[derive(Debug, Clone)]
pub struct FsUsage {
    pub device: String,
    pub mountpoint: String,
    pub total_bytes: u64,
    pub free_bytes: u64,
    pub total_inodes: u64,
    pub free_inodes: u64,
}

pub fn collect(mounts: &Source<Vec<Mount>>) -> Source<Vec<FsUsage>> {
    let start = Instant::now();
    let mounts = match mounts.get() {
        Some(m) => m,
        None => {
            return Source::failed(Status::Unavailable, "mounts unavailable", start.elapsed());
        }
    };
    let mut usages = Vec::new();
    for (device, mountpoint) in dedupe_filesystems(mounts) {
        let bytes_out = match run_command("df", &["-P", "-B1", "--", &mountpoint], DF_TIMEOUT) {
            Ok(o) => o,
            Err(e) => {
                return Source::failed(
                    Status::Unavailable,
                    format!("{}: {}", mountpoint, e),
                    start.elapsed(),
                );
            }
        };
        let inodes_out = match run_command("df", &["-P", "-i", "--", &mountpoint], DF_TIMEOUT) {
            Ok(o) => o,
            Err(e) => {
                return Source::failed(
                    Status::Unavailable,
                    format!("{}: {}", mountpoint, e),
                    start.elapsed(),
                );
            }
        };
        let (total_bytes, free_bytes) = match parse_df_bytes(&bytes_out) {
            Ok(v) => v,
            Err(e) => {
                return Source::failed(
                    Status::Malformed,
                    format!("{}: {}", mountpoint, e),
                    start.elapsed(),
                );
            }
        };
        let (total_inodes, free_inodes) = match parse_df_inodes(&inodes_out) {
            Ok(v) => v,
            Err(e) => {
                return Source::failed(
                    Status::Malformed,
                    format!("{}: {}", mountpoint, e),
                    start.elapsed(),
                );
            }
        };
        usages.push(FsUsage {
            device,
            mountpoint,
            total_bytes,
            free_bytes,
            total_inodes,
            free_inodes,
        });
    }
    Source::ok(
        usages,
        "df -P -B1 / df -P -i per material filesystem",
        start.elapsed(),
    )
}

pub fn dedupe_filesystems(mounts: &[Mount]) -> Vec<(String, String)> {
    let mut seen: Vec<String> = Vec::new();
    let mut filesystems = Vec::new();
    for m in mounts {
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
    lines
        .find(|l| !l.trim().is_empty())
        .ok_or_else(|| "df produced no data line".to_string())
}

fn parse_u64(field: &str, line: &str) -> Result<u64, String> {
    field
        .parse()
        .map_err(|_| format!("malformed df line: {:?}", line))
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert!(parse_df_bytes(
            "Filesystem 1B-blocks Used Available Capacity Mounted on\n"
        )
        .is_err());
        assert!(parse_df_inodes("garbage\n").is_err());
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
        assert_eq!(
            filesystems[0],
            ("/dev/sda1".to_string(), "/".to_string())
        );
        assert_eq!(
            filesystems[1],
            ("tmpfs".to_string(), "/tmp".to_string())
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
