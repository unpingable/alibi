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
        let mountpoint = checked_unescape(fields[1])?;
        let device = checked_unescape(fields[0])?;
        let ro = fields[3].split(',').any(|opt| opt == "ro");
        let rw = fields[3].split(',').any(|opt| opt == "rw");
        if ro == rw {
            return Err(format!(
                "mount access mode is missing or contradictory: {:?}",
                line
            ));
        }
        // The root row is always retained, even when its filesystem type is
        // outside the model, so evaluators can tell "root is a type we can
        // reason about" from "root is something else" (or missing entirely).
        // Non-root, non-material rows are dropped and only counted.
        if !material && mountpoint != "/" {
            excluded += 1;
            continue;
        }
        mounts.push(Mount {
            device,
            mountpoint,
            fstype,
            ro,
            write_capable,
            material,
        });
    }
    Ok((mounts, excluded))
}

pub fn unescape(s: &str) -> String {
    checked_unescape(s).unwrap_or_else(|_| s.to_string())
}

fn checked_unescape(s: &str) -> Result<String, String> {
    let bytes = s.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'\\' {
            let digits = bytes
                .get(i + 1..i + 4)
                .filter(|digits| digits.iter().all(|byte| (b'0'..=b'7').contains(byte)))
                .ok_or_else(|| format!("malformed mount escape: {:?}", s))?;
            let value = digits
                .iter()
                .fold(0u16, |value, digit| value * 8 + u16::from(digit - b'0'));
            if value > 255 || value == 0 {
                return Err(format!("invalid mount escape: {:?}", s));
            }
            out.push(value as u8);
            i += 4;
            continue;
        }
        out.push(bytes[i]);
        i += 1;
    }
    String::from_utf8(out).map_err(|_| format!("non-UTF-8 decoded mount name: {:?}", s))
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

    #[test]
    fn mount_mode_must_be_explicit_and_unambiguous() {
        for options in ["nonsense", "ro,rw", "errors=remount-ro"] {
            assert!(parse_mounts(&format!("/dev/vda / ext4 {} 0 0\n", options)).is_err());
        }
    }

    #[test]
    fn malformed_escapes_fail_without_utf8_boundary_panics() {
        for name in ["/\\éé", "/\\12", "/\\777", "/\\000", "/\\377"] {
            assert!(parse_mounts(&format!("/dev/vda {} ext4 rw 0 0\n", name)).is_err());
        }
        let (mounts, _) = parse_mounts("/dev/vda /café\\040data ext4 rw 0 0\n").unwrap();
        assert_eq!(mounts[0].mountpoint, "/café data");
    }

    #[test]
    fn unsupported_root_row_is_retained_as_non_material() {
        for (table, fstype) in [
            (
                "rpool/ROOT/ubuntu / zfs rw,relatime,xattr,posixacl 0 0\nproc /proc proc rw 0 0\n",
                "zfs",
            ),
            (
                "overlay / overlay rw,relatime,lowerdir=/l,upperdir=/u,workdir=/w 0 0\nproc /proc proc rw 0 0\nshm /dev/shm tmpfs rw 0 0\n",
                "overlay",
            ),
            (
                "10.0.0.5:/export/root / nfs4 rw,relatime,vers=4.2 0 0\nproc /proc proc rw 0 0\n",
                "nfs4",
            ),
        ] {
            let (mounts, excluded) = parse_mounts(table).unwrap();
            let root = mounts
                .iter()
                .find(|m| m.mountpoint == "/")
                .unwrap_or_else(|| panic!("{} root row dropped", fstype));
            assert_eq!(root.fstype, fstype);
            assert!(!root.material);
            assert!(!root.write_capable);
            assert_eq!(excluded, 1, "{}: only /proc is excluded", fstype);
        }
    }

    #[test]
    fn malformed_root_row_fails_whole_parse() {
        assert!(parse_mounts("/dev/sda1 / ext4\nproc /proc proc rw 0 0\n").is_err());
    }
}
