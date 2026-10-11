#!/usr/bin/env python3
"""One bounded TCG guest producer, intended for an admitted user-systemd unit.

No external network backend, shared directory, host block device, or SSH key.
Terminal status describes execution only; independent evidence decides verdicts.
"""
import argparse
from datetime import datetime, timezone
import hashlib
import json
import os
from pathlib import Path
import re
import resource
import signal
import subprocess
import time
import uuid
import sys
sys.dont_write_bytecode = True

from capture_frames import extract

FLOOR=64_424_509_440

def sha(path):
    h=hashlib.sha256()
    with Path(path).open('rb') as f:
        for block in iter(lambda:f.read(1024*1024),b''): h.update(block)
    return h.hexdigest()

def write_new(path, value):
    with path.open('x') as f:
        json.dump(value,f,indent=2); f.write('\n'); f.flush(); os.fsync(f.fileno())
    fd=os.open(path.parent,os.O_DIRECTORY)
    try: os.fsync(fd)
    finally: os.close(fd)

def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--kernel',type=Path,required=True)
    parser.add_argument('--initramfs',type=Path,required=True)
    parser.add_argument('--output-dir',type=Path,required=True)
    parser.add_argument('--checkpoint',type=Path,required=True)
    parser.add_argument('--source-revision',required=True)
    parser.add_argument('--owner',required=True)
    parser.add_argument('--retirement',required=True)
    parser.add_argument('--run-id',default=str(uuid.uuid4()))
    parser.add_argument('--case-id',default='baseline')
    parser.add_argument('--timeout',type=int,default=180)
    parser.add_argument('--memory-mib',type=int,default=512)
    parser.add_argument('--isolated-nics',type=int,default=0,choices=(0,1,2))
    parser.add_argument('--carrier-control',action='store_true')
    parser.add_argument('--nic-model',choices=('virtio-net-pci','e1000'),default='virtio-net-pci')
    parser.add_argument('--qemu',type=Path,default=Path('/usr/bin/qemu-system-x86_64'))
    args=parser.parse_args()
    if not re.fullmatch(r'[a-zA-Z0-9_-]{1,100}',args.run_id): parser.error('invalid run ID')
    if not re.fullmatch(r'[a-zA-Z0-9_-]{1,100}',args.case_id): parser.error('invalid case ID')
    if not 1<=args.timeout<=3600: parser.error('timeout outside 1..3600 seconds')
    if not 128<=args.memory_mib<=2048: parser.error('memory outside 128..2048 MiB')
    if args.carrier_control and (args.case_id not in ('G09','G15') or args.isolated_nics!=1):
        parser.error('carrier control requires G09/G15 and exactly one isolated NIC')
    if args.nic_model!='virtio-net-pci' and not args.carrier_control:
        parser.error('alternate NIC model is scoped to explicit carrier cases')
    for p in (args.kernel,args.initramfs,args.checkpoint,args.qemu):
        if not p.is_file(): parser.error('missing admitted input: '+str(p))
    for path in ('/','/data'):
        fs=os.statvfs(path)
        if fs.f_bavail*fs.f_frsize-234_881_024<FLOOR:
            parser.error('224 MiB capture envelope would cross host reserve: '+path)
    out=args.output_dir.absolute()
    destination=os.statvfs(out.parent)
    if destination.f_bavail*destination.f_frsize<234_881_024 or destination.f_favail<16:
        parser.error('destination lacks admitted 224 MiB / 16-inode capture envelope')
    out.mkdir(parents=False,exist_ok=False)
    serial=out/'serial.log'
    stderr=out/'qemu.stderr'
    command=[str(args.qemu.resolve()),'-accel','tcg','-machine','q35','-cpu','max',
             '-m',str(args.memory_mib),'-smp','1','-nodefaults','-display','none',
             '-monitor','none','-serial','file:'+str(serial),'-no-reboot',
             '-kernel',str(args.kernel.resolve()),'-initrd',str(args.initramfs.resolve()),
             '-append','console=ttyS0 panic=1 rdinit=/init alibi_run='+args.run_id+' alibi_case='+args.case_id]
    if args.isolated_nics:
        for n in range(args.isolated_nics):
            command += ['-netdev',f'hubport,id=n{n},hubid={n}',
                        '-device',f'{args.nic_model},id=labnic{n},netdev=n{n},mac=52:54:00:12:34:{n+1:02x}']
    else: command += ['-nic','none']
    control=None;control_path=None;control_result=None
    if args.carrier_control:
        control_directory=out.parent/('.qmp-'+uuid.uuid4().hex[:12])
        control_path=control_directory/'q'
        if len(os.fsencode(control_path))>=104:parser.error('QMP path exceeds conservative Unix socket bound')
        control_directory.mkdir(mode=0o700)
        command += ['-qmp','unix:'+str(control_path)+',server=on,wait=off']
    start=dict(schema_version=1,run_id=args.run_id,case_id=args.case_id,owner=args.owner,
               purpose='Independent-kernel Alibi qualification trial',retirement=args.retirement,
               host=os.uname().nodename,cwd=os.getcwd(),source_revision=args.source_revision,
               started_at=datetime.now(timezone.utc).isoformat(),runner_pid=os.getpid(),
               systemd_invocation_id=os.environ.get('INVOCATION_ID'),
               checkpoint=str(args.checkpoint.resolve()),checkpoint_sha256=sha(args.checkpoint),
               runner_sha256=sha(__file__),capture_parser_sha256=sha(Path(__file__).with_name("capture_frames.py")),
               kernel_sha256=sha(args.kernel),
               initramfs_sha256=sha(args.initramfs),qemu_sha256=sha(args.qemu),
               timeout_seconds=args.timeout,memory_mib=args.memory_mib,
               command=command,carrier_control=args.carrier_control,control_socket=str(control_path) if control_path else None,
               carrier_controller_sha256=sha(Path(__file__).with_name('carrier_control.py')) if args.carrier_control else None,expected_terminal=str(out/'TERMINAL.json'),
               serial=str(serial),stderr=str(stderr),
               recovery='Inspect original service InvocationID, START.json, EXECUTION.json, serial.log, and TERMINAL.json; do not restart this run.')
    write_new(out/'START.json',start)
    started=time.monotonic()
    result='LAUNCH_ERROR'; returncode=None; error=None
    def limits():
        resource.setrlimit(resource.RLIMIT_FSIZE,(67_108_864,67_108_864))
        os.umask(0o077)
    try:
        with stderr.open('xb') as err:
            process=subprocess.Popen(command,stdin=subprocess.DEVNULL,stdout=subprocess.DEVNULL,
                                     stderr=err,start_new_session=True,preexec_fn=limits)
            write_new(out/'EXECUTION.json',dict(qemu_pid=process.pid,
                       proc_start_identity=Path(f'/proc/{process.pid}/stat').read_text(),
                       systemd_invocation_id=os.environ.get('INVOCATION_ID')))
            try:
                if args.carrier_control:
                    from carrier_control import CarrierController
                    control=CarrierController(control_path,serial,args.run_id,args.case_id,out/'carrier-control.jsonl')
                    control.connect()
                    while process.poll() is None:
                        control.poll()
                        if time.monotonic()-started>args.timeout:
                            raise subprocess.TimeoutExpired(command,args.timeout)
                        time.sleep(.05)
                    control.poll()
                    returncode=process.returncode
                else:
                    returncode=process.wait(timeout=args.timeout)
                result='QEMU_EXITED'
            except subprocess.TimeoutExpired:
                os.killpg(process.pid,signal.SIGKILL)
                returncode=process.wait()
                result='TIMEOUT'
    except Exception as exc:
        error=repr(exc)
        if 'process' in locals() and process.poll() is None:
            os.killpg(process.pid,signal.SIGKILL); returncode=process.wait()
    if control is not None:
        try:control_result=control.close()
        except Exception as exc:control_result={'retirement_error':repr(exc),'socket':str(control_path)}
    elif control_path is not None:
        control_result={'retirement_error':'controller was not constructed; inspect exact directory','socket':str(control_path)}
    elapsed=time.monotonic()-started
    data=serial.read_text(errors='replace') if serial.exists() else ''
    marker=re.search(r'^ALIBI_GUEST_COMPLETE '+re.escape(args.run_id)+r' scenario_rc=(\d+)\s*$',data,re.M)
    try:
        capture_result=extract(serial,args.run_id,out/'captures') if serial.exists() else {'errors':['missing serial log'],'captures':[]}
    except Exception as exc:
        capture_result={'errors':['capture extraction failed: '+repr(exc)],'captures':[]}
    usage=resource.getrusage(resource.RUSAGE_CHILDREN)
    terminal=dict(schema_version=1,run_id=args.run_id,execution_disposition=result,
                  qemu_exit_status=returncode,elapsed_seconds=elapsed,error=error,
                  guest_complete=bool(marker),scenario_exit_status=int(marker[1]) if marker else None,
                  finished_at=datetime.now(timezone.utc).isoformat(),
                  start_sha256=sha(out/'START.json'),
                  serial_sha256=sha(serial) if serial.exists() else None,
                  serial_bytes=serial.stat().st_size if serial.exists() else 0,
                  stderr_sha256=sha(stderr) if stderr.exists() else None,
                  child_user_seconds=usage.ru_utime,child_system_seconds=usage.ru_stime,
                  child_maxrss_kib=usage.ru_maxrss,
                  carrier_control=control_result,carrier_log_sha256=sha(out/'carrier-control.jsonl') if (out/'carrier-control.jsonl').exists() else None,
                  capture_errors=capture_result['errors'],captured_files=len(capture_result['captures']),
                  acceptance='NOT_EVALUATED: terminal execution is not qualification acceptance')
    write_new(out/'TERMINAL.json',terminal)
    print(json.dumps(terminal))
    return 0 if result=='QEMU_EXITED' and returncode==0 and marker and int(marker[1])==0 else 1

if __name__=='__main__':
    raise SystemExit(main())
