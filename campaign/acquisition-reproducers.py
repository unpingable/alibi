#!/usr/bin/env python3
"""Bounded acquisition fixtures. Usage: SCRIPT BINARY NEW_EVIDENCE_DIRECTORY.

These are command substitutions, not guest-kernel fault qualification. Every
fixture is finite. No production/host configuration is modified. Retain exact
fixture sources and receipt; executable fixture modes are retired after use.
"""
import hashlib
import json
import os
from pathlib import Path
import signal
import subprocess
import sys
import time

binary = Path(sys.argv[1]).resolve()
out = Path(sys.argv[2]).resolve()
out.mkdir(parents=True, exist_ok=False)
raw = b'<6>[    0.000000] Linux version fixture-control\n'
journal = b'[    0.000000] kernel: Linux version fixture-control\n'
def emit(value):
    return 'import sys\nsys.stdout.buffer.write(' + repr(value) + ')\n'
fail = 'raise SystemExit(1)\n'
cases = [
    ('invalid-utf8', emit(b'\xff\n'), emit(b'\xff\n'), 'UNKNOWN'),
    ('non-log', emit(b'not a kernel log\n'), emit(b'not a kernel log\n'), 'UNKNOWN'),
    ('structured-dmesg-control', emit(raw), fail, 'CLEAR'),
    ('structured-journal-control', fail, emit(journal), 'CLEAR'),
    ('partial-positive', emit(b'<3>[    1.000000] Oops: fixture\ninvalid record\n'), fail, 'DEGRADED'),
    ('truncated-record', emit(raw.rstrip(b'\n')), fail, 'UNKNOWN'),
]
for stream in ['stdout', 'stderr']:
    redirects = ', stderr=subprocess.DEVNULL' if stream == 'stdout' else ', stdout=subprocess.DEVNULL'
    source = ("import os, subprocess, pathlib, sys\n"
              "child = subprocess.Popen(['/bin/sleep', '4']" + redirects + ")\n"
              "pathlib.Path(os.environ['ALIBI_FIXTURE_PID_FILE']).write_text(str(child.pid))\n"
              'sys.stdout.buffer.write(' + repr(raw) + ')\nsys.stdout.flush()\n')
    cases.append(('inherited-' + stream, source, fail, 'UNKNOWN'))
cases.append(('output-cap', "import sys\nsys.stdout.buffer.write(b'x' * (8 * 1024 * 1024 + 1))\n", fail, 'UNKNOWN'))
records = []
for name, dmesg, journalctl, expected in cases:
    case = out / name
    case.mkdir()
    for command, source in [('dmesg', dmesg), ('journalctl', journalctl)]:
        path = case / command
        path.write_text('#!' + sys.executable + '\n' + source)
        path.chmod(0o700)
    env = dict(os.environ, PATH=str(case), ALIBI_FIXTURE_PID_FILE=str(case/'child.pid'))
    started = time.monotonic()
    process = subprocess.Popen([str(binary), 'check', '--json', 'kernel-scream'],
        stdout=subprocess.PIPE, stderr=subprocess.PIPE, env=env, start_new_session=True)
    timed_out = False
    try:
        stdout, stderr = process.communicate(timeout=12)
    except subprocess.TimeoutExpired:
        timed_out = True
        os.killpg(process.pid, signal.SIGKILL)
        stdout, stderr = process.communicate(timeout=2)
    elapsed = time.monotonic() - started
    (case/'stdout.json').write_bytes(stdout)
    (case/'stderr').write_bytes(stderr)
    try:
        actual = json.loads(stdout)['verdict']
    except (ValueError, KeyError):
        actual = None
    child_state = None
    if (case/'child.pid').exists():
        pid = int((case/'child.pid').read_text())
        try:
            child_state = Path('/proc', str(pid), 'stat').read_text().rsplit(')', 1)[1].split()[0]
        except FileNotFoundError:
            child_state = 'absent'
    records.append(dict(case=name, expected=expected, actual=actual,
        verdict_matches=actual == expected, exit_status=process.returncode,
        elapsed_ms=round(elapsed*1000, 3), outer_timeout=timed_out,
        fixture_child_state=child_state,
        stdout_sha256=hashlib.sha256(stdout).hexdigest(),
        stderr_sha256=hashlib.sha256(stderr).hexdigest()))
    for command in ['dmesg', 'journalctl']:
        (case/command).chmod(0o600)
receipt = dict(schema_version=1, binary=str(binary),
    binary_sha256=hashlib.sha256(binary.read_bytes()).hexdigest(),
    owner='alibi campaign root', purpose='bounded acquisition regression evidence',
    retirement='review at campaign closeout; retain minimal fixture source and result custody',
    scope='local command substitution only; not kernel-fault qualification', records=records)
(out/'RECEIPT.json').write_text(json.dumps(receipt, indent=2)+'\n')
print(json.dumps(records, indent=2))
