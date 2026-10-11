#!/usr/bin/env python3
"""Bounded external-CLI SIGCHLD inheritance control; no PID-reuse experiment.
Usage: SCRIPT BINARY NEW_OUTPUT_DIRECTORY. All signal changes occur solely in
a disposable launcher process immediately before exec of the requested binary.
"""
from pathlib import Path
import hashlib
import json
import os
import subprocess
import sys
import time

binary = Path(sys.argv[1]).resolve()
out = Path(sys.argv[2]).resolve()
out.mkdir(exist_ok=False, parents=True)
commands = out / 'commands'
commands.mkdir()
for name, line in [('dmesg', '<6>[    0.000000] Linux version fixture'),
                   ('journalctl', '[    0.000000] kernel: Linux version fixture')]:
    path = commands / name
    path.write_text('#!' + sys.executable + '\nimport os,pathlib\n'
        "pathlib.Path(os.environ['ALIBI_SPAWN_MARKER']).write_text('command executed')\n"
        + 'print(' + repr(line) + ')\n')
    path.chmod(0o700)
launcher = out / 'ignore-launcher.py'
launcher.write_text('import os,signal,sys\nsignal.signal(signal.SIGCHLD,signal.SIG_IGN)\n'
                    'os.execv(sys.argv[1],sys.argv[1:])\n')
records = []
for name, prefix in [('default', []), ('ignored', [sys.executable, str(launcher)])]:
    marker = out / (name + '.spawned')
    argv = prefix + [str(binary), 'check', '--json', 'kernel-scream']
    started = time.monotonic()
    result = subprocess.run(argv, capture_output=True, timeout=10,
        env=dict(os.environ, PATH=str(commands), ALIBI_SPAWN_MARKER=str(marker)))
    (out / (name + '.stdout')).write_bytes(result.stdout)
    (out / (name + '.stderr')).write_bytes(result.stderr)
    receipt = json.loads(result.stdout)
    records.append(dict(case=name, argv=argv, exit_status=result.returncode,
        elapsed_ms=(time.monotonic()-started)*1000, command_marker=marker.exists(),
        verdict=receipt['verdict'],
        stdout_sha256=hashlib.sha256(result.stdout).hexdigest(),
        stderr_sha256=hashlib.sha256(result.stderr).hexdigest()))
for path in commands.iterdir():
    path.chmod(0o600)
(out / 'RECEIPT.json').write_text(json.dumps(dict(schema_version=1,
    binary=str(binary), binary_sha256=hashlib.sha256(binary.read_bytes()).hexdigest(),
    records=records, owner='campaign root', purpose='SIGCHLD preflight external CLI evidence',
    retirement='review minimal replay inputs and receipts at campaign closeout',
    limits=['no PIDreuse', 'no unrelated signaling', 'no concurrent external reaper qualification']),
    indent=2)+'\n')
print(json.dumps(records, indent=2))
