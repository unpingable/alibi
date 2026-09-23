use crate::collect::{Facts, Iface, SourceId, Status};
use crate::eval::{
    base_verdict, first_plus_more, patterns, Evaluator, Finding, SourceRole, Verdict,
};

// Every source here is required to exonerate a failure class the accusation
// names: counters and routes for error/drop deltas and default-route absence,
// the kernel log for link flaps and conntrack-full events, and conntrack
// counts for table pressure. Conntrack is required only while connection
// tracking is present on the host; when the subsystem is absent (files not
// found) that failure class is not applicable and the finding records the
// source as supporting instead. Denied or malformed conntrack is a collection
// failure, not absence, and blocks CLEAR.
const SOURCES: &[SourceRole] = &[
    SourceRole::required(SourceId::Net),
    SourceRole::required(SourceId::Routes),
    SourceRole::required(SourceId::Conntrack),
    SourceRole::required(SourceId::KernelLog),
];

fn roles_without_conntrack_subsystem() -> Vec<SourceRole> {
    SOURCES
        .iter()
        .map(|role| {
            if role.source == SourceId::Conntrack {
                SourceRole::supporting(SourceId::Conntrack)
            } else {
                *role
            }
        })
        .collect()
}

pub struct NetworkChokePoint;

impl Evaluator for NetworkChokePoint {
    fn accusation(&self) -> &'static str {
        "network-choke-point"
    }

    fn source_roles(&self) -> &'static [SourceRole] {
        SOURCES
    }

    fn evaluate_evidence(&self, facts: &Facts) -> Finding {
        let mut finding = Finding::new(self.accusation());
        let mut required_failures: Vec<String> = Vec::new();

        if let Some(net) = facts.net.get() {
            if net.interfaces.is_empty() {
                finding.verdict = Verdict::NotApplicable;
                finding.summary = "no non-loopback interfaces on this host".to_string();
                finding.source_roles = vec![SourceRole::required(SourceId::Net)];
                return finding;
            }
        }

        match facts.net.get() {
            Some(net) => {
                let window = net.sample_window.as_secs();
                for iface in &net.interfaces {
                    for (label, first, second) in required_counter_pairs(iface) {
                        match (first, second) {
                            (Some(a), Some(b)) if b >= a => {
                                let delta = b - a;
                                if delta > 0 {
                                    finding.indications.push(format!(
                                        "{}: {} +{} in {}s window",
                                        iface.name, label, delta, window
                                    ));
                                }
                            }
                            (Some(a), Some(b)) => required_failures.push(format!(
                                "{}: {} counter decreased from {} to {}",
                                iface.name, label, a, b
                            )),
                            _ => required_failures.push(format!(
                                "{}: {} counter sample incomplete",
                                iface.name, label
                            )),
                        }
                    }
                    finding.evidence.push(iface_evidence(iface, window));
                }
            }
            None => {
                let msg = format!(
                    "interface facts unavailable: {}",
                    facts.net.failure_reason()
                );
                required_failures.push(msg.clone());
                finding.skipped.push(msg);
            }
        }

        match facts.routes.get() {
            Some(routes) => {
                if facts.routes.is_complete() && !routes.v4_default && !routes.v6_default {
                    finding.indications.push(
                        "no default route in IPv4 or IPv6 tables (expected on most general-purpose hosts)"
                            .to_string(),
                    );
                } else if routes.v4_default || routes.v6_default {
                    let mut families = Vec::new();
                    if routes.v4_default {
                        families.push("IPv4");
                    }
                    if routes.v6_default {
                        families.push("IPv6");
                    }
                    finding
                        .evidence
                        .push(format!("default route present ({})", families.join(", ")));
                }
            }
            None => {
                let msg = format!(
                    "route tables unavailable: {}",
                    facts.routes.failure_reason()
                );
                required_failures.push(msg.clone());
                finding.skipped.push(msg);
            }
        }

        match facts.conntrack.status {
            Status::Ok => {
                if let Some(c) = facts.conntrack.get() {
                    let pct = pct_of(c.count, c.max);
                    let line = format!("conntrack table {}/{} ({}% of max)", c.count, c.max, pct);
                    if c.max > 0 && (c.count as u128) * 100 >= (c.max as u128) * 90 {
                        finding.indications.push(line);
                    } else {
                        finding.evidence.push(line);
                    }
                }
            }
            Status::Unsupported => {
                finding
                    .skipped
                    .push("conntrack not present (no connection tracking in use)".to_string());
                finding.source_roles = roles_without_conntrack_subsystem();
            }
            _ => {
                let msg = format!(
                    "conntrack unavailable: {}",
                    facts.conntrack.failure_reason()
                );
                required_failures.push(msg.clone());
                finding.skipped.push(msg);
            }
        }

        match facts.kernel_log.get() {
            Some(log) => {
                if let Some(net) = facts.net.get() {
                    for iface in &net.interfaces {
                        let flaps = log
                            .lines
                            .iter()
                            .filter(|l| {
                                l.contains(&iface.name)
                                    && patterns::line_label_for(l, "net-down").is_some()
                            })
                            .count();
                        if flaps >= 3 {
                            finding.indications.push(format!(
                                "{}: {} link-down events in kernel log window",
                                iface.name, flaps
                            ));
                        }
                    }
                }
                let (matches, _) = patterns::matches_for(&log.lines, self.accusation());
                for (line, label) in matches {
                    finding.indications.push(format!("{}: {}", label, line));
                }
            }
            None => {
                let msg = format!(
                    "kernel log unavailable: {}",
                    facts.kernel_log.failure_reason()
                );
                required_failures.push(msg.clone());
                finding.skipped.push(msg);
            }
        }

        finding.verdict = base_verdict(&finding.indications, &required_failures);
        finding.summary = match finding.verdict {
            Verdict::Clear => "no host-local network pathology observed".to_string(),
            Verdict::Degraded => first_plus_more(&finding.indications),
            Verdict::Unknown => required_failures.join("; "),
            Verdict::NotApplicable => unreachable!(),
        };
        finding
    }
}

fn required_counter_pairs(iface: &Iface) -> [(&'static str, Option<u64>, Option<u64>); 4] {
    [
        ("RX errors", iface.first.rx_errors, iface.second.rx_errors),
        ("TX errors", iface.first.tx_errors, iface.second.tx_errors),
        ("RX drops", iface.first.rx_dropped, iface.second.rx_dropped),
        ("TX drops", iface.first.tx_dropped, iface.second.tx_dropped),
    ]
}

fn pct_of(count: u64, max: u64) -> u64 {
    if max == 0 {
        return 0;
    }
    ((count as u128) * 100 / max as u128) as u64
}

fn counter_delta(first: Option<u64>, second: Option<u64>) -> String {
    match (first, second) {
        (Some(a), Some(b)) if b >= a => format!("+{}", b - a),
        (Some(_), Some(_)) => "reset".to_string(),
        _ => "unavailable".to_string(),
    }
}

fn iface_evidence(iface: &Iface, window_secs: u64) -> String {
    let kind = if iface.is_virtual {
        "virtual"
    } else {
        "physical"
    };
    let carrier = match iface.carrier {
        Some(true) => "carrier",
        Some(false) => "no-carrier",
        None => "carrier-unknown",
    };
    let rx = counter_delta(iface.first.rx_packets, iface.second.rx_packets);
    let tx = counter_delta(iface.first.tx_packets, iface.second.tx_packets);
    format!(
        "{}: operstate={} {} {}; rx packets {}, tx packets {} in {}s window",
        iface.name, iface.operstate, carrier, kind, rx, tx, window_secs
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collect::{Conntrack, KernelLog, NetFacts, RouteFacts, Source};
    use crate::eval::fixtures::{self, src_ok};
    use crate::eval::EvidenceRole;
    use std::time::Duration;

    #[test]
    fn not_applicable_without_interfaces() {
        let mut facts = fixtures::clean_facts();
        if let Some(net) = facts.net.value.as_mut() {
            net.interfaces.clear();
        }
        let finding = NetworkChokePoint.evaluate(&facts);
        assert_eq!(finding.verdict, Verdict::NotApplicable);
        assert_eq!(finding.summary, "no non-loopback interfaces on this host");
    }

    #[test]
    fn clear_on_clean_fixture() {
        let finding = NetworkChokePoint.evaluate(&fixtures::clean_facts());
        assert_eq!(finding.verdict, Verdict::Clear);
        assert_eq!(finding.summary, "no host-local network pathology observed");
    }

    #[test]
    fn degraded_on_rx_error_delta() {
        let mut facts = fixtures::clean_facts();
        if let Some(net) = facts.net.value.as_mut() {
            net.interfaces[0].second.rx_errors = Some(5);
        }
        let finding = NetworkChokePoint.evaluate(&facts);
        assert_eq!(finding.verdict, Verdict::Degraded);
        assert!(finding
            .indications
            .iter()
            .any(|i| i.contains("eno1: RX errors +5 in 2s window")));
    }

    #[test]
    fn degraded_without_any_default_route() {
        let mut facts = fixtures::clean_facts();
        facts.routes = src_ok(RouteFacts {
            v4_default: false,
            v6_default: false,
        });
        let finding = NetworkChokePoint.evaluate(&facts);
        assert_eq!(finding.verdict, Verdict::Degraded);
        assert!(finding
            .indications
            .iter()
            .any(|i| i.contains("no default route")));
    }

    #[test]
    fn degraded_on_conntrack_pressure() {
        let mut facts = fixtures::clean_facts();
        facts.conntrack = src_ok(Conntrack {
            count: 95,
            max: 100,
        });
        let finding = NetworkChokePoint.evaluate(&facts);
        assert_eq!(finding.verdict, Verdict::Degraded);
        assert!(finding
            .indications
            .iter()
            .any(|i| i.contains("conntrack table 95/100 (95% of max)")));
    }

    #[test]
    fn unknown_when_net_unavailable() {
        let mut facts = fixtures::clean_facts();
        facts.net = Source::failed(
            Status::Unavailable,
            "/sys/class/net: permission denied",
            Duration::from_millis(1),
        );
        let finding = NetworkChokePoint.evaluate(&facts);
        assert_eq!(finding.verdict, Verdict::Unknown);
    }

    #[test]
    fn missing_required_counter_cannot_clear() {
        let mut facts = fixtures::clean_facts();
        facts.net.value.as_mut().unwrap().interfaces[0]
            .second
            .rx_errors = None;
        let finding = NetworkChokePoint.evaluate(&facts);
        assert_eq!(finding.verdict, Verdict::Unknown);
        assert!(finding.summary.contains("counter sample incomplete"));
    }

    #[test]
    fn positive_counter_survives_another_counter_failure() {
        let mut facts = fixtures::clean_facts();
        let iface = &mut facts.net.value.as_mut().unwrap().interfaces[0];
        iface.second.rx_errors = Some(4);
        iface.second.tx_dropped = None;
        facts.net.completeness = crate::collect::Completeness::Partial;
        facts.net.status = Status::Unavailable;
        facts.net.detail = Some("tx_dropped disappeared".to_string());
        let finding = NetworkChokePoint.evaluate(&facts);
        assert_eq!(finding.verdict, Verdict::Degraded);
        assert!(finding
            .indications
            .iter()
            .any(|i| i.contains("RX errors +4")));
    }

    #[test]
    fn optional_packet_deltas_report_unavailable_and_reset() {
        let mut facts = fixtures::clean_facts();
        let iface = &mut facts.net.value.as_mut().unwrap().interfaces[0];
        iface.second.rx_packets = None;
        iface.first.tx_packets = Some(9);
        iface.second.tx_packets = Some(2);
        let finding = NetworkChokePoint.evaluate(&facts);
        assert_eq!(finding.verdict, Verdict::Clear);
        assert!(finding
            .evidence
            .iter()
            .any(|e| { e.contains("rx packets unavailable") && e.contains("tx packets reset") }));
    }

    #[test]
    fn counter_reset_cannot_clear() {
        let mut facts = fixtures::clean_facts();
        let iface = &mut facts.net.value.as_mut().unwrap().interfaces[0];
        iface.first.rx_errors = Some(9);
        iface.second.rx_errors = Some(2);
        let finding = NetworkChokePoint.evaluate(&facts);
        assert_eq!(finding.verdict, Verdict::Unknown);
        assert!(finding.summary.contains("counter decreased"));
    }

    #[test]
    fn partial_empty_interface_enumeration_is_unknown_not_not_applicable() {
        let mut facts = fixtures::clean_facts();
        facts.net = Source::partial(
            NetFacts {
                interfaces: vec![],
                sample_window: Duration::from_secs(2),
            },
            Status::Unavailable,
            "/sys/class/net: directory entry disappeared",
            "/sys/class/net",
            Duration::from_millis(1),
        );
        let finding = NetworkChokePoint.evaluate(&facts);
        assert_eq!(finding.verdict, Verdict::Unknown);
    }

    #[test]
    fn complete_empty_interface_domain_is_not_applicable_even_if_routes_unavailable() {
        let mut facts = fixtures::clean_facts();
        facts.net = src_ok(NetFacts {
            interfaces: vec![],
            sample_window: Duration::from_secs(2),
        });
        facts.routes = Source::failed(
            Status::Unavailable,
            "/proc/net/route: permission denied",
            Duration::from_millis(1),
        );
        let finding = NetworkChokePoint.evaluate(&facts);
        assert_eq!(finding.verdict, Verdict::NotApplicable);
    }

    #[test]
    fn partial_route_tables_do_not_false_degrade_or_clear() {
        let mut facts = fixtures::clean_facts();
        facts.routes = Source::partial(
            RouteFacts {
                v4_default: false,
                v6_default: false,
            },
            Status::Malformed,
            "/proc/net/ipv6_route: malformed row",
            "/proc/net/route; /proc/net/ipv6_route",
            Duration::from_millis(1),
        );
        let finding = NetworkChokePoint.evaluate(&facts);
        assert_eq!(finding.verdict, Verdict::Unknown);
        assert!(!finding.indications.iter().any(|i| i.contains("no default")));
        assert!(!finding
            .evidence
            .iter()
            .any(|e| e.contains("default route present ()")));
    }

    #[test]
    fn degraded_on_repeated_link_down_events() {
        let mut facts = fixtures::clean_facts();
        facts.kernel_log = src_ok(KernelLog {
            lines: vec![
                "ixgbe 0000:01:00.0 eno1: NIC Link is Down".to_string(),
                "ixgbe 0000:01:00.0 eno1: NIC Link is Down".to_string(),
                "ixgbe 0000:01:00.0 eno1: NIC Link is Down".to_string(),
            ],
            method: "dmesg".to_string(),
        });
        let finding = NetworkChokePoint.evaluate(&facts);
        assert_eq!(finding.verdict, Verdict::Degraded);
        assert!(finding
            .indications
            .iter()
            .any(|i| i.contains("eno1: 3 link-down events")));
    }

    #[test]
    fn clear_with_skipped_when_conntrack_unsupported() {
        let mut facts = fixtures::clean_facts();
        facts.conntrack = Source::failed(
            Status::Unsupported,
            "nf_conntrack_count: no such file",
            Duration::from_millis(1),
        );
        let finding = NetworkChokePoint.evaluate(&facts);
        assert_eq!(finding.verdict, Verdict::Clear);
        assert!(finding
            .skipped
            .iter()
            .any(|s| s == "conntrack not present (no connection tracking in use)"));
        let conntrack = finding
            .source_roles
            .iter()
            .find(|r| r.source == SourceId::Conntrack)
            .unwrap();
        assert_eq!(conntrack.role, EvidenceRole::Supporting);
        assert!(finding
            .source_roles
            .iter()
            .filter(|r| r.source != SourceId::Conntrack)
            .all(|r| r.role == EvidenceRole::Required));
    }

    #[test]
    fn conntrack_is_a_required_role_when_present() {
        let finding = NetworkChokePoint.evaluate(&fixtures::clean_facts());
        assert!(finding
            .source_roles
            .iter()
            .all(|r| r.role == EvidenceRole::Required));
        assert!(finding
            .source_roles
            .iter()
            .any(|r| r.source == SourceId::KernelLog));
    }

    fn flap_storm_log() -> Source<KernelLog> {
        src_ok(KernelLog {
            lines: vec![
                "r8169 0000:07:00.0 eno1: Link is Down".to_string(),
                "r8169 0000:07:00.0 eno1: Link is Up - 1Gbps/Full".to_string(),
                "r8169 0000:07:00.0 eno1: Link is Down".to_string(),
                "r8169 0000:07:00.0 eno1: Link is Up - 1Gbps/Full".to_string(),
                "r8169 0000:07:00.0 eno1: Link is Down".to_string(),
            ],
            method: "dmesg".to_string(),
        })
    }

    fn denied_log() -> Source<KernelLog> {
        Source::failed(
            Status::Unavailable,
            "dmesg: read kernel buffer failed: Operation not permitted; journalctl: no entries",
            Duration::from_millis(1),
        )
    }

    #[test]
    fn link_flap_storm_is_degraded_when_log_is_readable() {
        let mut facts = fixtures::clean_facts();
        facts.kernel_log = flap_storm_log();
        let finding = NetworkChokePoint.evaluate(&facts);
        assert_eq!(finding.verdict, Verdict::Degraded);
        assert!(finding
            .indications
            .iter()
            .any(|i| i.contains("eno1: 3 link-down events")));
    }

    #[test]
    fn unknown_not_clear_when_kernel_log_is_denied() {
        let mut facts = fixtures::clean_facts();
        facts.kernel_log = denied_log();
        let finding = NetworkChokePoint.evaluate(&facts);
        assert_eq!(finding.verdict, Verdict::Unknown);
        assert!(finding.summary.contains("kernel log unavailable"));
    }

    #[test]
    fn conntrack_pressure_denied_is_unknown_not_clear() {
        for (status, detail) in [
            (
                Status::Unavailable,
                "/proc/sys/net/netfilter/nf_conntrack_count: Permission denied (os error 13)",
            ),
            (
                Status::Malformed,
                "/proc/sys/net/netfilter/nf_conntrack_max: not a number",
            ),
        ] {
            let mut facts = fixtures::clean_facts();
            facts.conntrack = Source::failed(status, detail, Duration::from_millis(1));
            let finding = NetworkChokePoint.evaluate(&facts);
            assert_eq!(finding.verdict, Verdict::Unknown, "{:?}", status);
            assert!(finding.summary.contains("conntrack unavailable"));
            assert!(finding
                .source_roles
                .iter()
                .any(|r| r.source == SourceId::Conntrack && r.role == EvidenceRole::Required));
        }
    }

    #[test]
    fn privilege_loss_never_improves_the_verdict() {
        // Scenario A: the only positive evidence lives in the kernel log.
        let mut readable = fixtures::clean_facts();
        readable.kernel_log = flap_storm_log();
        assert_eq!(
            NetworkChokePoint.evaluate(&readable).verdict,
            Verdict::Degraded
        );
        let mut denied = fixtures::clean_facts();
        denied.kernel_log = denied_log();
        assert_eq!(
            NetworkChokePoint.evaluate(&denied).verdict,
            Verdict::Unknown
        );

        // Scenario B: the only positive evidence is conntrack pressure.
        let mut readable = fixtures::clean_facts();
        readable.conntrack = src_ok(Conntrack {
            count: 95,
            max: 100,
        });
        assert_eq!(
            NetworkChokePoint.evaluate(&readable).verdict,
            Verdict::Degraded
        );
        let mut denied = fixtures::clean_facts();
        denied.conntrack = Source::failed(
            Status::Unavailable,
            "nf_conntrack_count: Permission denied (os error 13)",
            Duration::from_millis(1),
        );
        assert_eq!(
            NetworkChokePoint.evaluate(&denied).verdict,
            Verdict::Unknown
        );

        // Scenario C: positive counter evidence survives losing the log and
        // conntrack; the verdict stays DEGRADED and the losses are recorded.
        let mut facts = fixtures::clean_facts();
        facts.net.value.as_mut().unwrap().interfaces[0]
            .second
            .rx_errors = Some(5);
        facts.kernel_log = denied_log();
        facts.conntrack = Source::failed(
            Status::Unavailable,
            "nf_conntrack_count: Permission denied (os error 13)",
            Duration::from_millis(1),
        );
        let finding = NetworkChokePoint.evaluate(&facts);
        assert_eq!(finding.verdict, Verdict::Degraded);
        assert!(finding
            .skipped
            .iter()
            .any(|s| s.contains("kernel log unavailable")));
        assert!(finding
            .skipped
            .iter()
            .any(|s| s.contains("conntrack unavailable")));
    }
}
