// Deterministic characterization, not a host qualification or acceptance contract.
use alibi::collect::net::Counters;
use alibi::collect::*;
use alibi::eval::{
    kernel_scream::KernelScream, memrot::Memrot, network_choke_point::NetworkChokePoint,
    silent_killer::SilentKiller,
};
use alibi::eval::{Evaluator, Verdict};
use std::cell::RefCell;
use std::time::{Duration, Instant, UNIX_EPOCH};
fn ok<T>(v: T) -> Source<T> {
    Source::ok(v, "deterministic fixture", Duration::ZERO)
}
fn log(lines: Vec<String>) -> Source<KernelLog> {
    ok(KernelLog {
        lines,
        method: "fixture dmesg representation".into(),
    })
}
fn clean() -> Facts {
    let c = Counters {
        rx_errors: Some(0),
        rx_dropped: Some(0),
        tx_errors: Some(0),
        tx_dropped: Some(0),
        rx_packets: Some(1),
        tx_packets: Some(1),
    };
    Facts {
        hostname: "temporal-fixture".into(),
        boot_time: Some(1_700_000_000),
        kernel_log: log(vec!["<6>[ 1.000000] boot record".into()]),
        mounts: ok(vec![Mount {
            device: "/dev/fixture".into(),
            mountpoint: "/".into(),
            fstype: "ext4".into(),
            ro: false,
            write_capable: true,
            material: true,
        }]),
        fs_usage: ok(vec![FsUsage {
            device: "/dev/fixture".into(),
            mountpoint: "/".into(),
            bytes: Some(FsCapacity {
                total: 1000,
                free: 800,
            }),
            inodes: Some(FsCapacity {
                total: 1000,
                free: 900,
            }),
        }]),
        edac: ok(Edac {
            controllers: vec![EdacController {
                name: "mc0".into(),
                ce: Some(0),
                ue: Some(0),
            }],
        }),
        net: ok(NetFacts {
            interfaces: vec![Iface {
                name: "eno1".into(),
                is_virtual: false,
                operstate: "up".into(),
                carrier: Some(true),
                first: c,
                second: c,
            }],
            sample_window: Duration::from_secs(2),
        })
        .with_window(Duration::from_millis(100), Duration::from_millis(2100)),
        routes: ok(RouteFacts {
            v4_default: true,
            v6_default: false,
        }),
        conntrack: ok(Conntrack { count: 0, max: 100 }),
    }
}
fn flaps(at: u64) -> Vec<String> {
    (0..5)
        .map(|i| format!("<6>[ {}.000000] eno1: Link is Down", at + i))
        .collect()
}
thread_local! { static LOG:RefCell<Vec<String>>=const{RefCell::new(Vec::new())}; static ORDER:RefCell<Vec<&'static str>>=const{RefCell::new(Vec::new())}; }
fn mark(s: &'static str) {
    ORDER.with(|o| o.borrow_mut().push(s));
}
// Collector substitutes: no /proc, /sys, command execution, sleep or fault injection.
mod ordered {
    use super::*;
    fn hostname() -> String {
        "temporal-fixture".into()
    }
    fn boot_time() -> Option<u64> {
        Some(1_700_000_000)
    }
    fn not_requested<T>() -> Source<T> {
        Source::failed(Status::Unsupported, "not requested", Duration::ZERO)
    }
    mod mounts {
        use super::*;
        pub fn collect() -> Source<Vec<Mount>> {
            mark("mounts");
            clean().mounts
        }
    }
    mod fsusage {
        use super::*;
        pub fn collect(_: &Source<Vec<Mount>>) -> Source<Vec<FsUsage>> {
            mark("fs-usage");
            clean().fs_usage
        }
    }
    mod edac {
        use super::*;
        pub fn collect() -> Source<Edac> {
            mark("edac");
            clean().edac
        }
    }
    mod kernellog {
        use super::*;
        pub fn collect() -> Source<KernelLog> {
            mark("kernel-log");
            log(LOG.with(|l| l.borrow().clone()))
        }
    }
    mod net {
        use super::*;
        pub fn collect_net(_: Instant) -> Source<NetFacts> {
            mark("net-start");
            LOG.with(|l| l.borrow_mut().extend(flaps(100)));
            mark("five-flaps-during-sample");
            mark("net-end");
            clean().net
        }
        pub fn collect_routes() -> Source<RouteFacts> {
            mark("routes");
            clean().routes
        }
        pub fn collect_conntrack() -> Source<Conntrack> {
            mark("conntrack");
            clean().conntrack
        }
    }
    include!("ordered.rs"); // verbatim collect_facts_for extracted from pinned source
}
fn emit(id: &str, f: &Facts, e: &dyn Evaluator, expected: Verdict) {
    let finding = e.evaluate(f);
    assert_eq!(finding.verdict, expected, "{id}");
    println!("{id}: {}", finding.verdict.as_str());
    let plan = CollectionPlan::new(e.source_roles().iter().map(|r| r.source).collect());
    let r = alibi::receipt::build(
        f,
        vec![finding],
        UNIX_EPOCH + Duration::from_secs(1_700_000_000 + 300 * 86400),
        Duration::from_millis(2200),
        &plan,
    );
    let out = std::env::var("TEMPORAL_RESULTS").unwrap();
    std::fs::write(format!("{out}/{id}.json"), alibi::receipt::json::render(&r)).unwrap();
    std::fs::write(
        format!("{out}/{id}.txt"),
        alibi::receipt::human::render(&r, false),
    )
    .unwrap();
}
fn main() {
    emit(
        "clean-network",
        &clean(),
        &NetworkChokePoint,
        Verdict::Clear,
    );
    let mut f = clean();
    f.kernel_log = log(flaps(60));
    emit(
        "old-flaps-300-day-boot",
        &f,
        &NetworkChokePoint,
        Verdict::Degraded,
    );
    f.kernel_log = log(flaps(300 * 86400 - 10));
    emit("recent-flaps", &f, &NetworkChokePoint, Verdict::Degraded);
    f.kernel_log = log(vec![
        "<3>[ 60.000000] EXT4-fs error: historical fixture".into()
    ]);
    emit(
        "old-fs-error-current-rw",
        &f,
        &SilentKiller,
        Verdict::Degraded,
    );
    emit(
        "retained-log-clean",
        &clean(),
        &KernelScream,
        Verdict::Clear,
    );
    // Same retained record is compatible with an omitted prior panic OR no panic.
    f = clean();
    f.kernel_log = log(vec![
        "<6>[ 900.000000] quiet record after retention loss".into()
    ]);
    emit("unknown-retention", &f, &KernelScream, Verdict::Clear);
    f.kernel_log = log(vec!["<0>[ 60.000000] Kernel panic - fixture".into()]);
    emit(
        "retained-panic-control",
        &f,
        &KernelScream,
        Verdict::Degraded,
    );
    f.kernel_log = log(vec![
        "<6>[ 60.000000] INFO: task worker:1234 blocked for more than 120 seconds.".into(),
    ]);
    emit(
        "hung-task-not-recognized",
        &f,
        &KernelScream,
        Verdict::Clear,
    );
    f = clean();
    f.edac = Source::failed(Status::Unsupported, "fixture no EDAC", Duration::ZERO);
    emit("no-edac", &f, &Memrot, Verdict::Unknown);
    emit("selected-kernel-no-edac", &f, &KernelScream, Verdict::Clear);
    let mut findings = alibi::eval::registry()
        .iter()
        .map(|e| e.evaluate(&f))
        .collect::<Vec<_>>();
    assert_eq!(alibi::receipt::overall_verdict(&findings), Verdict::Unknown);
    findings[0].verdict = Verdict::Degraded;
    assert_eq!(
        alibi::receipt::overall_verdict(&findings),
        Verdict::Degraded
    );
    LOG.with(|l| *l.borrow_mut() = vec!["<6>[ 1.000000] boot record".into()]);
    let plan = CollectionPlan::new(vec![
        SourceId::Net,
        SourceId::Routes,
        SourceId::Conntrack,
        SourceId::KernelLog,
    ]);
    let f = ordered::collect_facts_for(&plan, Instant::now());
    emit(
        "flaps-after-log-snapshot",
        &f,
        &NetworkChokePoint,
        Verdict::Clear,
    );
    let order = ORDER.with(|o| o.borrow().clone());
    assert_eq!(
        order,
        vec![
            "kernel-log",
            "net-start",
            "five-flaps-during-sample",
            "net-end",
            "routes",
            "conntrack"
        ]
    );
    println!("collection-order: {order:?}");
    let mut after = f;
    after.kernel_log = log(LOG.with(|l| l.borrow().clone()));
    emit(
        "same-fixture-log-after-sample-control",
        &after,
        &NetworkChokePoint,
        Verdict::Degraded,
    );
}
