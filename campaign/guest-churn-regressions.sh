#!/bin/sh
# Disclosed successor regression cases. Mutations are guest-only.
set -u
export LC_ALL=C PATH=/bin:/sbin:/usr/bin:/usr/sbin
case " $(cat /proc/cmdline) " in *" alibi_run="*) ;; *) exit 64;; esac
[ "$(hostname)" = alibi-disposable-guest ] && [ "$(id -u)" = 0 ] || exit 64
[ "$(stat -f -c %T /)" = tmpfs ] || exit 64
SELECTED=
for word in $(cat /proc/cmdline); do case "$word" in alibi_case=*) SELECTED=${word#alibi_case=};; esac; done
case "$SELECTED" in C01|C02|C03|C04) ;; *) exit 64;; esac
. /lib/alibi-lab.sh
LAB=/run/alibi-churn
mkdir "$LAB" || exit 64
stamp() { cat /proc/uptime; }
observe() {
 printf 'CHURN_OBSERVE %s uptime=' "$1"; stamp
 /bin/ip -details -statistics link show
 /bin/ip -4 route show table all; /bin/ip -6 route show table all
 cat /proc/net/route /proc/net/ipv6_route
 for n in /sys/class/net/*; do
  printf 'DEVICE %s ' "$n"; cat "$n/ifindex"
  for k in rx_packets tx_packets rx_errors tx_errors rx_dropped tx_dropped; do printf '%s=' "$k"; cat "$n/statistics/$k"; done
 done
 /bin/dmesg
}
IFACE=$(/bin/ip -o link show | awk -F': ' '$2!="lo" {sub(/@.*/,"",$2);print $2;exit}')
[ -n "$IFACE" ] || exit 125
/bin/ip link set lo up
/bin/ip link set "$IFACE" up
/bin/ip addr add 192.0.2.10/24 dev "$IFACE"
/bin/ip route add default dev "$IFACE"
insmod /modules/veth.ko || exit 125
/bin/ip link add aq0 type veth peer name aq1 || exit 125
/bin/ip link set aq0 up; /bin/ip link set aq1 up
old0=$(cat /sys/class/net/aq0/ifindex); old1=$(cat /sys/class/net/aq1/ifindex)
mac0=$(cat /sys/class/net/aq0/address); mac1=$(cat /sys/class/net/aq1/address)
observe baseline
lab_run "$SELECTED-baseline" /bin/alibi check --json network-choke-point || exit 125
replace_same_index() {
 printf 'REPLACE_START uptime='; stamp
 printf 'OLD_INDICES %s %s MACS %s %s\n' "$old0" "$old1" "$mac0" "$mac1"
 /bin/ip link del aq0 || return 1
 /bin/ip link add aq0 index "$old0" type veth peer name aq1 index "$old1" || return 1
 /bin/ip link set aq0 address "$mac0" && /bin/ip link set aq1 address "$mac1" || return 1
 /bin/ip link set aq0 up && /bin/ip link set aq1 up || return 1
 printf 'NEW_INDICES '; cat /sys/class/net/aq0/ifindex /sys/class/net/aq1/ifindex
 printf 'REPLACE_END uptime='; stamp
}
if [ "$SELECTED" = C03 ]; then
 replace_same_index || { echo 'CHURN_F7 replacement failed'; exit 125; }
 observe before
 lab_run "$SELECTED" /bin/alibi check --json network-choke-point || exit 125
else
 if [ "$SELECTED" = C04 ]; then
  mkdir -p /var/run/netns
  /bin/ip netns add aqspace || { echo 'CHURN_F7 netns unavailable'; exit 125; }
 fi
 observe before
 (
  sleep 0.8
  case "$SELECTED" in
  C01) replace_same_index ;;
  C02)
   printf 'RENAME_START uptime='; stamp
   /bin/ip link set aq0 name aqold && /bin/ip link add aq0 type veth peer name aq2 && /bin/ip link set aq0 up && /bin/ip link set aq2 up || exit 1
   /bin/ip -o link show
   printf 'RENAME_END uptime='; stamp
   ;;
  C04)
   printf 'MOVE_START uptime='; stamp
   cat /sys/class/net/aq0/ifindex
   /bin/ip link set aq0 netns aqspace || exit 1
   /bin/ip -n aqspace -o link show
   sleep 0.2
   /bin/ip -n aqspace link set aq0 netns 1 || exit 1
   /bin/ip link set aq0 up || exit 1
   cat /sys/class/net/aq0/ifindex
   printf 'MOVE_END uptime='; stamp
   ;;
  esac
 ) >"$LAB/transition" 2>&1 &
 actor=$!
 lab_run "$SELECTED" /bin/alibi check --json network-choke-point || exit 125
 wait "$actor"; rc=$?
 printf 'TRANSITION_EXIT %s\n' "$rc"; cat "$LAB/transition"
 [ "$rc" -eq 0 ] || { echo 'CHURN_F7 intended transition failed'; exit 125; }
fi
observe after
/bin/ip link del aq0 || exit 125
if [ "$SELECTED" = C02 ]; then /bin/ip link del aqold || exit 125; fi
if [ "$SELECTED" = C04 ]; then /bin/ip netns del aqspace || exit 125; fi
observe restored
lab_run "$SELECTED-restored" /bin/alibi check --json network-choke-point || exit 125
printf 'CHURN_COMPLETE %s\n' "$SELECTED"
