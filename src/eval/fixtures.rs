use crate::collect::net::Counters;
use crate::collect::{
    Conntrack, Edac, Facts, FsUsage, Iface, KernelLog, Mount, NetFacts, RouteFacts, Source,
};
use std::time::Duration;

pub fn src_ok<T>(value: T) -> Source<T> {
    Source::ok(value, "fixture", Duration::from_millis(1))
}

const COUNTERS_ZERO: Counters = Counters {
    rx_errors: Some(0),
    rx_dropped: Some(0),
    tx_errors: Some(0),
    tx_dropped: Some(0),
    rx_packets: Some(1000),
    tx_packets: Some(2000),
};

pub fn clean_facts() -> Facts {
    Facts {
        hostname: "fixture-host".to_string(),
        boot_time: Some(1_700_000_000),
        kernel_log: src_ok(KernelLog {
            lines: vec![
                "Linux version 6.8.0-41-generic (buildd@lcy02) #41-Ubuntu SMP".to_string(),
                "pci 0000:00:02.0: [mem 0xfed00000-0xfed003ff]".to_string(),
                "systemd[1]: Reached target Multi-User System.".to_string(),
            ],
            method: "dmesg".to_string(),
        }),
        mounts: src_ok(vec![
            Mount {
                device: "/dev/sda1".to_string(),
                mountpoint: "/".to_string(),
                fstype: "ext4".to_string(),
                ro: false,
                write_capable: true,
                material: true,
            },
            Mount {
                device: "tmpfs".to_string(),
                mountpoint: "/tmp".to_string(),
                fstype: "tmpfs".to_string(),
                ro: false,
                write_capable: false,
                material: true,
            },
        ]),
        fs_usage: src_ok(vec![
            FsUsage {
                device: "/dev/sda1".to_string(),
                mountpoint: "/".to_string(),
                total_bytes: 1000,
                free_bytes: 500,
                total_inodes: 1000,
                free_inodes: 900,
            },
            FsUsage {
                device: "tmpfs".to_string(),
                mountpoint: "/tmp".to_string(),
                total_bytes: 1000,
                free_bytes: 800,
                total_inodes: 1000,
                free_inodes: 999,
            },
        ]),
        edac: src_ok(Edac {
            controllers: vec![("mc0".to_string(), 0, 0)],
        }),
        net: src_ok(NetFacts {
            interfaces: vec![Iface {
                name: "eno1".to_string(),
                is_virtual: false,
                operstate: "up".to_string(),
                carrier: Some(true),
                first: COUNTERS_ZERO,
                second: COUNTERS_ZERO,
            }],
            sample_window: Duration::from_secs(2),
        }),
        routes: src_ok(RouteFacts {
            v4_default: true,
            v6_default: false,
        }),
        conntrack: src_ok(Conntrack {
            count: 10,
            max: 262_144,
        }),
    }
}
