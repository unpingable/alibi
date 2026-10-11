#!/bin/sh
# Final sealed successor evaluation. Guest-only; no self-asserted verdicts.
set -u
export LC_ALL=C PATH=/bin:/sbin:/usr/bin:/usr/sbin
case " $(cat /proc/cmdline) " in *" alibi_run="*) ;; *) exit 64;; esac
[ "$(hostname)" = alibi-disposable-guest ] || exit 64
[ "$(/bin/stat -f -c %T /)" = tmpfs ] || exit 64
[ "$(id -u)" = 0 ] || exit 64
CASE_ID=
for word in $(cat /proc/cmdline); do case "$word" in alibi_case=*) CASE_ID=${word#alibi_case=};; esac; done
case "$CASE_ID" in Q01|Q02|Q03|Q04|Q05|Q06|Q07|Q08|Q09|Q10) ;; *) exit 64;; esac
LAB=/run/alibi-new-holdout
[ ! -e "$LAB" ] || exit 64
mkdir "$LAB" "$LAB/mnt" "$LAB/commands" || exit 1
. /lib/alibi-lab.sh
stamp() { cat /proc/uptime; }
f7() { printf 'QUALIFICATION_GAP %s %s\n' "$CASE_ID" "$1"; }
loadmods() {
 for name in "$@"; do
  [ ! -f "/modules/$name.ko" ] || insmod "/modules/$name.ko" 2>/dev/null || :
 done
 cat /proc/modules
}
IFACE=$(/bin/ip -o link show | awk -F': ' '$2!="lo" {sub(/@.*/,"",$2);print $2;exit}')
[ -n "$IFACE" ] || { f7 'isolated NIC absent'; exit 0; }
/bin/ip link set lo up || exit 1
/bin/ip link set "$IFACE" up || exit 1
/bin/ip addr add 192.0.2.10/24 dev "$IFACE" || exit 1
/bin/ip route add default dev "$IFACE" || exit 1
echo 1 >/proc/sys/kernel/dmesg_restrict
observe() {
 printf 'ORACLE_BEGIN %s %s uptime=' "$CASE_ID" "$1"; stamp
 cat /proc/self/mountinfo
 /bin/df -P -B1; /bin/df -P -i
 /bin/stat -f -c 'STATFS target=%n id=%i type=%T blocks=%b available=%a inodes=%c free_inodes=%d' / "$LAB/mnt"
 /bin/ip -details -statistics link show
 for dir in /sys/class/net/*; do
  printf 'IDENTITY %s ' "${dir##*/}"; cat "$dir/ifindex" "$dir/iflink"
  for counter in rx_errors rx_dropped tx_errors tx_dropped; do printf '%s=' "$counter"; cat "$dir/statistics/$counter"; done
 done
 /bin/ip -4 route show table all; /bin/ip -6 route show table all
 cat /proc/net/route /proc/net/ipv6_route
 for source in /proc/sys/net/netfilter/nf_conntrack_count /proc/sys/net/netfilter/nf_conntrack_max; do printf 'CONNTRACK %s ' "$source"; cat "$source" 2>&1; done
 ls -l /sys/devices/system/edac/mc
 /bin/dmesg --raw --force-prefix
 printf 'ORACLE_END %s %s uptime=' "$CASE_ID" "$1"; stamp
}
jsonrun() { lab_run "$CASE_ID" /bin/timeout 15 "$@" || exit 125; }
ct_setup() {
 loadmods libcrc32c nf_defrag_ipv4 nf_defrag_ipv6 nf_conntrack nfnetlink nf_tables nft_ct
 /bin/nft add table ip qct || return 1
 /bin/nft 'add chain ip qct output { type filter hook output priority 0; policy accept; }' || return 1
 /bin/nft 'add rule ip qct output ct state new counter accept' || return 1
}
ct_proof() {
 printf 'CONNTRACK_PROOF uptime='; stamp
 cat /proc/sys/net/netfilter/nf_conntrack_count /proc/sys/net/netfilter/nf_conntrack_max
 /bin/nft list table ip qct
}
veth_setup() {
 loadmods veth
 /bin/ip link add qleft type veth peer name qright || return 1
 /bin/ip link set qleft up || return 1
 /bin/ip link set qright up || return 1
}
identities() {
 printf 'IDENTITY_PROOF uptime='; stamp
 /bin/ip -o link show
 for name in qleft qright qtemp; do
  [ ! -e "/sys/class/net/$name/ifindex" ] || { printf '%s=' "$name"; cat "/sys/class/net/$name/ifindex"; }
 done
}
benign_commands() {
 cat >"$LAB/commands/dmesg" <<'EOS'
#!/bin/sh
printf '%s\n' '<6>[    0.010000] EDAC amd64: Hardware accepted DRAM ECC Enable'
EOS
 printf '#!/bin/sh\nexit 1\n' >"$LAB/commands/journalctl"
 chmod 700 "$LAB/commands/dmesg" "$LAB/commands/journalctl"
 printf 'COMMAND_SUBSTITUTION informational-capability-initialization\n'
 sha256sum "$LAB/commands/dmesg" "$LAB/commands/journalctl"
 cat "$LAB/commands/dmesg"
}
observe baseline
lab_run "$CASE_ID-baseline" /bin/alibi check --json || exit 125
case "$CASE_ID" in
Q01)
 if ct_setup; then
  echo 4096 >/proc/sys/net/netfilter/nf_conntrack_max
  /bin/bash -c 'for port in 31001 31002 31003 31004; do printf x >/dev/udp/192.0.2.254/$port; done'
  ct_proof
  count=$(cat /proc/sys/net/netfilter/nf_conntrack_count)
  if [ "$count" -gt 0 ] && [ "$count" -lt 100 ]; then
   observe before
   jsonrun /bin/alibi check --json kernel-scream silent-killer network-choke-point
   lab_run Q01-human /bin/alibi check kernel-scream silent-killer network-choke-point || exit 125
   observe after; ct_proof
  else f7 'live low-pressure conntrack condition absent'; fi
  /bin/nft delete table ip qct
 else f7 'conntrack setup unavailable'; fi
 ;;
Q02)
 if veth_setup; then
  observe before; identities
  (sleep .8; printf 'STATE_DOWN_BEGIN uptime='; stamp; /bin/ip link set qleft down; a=$?; identities; sleep .2; /bin/ip link set qleft up; b=$?; printf 'STATE_UP_END down_rc=%s up_rc=%s uptime=' "$a" "$b"; stamp; identities) >"$LAB/transition" 2>&1 &
  actor=$!
  jsonrun /bin/alibi check --json network-choke-point
  wait "$actor"; printf 'ACTOR_STATUS %s\n' "$?"
  lab_run Q02-transition /bin/cat "$LAB/transition" || exit 125
  observe after; /bin/ip link del qleft
 else f7 'veth unavailable'; fi
 ;;
Q03)
 if veth_setup; then
  observe before; identities
  (
   sleep .8
   printf 'PERMUTATION_BEGIN uptime='; stamp; identities
   /bin/ip link set qleft down && /bin/ip link set qright down &&
   /bin/ip link set qleft name qtemp && /bin/ip link set qright name qleft && /bin/ip link set qtemp name qright &&
   /bin/ip link set qleft up && /bin/ip link set qright up
   rc=$?; printf 'PERMUTATION_END rc=%s uptime=' "$rc"; stamp; identities
  ) >"$LAB/transition" 2>&1 &
  actor=$!
  jsonrun /bin/alibi check --json network-choke-point
  wait "$actor"; printf 'ACTOR_STATUS %s\n' "$?"
  lab_run Q03-transition /bin/cat "$LAB/transition" || exit 125
  observe after; /bin/ip link del qleft
 else f7 'veth unavailable'; fi
 ;;
Q04)
 /bin/ip -4 route flush default; /bin/ip -6 route flush default
 printf 'not-an-integer\n' >"$LAB/ifindex"
 printf '%s\n' "$IFACE" >"$LAB/interface"
 cat >"$LAB/view.sh" <<'EOS'
#!/bin/sh
set -u
mount --make-rprivate / || exit 125
iface=$(cat /run/alibi-new-holdout/interface)
mount --bind /run/alibi-new-holdout/ifindex "/sys/class/net/$iface/ifindex" || exit 125
printf 'INDEPENDENT_IDENTITY_VIEW interface=%s value=' "$iface" >&2
cat "/sys/class/net/$iface/ifindex" >&2
exec /bin/alibi check "$@" network-choke-point
EOS
 observe before
 jsonrun /bin/unshare -m /bin/sh "$LAB/view.sh" --json
 lab_run Q04-human /bin/unshare -m /bin/sh "$LAB/view.sh" || exit 125
 observe after
 ;;
Q05)
 if mount -t tmpfs -o size=1048576,nr_inodes=128 q-full "$LAB/mnt"; then
  /bin/dd if=/dev/zero of="$LAB/mnt/fill" bs=65536 count=20 >"$LAB/fill.out" 2>"$LAB/fill.err"; rc=$?
  printf 'ACTUAL_FILL rc=%s\n' "$rc"; cat "$LAB/fill.err"
  /bin/stat -f -c 'FULL blocks=%b available=%a inodes=%c free=%d' "$LAB/mnt"
  if /bin/stat -f -c '%b %a' "$LAB/mnt" | awk '{exit !($1>0&&$2*20<=$1)}'; then
   observe before
   jsonrun /bin/alibi check --json silent-killer network-choke-point
   lab_run Q05-human /bin/alibi check silent-killer network-choke-point || exit 125
   observe after
  else f7 'capacity threshold absent'; fi
  umount "$LAB/mnt"
 else f7 'tmpfs unavailable'; fi
 ;;
Q06)
 if ct_setup; then
  echo 128 >/proc/sys/net/netfilter/nf_conntrack_max
  /bin/bash -c 'for ((port=32000;port<32120;port++)); do printf x >/dev/udp/192.0.2.254/$port; done'
  ct_proof
  count=$(cat /proc/sys/net/netfilter/nf_conntrack_count); maximum=$(cat /proc/sys/net/netfilter/nf_conntrack_max)
  if [ "$((count*10))" -ge "$((maximum*9))" ] && [ "$count" -lt "$maximum" ]; then
   observe before
   jsonrun /bin/alibi check --json silent-killer memrot network-choke-point
   lab_run Q06-human /bin/alibi check silent-killer memrot network-choke-point || exit 125
   observe after; ct_proof
  else f7 'conntrack pressure absent'; fi
  /bin/nft delete table ip qct
 else f7 'conntrack setup unavailable'; fi
 ;;
Q07)
 benign_commands
 observe before
 jsonrun env PATH="$LAB/commands:/bin:/sbin:/usr/bin:/usr/sbin" /bin/alibi check --json kernel-scream memrot
 lab_run Q07-human env PATH="$LAB/commands:/bin:/sbin:/usr/bin:/usr/sbin" /bin/alibi check kernel-scream memrot || exit 125
 observe after
 ;;
Q08)
 /bin/dd if=/dev/zero of="$LAB/fs.img" bs=1048576 count=32 >/dev/null 2>&1 || exit 1
 if /bin/mke2fs -q -F -t ext4 -N 256 "$LAB/fs.img"; then
  device=$(/bin/losetup --find --show "$LAB/fs.img") || exit 1
  if mount -t ext4 "$device" "$LAB/mnt" && mount -o remount,ro "$LAB/mnt"; then
   touch "$LAB/mnt/independent-write" 2>"$LAB/ro.err"; rc=$?
   printf 'READ_ONLY_WRITE rc=%s\n' "$rc"; cat "$LAB/ro.err"
   cat >"$LAB/commands/df" <<'EOS'
#!/bin/sh
for arg in "$@"; do
 if [ "$arg" = / ]; then printf 'campaign df evidence unavailable for root\n' >&2; exit 13; fi
done
exec /bin/df "$@"
EOS
   chmod 700 "$LAB/commands/df"
   printf 'COMMAND_SUBSTITUTION root-df-unavailable-other-targets-genuine\n'; cat "$LAB/commands/df"
   if [ "$rc" -ne 0 ]; then
    observe before
    jsonrun env PATH="$LAB/commands:/bin:/sbin:/usr/bin:/usr/sbin" /bin/alibi check --json kernel-scream silent-killer
    lab_run Q08-human env PATH="$LAB/commands:/bin:/sbin:/usr/bin:/usr/sbin" /bin/alibi check kernel-scream silent-killer || exit 125
    observe after
   else f7 'read-only write did not fail'; fi
   umount "$LAB/mnt" || exit 1
  else f7 'read-only mount unavailable'; fi
  /bin/losetup -d "$device"; rm "$LAB/fs.img"
 else f7 'ext4 creation unavailable'; fi
 ;;
Q09)
 benign_commands
 mkdir /run/alibi-tun
 mkdir -p /dev/net
 [ -e /dev/net/tun ] || /bin/busybox mknod /dev/net/tun c 10 200
 lab_run Q09-generator /bin/tun-drop-fixture 20 20 &
 generator=$!
 waitphase() {
  expected=$1; n=0
  while [ "$n" -lt 1000 ]; do
   [ ! -f /run/alibi-tun/phase ] || [ "$(cat /run/alibi-tun/phase)" != "$expected" ] || return 0
   sleep .01; n=$((n+1))
  done
  return 1
 }
 if waitphase READY; then
  /bin/ip link set aqtun0 up
  observe before
  touch /run/alibi-tun/go
  if waitphase EMITTING; then
   (
    n=0
    while [ ! -e "$LAB/stop-sampler" ] && [ "$n" -lt 300 ]; do
     printf 'TUN_SAMPLE uptime='; stamp
     printf 'counter='; cat /sys/class/net/aqtun0/statistics/rx_dropped
     sleep .05; n=$((n+1))
    done
   ) >"$LAB/samples" &
   sampler=$!
   jsonrun env PATH="$LAB/commands:/bin:/sbin:/usr/bin:/usr/sbin" /bin/alibi check --json memrot network-choke-point
   touch "$LAB/stop-sampler"; wait "$sampler"
   lab_run Q09-counter-trace /bin/cat "$LAB/samples" || exit 125
   observe after
  else f7 'TUN emitting phase absent'; fi
 else f7 'TUN ready phase absent'; fi
 touch /run/alibi-tun/stop; wait "$generator"
 ;;
Q10)
 printf '%s\n' "$IFACE" >"$LAB/interface"
 printf '25\n' >"$LAB/counter"
 cat >"$LAB/view.sh" <<'EOS'
#!/bin/sh
set -u
mount --make-rprivate / || exit 125
iface=$(cat /run/alibi-new-holdout/interface)
mount --bind /run/alibi-new-holdout/counter "/sys/class/net/$iface/statistics/rx_dropped" || exit 125
printf 'COUNTER_VIEW_BEFORE interface=%s value=' "$iface" >&2; cat "/sys/class/net/$iface/statistics/rx_dropped" >&2
(
 sleep .8
 printf 'COUNTER_CHANGE_BEGIN uptime='; cat /proc/uptime
 printf '0\n' >/run/alibi-new-holdout/counter
 printf 'COUNTER_CHANGE_END uptime='; cat /proc/uptime
 printf 'COUNTER_VIEW_AFTER='; cat "/sys/class/net/$iface/statistics/rx_dropped"
) >/run/alibi-new-holdout/change &
actor=$!
/bin/alibi check --json network-choke-point
rc=$?
wait "$actor"
cat /run/alibi-new-holdout/change >&2
exit "$rc"
EOS
 observe before
 jsonrun /bin/unshare -m /bin/sh "$LAB/view.sh"
 observe after
 ;;
esac
printf 'CASE_RESTORATION guest-termination-retires-private-views\n'
cat /proc/self/mountinfo
/bin/losetup -a
printf 'CASE_COMPLETE %s\n' "$CASE_ID"
