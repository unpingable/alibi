use crate::collect::{Facts, Iface, Status};
use crate::eval::{base_verdict, first_plus_more, patterns, Evaluator, Finding, Verdict};

pub struct NetworkChokePoint;

impl Evaluator for NetworkChokePoint {
    fn accusation(&self) -> &'static str {
        "network-choke-point"
    }

    fn evaluate(&self, facts: &Facts) -> Finding {
        let mut finding = Finding::new(self.accusation());
        let mut required_failures: Vec<String> = Vec::new();

        if let Some(net) = facts.net.get() {
            if net.interfaces.is_empty() {
                finding.verdict = Verdict::NotApplicable;
                finding.summary = "no non-loopback interfaces on this host".to_string();
                return finding;
            }
        }

        match facts.net.get() {
            Some(net) => {
                let window = net.sample_window.as_secs();
                for iface in &net.interfaces {
                    for (label, first, second) in [
                        ("RX errors", iface.first.rx_errors, iface.second.rx_errors),
                        ("TX errors", iface.first.tx_errors, iface.second.tx_errors),
                        ("RX drops", iface.first.rx_dropped, iface.second.rx_dropped),
                        ("TX drops", iface.first.tx_dropped, iface.second.tx_dropped),
                    ] {
                        if let (Some(a), Some(b)) = (first, second) {
                            let delta = b.saturating_sub(a);
                            if delta > 0 {
                                finding.indications.push(format!(
                                    "{}: {} +{} in {}s window",
                                    iface.name, label, delta, window
                                ));
                            }
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
                if !routes.v4_default && !routes.v6_default {
                    finding.indications.push(
                        "no default route in IPv4 or IPv6 tables (expected on most general-purpose hosts)"
                            .to_string(),
                    );
                } else {
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
            }
            _ => {
                finding.skipped.push(format!(
                    "conntrack check failed: {}",
                    facts.conntrack.failure_reason()
                ));
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
                finding
                    .skipped
                    .push("link-flap history unavailable".to_string());
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

fn pct_of(count: u64, max: u64) -> u64 {
    if max == 0 {
        return 0;
    }
    ((count as u128) * 100 / max as u128) as u64
}

fn counter_delta(first: Option<u64>, second: Option<u64>) -> u64 {
    match (first, second) {
        (Some(a), Some(b)) => b.saturating_sub(a),
        _ => 0,
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
        "{}: operstate={} {} {}; +{} rx/+{} tx packets in {}s window",
        iface.name, iface.operstate, carrier, kind, rx, tx, window_secs
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collect::{Conntrack, KernelLog, RouteFacts, Source};
    use crate::eval::fixtures::{self, src_ok};
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
    }
}
