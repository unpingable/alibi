use crate::collect::{Source, Status};
use std::fs;
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

pub fn collect_net() -> Source<NetFacts> {
    let start = Instant::now();
    let entries = match fs::read_dir(SYS_CLASS_NET) {
        Ok(d) => d,
        Err(e) => {
            return Source::failed(
                Status::Unavailable,
                format!("{}: {}", SYS_CLASS_NET, e),
                start.elapsed(),
            );
        }
    };
    let mut names: Vec<String> = entries
        .filter_map(Result::ok)
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .filter(|n| n != "lo")
        .collect();
    names.sort();
    let first: Vec<Counters> = names.iter().map(|n| read_counters(n)).collect();
    std::thread::sleep(NET_SAMPLE_WINDOW);
    let interfaces = names
        .iter()
        .zip(first)
        .map(|(name, first)| Iface {
            is_virtual: is_virtual(name),
            operstate: read_sysfs(&format!("{}/{}/operstate", SYS_CLASS_NET, name))
                .unwrap_or_else(|| "unknown".to_string()),
            carrier: read_carrier(name),
            second: read_counters(name),
            name: name.clone(),
            first,
        })
        .collect();
    Source::ok(
        NetFacts {
            interfaces,
            sample_window: NET_SAMPLE_WINDOW,
        },
        SYS_CLASS_NET,
        start.elapsed(),
    )
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

fn read_stat(name: &str, stat: &str) -> Option<u64> {
    read_sysfs(&format!("{}/{}/statistics/{}", SYS_CLASS_NET, name, stat))?
        .parse()
        .ok()
}

fn read_counters(name: &str) -> Counters {
    Counters {
        rx_errors: read_stat(name, "rx_errors"),
        rx_dropped: read_stat(name, "rx_dropped"),
        tx_errors: read_stat(name, "tx_errors"),
        tx_dropped: read_stat(name, "tx_dropped"),
        rx_packets: read_stat(name, "rx_packets"),
        tx_packets: read_stat(name, "tx_packets"),
    }
}

pub fn collect_routes() -> Source<RouteFacts> {
    let start = Instant::now();
    let v4_default = match fs::read_to_string("/proc/net/route") {
        Ok(content) => has_v4_default(&content),
        Err(e) => {
            return Source::failed(
                Status::Unavailable,
                format!("/proc/net/route: {}", e),
                start.elapsed(),
            );
        }
    };
    let v6_default = fs::read_to_string("/proc/net/ipv6_route")
        .map(|content| has_v6_default(&content))
        .unwrap_or(false);
    Source::ok(
        RouteFacts {
            v4_default,
            v6_default,
        },
        "/proc/net/route; /proc/net/ipv6_route",
        start.elapsed(),
    )
}

pub fn has_v4_default(content: &str) -> bool {
    content.lines().skip(1).any(|line| {
        let mut fields = line.split_whitespace();
        fields.next();
        fields.next() == Some("00000000")
    })
}

pub fn has_v6_default(content: &str) -> bool {
    content.lines().any(|line| {
        let mut fields = line.split_whitespace();
        match (fields.next(), fields.next()) {
            (Some(dest), Some(prefix)) => {
                dest == "00000000000000000000000000000000" && prefix == "00"
            }
            _ => false,
        }
    })
}

pub fn collect_conntrack() -> Source<Conntrack> {
    let start = Instant::now();
    let count_path = format!("{}/nf_conntrack_count", CONNTRACK_DIR);
    let max_path = format!("{}/nf_conntrack_max", CONNTRACK_DIR);
    let count_raw = match fs::read_to_string(&count_path) {
        Ok(c) => c,
        Err(e) => {
            return Source::failed(
                Status::Unsupported,
                format!("{}: {}", count_path, e),
                start.elapsed(),
            );
        }
    };
    let max_raw = match fs::read_to_string(&max_path) {
        Ok(c) => c,
        Err(e) => {
            return Source::failed(
                Status::Unsupported,
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
}
