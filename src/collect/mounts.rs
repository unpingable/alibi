use crate::collect::{Source, Status};
use std::fs;
use std::time::Instant;

const PROC_SELF_MOUNTS: &str = "/proc/self/mounts";
const WRITE_CAPABLE_FSTYPES: &[&str] = &[
    "ext2", "ext3", "ext4", "xfs", "btrfs", "f2fs", "jfs", "reiserfs",
];

#[derive(Debug, Clone)]
pub struct Mount {
    pub device: String,
    pub mountpoint: String,
    pub fstype: String,
    pub ro: bool,
    pub write_capable: bool,
    pub material: bool,
}

pub fn collect() -> Source<Vec<Mount>> {
    let start = Instant::now();
    let content = match fs::read_to_string(PROC_SELF_MOUNTS) {
        Ok(c) => c,
        Err(e) => {
            return Source::failed(
                Status::Unavailable,
                format!("{}: {}", PROC_SELF_MOUNTS, e),
                start.elapsed(),
            );
        }
    };
    match parse_mounts(&content) {
        Ok((mounts, excluded)) => {
            let mut source = Source::ok(mounts, PROC_SELF_MOUNTS, start.elapsed());
            source.detail = Some(format!("{} non-material mounts excluded", excluded));
            source
        }
        Err(e) => Source::failed(Status::Malformed, e, start.elapsed()),
    }
}

pub fn parse_mounts(content: &str) -> Result<(Vec<Mount>, usize), String> {
    let mut mounts = Vec::new();
    let mut excluded = 0;
    for line in content.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let fields: Vec<&str> = line.split_whitespace().collect();
        if fields.len() < 6 {
            return Err(format!("malformed mounts line: {:?}", line));
        }
        let fstype = fields[2].to_string();
        let write_capable = WRITE_CAPABLE_FSTYPES.contains(&fstype.as_str());
        let material = write_capable || fstype == "tmpfs";
        if !material {
            excluded += 1;
            continue;
        }
        let ro = fields[3].split(',').any(|opt| opt == "ro");
        mounts.push(Mount {
            device: unescape(fields[0]),
            mountpoint: unescape(fields[1]),
            fstype,
            ro,
            write_capable,
            material,
        });
    }
    Ok((mounts, excluded))
}

pub fn unescape(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'\\' && i + 3 < bytes.len() {
            if let Ok(v) = u32::from_str_radix(&s[i + 1..i + 4], 8) {
                if v <= 0xff {
                    out.push(v as u8);
                    i += 4;
                    continue;
                }
            }
        }
        out.push(bytes[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "\
proc /proc proc rw,nosuid,nodev,noexec,relatime 0 0
/dev/sda1 / ext4 rw,relatime,errors=remount-ro 0 0
/dev/sdb1 /data xfs ro,relatime 0 0
/dev/sdc1 /mnt/my\\040data ext4 rw,relatime 0 0
tmpfs /run tmpfs rw,nosuid,nodev 0 0
/dev/loop0 /snap/core squashfs ro,nodev,relatime 0 0
";

    #[test]
    fn keeps_only_material_mounts() {
        let (mounts, excluded) = parse_mounts(SAMPLE).unwrap();
        assert!(mounts.iter().all(|m| m.material));
        assert!(mounts.iter().any(|m| m.mountpoint == "/"));
        assert!(mounts.iter().any(|m| m.mountpoint == "/data"));
        assert!(mounts.iter().any(|m| m.mountpoint == "/run"));
        assert!(!mounts.iter().any(|m| m.mountpoint == "/proc"));
        assert!(!mounts.iter().any(|m| m.fstype == "squashfs"));
        assert_eq!(excluded, 2);
    }

    #[test]
    fn squashfs_read_only_row_yields_no_mount_entry() {
        let (mounts, _) = parse_mounts(SAMPLE).unwrap();
        assert!(!mounts.iter().any(|m| m.mountpoint == "/snap/core"));
    }

    #[test]
    fn unescapes_octal_escaped_mountpoints() {
        let (mounts, _) = parse_mounts(SAMPLE).unwrap();
        assert!(mounts.iter().any(|m| m.mountpoint == "/mnt/my data"));
    }

    #[test]
    fn detects_exact_ro_option_token() {
        let (mounts, _) = parse_mounts(SAMPLE).unwrap();
        let data = mounts.iter().find(|m| m.mountpoint == "/data").unwrap();
        assert!(data.ro);
        let root = mounts.iter().find(|m| m.mountpoint == "/").unwrap();
        assert!(!root.ro);
    }

    #[test]
    fn malformed_line_fails_whole_parse() {
        assert!(parse_mounts("garbage line\n").is_err());
    }
}
