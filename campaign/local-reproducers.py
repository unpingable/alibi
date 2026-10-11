#!/usr/bin/env python3
"""Bounded substitution/parser checks, never real-host fault qualification.

Usage: local-reproducers.py BASELINE_BINARY BASELINE_RLIB NEW_OUTPUT_DIRECTORY
Requires the already-built matching release rlib. Does not run cargo or mutate host
configuration. Retires its reproducible helper executable after recording a hash.
"""
import datetime
import hashlib
import json
import os
from pathlib import Path
import signal
import subprocess
import sys
import time

binary, library, destination = map(lambda s: Path(s).resolve(), sys.argv[1:])
destination.mkdir(exist_ok=False, parents=True)
script_dir = Path(__file__).resolve().parent
records = []
def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()
def invoke(name, argv, env=None, timeout=15):
    started = time.monotonic()
    child = subprocess.Popen(list(map(str, argv)), stdout=subprocess.PIPE,
                             stderr=subprocess.PIPE, env=env, start_new_session=True)
    timed_out = False
    try:
        stdout, stderr = child.communicate(timeout=timeout)
    except subprocess.TimeoutExpired:
        timed_out = True
        os.killpg(child.pid, signal.SIGKILL)
        stdout, stderr = child.communicate(timeout=2)
    (destination / (name + '.stdout')).write_bytes(stdout)
    (destination / (name + '.stderr')).write_bytes(stderr)
    record = dict(case=name, argv=list(map(str, argv)), exit_status=child.returncode,
                  elapsed_ms=round((time.monotonic()-started)*1000, 3),
                  outer_timeout=timed_out, classification='local substitution/parser fixture',
                  stdout_sha256=hashlib.sha256(stdout).hexdigest(),
                  stderr_sha256=hashlib.sha256(stderr).hexdigest())
    records.append(record)
    return record
helper = destination/'reproducer'
compiled = invoke('compile-helper', ['rustc', '--edition=2021', script_dir/'local-reproducers.rs',
    '--extern', 'alibi='+str(library), '-L', 'dependency='+str(library.parent/'deps'), '-o', helper])
if compiled['exit_status'] != 0:
    raise SystemExit('helper compilation failed; retained logs')
helper_hash = digest(helper)
for case in ['route-mask', 'pipe-timeout', 'malformed-parsers']:
    invoke(case, [helper, case], timeout=5)
fixture = destination/'commands'
fixture.mkdir()
for name, text in [('malformed-log', 'not a kernel log'), ('healthy-log-control', '[    0.000000] Linux version fixture-control'), ('empty-log-control', '')]:
    (fixture/'dmesg').write_text('#!/bin/sh\nprintf \'%s\\n\' '+repr(text)+'\n')
    (fixture/'journalctl').write_text('#!/bin/sh\nexit 1\n')
    for path in fixture.iterdir():
        path.chmod(0o700)
    (destination/(name+'.fixture')).write_text((fixture/'dmesg').read_text())
    env = dict(os.environ, PATH=str(fixture))
    invoke(name, [binary, 'check', '--json', 'kernel-scream'], env=env, timeout=5)
helper_bytes = helper.stat().st_size
helper.unlink()
for path in fixture.iterdir():
    path.unlink()
fixture.rmdir()
receipt = dict(schema_version=1, timestamp=datetime.datetime.now(datetime.timezone.utc).isoformat(),
    owner='alibi qualification root', purpose='bounded implementation counterexamples',
    baseline_binary=str(binary), baseline_binary_sha256=digest(binary),
    baseline_rlib=str(library), baseline_rlib_sha256=digest(library),
    helper_source_sha256=digest(script_dir/'local-reproducers.rs'), helper_sha256=helper_hash,
    helper_bytes_created_and_reclaimed=helper_bytes, records=records,
    custody='Retain result logs and fixtures with campaign findings; reproducible helper retired immediately.',
    retirement_condition='Minimal evidence follows campaign release-review custody; review at campaign closeout.',
    limitations=['Not guest fault qualification', 'Parser calls are library calls; kernel log cases use supported external CLI'])
(destination/'RECEIPT.json').write_text(json.dumps(receipt, indent=2)+'\n')
print(json.dumps(dict(output=str(destination), retained_bytes=sum(p.stat().st_size for p in destination.iterdir()), helper_bytes_reclaimed=helper_bytes)))
