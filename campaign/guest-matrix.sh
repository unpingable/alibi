#!/bin/sh
# Campaign-owned disposable guest only. This script never launches a VM.
# Specification: docs/qualification-20261011/TEST-MATRIX.md (G01-G38).
set -u
export LC_ALL=C
case " $(cat /proc/cmdline) " in *" alibi_run="*) ;; *) echo 'REFUSED: no guest boot identity'; exit 64;; esac
[ "$(hostname)" = alibi-disposable-guest ] || exit 64
SELECTED=
for word in $(cat /proc/cmdline); do case "$word" in alibi_case=*) SELECTED=${word#alibi_case=};; esac; done
[ -n "$SELECTED" ] || { echo "REFUSED: missing alibi_case"; exit 64; }
ROOT_TYPE=$(awk '$2=="/" {type=$3} END {print type}' /proc/self/mounts)
case "$ROOT_TYPE:$SELECTED" in tmpfs:*|overlay:G22) ;; *) echo 'REFUSED: root model mismatch'; exit 64;; esac
[ "$(id -u)" = 0 ] || exit 64
[ ! -e /run/alibi-matrix ] || { echo 'REFUSED: existing matrix occurrence'; exit 64; }
mkdir /run/alibi-matrix || exit 1
LAB=/run/alibi-matrix
BIN=$(command -v alibi)
IFACE=$(/bin/ip -o link show | awk -F': ' '$2 != "lo" {sub(/@.*/,"",$2);print $2;exit}')
[ -n "$IFACE" ] || [ "$SELECTED" = G02 ] || { echo 'MATRIX_F7 no-isolated-NIC'; exit 1; }
printf 'MATRIX_BEGIN run=%s root=%s baseline-adaptation=declared\n' "${RUN_ID:-unknown}" "$ROOT_TYPE"
uname -a
sha256sum "$BIN"
# Only supplied, campaign-selected modules are considered; no host devices exist.
for pass in 1 2 3; do
  for module in /modules/*.ko; do
    [ -f "$module" ] || continue
    case "$module" in *crc16.ko|*mbcache.ko|*jbd2.ko|*ext4.ko|*loop.ko|*dummy.ko|*veth.ko)
      case "$module" in
        */dummy.ko) insmod "$module" numdummies=0 2>/dev/null || :;;
        *) insmod "$module" 2>/dev/null || :;;
      esac;;
    esac
  done
done
cat /proc/modules
/bin/ip link set lo up
if [ -n "$IFACE" ]; then
  /bin/ip link set "$IFACE" up
  /bin/ip addr add 192.0.2.10/24 dev "$IFACE" 2>/dev/null || :
fi
reset_route() {
  [ -n "$IFACE" ] || return 0
  /bin/ip -4 route flush default 2>/dev/null || :
  /bin/ip -6 route flush default 2>/dev/null || :
  /bin/ip -4 route add default dev "$IFACE"
}
reset_route || exit 1
stamp() { cat /proc/uptime; }
f7() { printf '\nCASE_BEGIN %s kind=unexecuted\nCASE_F7 %s %s\nCASE_END %s disposition=F7\n' "$1" "$1" "$2" "$1"; }
observe() {
  printf 'ORACLE_BEGIN %s phase=%s uptime=' "$CASE" "$1"; stamp
  id
  cat /proc/self/mountinfo
  /bin/df -P -B1
  /bin/df -P -i
  for path in / "$LAB/mnt" "$LAB/bind" "$LAB/small"; do
    [ -d "$path" ] && /bin/stat -f -c 'STATFS path=%n type=%T blocks=%b free=%f available=%a blocksize=%S inodes=%c free_inodes=%d' "$path"
  done
  /bin/ip -details -statistics link show
  /bin/ip -4 route show table all
  /bin/ip -6 route show table all
  /bin/ip neigh show
  cat /proc/net/route /proc/net/ipv6_route
  for path in /proc/sys/net/netfilter/nf_conntrack_count /proc/sys/net/netfilter/nf_conntrack_max; do
    if [ -e "$path" ]; then printf 'SOURCE %s ' "$path"; cat "$path"; else printf 'SOURCE_ABSENT %s\n' "$path"; fi
  done
  ls -l /sys/devices/system/edac/mc 2>&1 || :
  dmesg
  printf 'ORACLE_END %s phase=%s uptime=' "$CASE" "$1"; stamp
}
start() {
  CASE=$1
  printf '\nCASE_BEGIN %s kind=%s expected=%s\n' "$CASE" "$2" "$3"
}
# Arguments are an actual CLI invocation. Nonzero verdict status is evidence.
run() {
  observe before
  printf 'ALIBI_RECEIPT_BEGIN %s uptime=' "$CASE"; stamp
  lab_run "$CASE" /bin/timeout 15 "$@" || { echo "CAPTURE_FAILED $CASE"; exit 125; }
  printf '\nALIBI_RECEIPT_END %s uptime=' "$CASE"; stamp
  observe after
}
finish() { printf 'CASE_END %s disposition=EXECUTED_UNADJUDICATED\n' "$CASE"; }
plain() { start "$1" "$2" "$3"; shift 3; run "$BIN" check --json "$@"; finish; }
missing_defaults() {
  /bin/ip -4 route flush default && /bin/ip -6 route flush default
}
mkdir "$LAB/mnt" "$LAB/bind" "$LAB/small"
LOOP=
new_ext4() {
  [ -z "$LOOP" ] || return 1
  command -v mke2fs >/dev/null && command -v losetup >/dev/null || return 1
  dd if=/dev/zero of="$LAB/ext4.img" bs=1048576 count=32 >/dev/null 2>&1 || return 1
  /bin/mke2fs -q -F -t ext4 -N 256 "$LAB/ext4.img" || return 1
  LOOP=$(/bin/losetup --find --show "$LAB/ext4.img") || { LOOP=; return 1; }
  mount -t ext4 "$LOOP" "$LAB/mnt" || { /bin/losetup -d "$LOOP"; LOOP=; return 1; }
}
retire_ext4() {
  umount "$LAB/bind" 2>/dev/null || :
  if [ -n "$LOOP" ]; then
    umount "$LAB/mnt" || { echo 'RESTORATION_FAILED mounted-loop'; exit 1; }
    /bin/losetup -d "$LOOP" || { echo 'RESTORATION_FAILED loop-detach'; exit 1; }
    LOOP=
  fi
  rm -f "$LAB/ext4.img"
}
fill_blocks() {
  dd if=/dev/zero of="$LAB/mnt/fill" bs=1048576 count=40 2>"$LAB/fill.err"; code=$?
  printf 'INDEPENDENT_FILL rc=%s\n' "$code"; cat "$LAB/fill.err"
  /bin/stat -f -c '%b %a' "$LAB/mnt" | awk '{exit !($1>0 && $2*20<=$1)}'
}
mkdir "$LAB/commands"
fixture() {
  printf '#!/bin/sh\n%s\n' "$2" >"$LAB/commands/$1"
  chmod 700 "$LAB/commands/$1"
}
clear_fixtures() { rm -f "$LAB/commands/df" "$LAB/commands/dmesg" "$LAB/commands/journalctl"; }
load_selected_modules() {
  for module_pass in 1 2 3 4; do
    for module_name in "$@"; do
      [ ! -f "/modules/$module_name.ko" ] || insmod "/modules/$module_name.ko" 2>/dev/null || :
    done
  done
  cat /proc/modules
}

setup_drop_link() {
  command -v bash >/dev/null || return 1
  load_selected_modules veth
  /bin/ip link add aqdrop0 type veth peer name aqdrop1 || return 1
  /bin/ip addr add 192.0.3.1/24 dev aqdrop0 || return 1
  /bin/ip link set aqdrop0 up || return 1
  /bin/ip link set aqdrop1 down || return 1
  /bin/ip neigh add 192.0.3.2 lladdr 02:00:00:00:03:02 nud permanent dev aqdrop0 || return 1
}
write_drop_sender() {
  cat >"$LAB/drop-sender" <<'EOS'
#!/bin/bash
for ((n=31000;n<31048;n++)); do
  printf x > /dev/udp/192.0.3.2/$n
  printf 'INDEPENDENT_DROP_SAMPLE uptime='
  cat /proc/uptime
  printf 'tx_dropped='
  cat /sys/class/net/aqdrop0/statistics/tx_dropped
  sleep 0.05
done
EOS
}

case_G01() {
plain G01 real-guest-control conditional-C-K-S-N kernel-scream silent-killer network-choke-point
}

case_G02() {
if [ -z "$IFACE" ]; then
  plain G02 real-guest-applicability NA network-choke-point
else f7 G02 'Controller must use --isolated-nics 0 for a genuinely NIC-less guest'; fi
}

case_G03() {
if command -v setpriv >/dev/null; then
  echo 1 >/proc/sys/kernel/dmesg_restrict
  start G03 real-guest-permission U-all-without-other-positives
  /bin/setpriv --reuid 65534 --regid 65534 --clear-groups /bin/dmesg >"$LAB/access.stdout" 2>"$LAB/access.stderr"; a=$?
  printf 'INDEPENDENT_LOG_ACCESS rc=%s\n' "$a"; cat "$LAB/access.stderr"
  run /bin/setpriv --reuid 65534 --regid 65534 --clear-groups "$BIN" check --json
  finish
else f7 G03 'setpriv unavailable'; fi
}

case_G04() {
plain G04 real-guest-applicability U-if-EDAC-absent memrot
}

case_G05() {
if missing_defaults; then plain G05 real-guest-route D-if-tables-complete network-choke-point; else f7 G05 'default removal failed'; fi
}

case_G06() {
missing_defaults || { f7 G06 "route removal failed"; return; }
if /bin/ip route add unreachable default; then plain G06 real-guest-route D-if-tables-complete network-choke-point; else f7 G06 'unreachable route unsupported'; fi
}

case_G07() {
missing_defaults
if /bin/ip route add blackhole default; then plain G07 real-guest-route D-if-tables-complete network-choke-point; else f7 G07 'blackhole route unsupported'; fi
}

case_G08() {
if /bin/ip link add aqdummy type dummy; then
  /bin/ip link set aqdummy down
  plain G08 real-guest-interface bounded-C-allowed network-choke-point
  /bin/ip link del aqdummy
else f7 G08 'dummy link unavailable'; fi
}

case_G09() {
f7 G09 'Emulated physical carrier toggle requires controller synchronization; no physical-carrier claim from veth'
}

case_G10() {
missing_defaults
if /bin/ip route add default via 192.0.2.254 dev "$IFACE"; then
  start G10 real-guest-next-hop bounded-C-allowed
  /bin/timeout 3 ping -c 1 -W 1 192.0.2.254; p=$?
  printf 'INDEPENDENT_NEIGHBOR_PROBE rc=%s\n' "$p"; /bin/ip neigh show
  run "$BIN" check --json network-choke-point
  finish
else f7 G10 'unreachable next-hop setup failed'; fi
}

case_G11() {
# Resolver and probes are guest-local; there is no host external network backend.
echo 'nameserver 192.0.2.253' >/etc/resolv.conf
if command -v nslookup >/dev/null; then
  start G11 real-guest-dns bounded-C-allowed
  cat /etc/resolv.conf
  /bin/timeout 3 nslookup alibi-qualification.invalid; p=$?
  printf 'INDEPENDENT_DNS_PROBE rc=%s\n' "$p"
  run "$BIN" check --json network-choke-point
  finish
else f7 G11 'resolver query utility absent'; fi
}

case_G12() {
load_selected_modules nfnetlink nf_tables nft_counter
if command -v nft >/dev/null && /bin/nft add table inet aqfilter && /bin/nft 'add chain inet aqfilter output { type filter hook output priority 0; policy accept; }' && /bin/nft 'add rule inet aqfilter output ip daddr 127.0.0.1 counter drop'; then
  start G12 real-guest-filter bounded-C-allowed-if-no-named-positive
  /bin/timeout 3 ping -c 1 -W 1 127.0.0.1 >"$LAB/filter-probe" 2>&1; p=$?
  printf 'INDEPENDENT_FILTER_PROBE rc=%s\n' "$p"; cat "$LAB/filter-probe"
  /bin/nft -a list table inet aqfilter >"$LAB/filter-rules"; cat "$LAB/filter-rules"
  if grep -Eq 'counter packets [1-9][0-9]*' "$LAB/filter-rules"; then
    run "$BIN" check --json network-choke-point; finish
  else f7 G12 'rule installed but independent packet counter did not show actual filtering'; fi
  /bin/nft delete table inet aqfilter
else f7 G12 'nft filter setup failed'; fi
}

case_G13() {
if setup_drop_link; then
  start G13 real-guest-interface-drops D-only-if-independent-delta-overlaps-sample
  write_drop_sender
  observe before
  before=$(cat /sys/class/net/aqdrop0/statistics/tx_dropped)
  (sleep 0.3; /bin/timeout 6 /bin/bash "$LAB/drop-sender") >"$LAB/drop-ledger" 2>&1 &
  ACTOR=$!
  lab_run "$CASE" /bin/timeout 15 "$BIN" check --json network-choke-point || exit 125
  wait "$ACTOR"; ar=$?
  after=$(cat /sys/class/net/aqdrop0/statistics/tx_dropped)
  cat "$LAB/drop-ledger"
  printf 'INDEPENDENT_DROP_DELTA before=%s after=%s actor_rc=%s
' "$before" "$after" "$ar"
  observe after
  if [ "$after" -gt "$before" ]; then finish; else f7 G13 'no independently observed interface counter increase'; fi
  /bin/ip link del aqdrop0
else f7 G13 'bounded veth-drop fixture setup unavailable'; fi
}

case_G14() {
if setup_drop_link; then
  start G14 real-guest-historical-counters bounded-C-if-all-counters-stable
  write_drop_sender
  before=$(cat /sys/class/net/aqdrop0/statistics/tx_dropped)
  /bin/timeout 6 /bin/bash "$LAB/drop-sender" >"$LAB/drop-ledger" 2>&1; ar=$?
  after=$(cat /sys/class/net/aqdrop0/statistics/tx_dropped)
  cat "$LAB/drop-ledger"
  printf 'INDEPENDENT_HISTORICAL_DROP before=%s after=%s actor_rc=%s
' "$before" "$after" "$ar"
  if [ "$after" -gt "$before" ]; then
    run "$BIN" check --json network-choke-point
    final=$(cat /sys/class/net/aqdrop0/statistics/tx_dropped)
    printf 'INDEPENDENT_HISTORICAL_DROP_FINAL count=%s
' "$final"
    if [ "$final" = "$after" ]; then finish; else f7 G14 'counter changed during intended stable historical interval'; fi
  else f7 G14 'historical nonzero drop induction not established'; fi
  /bin/ip link del aqdrop0
else f7 G14 'bounded veth-drop fixture setup unavailable'; fi
}

case_G15() {
f7 G15 'virtual link transition need not emit three documented kernel down events; no fabricated messages'
}

case_G16() {
load_selected_modules libcrc32c nf_defrag_ipv4 nf_defrag_ipv6 nf_conntrack nfnetlink nf_tables nft_ct nft_counter
if command -v nft >/dev/null && command -v bash >/dev/null && [ -w /proc/sys/net/netfilter/nf_conntrack_max ] && /bin/nft add table ip aqct && /bin/nft 'add chain ip aqct output { type filter hook output priority 0; policy accept; }' && /bin/nft 'add rule ip aqct output ct state new counter accept'; then
  start G16 real-guest-conntrack D-if-actual-count-at-least90percent
  OLD_MAX=$(cat /proc/sys/net/netfilter/nf_conntrack_max)
  echo 128 >/proc/sys/net/netfilter/nf_conntrack_max
  /bin/timeout 8 /bin/bash -c 'for ((n=30000;n<30120;n++)); do printf x > /dev/udp/192.0.2.254/$n; done' >"$LAB/ct-flow.stdout" 2>"$LAB/ct-flow.stderr"; p=$?
  printf 'INDEPENDENT_FLOW_GENERATOR rc=%s\n' "$p"; cat "$LAB/ct-flow.stderr"
  count=$(cat /proc/sys/net/netfilter/nf_conntrack_count); maximum=$(cat /proc/sys/net/netfilter/nf_conntrack_max)
  printf 'INDEPENDENT_CONNTRACK count=%s max=%s uptime=' "$count" "$maximum"; stamp
  /bin/nft -a list table ip aqct
  if [ "$maximum" -gt 0 ] && [ "$((count*100))" -ge "$((maximum*90))" ]; then
    run "$BIN" check --json network-choke-point; finish
  else f7 G16 'actual conntrack threshold not reached; generated traffic alone insufficient'; fi
  echo "$OLD_MAX" >/proc/sys/net/netfilter/nf_conntrack_max
  /bin/nft delete table ip aqct
else f7 G16 'conntrack hook/setup or bounded flow generator unavailable'; fi
}

case_G17() {
if [ ! -e /proc/sys/net/netfilter/nf_conntrack_count ] && [ ! -e /proc/sys/net/netfilter/nf_conntrack_max ]; then
  plain G17 real-guest-applicability bounded-C-allowed network-choke-point
else f7 G17 'conntrack already present; no assumption that removal is safe or sufficient'; fi
}

case_G18() {
if new_ext4; then
  start G18 real-guest-ext4 D
  if fill_blocks; then run "$BIN" check --json silent-killer; finish; else echo 'CASE_F7 G18 threshold-not-established'; fi
  retire_ext4
else f7 G18 'ext4-loop creation failed'; retire_ext4; fi
}

case_G19() {
if new_ext4; then
  start G19 real-guest-ext4 D
  n=0
  while [ "$n" -lt 1024 ]; do
    if ! touch "$LAB/mnt/inode-$n" 2>"$LAB/inode.err"; then break; fi
    n=$((n+1))
  done
  printf 'INDEPENDENT_INODE_CREATES count=%s\n' "$n"; cat "$LAB/inode.err"
  if /bin/stat -f -c '%c %d' "$LAB/mnt" | awk '{exit !($1>0 && $2*20<=$1)}'; then run "$BIN" check --json silent-killer; finish; else echo 'CASE_F7 G19 inode-threshold-not-established'; fi
  retire_ext4
else f7 G19 'ext4-loop creation failed'; retire_ext4; fi
}

case_G20() {
if new_ext4; then
  if mount -o remount,ro "$LAB/mnt"; then
    start G20 real-guest-ext4 D
    touch "$LAB/mnt/readonly-proof" 2>"$LAB/ro.err"; p=$?
    printf 'INDEPENDENT_RO_WRITE rc=%s\n' "$p"; cat "$LAB/ro.err"
    run "$BIN" check --json silent-killer; finish
  else f7 G20 'read-only remount failed'; fi
  retire_ext4
else f7 G20 'ext4-loop creation failed'; retire_ext4; fi
}

case_G21() {
if new_ext4; then
  if mount --bind "$LAB/mnt" "$LAB/bind" && mount -o remount,bind,ro "$LAB/bind"; then
    start G21 real-guest-bind bounded-C-allowed
    touch "$LAB/mnt/writable-proof"; p=$?; printf 'INDEPENDENT_BACKING_WRITE rc=%s\n' "$p"
    touch "$LAB/bind/readonly-proof" 2>"$LAB/ro.err"; p=$?; printf 'INDEPENDENT_BIND_WRITE rc=%s\n' "$p"; cat "$LAB/ro.err"
    run "$BIN" check --json silent-killer; finish
  else f7 G21 'read-only bind creation failed'; fi
  retire_ext4
else f7 G21 'ext4-loop creation failed'; retire_ext4; fi
}

case_G22() {
if [ "$ROOT_TYPE" = overlay ]; then
  plain G22 real-guest-overlay-root U-if-no-supported-positive silent-killer
else f7 G22 'actual overlay boot missing; controller must supply --overlay-case G22'; fi
}

case_G23() {
if mount -t tmpfs -o size=1048576,nr_inodes=128 aqsmall "$LAB/small"; then
  start G23 real-guest-tmpfs D
  dd if=/dev/zero of="$LAB/small/fill" bs=1048576 count=2 2>"$LAB/fill.err"; p=$?
  printf 'INDEPENDENT_TMPFS_FILL rc=%s\n' "$p"; cat "$LAB/fill.err"
  if /bin/stat -f -c '%b %a' "$LAB/small" | awk '{exit !($1>0 && $2*20<=$1)}'; then run "$BIN" check --json silent-killer; finish; else echo 'CASE_F7 G23 threshold-not-established'; fi
  umount "$LAB/small" || exit 1
else f7 G23 'small tmpfs mount failed'; fi
}

case_G24() {
load_selected_modules xfs
if command -v mkfs.xfs >/dev/null && command -v xfs_io >/dev/null && command -v losetup >/dev/null; then
  available=$(/bin/stat -f -c '%a %S' "$LAB" | awk '{printf "%.0f",$1*$2}')
  if [ "$available" -lt 408944640 ]; then f7 G24 'guest lacks350MiB image plus40MiB free reserve; requires768MiB guest'; return; fi
  start G24 real-guest-XFS conditional-D-from-real-shutdown-log
  dd if=/dev/zero of="$LAB/xfs.img" bs=1048576 count=350 >/dev/null 2>&1 || { f7 G24 'image allocation failed'; return; }
  if /bin/mkfs.xfs -f "$LAB/xfs.img" && XLOOP=$(/bin/losetup --find --show "$LAB/xfs.img") && mount -t xfs "$XLOOP" "$LAB/mnt"; then
    printf baseline >"$LAB/mnt/baseline"
    sync
    /bin/stat -f "$LAB/mnt"
    /bin/timeout 5 /bin/xfs_io -x -c shutdown "$LAB/mnt" >"$LAB/xfs-shutdown.stdout" 2>"$LAB/xfs-shutdown.stderr"; p=$?
    printf 'INDEPENDENT_XFS_SHUTDOWN rc=%s\n' "$p"; cat "$LAB/xfs-shutdown.stdout" "$LAB/xfs-shutdown.stderr"
    touch "$LAB/mnt/new-write" 2>"$LAB/xfs-write.stderr"; w=$?
    printf 'INDEPENDENT_XFS_WRITE rc=%s\n' "$w"; cat "$LAB/xfs-write.stderr"
    dmesg >"$LAB/xfs-log"
    if [ "$w" -ne 0 ] && grep -qi 'Input/output error' "$LAB/xfs-write.stderr" && grep -Eqi 'XFS.*(shutdown|shut down|Shutting down)' "$LAB/xfs-log"; then
      run "$BIN" check --json silent-killer kernel-scream; finish
    else f7 G24 'shutdown/EIO/log conjunction not independently established'; fi
    /bin/timeout 5 umount "$LAB/mnt"; u=$?
    printf 'XFS_RESTORATION unmount_rc=%s\n' "$u"
    if [ "$u" -eq 0 ]; then /bin/losetup -d "$XLOOP" && rm -f "$LAB/xfs.img"; fi
  else f7 G24 'XFS creation or mount failed; exact guest substrate retired on VM termination'; fi
else f7 G24 'XFS tools unavailable'; fi
}

case_G25() {
if command -v dmsetup >/dev/null && new_ext4; then
  start G25 real-guest-dm conditional-D-only-from-real-supported-log
  umount "$LAB/mnt" || { f7 G25 'initial loop unmount failed'; return; }
  if /bin/dmsetup --noudevsync --noudevrules create aqerror --table "0 65536 linear $LOOP 0" && /bin/dmsetup mknodes aqerror && mount -t ext4 /dev/mapper/aqerror "$LAB/mnt"; then
    dd if=/dev/zero of="$LAB/mnt/baseline" bs=4096 count=8 >/dev/null 2>&1
    sync
    /bin/dmsetup table aqerror
    /bin/dmsetup --noudevsync suspend aqerror && /bin/dmsetup --noudevsync reload aqerror --table '0 65536 error' && /bin/dmsetup --noudevsync resume aqerror
    transition=$?
    printf 'INDEPENDENT_DM_TRANSITION rc=%s\n' "$transition"; /bin/dmsetup table aqerror; /bin/dmsetup status aqerror
    /bin/timeout 5 dd if=/dev/mapper/aqerror of=/dev/null bs=4096 count=1 >"$LAB/dm-read.stdout" 2>"$LAB/dm-read.stderr"; p=$?
    printf 'INDEPENDENT_DM_READ rc=%s\n' "$p"; cat "$LAB/dm-read.stderr"
    /bin/timeout 5 dd if=/dev/zero of="$LAB/mnt/after-error" bs=4096 count=1 conv=fsync >"$LAB/dm-write.stdout" 2>"$LAB/dm-write.stderr"; w=$?
    printf 'INDEPENDENT_DM_WRITE rc=%s\n' "$w"; cat "$LAB/dm-write.stderr"
    if [ "$transition" -eq 0 ] && [ "$p" -ne 0 ]; then
      run "$BIN" check --json kernel-scream silent-killer; finish
    else f7 G25 'actual error target/read failure not established'; fi
    /bin/timeout 5 umount "$LAB/mnt"; u=$?
    printf 'DM_RESTORATION unmount_rc=%s\n' "$u"
    if [ "$u" -eq 0 ]; then
      /bin/timeout 5 /bin/dmsetup --noudevsync remove aqerror; m=$?
      printf 'DM_RESTORATION remove_rc=%s\n' "$m"
      if [ "$m" -eq 0 ]; then /bin/losetup -d "$LOOP" && rm -f "$LAB/ext4.img"; LOOP=; fi
    fi
  else f7 G25 'device-mapper setup failed; exact guest substrate retired on VM termination'; fi
else f7 G25 'dmsetup or ext4-loop unavailable'; fi
}

case_G26() {
if command -v unshare >/dev/null && command -v setpriv >/dev/null; then
  start G26 guest-permission-view U-if-denied-counter-and-log-readable
  TARGET="/sys/class/net/$IFACE/statistics/rx_errors"
  cat "$TARGET" >"$LAB/denied-counter" || { f7 G26 'original counter unreadable'; return; }
  chmod 000 "$LAB/denied-counter"
  cat >"$LAB/permission-view.sh" <<'EOS'
#!/bin/sh
mount --make-rprivate / || exit 125
mount --bind "$1" "$2" || exit 125
/bin/setpriv --reuid 65534 --regid 65534 --clear-groups --inh-caps +syslog --ambient-caps +syslog /bin/dmesg >/dev/null
logrc=$?
/bin/setpriv --reuid 65534 --regid 65534 --clear-groups --inh-caps +syslog --ambient-caps +syslog cat "$2" >/dev/null
counterrc=$?
printf 'INDEPENDENT_PERMISSION_VIEW log_rc=%s counter_rc=%s
' "$logrc" "$counterrc" >&2
[ "$logrc" -eq 0 ] && [ "$counterrc" -ne 0 ] || exit 125
exec /bin/setpriv --reuid 65534 --regid 65534 --clear-groups --inh-caps +syslog --ambient-caps +syslog "$3" check --json network-choke-point
EOS
  run /bin/unshare -m /bin/sh "$LAB/permission-view.sh" "$LAB/denied-counter" "$TARGET" "$BIN"
  finish
  chmod 600 "$LAB/denied-counter"
else f7 G26 'namespace or selective-capability tool unavailable'; fi
}

case_G27() {
fixture df "printf '%s\\n' 'Filesystem truncated' 'bad'"
start G27 guest-command-substitution U
sha256sum "$LAB/commands/df"
run env PATH="$LAB/commands:/bin:/sbin:/usr/bin:/usr/sbin" "$BIN" check --json silent-killer
finish
clear_fixtures
}

case_G28() {
fixture dmesg 'exit 0'
fixture journalctl 'exit 0'
start G28 guest-command-substitution U
sha256sum "$LAB/commands/dmesg" "$LAB/commands/journalctl"
run env PATH="$LAB/commands:/bin:/sbin:/usr/bin:/usr/sbin" "$BIN" check --json kernel-scream
finish
clear_fixtures
}

case_G29() {
echo 'nameserver 192.0.2.253' >/etc/resolv.conf
if command -v nslookup >/dev/null; then
  start G29 real-guest-compound bounded-C-allowed
  cat /etc/resolv.conf
  /bin/timeout 3 nslookup alibi-qualification.invalid; p=$?; printf 'INDEPENDENT_DNS_PROBE rc=%s\n' "$p"
  run "$BIN" check --json network-choke-point silent-killer
  finish
else f7 G29 'resolver query utility absent'; fi
}

case_G30() {
f7 G30 'minimal initramfs has no systemd service manager; plain process failure is not systemd-service qualification'
}

case_G31() {
echo 1 >/proc/sys/kernel/dmesg_restrict
if command -v setpriv >/dev/null && missing_defaults; then
  start G31 real-guest-compound D-with-log-limitation
  /bin/setpriv --reuid 65534 --regid 65534 --clear-groups /bin/dmesg 2>"$LAB/access.stderr"; p=$?
  printf 'INDEPENDENT_LOG_ACCESS rc=%s\n' "$p"; cat "$LAB/access.stderr"
  run /bin/setpriv --reuid 65534 --regid 65534 --clear-groups "$BIN" check --json network-choke-point
  finish
else f7 G31 'unprivileged invocation or route mutation unavailable'; fi
}

case_G32() {
# Timed transitions are observation candidates, never asserted synchronized by sleep alone.
if /bin/ip link add aqv0 type veth peer name aqv1; then
  start G32 real-guest-transition U-or-D-if-transition-within-sample-else-F7
  observe before
  (sleep 1; printf 'TRANSITION G32 before-delete uptime='; stamp; /bin/ip link del aqv0; printf 'TRANSITION G32 after-delete uptime='; stamp) >"$LAB/transition" 2>&1 &
  ACTOR=$!
  printf 'ALIBI_RECEIPT_BEGIN G32 uptime='; stamp
  lab_run "$CASE" /bin/timeout 15 "$BIN" check --json network-choke-point || { echo 'CAPTURE_FAILED G32'; exit 125; }
  wait "$ACTOR"; ar=$?
  printf '\nALIBI_RECEIPT_END G32 uptime='; stamp
  cat "$LAB/transition"; printf 'TRANSITION_ACTOR rc=%s\n' "$ar"
  observe after; finish
else f7 G32 'veth unavailable'; fi
}

case_G33() {
if new_ext4; then
  start G33 real-guest-before-transition D
  if fill_blocks; then
    printf 'THRESHOLD_ESTABLISHED G33 uptime='; stamp
    run "$BIN" check --json silent-killer; finish
    for repeat in 2 3; do plain "G33-repeat-$repeat" real-guest-repeat D silent-killer; done
  else echo 'CASE_F7 G33 threshold-not-established'; fi
  retire_ext4
else f7 G33 'ext4-loop creation failed'; retire_ext4; fi
}

case_G34() {
if new_ext4; then
  start G34 real-guest-recovery bounded-C-allowed
  if fill_blocks; then
    rm "$LAB/mnt/fill"
    printf 'FAULT_REMOVED G34 uptime='; stamp
    run "$BIN" check --json silent-killer; finish
  else echo 'CASE_F7 G34 original-threshold-not-established'; fi
  retire_ext4
else f7 G34 'ext4-loop creation failed'; retire_ext4; fi
}

case_G35() {
if new_ext4; then
  start G35 intervention-assisted-real-mount-transition U-or-supported-positive
  /bin/stat -f "$LAB/mnt"
  # Wrapper changes actual kernel mount state at a known command-acquisition boundary.
  # It prints no substitute df data; stdout comes solely from real GNU df.
  cat >"$LAB/commands/df" <<'EOS'
#!/bin/sh
matched=no
for arg in "$@"; do [ "$arg" != /run/alibi-matrix/mnt ] || matched=yes; done
if [ "$matched" = yes ] && [ ! -e /run/alibi-matrix/unmounted ]; then
  {
    printf 'TRANSITION_G35_BEFORE uptime='; cat /proc/uptime
    cat /proc/self/mountinfo
    /bin/stat -f /run/alibi-matrix/mnt
    umount /run/alibi-matrix/mnt
    code=$?
    printf 'TRANSITION_G35_UNMOUNT rc=%s
' "$code"
    if [ "$code" -eq 0 ]; then touch /run/alibi-matrix/unmounted; fi
    printf 'TRANSITION_G35_AFTER uptime='; cat /proc/uptime
    cat /proc/self/mountinfo
    /bin/stat -f /run/alibi-matrix/mnt
  } >>/run/alibi-matrix/mount-transition 2>&1
fi
exec /bin/df "$@"
EOS
  chmod 700 "$LAB/commands/df"
  sha256sum "$LAB/commands/df"
  run env PATH="$LAB/commands:/bin:/sbin:/usr/bin:/usr/sbin" "$BIN" check --json silent-killer
  cat "$LAB/mount-transition" 2>/dev/null || :
  if [ -e "$LAB/unmounted" ]; then
    finish
    /bin/losetup -d "$LOOP" && rm -f "$LAB/ext4.img"; LOOP=
  else f7 G35 'wrapper never established actual unmount before real df'; retire_ext4; fi
  clear_fixtures
else f7 G35 'ext4-loop unavailable'; retire_ext4; fi
}

case_G36() {
f7 G36 'no tracing barrier establishing route read before transition; timing guesses insufficient'
}

case_G37() {
printf '#!/bin/sh\n/bin/sleep 4 &\nprintf '\''%%s\\n'\'' '\''[    0.000000] Linux version pipe-fixture'\''\nexit 0\n' >"$LAB/commands/dmesg"
chmod 700 "$LAB/commands/dmesg"
fixture journalctl 'exit 1'
start G37 guest-command-substitution U-and-no-success-after-3s-deadline
sha256sum "$LAB/commands/dmesg" "$LAB/commands/journalctl"
run env PATH="$LAB/commands:/bin:/sbin:/usr/bin:/usr/sbin" "$BIN" check --json kernel-scream
# The finite child lives 4s; bounded extra wait ensures substrate is not retired early.
sleep 4
finish
clear_fixtures
}

case_G38() {
if command -v httpd >/dev/null && command -v wget >/dev/null; then
  mkdir -p "$LAB/www/cgi-bin"
  printf '#!/bin/sh\nprintf '\''Status: 500 Internal Server Error\\r\\nContent-Type: text/plain\\r\\n\\r\\ndeliberate guest-only application failure\\n'\''\n' >"$LAB/www/cgi-bin/fail"
  chmod 700 "$LAB/www/cgi-bin/fail"
  httpd -f -p 127.0.0.1:18080 -h "$LAB/www" >"$LAB/httpd.stdout" 2>"$LAB/httpd.stderr" &
  SERVER=$!
  sleep 1
  /bin/timeout 3 wget -S -O "$LAB/application.body" http://127.0.0.1:18080/cgi-bin/fail >"$LAB/client.stdout" 2>"$LAB/client.stderr"; p=$?
  printf 'INDEPENDENT_APPLICATION_PROBE rc=%s server_pid=%s\n' "$p" "$SERVER"
  cat "$LAB/client.stderr" "$LAB/client.stdout" "$LAB/httpd.stderr"
  if grep -q '500' "$LAB/client.stderr"; then
    plain G38 real-guest-application bounded-C-N-S-U-M-if-no-EDAC network-choke-point silent-killer memrot
  else f7 G38 'independent client did not establish intended HTTP500 response'; fi
  kill "$SERVER" 2>/dev/null || :
  wait "$SERVER" 2>/dev/null || :
else f7 G38 'guest httpd/wget unavailable'; fi
}

case_LE08_guest() {
fixture dmesg "printf '%s\\n' 'not a kernel log'"
fixture journalctl 'exit 1'
start LE08-guest guest-command-substitution U
run env PATH="$LAB/commands:/bin:/sbin:/usr/bin:/usr/sbin" "$BIN" check --json kernel-scream
finish
clear_fixtures
}

case_LE04_guest() {
fixture dmesg "printf '[    0.000000] Linux version control\\377\\n'"
fixture journalctl 'exit 1'
start LE04-guest guest-command-substitution U
run env PATH="$LAB/commands:/bin:/sbin:/usr/bin:/usr/sbin" "$BIN" check --json kernel-scream
finish
clear_fixtures
}

[ -r /lib/alibi-lab.sh ] || { echo 'REFUSED: capture helper absent'; exit 125; }
. /lib/alibi-lab.sh
plain "$SELECTED-baseline" real-guest-control conditional-baseline kernel-scream silent-killer memrot network-choke-point
case "$SELECTED" in
  G01) case_G01;;
  G02) case_G02;;
  G03) case_G03;;
  G04) case_G04;;
  G05) case_G05;;
  G06) case_G06;;
  G07) case_G07;;
  G08) case_G08;;
  G09) case_G09;;
  G10) case_G10;;
  G11) case_G11;;
  G12) case_G12;;
  G13) case_G13;;
  G14) case_G14;;
  G15) case_G15;;
  G16) case_G16;;
  G17) case_G17;;
  G18) case_G18;;
  G19) case_G19;;
  G20) case_G20;;
  G21) case_G21;;
  G22) case_G22;;
  G23) case_G23;;
  G24) case_G24;;
  G25) case_G25;;
  G26) case_G26;;
  G27) case_G27;;
  G28) case_G28;;
  G29) case_G29;;
  G30) case_G30;;
  G31) case_G31;;
  G32) case_G32;;
  G33) case_G33;;
  G34) case_G34;;
  G35) case_G35;;
  G36) case_G36;;
  G37) case_G37;;
  G38) case_G38;;
  LE08-guest) case_LE08_guest;;
  LE04-guest) case_LE04_guest;;
  *) echo "REFUSED: unknown alibi_case=$SELECTED"; exit 64;;
esac
reset_route
printf 'RESTORATION_BEGIN\n'
cat /proc/self/mountinfo
/bin/losetup -a 2>/dev/null || :
/bin/ip -4 route show table all
printf 'RESTORATION_END disposable-guest-termination-next\n'
printf 'MATRIX_COMPLETE run=%s\n' "${RUN_ID:-unknown}"
