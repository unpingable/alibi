#!/bin/sh
# Declared open controls for exact GNU df correspondence, not sealed holdouts.
set -eu
case " $(cat /proc/cmdline) " in *" alibi_run="*) ;; *) exit 64;; esac
[ "$(hostname)" = alibi-disposable-guest ] || exit 64
[ "$(/bin/stat -f -c %T /)" = tmpfs ] || exit 64
. /lib/alibi-lab.sh
mkdir -p '/run/identity/path  with spaces' '/run/identity/bind view'
/bin/dd if=/dev/zero of=/run/identity/fs.img bs=1048576 count=32 2>/dev/null
/bin/mke2fs -q -F -t ext4 /run/identity/fs.img
loop=$(/bin/losetup --find --show /run/identity/fs.img)
mount -t ext4 "$loop" '/run/identity/path  with spaces'
printf control >'/run/identity/path  with spaces/proof'
[ "$(cat '/run/identity/path  with spaces/proof')" = control ]
observe() {
  printf 'IDENTITY_ORACLE phase=%s uptime=' "$1"; cat /proc/uptime
  cat /proc/self/mountinfo
  /bin/df -P -B1 -- '/run/identity/path  with spaces'
  /bin/df -P -i -- '/run/identity/path  with spaces'
  /bin/stat -f -c 'STATFS path=%n type=%T blocks=%b available=%a inodes=%c free_inodes=%d' '/run/identity/path  with spaces'
  /bin/dmesg --raw --force-prefix
}
observe spaced-before
lab_run FS01-spaced-mount /bin/alibi check --json silent-killer
observe spaced-after
mount --bind '/run/identity/path  with spaces' '/run/identity/bind view'
mount -o remount,bind,ro '/run/identity/bind view'
if touch '/run/identity/bind view/denied' 2>/run/identity/ro.stderr; then exit 125; fi
cat /run/identity/ro.stderr
printf still-writable >'/run/identity/path  with spaces/second-proof'
observe bind-before
lab_run FS02-spaced-bind /bin/alibi check --json silent-killer
observe bind-after
umount '/run/identity/bind view'
umount '/run/identity/path  with spaces'
/bin/losetup -d "$loop"
rm /run/identity/fs.img
observe_restored=FS03-restored
lab_run "$observe_restored" /bin/alibi check --json silent-killer
printf 'IDENTITY_CONTROL_RESTORATION_COMPLETE\n'
