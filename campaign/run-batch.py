#!/usr/bin/env python3
"""Serial campaign producer. Run a sealed plan under detached user-systemd."""
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys
import time


def save(path, value):
    temporary = path.with_suffix('.new')
    with temporary.open('w') as stream:
        json.dump(value, stream, indent=2)
        stream.write('\n')
        stream.flush()
        os.fsync(stream.fileno())
    temporary.replace(path)


plan_path = Path(sys.argv[1]).resolve()
plan = json.loads(plan_path.read_text())
root = Path(plan['directory'])
record = dict(plan_sha256=hashlib.sha256(plan_path.read_bytes()).hexdigest(),
              invocation_id=os.environ.get('INVOCATION_ID'), pid=os.getpid(),
              started=time.time(), records=[], phase='starting')
save(root / 'START.json', record)
for case in plan['cases']:
    allocated = 0
    seen = set()
    for entry in Path(plan['allocation_root']).rglob('*'):
        stat = entry.lstat()
        identity = (stat.st_dev, stat.st_ino)
        if identity not in seen:
            seen.add(identity)
            allocated += stat.st_blocks * 512
    record['allocated_bytes_before_case'] = allocated
    if allocated + 234881024 + plan.get('pending_allocation_reserve', 0) > plan['allocation_cap']:
        record.update(phase='storage-refused', current=case['case_id'])
        save(root / 'ADMISSION-REFUSED.json', record)
        raise SystemExit(1)
    record.update(phase='running', current=case['case_id'], run_id=case['run_id'])
    save(root / 'PROGRESS.json', record)
    argv = ['/usr/bin/python3', plan['runner'], '--kernel', case['kernel'],
            '--initramfs', case['initramfs'], '--output-dir', case['output'],
            '--checkpoint', plan['checkpoint'], '--source-revision', case['source_revision'],
            '--owner', 'root', '--retirement', plan['retirement'], '--run-id', case['run_id'],
            '--case-id', case['case_id'], '--timeout', str(case.get('timeout', 120)),
            '--memory-mib', str(case.get('memory_mib', 512)),
            '--isolated-nics', str(case.get('nics', 1))]
    completed = subprocess.run(argv, stdin=subprocess.DEVNULL)
    terminal = Path(case['output']) / 'TERMINAL.json'
    result = dict(case=case['case_id'], run_id=case['run_id'], exit_status=completed.returncode,
                  terminal=str(terminal), terminal_exists=terminal.exists())
    if terminal.exists():
        result['terminal_sha256'] = hashlib.sha256(terminal.read_bytes()).hexdigest()
    record['records'].append(result)
    save(root / 'PROGRESS.json', record)
    if not terminal.exists():
        record.update(phase='indeterminate', reason='Missing terminal; reconcile original before any successor')
        save(root / 'INDETERMINATE.json', record)
        raise SystemExit(1)
record.update(phase='terminal', finished=time.time())
save(root / 'COMPLETE.json', record)
