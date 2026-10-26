use crate::collect::{Source, Status};
use std::fs::{self, File};
use std::io::{ErrorKind, Read, Seek, SeekFrom};
use std::os::fd::AsRawFd;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

pub const NET_SAMPLE_WINDOW: Duration = Duration::from_secs(2);
const SYS_CLASS_NET: &str = "/sys/class/net";
const CONNTRACK_DIR: &str = "/proc/sys/net/netfilter";

#[derive(Debug)]
pub struct NetFacts {
    pub interfaces: Vec<Iface>,
    pub sample_window: Duration,
}

#[derive(Debug, Clone)]
pub struct Iface {
    pub name: String,
    pub is_virtual: bool,
    pub operstate: String,
    pub carrier: Option<bool>,
    pub first: Counters,
    pub second: Counters,
}

#[derive(Debug, Clone, Copy)]
pub struct Counters {
    pub rx_errors: Option<u64>,
    pub rx_dropped: Option<u64>,
    pub tx_errors: Option<u64>,
    pub tx_dropped: Option<u64>,
    pub rx_packets: Option<u64>,
    pub tx_packets: Option<u64>,
}

#[derive(Debug, Clone, Copy)]
pub struct RouteFacts {
    pub v4_default: bool,
    pub v6_default: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct Conntrack {
    pub count: u64,
    pub max: u64,
}

pub fn collect_net(t0: Instant) -> Source<NetFacts> {
    let start = Instant::now();
    let (names, mut issues) = match interface_names() {
        Ok(result) => result,
        Err(error) => {
            return Source::failed(Status::Unavailable, error, start.elapsed());
        }
    };
    let mut issue_status = Status::Unavailable;
    let sample_started_at = t0.elapsed();
    // Each interface directory and every counter descriptor stay open across
    // both reads. A pathname, MAC address or ifindex is not a generation token.
    let mut pinned: Vec<_> = names
        .iter()
        .map(|name| {
            PinnedInterface::open(Path::new(SYS_CLASS_NET), name)
                .map_err(|detail| issues.push(detail))
                .ok()
        })
        .collect();
    let first: Vec<Counters> = pinned
        .iter_mut()
        .map(|interface| {
            let (counters, counter_issues) = match interface {
                Some(interface) => interface.read_counters(),
                None => (Counters::missing(), Vec::new()),
            };
            add_counter_issues(&mut issues, &mut issue_status, counter_issues);
            counters
        })
        .collect();
    std::thread::sleep(NET_SAMPLE_WINDOW);
    match interface_names() {
        Ok((second_names, second_issues)) => {
            issues.extend(second_issues);
            if let Some(detail) = topology_change_detail(&names, &second_names) {
                issues.push(detail);
            }
        }
        Err(error) => issues.push(error),
    }
    let interfaces: Vec<Iface> = names
        .iter()
        .zip(first)
        .zip(pinned.iter_mut())
        .map(|((name, first), interface)| {
            let (second, counter_issues) = match interface {
                Some(interface) => {
                    if let Err(detail) = interface.check_name() {
                        issues.push(detail);
                    }
                    interface.read_counters()
                }
                None => (Counters::missing(), Vec::new()),
            };
            add_counter_issues(&mut issues, &mut issue_status, counter_issues);
            for (label, before, after) in required_counter_pairs(first, second) {
                if let (Some(before), Some(after)) = (before, after) {
                    if after < before {
                        issue_status = Status::Malformed;
                        issues.push(format!(
                            "{}: {} counter decreased from {} to {} (reset, replacement, or wrap)",
                            name, label, before, after
                        ));
                    }
                }
            }
            Iface {
                is_virtual: interface.as_ref().is_some_and(|i| i.is_virtual),
                operstate: interface
                    .as_ref()
                    .and_then(|i| i.read_attribute("operstate"))
                    .unwrap_or_else(|| "unknown".to_string()),
                carrier: interface.as_ref().and_then(|i| {
                    match i.read_attribute("carrier")?.as_str() {
                        "1" => Some(true),
                        "0" => Some(false),
                        _ => None,
                    }
                }),
                second,
                name: name.clone(),
                first,
            }
        })
        .collect();
    let sample_ended_at = t0.elapsed();
    let facts = NetFacts {
        interfaces,
        sample_window: sample_ended_at.saturating_sub(sample_started_at),
    };
    let source = if issues.is_empty() {
        Source::ok(
            facts,
            "/sys/class/net (pinned counter descriptors; seek/read twice)",
            start.elapsed(),
        )
    } else if !facts.interfaces.is_empty() {
        Source::partial(
            facts,
            issue_status,
            issues.join("; "),
            "/sys/class/net (pinned counter descriptors; seek/read twice)",
            start.elapsed(),
        )
    } else {
        Source::failed(issue_status, issues.join("; "), start.elapsed())
    };
    source.with_window(sample_started_at, sample_ended_at)
}

fn topology_change_detail(first: &[String], second: &[String]) -> Option<String> {
    (first != second).then(|| {
        format!(
            "{}: interface topology changed during sample (first={:?}, second={:?})",
            SYS_CLASS_NET, first, second
        )
    })
}

fn interface_names() -> Result<(Vec<String>, Vec<String>), String> {
    let entries =
        fs::read_dir(SYS_CLASS_NET).map_err(|error| format!("{}: {}", SYS_CLASS_NET, error))?;
    let mut names = Vec::new();
    let mut issues = Vec::new();
    for entry in entries {
        match entry {
            Ok(entry) => {
                let name = entry.file_name().to_string_lossy().into_owned();
                if name != "lo" {
                    names.push(name);
                }
            }
            Err(error) => issues.push(format!("{}: directory entry: {}", SYS_CLASS_NET, error)),
        }
    }
    names.sort();
    Ok((names, issues))
}

fn add_counter_issues(
    issues: &mut Vec<String>,
    issue_status: &mut Status,
    counter_issues: Vec<(Status, String)>,
) {
    for (status, detail) in counter_issues {
        if status == Status::Malformed {
            *issue_status = Status::Malformed;
        }
        issues.push(detail);
    }
}

fn required_counter_pairs(
    first: Counters,
    second: Counters,
) -> [(&'static str, Option<u64>, Option<u64>); 4] {
    [
        ("RX errors", first.rx_errors, second.rx_errors),
        ("TX errors", first.tx_errors, second.tx_errors),
        ("RX drops", first.rx_dropped, second.rx_dropped),
        ("TX drops", first.tx_dropped, second.tx_dropped),
    ]
}

impl Counters {
    fn missing() -> Self {
        Self {
            rx_errors: None,
            rx_dropped: None,
            tx_errors: None,
            tx_dropped: None,
            rx_packets: None,
            tx_packets: None,
        }
    }
}

const COUNTERS: [&str; 6] = [
    "rx_errors",
    "rx_dropped",
    "tx_errors",
    "tx_dropped",
    "rx_packets",
    "tx_packets",
];

struct PinnedInterface {
    directory: File,
    path: PathBuf,
    counters: Vec<Result<File, String>>,
    is_virtual: bool,
}

impl PinnedInterface {
    fn open(root: &Path, name: &str) -> Result<Self, String> {
        let path = root.join(name);
        let directory =
            File::open(&path).map_err(|error| format!("{}: {}", path.display(), error))?;
        if !directory.metadata().map_err(|e| e.to_string())?.is_dir() {
            return Err(format!("{}: not an interface directory", path.display()));
        }
        // The proc descriptor link resolves the held directory object, even if
        // the original name is replaced while its children are being opened.
        // Never fall back to reopening the interface pathname.
        let anchored = PathBuf::from(format!("/proc/self/fd/{}", directory.as_raw_fd()));
        let counters = COUNTERS
            .iter()
            .map(|stat| {
                File::open(anchored.join("statistics").join(stat))
                    .map_err(|error| error.to_string())
            })
            .collect();
        let is_virtual = fs::read_link(&anchored)
            .map(|target| target.to_string_lossy().contains("/devices/virtual/"))
            .unwrap_or(false);
        let interface = Self {
            directory,
            path,
            counters,
            is_virtual,
        };
        interface.check_name()?;
        Ok(interface)
    }

    fn anchored(&self) -> PathBuf {
        PathBuf::from(format!("/proc/self/fd/{}", self.directory.as_raw_fd()))
    }

    fn check_name(&self) -> Result<(), String> {
        let held = self.directory.metadata().map_err(|e| e.to_string())?;
        let visible = fs::metadata(&self.path).map_err(|error| {
            format!(
                "{}: interface identity unavailable: {}",
                self.path.display(),
                error
            )
        })?;
        if held.dev() != visible.dev() || held.ino() != visible.ino() {
            return Err(format!(
                "{}: interface object changed during sample",
                self.path.display()
            ));
        }
        Ok(())
    }

    fn read_attribute(&self, name: &str) -> Option<String> {
        fs::read_to_string(self.anchored().join(name))
            .ok()
            .map(|s| s.trim().to_string())
    }

    fn read_counters(&mut self) -> (Counters, Vec<(Status, String)>) {
        let mut values = [None; 6];
        let mut issues = Vec::new();
        for (index, descriptor) in self.counters.iter_mut().enumerate() {
            let value = match descriptor {
                Ok(file) => read_counter(file),
                Err(error) => Err((Status::Unavailable, error.clone())),
            };
            match value {
                Ok(value) => values[index] = Some(value),
                Err((status, error)) if index < 4 => issues.push((
                    status,
                    format!(
                        "{}/statistics/{}: pinned counter unavailable: {}",
                        self.path.display(),
                        COUNTERS[index],
                        error
                    ),
                )),
                Err(_) => {}
            }
        }
        (
            Counters {
                rx_errors: values[0],
                rx_dropped: values[1],
                tx_errors: values[2],
                tx_dropped: values[3],
                rx_packets: values[4],
                tx_packets: values[5],
            },
            issues,
        )
    }
}

fn read_counter(file: &mut File) -> Result<u64, (Status, String)> {
    // Seeking to zero forces a fresh sysfs seq-file read, rather than retaining
    // the prior userspace value or consuming an already-buffered suffix. On
    // device removal kernfs deactivation makes the old read fail; a same-name
    // successor cannot supply this descriptor's second observation.
    file.seek(SeekFrom::Start(0))
        .map_err(|e| (Status::Unavailable, e.to_string()))?;
    let mut raw = String::new();
    file.take(129)
        .read_to_string(&mut raw)
        .map_err(|e| (Status::Unavailable, e.to_string()))?;
    if raw.len() > 128 {
        return Err((Status::Malformed, "counter exceeds 128 bytes".to_string()));
    }
    raw.trim()
        .parse()
        .map_err(|_| (Status::Malformed, "not a number".to_string()))
}

pub fn collect_routes() -> Source<RouteFacts> {
    let start = Instant::now();
    let v4_read = fs::read_to_string("/proc/net/route");
    let v6_read = fs::read_to_string("/proc/net/ipv6_route");
    let mut source = routes_from_reads(
        v4_read,
        v6_read,
        || fs::read_to_string("/sys/module/ipv6/parameters/disable"),
        start.elapsed(),
    );
    source.elapsed = start.elapsed();
    source
}

fn routes_from_reads(
    v4_read: std::io::Result<String>,
    v6_read: std::io::Result<String>,
    disable: impl FnOnce() -> std::io::Result<String>,
    elapsed: Duration,
) -> Source<RouteFacts> {
    let mut issues = Vec::new();
    let mut issue_status = Status::Unavailable;
    let v4_default = match v4_read {
        Ok(content) => match parse_v4_default(&content) {
            Ok(value) => Some(value),
            Err(error) => {
                issue_status = Status::Malformed;
                issues.push(format!("/proc/net/route: {}", error));
                None
            }
        },
        Err(error) => {
            issues.push(format!("/proc/net/route: {}", error));
            None
        }
    };
    // Only an explicit kernel module disable parameter establishes that a
    // missing IPv6 table is expected. A missing file alone is not absence.
    let disabled = v6_read
        .as_ref()
        .is_err_and(|e| e.kind() == ErrorKind::NotFound)
        && disable().is_ok_and(|value| ipv6_explicitly_disabled(&value));
    let v6_default = match v6_read {
        Ok(content) => match parse_v6_default(&content) {
            Ok(value) => Some(value),
            Err(error) => {
                issue_status = Status::Malformed;
                issues.push(format!("/proc/net/ipv6_route: {}", error));
                None
            }
        },
        Err(_) if disabled => Some(false),
        Err(error) => {
            issues.push(format!("/proc/net/ipv6_route: {}", error));
            None
        }
    };
    if disabled {
        issues.push("IPv6 route table absent; /sys/module/ipv6/parameters/disable explicitly disables IPv6 (not applicable)".to_string());
    }
    let facts = RouteFacts {
        v4_default: v4_default.unwrap_or(false),
        v6_default: v6_default.unwrap_or(false),
    };
    let method = "/proc/net/route; /proc/net/ipv6_route";
    if v4_default.is_some() && v6_default.is_some() {
        let mut source = Source::ok(facts, method, elapsed);
        if !issues.is_empty() {
            source.detail = Some(issues.join("; "));
        }
        source
    } else if v4_default.is_some() || v6_default.is_some() {
        Source::partial(facts, issue_status, issues.join("; "), method, elapsed)
    } else {
        Source::failed(issue_status, issues.join("; "), elapsed)
    }
}

fn ipv6_explicitly_disabled(value: &str) -> bool {
    value.trim() == "1"
}

pub fn has_v4_default(content: &str) -> bool {
    parse_v4_default(content).unwrap_or(false)
}

pub fn has_v6_default(content: &str) -> bool {
    parse_v6_default(content).unwrap_or(false)
}

/// Route flag bit shared by `/proc/net/route` and `/proc/net/ipv6_route`:
/// the route rejects traffic (`unreachable`, `prohibit`, or the kernel's
/// IPv6 null entry). A rejecting default route is not a usable default route.
const RTF_REJECT: u32 = 0x0200;

fn parse_route_flags(field: &str) -> Option<u32> {
    if field.is_empty() || !field.bytes().all(|b| b.is_ascii_hexdigit()) {
        return None;
    }
    u32::from_str_radix(field, 16).ok()
}

pub fn parse_v4_default(content: &str) -> Result<bool, String> {
    let mut lines = content.lines();
    let header = lines
        .next()
        .ok_or_else(|| "route table is empty".to_string())?;
    if !header
        .split_whitespace()
        .any(|field| field == "Destination")
    {
        return Err("route table header is malformed".to_string());
    }
    let mut found = false;
    for line in lines.filter(|line| !line.trim().is_empty()) {
        let fields: Vec<&str> = line.split_whitespace().collect();
        if fields.len() < 8
            || fields[1].len() != 8
            || !fields[1].bytes().all(|b| b.is_ascii_hexdigit())
            || fields[7].len() != 8
            || !fields[7].bytes().all(|b| b.is_ascii_hexdigit())
        {
            return Err(format!("malformed route row: {:?}", line));
        }
        let flags = parse_route_flags(fields[3])
            .ok_or_else(|| format!("malformed route row: {:?}", line))?;
        // A default route is usable only if it does not reject traffic and has
        // an output device. The kernel prints "*" for routes with no nexthop
        // device (`blackhole`, which carries no RTF_REJECT, as well as
        // `unreachable`/`prohibit`); no unicast route lacks one.
        // A zero destination can also name 0.0.0.0/8 or 0.0.0.0/32.
        // Only a zero netmask makes the row a default route.
        found |= fields[1] == "00000000"
            && fields[7] == "00000000"
            && flags & RTF_REJECT == 0
            && fields[0] != "*";
    }
    Ok(found)
}

pub fn parse_v6_default(content: &str) -> Result<bool, String> {
    let mut found = false;
    for line in content.lines().filter(|line| !line.trim().is_empty()) {
        let fields: Vec<&str> = line.split_whitespace().collect();
        if fields.len() < 10
            || fields[0].len() != 32
            || !fields[0].bytes().all(|b| b.is_ascii_hexdigit())
            || fields[1].len() != 2
            || !fields[1].bytes().all(|b| b.is_ascii_hexdigit())
        {
            return Err(format!("malformed IPv6 route row: {:?}", line));
        }
        let flags = parse_route_flags(fields[8])
            .ok_or_else(|| format!("malformed IPv6 route row: {:?}", line))?;
        found |= fields[0] == "00000000000000000000000000000000"
            && fields[1] == "00"
            && flags & RTF_REJECT == 0;
    }
    Ok(found)
}

pub fn collect_conntrack() -> Source<Conntrack> {
    let start = Instant::now();
    let count_path = format!("{}/nf_conntrack_count", CONNTRACK_DIR);
    let max_path = format!("{}/nf_conntrack_max", CONNTRACK_DIR);
    let count_raw = match fs::read_to_string(&count_path) {
        Ok(c) => c,
        Err(e) => {
            return Source::failed(
                io_status(&e),
                format!("{}: {}", count_path, e),
                start.elapsed(),
            );
        }
    };
    // The count file was readable, so connection tracking is present; any
    // failure on the max file is a collection failure, never absence.
    let max_raw = match fs::read_to_string(&max_path) {
        Ok(c) => c,
        Err(e) => {
            return Source::failed(
                Status::Unavailable,
                format!("{}: {}", max_path, e),
                start.elapsed(),
            );
        }
    };
    let count = match count_raw.trim().parse() {
        Ok(v) => v,
        Err(_) => {
            return Source::failed(
                Status::Malformed,
                format!("{}: not a number", count_path),
                start.elapsed(),
            );
        }
    };
    let max = match max_raw.trim().parse() {
        Ok(v) => v,
        Err(_) => {
            return Source::failed(
                Status::Malformed,
                format!("{}: not a number", max_path),
                start.elapsed(),
            );
        }
    };
    Source::ok(Conntrack { count, max }, CONNTRACK_DIR, start.elapsed())
}

fn io_status(error: &std::io::Error) -> Status {
    if error.kind() == ErrorKind::NotFound {
        Status::Unsupported
    } else {
        Status::Unavailable
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ROUTE_WITH_DEFAULT: &str = "\
Iface\tDestination\tGateway\tFlags\tRefCnt\tUse\tMetric\tMask\tMTU\tWindow\tIRTT
eth0\t00000000\t0101A8C0\t0003\t0\t0\t100\t00000000\t0\t0\t0
eth0\t0001A8C0\t00000000\t0001\t0\t0\t100\t00FFFFFF\t0\t0\t0
";

    const ROUTE_WITHOUT_DEFAULT: &str = "\
Iface\tDestination\tGateway\tFlags\tRefCnt\tUse\tMetric\tMask\tMTU\tWindow\tIRTT
eth0\t0001A8C0\t00000000\t0001\t0\t0\t100\t00FFFFFF\t0\t0\t0
";

    #[test]
    fn routes_read_results_preserve_applicability_and_failures() {
        use crate::eval::{Evaluator, Verdict};
        let v4 = "Iface Destination Gateway Flags RefCnt Use Metric Mask MTU Window IRTT\neth0 00000000 0100000A 0003 0 0 0 00000000 0 0 0\n";
        let missing = || Err(std::io::Error::from(ErrorKind::NotFound));
        for (v6, disable, expected, complete) in [
            (missing(), Ok("1".into()), Verdict::Clear, true),
            (missing(), Ok("0".into()), Verdict::Unknown, false),
            (missing(), Ok("Y".into()), Verdict::Unknown, false),
            (missing(), missing(), Verdict::Unknown, false),
            (
                missing(),
                Err(std::io::Error::from(ErrorKind::PermissionDenied)),
                Verdict::Unknown,
                false,
            ),
            (
                Err(std::io::Error::from(ErrorKind::PermissionDenied)),
                Ok("1".into()),
                Verdict::Unknown,
                false,
            ),
            (
                Ok("malformed".into()),
                Ok("1".into()),
                Verdict::Unknown,
                false,
            ),
            (Ok(String::new()), Ok("1".into()), Verdict::Clear, true),
        ] {
            let mut facts = crate::eval::fixtures::clean_facts();
            facts.routes = routes_from_reads(Ok(v4.into()), v6, || disable, Duration::ZERO);
            assert_eq!(facts.routes.is_complete(), complete);
            assert!(facts.routes.get().unwrap().v4_default);
            assert_eq!(
                crate::eval::network_choke_point::NetworkChokePoint
                    .evaluate(&facts)
                    .verdict,
                expected
            );
        }
        let source = routes_from_reads(
            Err(std::io::Error::from(ErrorKind::PermissionDenied)),
            missing(),
            || Ok("1".into()),
            Duration::ZERO,
        );
        assert!(!source.is_complete());
        assert!(source.failure_reason().contains("/proc/net/route"));
        assert!(source.failure_reason().contains("explicitly disables IPv6"));
        let source = routes_from_reads(
            Ok(v4.into()),
            Ok("malformed".into()),
            || Ok("1".into()),
            Duration::ZERO,
        );
        assert_eq!(source.status, Status::Malformed);
    }

    #[test]
    fn routes_only_read_disable_when_ipv6_table_is_missing() {
        use crate::eval::{Evaluator, Verdict};
        for v6 in [
            Ok(V6_GATEWAY_DEFAULT.into()),
            Ok("malformed".into()),
            Err(std::io::Error::from(ErrorKind::PermissionDenied)),
        ] {
            let source = routes_from_reads(
                Ok(ROUTE_WITHOUT_DEFAULT.into()),
                v6,
                || panic!("must not read disable parameter for a present or denied table"),
                Duration::ZERO,
            );
            let mut facts = crate::eval::fixtures::clean_facts();
            let expected = if source.is_complete() {
                Verdict::Clear
            } else {
                Verdict::Unknown
            };
            if source.is_complete() {
                assert!(source.get().unwrap().v6_default);
            }
            facts.routes = source;
            assert_eq!(
                crate::eval::network_choke_point::NetworkChokePoint
                    .evaluate(&facts)
                    .verdict,
                expected
            );
        }
        for (v4, expected) in [
            (Ok(ROUTE_WITHOUT_DEFAULT.into()), Verdict::Degraded),
            (
                Err(std::io::Error::from(ErrorKind::NotFound)),
                Verdict::Unknown,
            ),
        ] {
            let mut facts = crate::eval::fixtures::clean_facts();
            facts.routes = routes_from_reads(
                v4,
                Err(std::io::Error::from(ErrorKind::NotFound)),
                || Ok("1".into()),
                Duration::ZERO,
            );
            assert_eq!(
                crate::eval::network_choke_point::NetworkChokePoint
                    .evaluate(&facts)
                    .verdict,
                expected
            );
        }
    }

    #[test]
    fn ipv6_disable_requires_explicit_kernel_parameter() {
        for value in ["1\n", "1"] {
            assert!(ipv6_explicitly_disabled(value));
        }
        for value in ["", "0", "N", "Y", "y", "yes", "garbage"] {
            assert!(!ipv6_explicitly_disabled(value));
        }
    }

    #[test]
    fn detects_v4_default_route() {
        assert!(has_v4_default(ROUTE_WITH_DEFAULT));
        assert!(!has_v4_default(ROUTE_WITHOUT_DEFAULT));
    }

    #[test]
    fn zero_network_prefixes_are_not_default_routes() {
        let header = "Iface Destination Gateway Flags RefCnt Use Metric Mask MTU Window IRTT\n";
        for mask in ["000000FF", "FFFFFFFF"] {
            let row = format!("eth0 00000000 00000000 0001 0 0 0 {} 0 0 0\n", mask);
            assert_eq!(parse_v4_default(&format!("{}{}", header, row)), Ok(false));
            assert_eq!(
                parse_v4_default(&format!("{}{}", ROUTE_WITH_DEFAULT, row)),
                Ok(true)
            );
        }
        assert_eq!(parse_v4_default(ROUTE_WITH_DEFAULT), Ok(true));
    }

    #[test]
    fn malformed_ipv4_masks_fail_closed_even_after_a_default() {
        for mask in ["0", "0000000g", "000000000"] {
            let row = format!("eth0 00000000 00000000 0001 0 0 0 {} 0 0 0\n", mask);
            assert!(parse_v4_default(&format!("{}{}", ROUTE_WITH_DEFAULT, row)).is_err());
        }
    }

    #[test]
    fn short_malformed_route_lines_are_skipped() {
        assert!(!has_v4_default("Iface\tDestination\nshort\n"));
    }

    // The kernel's IPv6 null entry: `::/0` via `lo`, metric ffffffff,
    // flags RTF_REJECT | RTF_NONEXTHOP. Present on every IPv6-enabled host.
    const V6_NULL_ENTRY: &str = "00000000000000000000000000000000 00 00000000000000000000000000000000 00 00000000000000000000000000000000 ffffffff 00000001 00000000 00200200 lo\n";
    // A gateway default learned by router advertisement:
    // flags RTF_UP | RTF_GATEWAY | RTF_ADDRCONF.
    const V6_GATEWAY_DEFAULT: &str = "00000000000000000000000000000000 00 00000000000000000000000000000000 00 fe80000000000000021b21fffe0a1b2c 00000400 00000001 00000000 00040003 eth0\n";
    const V6_LINK_LOCAL: &str = "fe800000000000000000000000000000 40 00000000000000000000000000000000 00 00000000000000000000000000000000 00000100 00000000 00000000 00000001 eth0\n";

    #[test]
    fn detects_v6_default_route() {
        let with_default = format!("{}{}", V6_LINK_LOCAL, V6_GATEWAY_DEFAULT);
        assert!(has_v6_default(&with_default));
        assert!(!has_v6_default(V6_LINK_LOCAL));
    }

    #[test]
    fn ipv6_null_entry_alone_is_not_a_usable_default() {
        let table = format!("{}{}{}", V6_NULL_ENTRY, V6_LINK_LOCAL, V6_NULL_ENTRY);
        assert_eq!(parse_v6_default(&table), Ok(false));
    }

    #[test]
    fn ipv6_null_entry_does_not_hide_a_real_default() {
        let table = format!("{}{}{}", V6_NULL_ENTRY, V6_GATEWAY_DEFAULT, V6_NULL_ENTRY);
        assert_eq!(parse_v6_default(&table), Ok(true));
    }

    #[test]
    fn ipv4_reject_default_is_not_a_usable_default() {
        // `ip route add unreachable default` renders as a default row with
        // RTF_UP | RTF_REJECT and no device.
        let reject_only = "\
Iface\tDestination\tGateway\tFlags\tRefCnt\tUse\tMetric\tMask\tMTU\tWindow\tIRTT
*\t00000000\t00000000\t0201\t0\t0\t0\t00000000\t0\t0\t0
eth0\t0001A8C0\t00000000\t0001\t0\t0\t100\t00FFFFFF\t0\t0\t0
";
        assert_eq!(parse_v4_default(reject_only), Ok(false));
        let reject_plus_real = format!(
            "{}*\t00000000\t00000000\t0201\t0\t0\t0\t00000000\t0\t0\t0\n",
            ROUTE_WITH_DEFAULT
        );
        assert_eq!(parse_v4_default(&reject_plus_real), Ok(true));
    }

    #[test]
    fn ipv4_blackhole_default_is_not_a_usable_default() {
        // `ip route add blackhole default` carries no RTF_REJECT but has no
        // output device, which the kernel prints as "*".
        let blackhole_only = "\
Iface\tDestination\tGateway\tFlags\tRefCnt\tUse\tMetric\tMask\tMTU\tWindow\tIRTT
*\t00000000\t00000000\t0001\t0\t0\t0\t00000000\t0\t0\t0
eth0\t0001A8C0\t00000000\t0001\t0\t0\t100\t00FFFFFF\t0\t0\t0
";
        assert_eq!(parse_v4_default(blackhole_only), Ok(false));
        let blackhole_plus_real = format!(
            "{}*\t00000000\t00000000\t0001\t0\t0\t0\t00000000\t0\t0\t0\n",
            ROUTE_WITH_DEFAULT
        );
        assert_eq!(parse_v4_default(&blackhole_plus_real), Ok(true));
    }

    #[test]
    fn malformed_route_flags_fail_closed() {
        let v4 = "\
Iface\tDestination\tGateway\tFlags\tRefCnt\tUse\tMetric\tMask\tMTU\tWindow\tIRTT
eth0\t00000000\t0101A8C0\tzz\t0\t0\t100\t00000000\t0\t0\t0
";
        assert!(parse_v4_default(v4).is_err());
        let v6 = V6_GATEWAY_DEFAULT.replace("00040003", "0004000g");
        assert!(parse_v6_default(&v6).is_err());
    }

    #[test]
    fn topology_change_is_incomplete_evidence() {
        let first = vec!["eno1".to_string()];
        let second = vec!["eno1".to_string(), "eno2".to_string()];
        assert!(topology_change_detail(&first, &second)
            .unwrap()
            .contains("topology changed"));
        assert!(topology_change_detail(&first, &first).is_none());
    }

    struct CounterFixture(std::path::PathBuf);

    impl CounterFixture {
        fn new() -> Self {
            use std::sync::atomic::{AtomicU64, Ordering};
            static NEXT: AtomicU64 = AtomicU64::new(0);
            let path = std::env::temp_dir().join(format!(
                "alibi-pinned-net-{}-{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
            fs::create_dir(&path).unwrap();
            Self(path)
        }

        fn interface(&self, name: &str, value: u64) {
            let dir = self.0.join(name).join("statistics");
            fs::create_dir_all(&dir).unwrap();
            for stat in COUNTERS {
                fs::write(dir.join(stat), format!("{value}\n")).unwrap();
            }
            fs::write(self.0.join(name).join("operstate"), "up\n").unwrap();
        }
    }

    impl Drop for CounterFixture {
        fn drop(&mut self) {
            fs::remove_dir_all(&self.0).unwrap();
        }
    }

    #[test]
    fn retained_descriptors_do_not_reopen_a_replaced_name() {
        // Ordinary files establish pathname/descriptor mechanics only. Actual
        // kernfs removal must also be tested in a disposable Linux guest.
        let fixture = CounterFixture::new();
        fixture.interface("same0", 1);
        let mut interface = PinnedInterface::open(&fixture.0, "same0").unwrap();
        assert_eq!(interface.read_counters().0.rx_errors, Some(1));
        fs::rename(fixture.0.join("same0"), fixture.0.join("old0")).unwrap();
        fixture.interface("same0", 900);
        fs::write(fixture.0.join("old0/statistics/rx_errors"), "3\n").unwrap();
        assert!(interface
            .check_name()
            .unwrap_err()
            .contains("object changed"));
        let (second, issues) = interface.read_counters();
        assert!(issues.is_empty());
        assert_eq!(second.rx_errors, Some(3));
        assert_eq!(second.tx_errors, Some(1));
        assert_eq!(interface.read_attribute("operstate").as_deref(), Some("up"));
    }

    #[test]
    fn failed_open_is_never_replaced_with_a_later_counter() {
        let fixture = CounterFixture::new();
        fixture.interface("same0", 0);
        fs::remove_file(fixture.0.join("same0/statistics/rx_errors")).unwrap();
        let mut interface = PinnedInterface::open(&fixture.0, "same0").unwrap();
        fs::write(fixture.0.join("same0/statistics/rx_errors"), "900\n").unwrap();
        let (counters, issues) = interface.read_counters();
        assert_eq!(counters.rx_errors, None);
        assert_eq!(counters.tx_errors, Some(0));
        assert_eq!(issues.len(), 1);
    }

    #[test]
    fn seek_reloads_and_overlong_counter_is_not_accepted_as_a_prefix() {
        let fixture = CounterFixture::new();
        fixture.interface("same0", 0);
        let mut interface = PinnedInterface::open(&fixture.0, "same0").unwrap();
        assert_eq!(interface.read_counters().0.rx_errors, Some(0));
        let path = fixture.0.join("same0/statistics/rx_errors");
        fs::write(&path, "7\n").unwrap();
        assert_eq!(interface.read_counters().0.rx_errors, Some(7));
        fs::write(path, format!("7{}invalid", " ".repeat(127))).unwrap();
        let (counters, issues) = interface.read_counters();
        assert_eq!(counters.rx_errors, None);
        assert_eq!(issues[0].0, Status::Malformed);
    }

    #[test]
    fn conntrack_io_status_distinguishes_absence_from_denial() {
        assert_eq!(
            io_status(&std::io::Error::from(ErrorKind::NotFound)),
            Status::Unsupported
        );
        assert_eq!(
            io_status(&std::io::Error::from(ErrorKind::PermissionDenied)),
            Status::Unavailable
        );
    }
}
