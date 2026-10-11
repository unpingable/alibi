#!/bin/sh
# Sealed final evaluation: disclose only after candidate source/executable freeze.
set -u
export LC_ALL=C PATH=/bin:/sbin:/usr/bin:/usr/sbin
case " $(cat /proc/cmdline) " in *" alibi_run="*) ;; *) exit 64;; esac
[ "$(hostname)" = alibi-disposable-guest ] || exit 64
[ "$(/bin/stat -f -c %T /)" = tmpfs ] || exit 64
[ "$(id -u)" = 0 ] || exit 64
SELECTED=
for word in $(cat /proc/cmdline); do case "$word" in alibi_case=*) SELECTED=${word#alibi_case=};; esac; done
case "$SELECTED" in H01|H02|H03|H04|H05|H06) ;; *) exit 64;; esac
LAB=/run/alibi-holdout
[ ! -e "$LAB" ] || exit 64
mkdir "$LAB" "$LAB/mnt" "$LAB/commands" || exit 1
. /lib/alibi-lab.sh
IFACE=$(/bin/ip -o link show | awk -F': ' '$2!="lo" {sub(/@.*/,"",$2);print $2;exit}')
[ -n "$IFACE" ] || exit 125
/bin/ip link set lo up
/bin/ip link set "$IFACE" up
/bin/ip addr add 192.0.2.10/24 dev "$IFACE"
/bin/ip route add default dev "$IFACE"
echo 1 >/proc/sys/kernel/dmesg_restrict
stamp() { cat /proc/uptime; }
loadmods() {
 for pass in 1 2 3; do
  for name in "$@"; do [ ! -f "/modules/$name.ko" ] || insmod "/modules/$name.ko" 2>/dev/null || :; done
 done
 cat /proc/modules
}
observe() {
 printf 'HOLDOUT_ORACLE_BEGIN %s phase=%s uptime=' "$SELECTED" "$1"; stamp
 cat /proc/self/mountinfo
 /bin/df -P -B1; /bin/df -P -i
 /bin/stat -f / "$LAB/mnt"
 /bin/ip -details -statistics link show
 /bin/ip -4 route show table all; /bin/ip -6 route show table all
 cat /proc/net/route /proc/net/ipv6_route
 for path in /proc/sys/net/netfilter/nf_conntrack_count /proc/sys/net/netfilter/nf_conntrack_max; do
  printf 'SOURCE %s ' "$path"; cat "$path" 2>&1
 done
 ls -l /sys/devices/system/edac/mc
 /bin/dmesg
 printf 'HOLDOUT_ORACLE_END %s phase=%s uptime=' "$SELECTED" "$1"; stamp
}
run() { observe before; lab_run "$SELECTED" /bin/timeout 15 "$@" || exit 125; observe after; }
f7() { printf 'HOLDOUT_F7 %s %s\n' "$SELECTED" "$1"; }
make_loop() {
 dd if=/dev/zero of="$LAB/$1.img" bs=1048576 count=32 >/dev/null 2>&1 || return 1
 /bin/mke2fs -q -F -t ext4 -N 256 "$LAB/$1.img" || return 1
 /bin/losetup --find --show "$LAB/$1.img"
}
observe baseline
lab_run "$SELECTED-baseline" /bin/alibi check --json || exit 125
case "$SELECTED" in
H01)
 loadmods libcrc32c nf_defrag_ipv4 nf_defrag_ipv6 nf_conntrack nfnetlink nf_tables nft_ct
 if /bin/nft add table ip holdct && /bin/nft 'add chain ip holdct output { type filter hook output priority 0; policy accept; }' && /bin/nft 'add rule ip holdct output ct state new counter accept'; then
  /bin/bash -c 'printf x >/dev/udp/192.0.2.254/32123'
  count=$(cat /proc/sys/net/netfilter/nf_conntrack_count)
  maximum=$(cat /proc/sys/net/netfilter/nf_conntrack_max)
  printf 'INDEPENDENT_SUBSYSTEM count=%s max=%s\n' "$count" "$maximum"
  /bin/nft list table ip holdct
  if [ "$count" -gt 0 ]; then
   cat >"$LAB/pair-view.sh" <<'EOS'
#!/bin/sh
set -u
count=/proc/sys/net/netfilter/nf_conntrack_count
parent=/proc/sys/net/netfilter
# Bind the genuine live count file into the private view; only max is missing.
mount --make-rprivate / || exit 125
mkdir /run/alibi-holdout/view
: >/run/alibi-holdout/view/nf_conntrack_count
mount --bind "$count" /run/alibi-holdout/view/nf_conntrack_count || exit 125
mount --bind /run/alibi-holdout/view "$parent" || exit 125
printf 'INDEPENDENT_PAIR_VIEW count=' >&2
cat "$count" >&2
cat "$parent/nf_conntrack_max" >&2
rc=$?
printf 'INDEPENDENT_PAIR_VIEW max_rc=%s\n' "$rc" >&2
[ "$rc" -ne 0 ] || exit 125
exec /bin/alibi check --json network-choke-point
EOS
   run /bin/unshare -m /bin/sh "$LAB/pair-view.sh"
  else f7 'no independently established live conntrack entry'; fi
  /bin/nft delete table ip holdct
 else f7 'conntrack prerequisite unavailable'; fi
 ;;
H02)
 LOWER=$(make_loop lower) || { f7 'lower filesystem creation failed'; exit 0; }
 UPPER=$(make_loop upper) || { f7 'upper filesystem creation failed'; exit 0; }
 if mount -t ext4 "$LOWER" "$LAB/mnt"; then
  dd if=/dev/zero of="$LAB/mnt/full" bs=1048576 count=40 2>"$LAB/fill.err"; p=$?
  printf 'LOWER_FILL rc=%s\n' "$p"; cat "$LAB/fill.err"
  exec 9<"$LAB/mnt"
  /bin/stat -f -c 'LOWER blocks=%b available=%a inodes=%c free_inodes=%d' /proc/$$/fd/9
  if /bin/stat -f -c '%b %a' /proc/$$/fd/9 | awk '{exit !($1>0&&$2*20<=$1)}'; then
   if mount -t ext4 "$UPPER" "$LAB/mnt"; then
    /bin/stat -f -c 'VISIBLE blocks=%b available=%a inodes=%c free_inodes=%d' "$LAB/mnt"
    /bin/stat -f -c 'HIDDEN blocks=%b available=%a inodes=%c free_inodes=%d' /proc/$$/fd/9
    run /bin/alibi check --json silent-killer
    umount "$LAB/mnt" || exit 1
   else f7 'second mount failed'; fi
  else f7 'lower critical threshold absent'; fi
  exec 9<&-
  umount "$LAB/mnt" || exit 1
  /bin/losetup -d "$LOWER"; /bin/losetup -d "$UPPER"
  rm "$LAB/lower.img" "$LAB/upper.img"
 else f7 'first mount failed'; fi
 ;;
H03)
 loadmods veth
 if /bin/ip link add hdrop0 type veth peer name hdrop1 && /bin/ip addr add 192.0.3.1/24 dev hdrop0 && /bin/ip link set hdrop0 up && /bin/ip link set hdrop1 down && /bin/ip neigh add 192.0.3.2 lladdr 02:00:00:00:03:02 nud permanent dev hdrop0; then
  target=/sys/class/net/hdrop0/statistics/rx_errors
  cat "$target" >"$LAB/denied"; chmod 000 "$LAB/denied"
  cat >"$LAB/denied-view.sh" <<'EOS'
#!/bin/sh
mount --make-rprivate / || exit 125
mount --bind /run/alibi-holdout/denied /sys/class/net/hdrop0/statistics/rx_errors || exit 125
/bin/setpriv --reuid 65534 --regid 65534 --clear-groups --inh-caps +syslog --ambient-caps +syslog /bin/dmesg >/dev/null
logrc=$?
/bin/setpriv --reuid 65534 --regid 65534 --clear-groups --inh-caps +syslog --ambient-caps +syslog cat /sys/class/net/hdrop0/statistics/rx_errors >/dev/null
counterrc=$?
printf 'PERMISSION_PROOF log_rc=%s counter_rc=%s\n' "$logrc" "$counterrc" >&2
[ "$logrc" -eq 0 ] && [ "$counterrc" -ne 0 ] || exit 125
exec /bin/setpriv --reuid 65534 --regid 65534 --clear-groups --inh-caps +syslog --ambient-caps +syslog /bin/alibi check --json network-choke-point
EOS
  observe before
  (sleep 0.5; /bin/bash -c 'for ((n=32000;n<32048;n++)); do printf x >/dev/udp/192.0.3.2/$n; printf "DROP_SAMPLE uptime="; cat /proc/uptime; cat /sys/class/net/hdrop0/statistics/tx_dropped; sleep 0.05; done') >"$LAB/drop-ledger" 2>&1 &
  ACTOR=$!
  lab_run "$SELECTED" /bin/timeout 15 /bin/unshare -m /bin/sh "$LAB/denied-view.sh" || exit 125
  wait "$ACTOR"; actorrc=$?
  printf 'DROP_ACTOR rc=%s\n' "$actorrc"; cat "$LAB/drop-ledger"
  observe after
  /bin/ip link del hdrop0
 else f7 'drop fixture setup unavailable'; fi
 ;;
H04)
 # Real-controller registration cannot occur on this EDAC-less guest.
 # Replay independently sourced informational registration text as a labeled fixture.
 printf '#!/bin/sh\nprintf '\''%%s\\n'\'' '\''<6>[    0.000000] EDAC MC0: Giving out device to module fixture_edac controller fixture: DEV fixture (POLLED)'\''\n' >"$LAB/commands/dmesg"
 printf '#!/bin/sh\nexit 1\n' >"$LAB/commands/journalctl"
 chmod 700 "$LAB/commands/dmesg" "$LAB/commands/journalctl"
 printf 'HOLDOUT_KIND H04 command-substitution-no-real-memory-event\n'
 sha256sum "$LAB/commands/dmesg" "$LAB/commands/journalctl"
 cat "$LAB/commands/dmesg"
 run env PATH="$LAB/commands:/bin:/sbin:/usr/bin:/usr/sbin" /bin/alibi check --json memrot
 ;;
H05)
 loadmods veth
 if /bin/ip link add hsame0 type veth peer name hsame1; then
  /bin/ip link set hsame0 up; /bin/ip link set hsame1 up
  observe before
  (
   sleep 0.8
   printf 'REPLACEMENT_BEFORE uptime='; stamp
   cat /sys/class/net/hsame0/ifindex /sys/class/net/hsame1/ifindex
   /bin/ip link del hsame0
   /bin/ip link add hsame0 type veth peer name hsame1
   /bin/ip link set hsame0 up; /bin/ip link set hsame1 up
   printf 'REPLACEMENT_AFTER uptime='; stamp
   cat /sys/class/net/hsame0/ifindex /sys/class/net/hsame1/ifindex
  ) >"$LAB/replacement" 2>&1 &
  ACTOR=$!
  lab_run "$SELECTED" /bin/timeout 15 /bin/alibi check --json network-choke-point || exit 125
  wait "$ACTOR"; p=$?; printf 'REPLACEMENT_ACTOR rc=%s\n' "$p"
  cat "$LAB/replacement"
  observe after
  /bin/ip link del hsame0
 else f7 'veth unavailable'; fi
 ;;
H06)
 /bin/ip -4 route flush default; /bin/ip -6 route flush default
 /bin/setpriv --reuid 65534 --regid 65534 --clear-groups /bin/dmesg >"$LAB/log.stdout" 2>"$LAB/log.stderr"; p=$?
 printf 'DENIED_LOG_PROOF rc=%s\n' "$p"; cat "$LAB/log.stderr"
 observe before
 lab_run H06-human /bin/timeout 15 /bin/setpriv --reuid 65534 --regid 65534 --clear-groups /bin/alibi check network-choke-point || exit 125
 lab_run H06-json /bin/timeout 15 /bin/setpriv --reuid 65534 --regid 65534 --clear-groups /bin/alibi check --json network-choke-point || exit 125
 observe after
 ;;
esac
printf 'HOLDOUT_RESTORATION guest-termination-retires-exact-substrate\n'
cat /proc/self/mountinfo
/bin/losetup -a
printf 'HOLDOUT_COMPLETE %s\n' "$SELECTED"
