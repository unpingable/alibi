#!/bin/sh
# One actual campaign-owned NIC, controlled only by run-bound QMP requests.
set -eu
case " $(cat /proc/cmdline) " in *" alibi_run="*) ;; *) exit 64;; esac
[ "$(hostname)" = alibi-disposable-guest ] || exit 64
CASE_ID=
for word in $(cat /proc/cmdline); do case "$word" in alibi_case=*) CASE_ID=${word#alibi_case=};; esac; done
case "$CASE_ID" in G09|G15) ;; *) exit 64;; esac
. /lib/alibi-lab.sh
/bin/busybox insmod /modules/e1000.ko
IFACE=$(/bin/ip -o link show | awk -F': ' '$2!="lo" {sub(/@.*/,"",$2);print $2;exit}')
[ -n "$IFACE" ] || { echo 'CASE_F7 no-e1000-interface'; exit 1; }
export IFACE
/bin/ip link set lo up
/bin/ip link set "$IFACE" up
/bin/ip addr add 192.0.2.10/24 dev "$IFACE"
/bin/ip route add default dev "$IFACE"
mkdir /run/carrier
cat >/run/carrier/oracle.sh <<'ORACLE'
#!/bin/sh
printf 'ORACLE_INTERFACE=%s UPTIME=' "$IFACE"; cat /proc/uptime
for fact in carrier operstate flags; do printf '%s=' "$fact"; cat "/sys/class/net/$IFACE/$fact"; done
for path in "/sys/class/net/$IFACE"/statistics/*; do printf '%s=' "${path##*/}"; cat "$path"; done
/bin/ip -details -statistics link show "$IFACE"
/bin/ip -4 route show table all
cat /proc/net/route /proc/net/ipv6_route
printf 'RAW_DMESG_BEGIN\n'; /bin/dmesg --raw; printf 'RAW_DMESG_END\n'
ORACLE
wait_carrier() {
  local expected=$1 attempts=0 actual
  while [ "$attempts" -lt 200 ]; do
    actual=$(cat "/sys/class/net/$IFACE/carrier")
    [ "$actual" = "$expected" ] && return 0
    /bin/busybox sleep .05
    attempts=$((attempts+1))
  done
  printf 'CASE_F7 %s carrier-transition-not-observed expected=%s actual=%s\n' "$CASE_ID" "$expected" "$actual"
  return 1
}
wait_carrier 1
/bin/dmesg --raw >/run/carrier/baseline-dmesg
lab_run "$CASE_ID-control-oracle" /bin/sh /run/carrier/oracle.sh
lab_run "$CASE_ID-control" /bin/alibi check --json network-choke-point
sequence=0
transition() {
  local state=$1 expected=$2
  sequence=$((sequence+1))
  printf '\nALIBI_CARRIER_REQUEST %s %s %s\n' "$RUN_ID" "$sequence" "$state"
  wait_carrier "$expected"
  printf 'CARRIER_OBSERVED seq=%s state=%s uptime=' "$sequence" "$state"; cat /proc/uptime
  printf 'carrier='; cat "/sys/class/net/$IFACE/carrier"
  printf 'operstate='; cat "/sys/class/net/$IFACE/operstate"
  /bin/dmesg --raw >"/run/carrier/dmesg-$sequence"
  lab_run "$CASE_ID-transition-$sequence" /bin/cat "/run/carrier/dmesg-$sequence"
}
if [ "$CASE_ID" = G09 ]; then
  transition down 0
  /bin/ip route show default | grep -q "dev $IFACE"
  lab_run G09-fault-oracle /bin/sh /run/carrier/oracle.sh
  lab_run G09 /bin/alibi check --json network-choke-point
  transition up 1
else
  for attempt in 1 2 3; do
    transition down 0
    transition up 1
  done
  /bin/dmesg --raw >/run/carrier/final-dmesg
  count=$(awk -v iface="$IFACE" '$0 ~ ("(^|[^[:alnum:]_])" iface "([^[:alnum:]_]|$)") && /NIC Link is Down|Link is Down|Link is down|link is down|Link Down|Link down|link down/ {n++} END {print n+0}' /run/carrier/final-dmesg)
  before=$(awk -v iface="$IFACE" '$0 ~ ("(^|[^[:alnum:]_])" iface "([^[:alnum:]_]|$)") && /NIC Link is Down|Link is Down|Link is down|link is down|Link Down|Link down|link down/ {n++} END {print n+0}' /run/carrier/baseline-dmesg)
  added=$((count-before))
  printf 'INDEPENDENT_ATTRIBUTED_DOWN_EVENTS interface=%s baseline=%s total=%s added=%s\n' "$IFACE" "$before" "$count" "$added"
  lab_run G15-fault-oracle /bin/sh /run/carrier/oracle.sh
  lab_run G15 /bin/alibi check --json network-choke-point
  if [ "$added" -lt 3 ]; then echo 'CASE_F7 G15 specified-kernel-flap-message-count-not-established'; fi
fi
lab_run "$CASE_ID-restored-oracle" /bin/sh /run/carrier/oracle.sh
printf 'CASE_END %s carrier-transitions-independently-observed\n' "$CASE_ID"
