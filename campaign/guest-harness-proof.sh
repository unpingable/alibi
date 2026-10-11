#!/bin/sh
# Independent harness qualification inside the admitted isolated guest only.
set -eu
case " $(cat /proc/cmdline) " in *" alibi_run="*) ;; *) exit 64;; esac
[ "$(hostname)" = alibi-disposable-guest ] || exit 64
. /lib/alibi-lab.sh
printf 'HARNESS_PROOF_BEGIN\n'
command -V sh ip df dmesg losetup
ip -Version
ip -Version | grep -q iproute2
/bin/df --version | head -1
/bin/dmesg --version
/bin/losetup --version
[ "$(command -v ip)" = /bin/ip ]
# Zero NIC start independently distinguishes module default interfaces.
[ "$(ls /sys/class/net | grep -vc '^lo$')" -eq 0 ]
/bin/busybox insmod /modules/dummy.ko numdummies=0
[ "$(ls /sys/class/net | grep -vc '^lo$')" -eq 0 ]
ip link add aqdummy type dummy
ip -details link show aqdummy
[ -d /sys/class/net/aqdummy ]
ip link del aqdummy
[ "$(ls /sys/class/net | grep -vc '^lo$')" -eq 0 ]
printf 'DUMMY_ZERO_DEFAULT_AND_EXPLICIT_CREATE_DELETE_VERIFIED\n'
/bin/busybox insmod /modules/veth.ko
ip link add aqv0 type veth peer name aqv1
ip -details link show aqv0
ip link del aqv0
for name in libcrc32c nf_defrag_ipv4 nf_defrag_ipv6 nf_conntrack nfnetlink nf_tables nft_ct xfs; do
    /bin/busybox insmod /modules/$name.ko
done
/bin/nft add table ip labproof
/bin/nft 'add chain ip labproof output { type filter hook output priority 0; policy accept; }'
/bin/nft 'add rule ip labproof output ct state new counter accept'
/bin/nft list table ip labproof
cat /proc/sys/net/netfilter/nf_conntrack_count /proc/sys/net/netfilter/nf_conntrack_max
/bin/nft delete table ip labproof
printf 'NFT_CONNTRACK_SETUP_VERIFIED\n'
grep -w xfs /proc/filesystems
mkdir -p /run/proof/mnt
/bin/busybox truncate -s 32M /run/proof/ext4.img
/bin/mke2fs -q -F -t ext4 /run/proof/ext4.img
loop=$(/bin/losetup --find --show /run/proof/ext4.img)
/bin/busybox mount -t ext4 "$loop" /run/proof/mnt
printf actual-guest-write >/run/proof/mnt/check
[ "$(cat /run/proof/mnt/check)" = actual-guest-write ]
/bin/stat -f /run/proof/mnt
/bin/df -P -B1 /run/proof/mnt
/bin/busybox umount /run/proof/mnt
/bin/losetup -d "$loop"
rm /run/proof/ext4.img
printf 'EXT4_LOOP_RW_RESTORATION_VERIFIED\n'
lab_run harness-proof /bin/alibi check --json
printf 'HARNESS_PROOF_COMPLETE\n'
