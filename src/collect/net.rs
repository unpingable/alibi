use crate::collect::{Source, Status};
use std::fs;
use std::io::ErrorKind;
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
    let first: Vec<Counters> = names
        .iter()
        .map(|name| {
            let (counters, counter_issues) = read_counters(name);
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
        .map(|(name, first)| {
            let (second, counter_issues) = read_counters(name);
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
                is_virtual: is_virtual(name),
                operstate: read_sysfs(&format!("{}/{}/operstate", SYS_CLASS_NET, name))
                    .unwrap_or_else(|| "unknown".to_string()),
                carrier: read_carrier(name),
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
        Source::ok(facts, SYS_CLASS_NET, start.elapsed())
    } else if !facts.interfaces.is_empty() {
        Source::partial(
            facts,
            issue_status,
            issues.join("; "),
            SYS_CLASS_NET,
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

fn is_virtual(name: &str) -> bool {
    fs::read_link(format!("{}/{}", SYS_CLASS_NET, name))
        .map(|t| t.to_string_lossy().contains("/devices/virtual/"))
        .unwrap_or(false)
}

fn read_sysfs(path: &str) -> Option<String> {
    fs::read_to_string(path).ok().map(|s| s.trim().to_string())
}

fn read_carrier(name: &str) -> Option<bool> {
    match read_sysfs(&format!("{}/{}/carrier", SYS_CLASS_NET, name))?.as_str() {
        "1" => Some(true),
        "0" => Some(false),
        _ => None,
    }
}

fn read_stat(name: &str, stat: &str) -> Result<u64, (Status, String)> {
    let path = format!("{}/{}/statistics/{}", SYS_CLASS_NET, name, stat);
    let raw = fs::read_to_string(&path)
        .map_err(|error| (Status::Unavailable, format!("{}: {}", path, error)))?;
    raw.trim()
        .parse()
        .map_err(|_| (Status::Malformed, format!("{}: not a number", path)))
}

fn read_counters(name: &str) -> (Counters, Vec<(Status, String)>) {
    let mut required_issues = Vec::new();
    let mut required = |stat| match read_stat(name, stat) {
        Ok(value) => Some(value),
        Err(issue) => {
            required_issues.push(issue);
            None
        }
    };
    let counters = Counters {
        rx_errors: required("rx_errors"),
        rx_dropped: required("rx_dropped"),
        tx_errors: required("tx_errors"),
        tx_dropped: required("tx_dropped"),
        rx_packets: read_stat(name, "rx_packets").ok(),
        tx_packets: read_stat(name, "tx_packets").ok(),
    };
    (counters, required_issues)
}

pub fn collect_routes() -> Source<RouteFacts> {
    let start = Instant::now();
    let mut issues = Vec::new();
    let mut issue_status = Status::Unavailable;
    let v4_default = match fs::read_to_string("/proc/net/route") {
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
    let v6_default = match fs::read_to_string("/proc/net/ipv6_route") {
        Ok(content) => match parse_v6_default(&content) {
            Ok(value) => Some(value),
            Err(error) => {
                issue_status = Status::Malformed;
                issues.push(format!("/proc/net/ipv6_route: {}", error));
                None
            }
        },
        Err(error) => {
            issues.push(format!("/proc/net/ipv6_route: {}", error));
            None
        }
    };
    let facts = RouteFacts {
        v4_default: v4_default.unwrap_or(false),
        v6_default: v6_default.unwrap_or(false),
    };
    let method = "/proc/net/route; /proc/net/ipv6_route";
    if issues.is_empty() {
        Source::ok(facts, method, start.elapsed())
    } else if v4_default.is_some() || v6_default.is_some() {
        Source::partial(
            facts,
            issue_status,
            issues.join("; "),
            method,
            start.elapsed(),
        )
    } else {
        Source::failed(issue_status, issues.join("; "), start.elapsed())
    }
}

pub fn has_v4_default(content: &str) -> bool {
    parse_v4_default(content).unwrap_or(false)
}

pub fn has_v6_default(content: &str) -> bool {
    parse_v6_default(content).unwrap_or(false)
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
        {
            return Err(format!("malformed route row: {:?}", line));
        }
        found |= fields[1] == "00000000";
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
        found |= fields[0] == "00000000000000000000000000000000" && fields[1] == "00";
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
    let max_raw = match fs::read_to_string(&max_path) {
        Ok(c) => c,
        Err(e) => {
            return Source::failed(
                io_status(&e),
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
    fn detects_v4_default_route() {
        assert!(has_v4_default(ROUTE_WITH_DEFAULT));
        assert!(!has_v4_default(ROUTE_WITHOUT_DEFAULT));
    }

    #[test]
    fn short_malformed_route_lines_are_skipped() {
        assert!(!has_v4_default("Iface\tDestination\nshort\n"));
    }

    #[test]
    fn detects_v6_default_route() {
        let with_default = "00000000000000000000000000000000 00 00000000000000000000000000000000 00 00000000000000000000000000000000 000003e8 00000000 00000000 00000001 lo\n";
        assert!(has_v6_default(with_default));
        let without = "fe800000000000000000000000000000 40 00000000000000000000000000000000 00 00000000000000000000000000000000 00000100 00000000 00000000 00000001 eth0\n";
        assert!(!has_v6_default(without));
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
