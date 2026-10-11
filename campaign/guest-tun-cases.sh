#!/bin/sh
# Real guest TUN receive-drop qualification; never substitute counter files.
set -eu
case " $(cat /proc/cmdline) " in *" alibi_run="*) ;; *) exit 64;; esac
[ "$(hostname)" = alibi-disposable-guest ] || exit 64
CASE_ID=
for word in $(cat /proc/cmdline); do case "$word" in alibi_case=*) CASE_ID=${word#alibi_case=};; esac; done
case "$CASE_ID" in G13|G14) ;; *) exit 64;; esac
. /lib/alibi-lab.sh
mkdir /run/alibi-tun
mkdir -p /dev/net
[ -e /dev/net/tun ] || /bin/busybox mknod /dev/net/tun c 10 200
IFACE=$(/bin/ip -o link show | awk -F': ' '$2!="lo" {sub(/@.*/,"",$2);print $2;exit}')
[ -n "$IFACE" ] || exit 1
/bin/ip link set lo up
/bin/ip link set "$IFACE" up
/bin/ip addr add 192.0.2.10/24 dev "$IFACE"
/bin/ip route add default dev "$IFACE"
if [ "$CASE_ID" = G13 ]; then emit=20; else emit=4; fi
lab_run "$CASE_ID-generator" /bin/tun-drop-fixture "$emit" 20 &
generator=$!
wait_phase() {
    local expected=$1 count=0
    while [ "$count" -lt 1000 ]; do
        [ ! -f /run/alibi-tun/phase ] || [ "$(cat /run/alibi-tun/phase)" != "$expected" ] || return 0
        /bin/busybox sleep .01; count=$((count+1))
    done
    printf 'CASE_F7 %s helper-phase-not-observed expected=%s\n' "$CASE_ID" "$expected"
    return 1
}
wait_phase READY
/bin/ip link set aqtun0 up
before=$(cat /sys/class/net/aqtun0/statistics/rx_dropped)
printf 'TUN_ORACLE_READY uptime='; cat /proc/uptime
/bin/ip -details -statistics link show aqtun0
lab_run "$CASE_ID-control" /bin/alibi check --json network-choke-point
touch /run/alibi-tun/go
wait_phase EMITTING
attempt=0
while [ "$(cat /sys/class/net/aqtun0/statistics/rx_dropped)" -le "$before" ] && [ "$attempt" -lt 200 ]; do
    /bin/busybox sleep .01; attempt=$((attempt+1))
done
observed=$(cat /sys/class/net/aqtun0/statistics/rx_dropped)
printf 'TUN_COUNTER_PROOF before=%s observed=%s uptime=' "$before" "$observed"; cat /proc/uptime
if [ "$observed" -le "$before" ]; then
    echo "CASE_F7 $CASE_ID tun-rx-drop-not-established"
    touch /run/alibi-tun/stop; wait "$generator"; exit 0
fi
if [ "$CASE_ID" = G14 ]; then
    wait_phase HOLD
    stable1=$(cat /sys/class/net/aqtun0/statistics/rx_dropped)
    /bin/busybox sleep 3
    stable2=$(cat /sys/class/net/aqtun0/statistics/rx_dropped)
    printf 'TUN_HISTORICAL_PROOF first=%s second=%s uptime=' "$stable1" "$stable2"; cat /proc/uptime
    [ "$stable1" -gt 0 ] && [ "$stable1" -eq "$stable2" ] || { echo 'CASE_F7 G14 historical-counter-not-stable'; touch /run/alibi-tun/stop; wait "$generator"; exit 0; }
fi
sample_before=$(cat /sys/class/net/aqtun0/statistics/rx_dropped)
printf 'TUN_BEFORE_ALIBI phase=%s counter=%s uptime=' "$(cat /run/alibi-tun/phase)" "$sample_before"; cat /proc/uptime
# Independent kernel samples bracket and overlap the collection interval.
(
    count=0
    while [ ! -e /run/alibi-tun/sampler-stop ] && [ "$count" -lt 300 ]; do
        printf 'TUN_SAMPLE uptime='; cat /proc/uptime
        printf 'counter='; cat /sys/class/net/aqtun0/statistics/rx_dropped
        /bin/busybox sleep .05; count=$((count+1))
    done
) > /run/alibi-tun/samples &
sampler=$!
lab_run "$CASE_ID" /bin/alibi check --json network-choke-point
touch /run/alibi-tun/sampler-stop
wait "$sampler"
lab_run "$CASE_ID-counter-trace" /bin/cat /run/alibi-tun/samples
sample_after=$(cat /sys/class/net/aqtun0/statistics/rx_dropped)
printf 'TUN_AFTER_ALIBI phase=%s counter=%s uptime=' "$(cat /run/alibi-tun/phase)" "$sample_after"; cat /proc/uptime
if [ "$CASE_ID" = G14 ] && [ "$sample_before" -ne "$sample_after" ]; then
    echo 'CASE_F7 G14 counter-changed-during-collection'
fi
/bin/ip -details -statistics link show aqtun0
/bin/ip route show table all
/bin/dmesg --raw
touch /run/alibi-tun/stop
wait "$generator"
[ ! -e /sys/class/net/aqtun0 ] || { echo 'RESTORATION_FAILED tun-interface-retained'; exit 1; }
printf 'CASE_END %s independent-before=%s after=%s interface-retired\n' "$CASE_ID" "$sample_before" "$sample_after"
