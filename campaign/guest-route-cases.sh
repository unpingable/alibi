#!/bin/sh
# UNEXECUTED guest scenario. Never run on the development host.
set -eu
case " $(cat /proc/cmdline) " in
  *" alibi_run="*) ;;
  *) echo 'REFUSED: missing campaign guest boot identity' >&2; exit 64 ;;
esac
[ "$(hostname)" = alibi-disposable-guest ] || exit 64
[ "$(stat -f -c %T /)" = tmpfs ] || exit 64

iface=$(ip -o link show | awk -F': ' '$2 != "lo" {print $2; exit}')
[ -n "$iface" ] || { echo 'F7: no isolated guest NIC'; exit 1; }
ip link set lo up
ip link set "$iface" up
ip addr add 192.0.2.10/24 dev "$iface"

observe() {
  printf '\nORACLE_BEGIN %s uptime=' "$1"
  cat /proc/uptime
  ip -details link show
  ip -4 route show table all
  ip -6 route show table all
  cat /proc/net/route /proc/net/ipv6_route
  cat /proc/self/mountinfo
  dmesg
  printf 'ORACLE_END %s\nALIBI_RECEIPT_BEGIN %s\n' "$1" "$1"
  set +e
  alibi check network-choke-point --json
  code=$?
  set -e
  printf '\nALIBI_RECEIPT_END %s rc=%s uptime=' "$1" "$code"
  cat /proc/uptime
  cat /proc/net/route /proc/net/ipv6_route
}

# No external backend exists. This establishes table semantics only, not reachability.
ip route add default dev "$iface"
observe default-control
ip route del default dev "$iface"
observe missing-default
ip route add 0.0.0.0/8 dev "$iface"
observe zero-prefix-eight
ip route del 0.0.0.0/8 dev "$iface"
ip route add 0.0.0.0/32 dev "$iface"
observe zero-host-thirtytwo
ip route del 0.0.0.0/32 dev "$iface"
ip route add default dev "$iface"
observe restored-default
echo 'RESTORATION_RECORDED: disposable guest ends after this scenario'
