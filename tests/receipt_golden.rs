use alibi::collect::Status;
use alibi::eval::{Finding, Verdict};
use alibi::receipt::{json, Receipt, SourceRecord};
use alibi::timeutil::rfc3339_utc;
use std::time::Duration;

fn finding(
    accusation: &'static str,
    verdict: Verdict,
    summary: &str,
    indications: &[&str],
    evidence: &[&str],
    skipped: &[&str],
    ttv_ms: u64,
) -> Finding {
    let mut f = Finding::new(accusation);
    f.verdict = verdict;
    f.summary = summary.to_string();
    f.indications = indications.iter().map(|s| s.to_string()).collect();
    f.evidence = evidence.iter().map(|s| s.to_string()).collect();
    f.skipped = skipped.iter().map(|s| s.to_string()).collect();
    f.time_to_verdict = Duration::from_millis(ttv_ms);
    f
}

fn source(
    name: &'static str,
    status: Status,
    method: Option<&str>,
    detail: Option<&str>,
    elapsed_ms: u64,
) -> SourceRecord {
    SourceRecord {
        name,
        status,
        method: method.map(str::to_string),
        detail: detail.map(str::to_string),
        elapsed: Duration::from_millis(elapsed_ms),
    }
}

fn golden_receipt() -> Receipt {
    Receipt {
        host: "golden-host".to_string(),
        started_at: rfc3339_utc(1_757_000_000),
        elapsed: Duration::from_millis(2047),
        mtti: Duration::from_millis(2046),
        innocence_established: false,
        verdict: Verdict::Degraded,
        findings: vec![
            finding(
                "kernel-scream",
                Verdict::Clear,
                "no serious kernel/hardware events found",
                &[],
                &["kernel log acquired via dmesg (842 lines scanned)"],
                &[],
                2043,
            ),
            finding(
                "silent-killer",
                Verdict::Degraded,
                "/data (xfs, /dev/sdb1) mounted read-only; if intentional, treat as expected",
                &["/data (xfs, /dev/sdb1) mounted read-only; if intentional, treat as expected"],
                &[],
                &[],
                2044,
            ),
            finding(
                "memrot",
                Verdict::Unknown,
                "EDAC unavailable: platform exposes no EDAC (VM, or driver not loaded); no kernel-reported memory errors — hardware memory integrity not established",
                &[],
                &[],
                &["EDAC unavailable: platform exposes no EDAC (VM, or driver not loaded)"],
                2045,
            ),
            finding(
                "network-choke-point",
                Verdict::NotApplicable,
                "no non-loopback interfaces on this host",
                &[],
                &[],
                &[],
                2046,
            ),
        ],
        boot_time: Some(1_756_900_000),
        sources: vec![
            source("kernel-log", Status::Ok, Some("dmesg"), None, 14),
            source(
                "mounts",
                Status::Ok,
                Some("/proc/self/mounts"),
                Some("17 non-material mounts excluded"),
                1,
            ),
            source(
                "fs-usage",
                Status::Ok,
                Some("df -P -B1 / df -P -i per material filesystem"),
                None,
                41,
            ),
            source(
                "edac",
                Status::Unsupported,
                None,
                Some("platform exposes no EDAC (VM, or driver not loaded)"),
                0,
            ),
            source("net", Status::Ok, Some("/sys/class/net"), None, 2003),
            source(
                "routes",
                Status::Unavailable,
                None,
                Some("/proc/net/route: Permission denied (os error 13)"),
                0,
            ),
            source(
                "conntrack",
                Status::Malformed,
                None,
                Some("/proc/sys/net/netfilter/nf_conntrack_max: not a number"),
                0,
            ),
        ],
    }
}

// A failure here means the receipt contract changed. Regenerate the golden
// file deliberately — after reviewing the diff — never casually:
// ALIBI_UPDATE_GOLDEN=1 cargo test receipt_v1_matches_golden
#[test]
fn receipt_v1_matches_golden() {
    let rendered = json::render(&golden_receipt()) + "\n";
    let golden_path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/golden/receipt_v1.json");
    if std::env::var_os("ALIBI_UPDATE_GOLDEN").is_some() {
        std::fs::create_dir_all(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/golden")).unwrap();
        std::fs::write(golden_path, &rendered).unwrap();
    }
    let golden = std::fs::read_to_string(golden_path)
        .expect("golden file missing; regenerate deliberately with ALIBI_UPDATE_GOLDEN=1");
    assert_eq!(rendered, golden);
}

#[test]
fn receipt_v1_declares_receipt_version() {
    let rendered = json::render(&golden_receipt());
    assert!(rendered.contains("\"receipt_version\": 1"));
}
