#!/bin/sh
set -eu
case " $(cat /proc/cmdline) " in *" alibi_run="*) ;; *) exit 64;; esac
[ "$(hostname)" = alibi-disposable-guest ] || exit 64
. /lib/alibi-lab.sh
capture() { lab_run "$@"; }
capture kernel-format-unsupported dmesg --raw --force-prefix --color=never
capture kernel-format dmesg --raw --force-prefix
capture healthy-kernel alibi check kernel-scream --json
capture healthy-filesystem alibi check silent-killer --json
capture absent-edac alibi check memrot --json
mkdir /tmp/commands
printf '#!/bin/sh\nexit 1\n' >/tmp/commands/journalctl
printf '#!/bin/sh\nprintf "not a kernel log\\n"\n' >/tmp/commands/dmesg
chmod 700 /tmp/commands/*
capture non-log-oracle /tmp/commands/dmesg
capture non-log env PATH=/tmp/commands /bin/alibi check kernel-scream --json
printf '#!/bin/sh\nprintf "[    0.000000] Linux version control\\377\\n"\n' >/tmp/commands/dmesg
capture invalid-encoding-oracle /tmp/commands/dmesg
capture invalid-encoding env PATH=/tmp/commands /bin/alibi check kernel-scream --json
printf '#!/bin/sh\n/bin/sleep 4 &\nprintf "[    0.000000] Linux version retained-pipe-control\\n"\nexit 0\n' >/tmp/commands/dmesg
capture inherited-pipe env PATH=/tmp/commands /bin/alibi check kernel-scream --json
rm /tmp/commands/dmesg /tmp/commands/journalctl
rmdir /tmp/commands
capture restored-kernel alibi check kernel-scream --json
