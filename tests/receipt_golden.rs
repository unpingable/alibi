use alibi::collect::{Completeness, EvidenceWindow, SourceId, Status};
use alibi::eval::{Finding, SourceRole, Verdict};
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
    decision: (u64, &[SourceRole]),
) -> Finding {
    let mut f = Finding::new(accusation);
    f.verdict = verdict;
    f.summary = summary.to_string();
    f.indications = indications.iter().map(|s| s.to_string()).collect();
    f.evidence = evidence.iter().map(|s| s.to_string()).collect();
    f.skipped = skipped.iter().map(|s| s.to_string()).collect();
    f.source_roles = decision.1.to_vec();
    f.decided_at = Duration::from_millis(decision.0);
    f.innocence_at = (verdict == Verdict::Clear).then_some(f.decided_at);
    f
}

fn source(
    name: &'static str,
    status: Status,
    method: Option<&str>,
    detail: Option<&str>,
    elapsed_ms: u64,
    completed_at_ms: u64,
    window: Option<(u64, u64)>,
) -> SourceRecord {
    SourceRecord {
        name,
        status,
        completeness: if status == Status::Ok {
            Completeness::Complete
        } else {
            Completeness::None
        },
        method: method.map(str::to_string),
        detail: detail.map(str::to_string),
        elapsed: Duration::from_millis(elapsed_ms),
        completed_at: Duration::from_millis(completed_at_ms),
        window: window.map(|(started_at_ms, ended_at_ms)| EvidenceWindow {
            started_at: Duration::from_millis(started_at_ms),
            ended_at: Duration::from_millis(ended_at_ms),
        }),
    }
}

fn golden_receipt() -> Receipt {
    Receipt {
        host: "golden-host".to_string(),
        started_at: rfc3339_utc(1_757_000_000),
        decision_latency: Duration::from_millis(2063),
        mtti: None,
        interrogation_complete: Duration::from_millis(2064),
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
                (2060, &[SourceRole::required(SourceId::KernelLog)]),
            ),
            finding(
                "silent-killer",
                Verdict::Degraded,
                "/data (xfs, /dev/sdb1) mounted read-only; if intentional, treat as expected",
                &["/data (xfs, /dev/sdb1) mounted read-only; if intentional, treat as expected"],
                &[],
                &[],
                (
                    2061,
                    &[
                        SourceRole::required(SourceId::Mounts),
                        SourceRole::required(SourceId::FsUsage),
                        SourceRole::required(SourceId::KernelLog),
                    ],
                ),
            ),
            finding(
                "memrot",
                Verdict::Unknown,
                "EDAC unavailable: platform exposes no EDAC (VM, or driver not loaded); no kernel-reported memory errors — hardware memory integrity not established",
                &[],
                &[],
                &["EDAC unavailable: platform exposes no EDAC (VM, or driver not loaded)"],
                (
                    2062,
                    &[
                        SourceRole::required(SourceId::Edac),
                        SourceRole::required(SourceId::KernelLog),
                    ],
                ),
            ),
            finding(
                "network-choke-point",
                Verdict::NotApplicable,
                "no non-loopback interfaces on this host",
                &[],
                &[],
                &[],
                (
                    2063,
                    &[SourceRole::required(SourceId::Net)],
                ),
            ),
        ],
        boot_time: Some(1_756_900_000),
        sources: vec![
            source("kernel-log", Status::Ok, Some("dmesg"), None, 14, 56, None),
            source(
                "mounts",
                Status::Ok,
                Some("/proc/self/mounts"),
                Some("17 non-material mounts excluded"),
                1,
                1,
                None,
            ),
            source(
                "fs-usage",
                Status::Ok,
                Some("df -P -B1 / df -P -i per material filesystem"),
                None,
                41,
                42,
                None,
            ),
            source(
                "edac",
                Status::Unsupported,
                None,
                Some("platform exposes no EDAC (VM, or driver not loaded)"),
                0,
                56,
                None,
            ),
            source(
                "net",
                Status::Ok,
                Some("/sys/class/net"),
                None,
                2003,
                2059,
                Some((56, 2059)),
            ),
            source(
                "routes",
                Status::Ok,
                Some("/proc/net/route; /proc/net/ipv6_route"),
                None,
                0,
                2059,
                None,
            ),
            source(
                "conntrack",
                Status::Malformed,
                None,
                Some("/proc/sys/net/netfilter/nf_conntrack_max: not a number"),
                0,
                2059,
                None,
            ),
        ],
    }
}

// A failure here means the receipt contract changed. Regenerate the golden
// file deliberately — after reviewing the diff — never casually:
// ALIBI_UPDATE_GOLDEN=1 cargo test receipt_v2_matches_golden
#[test]
fn receipt_v2_matches_golden() {
    let rendered = json::render(&golden_receipt()) + "\n";
    let golden_path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/golden/receipt_v2.json");
    if std::env::var_os("ALIBI_UPDATE_GOLDEN").is_some() {
        std::fs::create_dir_all(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/golden")).unwrap();
        std::fs::write(golden_path, &rendered).unwrap();
    }
    let golden = std::fs::read_to_string(golden_path)
        .expect("golden file missing; regenerate deliberately with ALIBI_UPDATE_GOLDEN=1");
    assert_eq!(rendered, golden);
}

#[test]
fn receipt_v2_declares_receipt_version() {
    let rendered = json::render(&golden_receipt());
    assert!(rendered.contains("\"receipt_version\": 2"));
}

#[test]
fn default_human_exposes_canonical_envelope_without_verbose() {
    let receipt = golden_receipt();
    let rendered = alibi::receipt::human::render(&receipt, false);
    for source in &receipt.sources {
        assert!(rendered.contains(&format!(
            "{} status={} completeness={}",
            source.name,
            source.status.as_str(),
            source.completeness.as_str()
        )));
        if let Some(detail) = &source.detail {
            assert!(rendered.contains(detail));
        }
    }
    for expected in [
        "kernel-log(required)",
        "net(required)",
        "window_started_at_ms=56",
        "window_ended_at_ms=2059",
        "duration_ms=2003",
        "elapsed_ms=2003",
        "completed_at_ms=2059",
        "method=\"dmesg\"",
        "boot_time=2025-09-03T11:46:40Z",
        "conntrack status=malformed completeness=none",
    ] {
        assert!(
            rendered.contains(expected),
            "missing {expected}: {rendered}"
        );
    }
}
