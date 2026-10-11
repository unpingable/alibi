#!/usr/bin/env python3
"""Build an admitted, isolated Linux initramfs; never launch it.

Only trusted local executables are admitted: ldd inspects those executables.
No host filesystem/device is made available to the guest at runtime.
"""
import argparse
import gzip
import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import stat
import subprocess
import time

FLOOR = 64_424_509_440

def digest(p):
    return hashlib.sha256(Path(p).read_bytes()).hexdigest()

def main():
    a = argparse.ArgumentParser(description=__doc__)
    a.add_argument('--alibi', type=Path, required=True)
    a.add_argument('--output-dir', type=Path, required=True)
    a.add_argument('--admission', type=Path, required=True)
    a.add_argument('--owner', required=True)
    a.add_argument('--retirement', required=True)
    a.add_argument('--scenario', type=Path)
    a.add_argument('--tool', type=Path, action='append', default=[])
    a.add_argument('--module-file', type=Path, action='append', default=[])
    a.add_argument('--kernel-release')
    args = a.parse_args()
    if not args.admission.is_file():
        a.error('existing campaign storage admission receipt required')
    if args.module_file and not args.kernel_release:
        a.error('--kernel-release required with explicit module files')
    for path in ('/', '/data'):
        fs = os.statvfs(path)
        if fs.f_bavail * fs.f_frsize - 134_217_728 < FLOOR:
            a.error('128 MiB construction envelope would cross host reserve: ' + path)
    out = args.output_dir.absolute()
    destination = os.statvfs(out.parent)
    if destination.f_bavail * destination.f_frsize < 134_217_728 or destination.f_favail < 4096:
        a.error('destination lacks admitted 128 MiB / 4096-inode construction envelope')
    out.mkdir(parents=False, exist_ok=False)
    receipt = dict(owner=args.owner, purpose='Reproducible isolated Linux guest initramfs',
                   retirement=args.retirement, admission=str(args.admission.resolve()),
                   admission_sha256=digest(args.admission), created_unix=time.time(),
                   disposition='CONSTRUCTION_STARTED', files=[])
    manifest = out / 'BUILD.json'
    manifest.write_text(json.dumps(receipt, indent=2) + '\n')
    root = out / 'root'
    root.mkdir()
    for d in ('bin','sbin','usr/bin','usr/sbin','lib','lib64','etc','proc','sys','dev','tmp','run','newroot','modules'):
        (root/d).mkdir(parents=True, exist_ok=True)

    def copy(source, destination):
        source = Path(source)
        dest = root / destination.lstrip('/')
        dest.parent.mkdir(parents=True, exist_ok=True)
        if dest.exists():
            if digest(dest) != digest(source):
                raise RuntimeError('conflicting guest path: ' + destination)
            return
        if sum(item['bytes'] for item in receipt['files']) + source.stat().st_size > 62_914_560:
            raise RuntimeError('guest input copy budget exceeds 60 MiB; explicit envelope revision required')
        shutil.copy2(source, dest, follow_symlinks=True)
        receipt['files'].append(dict(source=str(source.absolute()), guest=destination,
                                     sha256=digest(dest), bytes=dest.stat().st_size))

    def executable(source, destination):
        copy(source, destination)
        result = subprocess.run(['ldd',str(source)], text=True, capture_output=True, timeout=10)
        text = result.stdout + result.stderr
        if 'not found' in text:
            raise RuntimeError('unresolved libraries for ' + str(source))
        if result.returncode and not any(s in text for s in ('not a dynamic executable','statically linked')):
            raise RuntimeError('ldd inspection failed: ' + text)
        for library in sorted(set(re.findall(r'(/[^\s()]+)', text))):
            if Path(library).is_file():
                copy(library, library)

    executable('/usr/bin/busybox', '/bin/busybox')
    applets = subprocess.check_output(['/usr/bin/busybox','--list'], text=True).splitlines()
    for applet in applets:
        if applet != 'busybox':
            (root/'bin'/applet).symlink_to('busybox')
    # Replace applet links with exact external tools where the interface matters.
    for source, destination in [(args.alibi,'/bin/alibi'),('/usr/bin/df','/bin/df'),
                                 ('/usr/bin/dmesg','/bin/dmesg'),('/usr/sbin/ip','/bin/ip')]:
        p = root / destination.lstrip('/')
        if p.is_symlink():
            p.unlink()
        executable(source, destination)
    for tool in args.tool:
        destination='/bin/'+tool.name
        p=root/destination.lstrip('/')
        if p.is_symlink(): p.unlink()
        executable(tool, destination)
    for module in args.module_file:
        copy(module, '/modules/'+module.name)
    (root/'etc/passwd').write_text('root:x:0:0:root:/:/bin/sh\nnobody:x:65534:65534:nobody:/:/bin/sh\n')
    (root/'etc/group').write_text('root:x:0:\nnogroup:x:65534:\n')
    (root/'etc/nsswitch.conf').write_text('passwd: files\ngroup: files\nhosts: files\n')
    (root/'etc/hostname').write_text('alibi-disposable-guest\n')
    scenario = args.scenario.read_text() if args.scenario else '''#!/bin/sh
printf 'ORACLE_UNAME_BEGIN\\n'
uname -a
printf 'ORACLE_UNAME_END\\n'
printf 'ALIBI_RECEIPT_BEGIN baseline\\n'
alibi check kernel-scream --json
rc=$?
printf '\\nALIBI_RECEIPT_END baseline rc=%s\\n' "$rc"
exit 0
'''
    (root/'scenario.sh').write_text(scenario)
    (root/'scenario.sh').chmod(0o755)
    (root/'init').write_text('''#!/bin/sh
export PATH=/bin:/sbin:/usr/bin:/usr/sbin LC_ALL=C
mount -t proc proc /proc || exec sh
mount -t sysfs sysfs /sys || exec sh
mount -t devtmpfs devtmpfs /dev || exec sh
exec </dev/console >/dev/console 2>&1
mkdir -p /dev/pts
mount -t devpts devpts /dev/pts
mount -t tmpfs -o size=70%,mode=755 guest-root /newroot || exec sh
for d in bin sbin usr lib lib64 etc modules; do cp -a /$d /newroot/ || exec sh; done
cp /scenario.sh /stage2 /newroot/
mkdir -p /newroot/proc /newroot/sys /newroot/dev /newroot/tmp /newroot/run
chmod 1777 /newroot/tmp
mount --move /proc /newroot/proc || exec sh
mount --move /sys /newroot/sys || exec sh
mount --move /dev /newroot/dev || exec sh
exec switch_root /newroot /stage2
''')
    release = args.kernel_release or ''
    (root/'stage2').write_text('''#!/bin/sh
export PATH=/bin:/sbin:/usr/bin:/usr/sbin LC_ALL=C
hostname alibi-disposable-guest
RUN_ID=unknown
for word in $(cat /proc/cmdline); do case "$word" in alibi_run=*) RUN_ID=${word#alibi_run=};; esac; done
export RUN_ID
printf '\\nALIBI_GUEST_STARTED %s\\n' "$RUN_ID"
uname -a
printf 'GUEST_MOUNTS_BEGIN\\n'
cat /proc/self/mounts
printf 'GUEST_MOUNTS_END\\n'
''' + ('''if [ "$(uname -r)" != ''' + repr(release) + ''' ]; then
  echo 'MODULE_KERNEL_MISMATCH'
  poweroff -f
fi
''' if release else '') + '''/bin/sh /scenario.sh
scenario_rc=$?
printf '\\nALIBI_GUEST_COMPLETE %s scenario_rc=%s\\n' "$RUN_ID" "$scenario_rc"
sync
poweroff -f
while :; do sleep 1; done
''')
    for name in ('init','stage2'):
        (root/name).chmod(0o755)
    receipt['scenario_sha256']=digest(root/'scenario.sh')
    receipt['module_kernel_release']=args.kernel_release
    archive=out/'initramfs.cpio.gz'
    # cpio sees relative, NUL-delimited names from this exclusively owned tree.
    names=['.'] + sorted(str(p.relative_to(root)) for p in root.rglob('*'))
    packed=subprocess.run(['cpio','--null','-o','--format=newc','--reproducible'],
                          cwd=root,input=('\0'.join(names)+'\0').encode(),capture_output=True,check=True)
    # Add /dev/console to newc without creating a host device node. The kernel
    # opens it before /init can mount devtmpfs. Replace only the cpio trailer.
    raw = packed.stdout
    cursor = 0
    while True:
        if raw[cursor:cursor+6] != b'070701':
            raise RuntimeError('unexpected cpio format')
        values = [int(raw[cursor+6+n*8:cursor+14+n*8],16) for n in range(13)]
        name_start = cursor + 110
        name = raw[name_start:name_start+values[11]-1]
        if name == b'TRAILER!!!': break
        data_start = (name_start + values[11] + 3) & ~3
        cursor = (data_start + values[6] + 3) & ~3
    def entry(name, mode, inode, major=0, minor=0):
        encoded = name.encode() + b'\0'
        values = [inode,mode,0,0,1,0,0,0,0,major,minor,len(encoded),0]
        item = b'070701' + ''.join(f'{v:08x}' for v in values).encode() + encoded
        return item + b'\0' * (-len(item) % 4)
    raw = raw[:cursor] + entry('dev/console',stat.S_IFCHR | 0o600,0x7ffffffe,5,1) + entry('TRAILER!!!',0,0)
    with archive.open('xb') as f:
        with gzip.GzipFile(fileobj=f,mode='wb',mtime=0) as compressed:
            compressed.write(raw)
    receipt.update(disposition='BUILT_NOT_BOOTED', archive=str(archive),
                   archive_sha256=digest(archive), archive_bytes=archive.stat().st_size,
                   script_sha256=digest(__file__),
                   guest_tree_bytes=sum(p.stat().st_size for p in root.rglob('*') if p.is_file() and not p.is_symlink()))
    manifest.write_text(json.dumps(receipt,indent=2)+'\n')
    print(json.dumps(dict(archive=str(archive), manifest=str(manifest),sha256=receipt['archive_sha256'])))

if __name__ == '__main__':
    main()
