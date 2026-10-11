#!/usr/bin/env python3
"""Root-admitted detached constructor for one module-free archive and39-case recovery plan."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import time
import uuid


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def save(path, data):
    with path.open('x') as stream:
        json.dump(data, stream, indent=2)
        stream.write('\n')
        stream.flush()
        os.fsync(stream.fileno())


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--candidate', type=Path, required=True)
    parser.add_argument('--source-revision', required=True)
    parser.add_argument('--runner', type=Path, required=True)
    parser.add_argument('--runner-sha256', required=True)
    args = parser.parse_args()
    root = Path('/data/git/alibi-qualification-evidence-20261011/guest-lab/requalification-20261011')
    lab = root.parent
    old = root / 'disclosed-batch'
    original = json.loads((old / 'PLAN.json').read_text())
    checkpoint = root / 'CHECKPOINT.json'
    active = json.loads(checkpoint.read_text()).get('active_producers', [])
    assert len(active) == 1 and active[0].get('kind') == 'archive-construction'
    assert Path(active[0]['script']).resolve() == Path(__file__).resolve()
    assert active[0]['script_sha256'] == sha(__file__)
    assert active[0]['source_revision'] == args.source_revision
    assert os.environ.get('INVOCATION_ID')
    assert original['source_revision'] == args.source_revision
    assert sha(args.candidate) == original['binary_sha256']
    assert sha(args.runner) == args.runner_sha256
    interrupted = json.loads((old / 'INDETERMINATE.json').read_text())
    assert sha(old / 'PLAN.json') == interrupted['plan_sha256']
    assert len(interrupted['records']) == 14 and interrupted['records'][-1]['case'] == 'G09'
    assert all(record['terminal_exists'] for record in interrupted['records'][:13])
    for record in interrupted['records'][:13]:
        assert sha(record['terminal']) == record['terminal_sha256']
    for field in ['archive_sha256', 'kernel_sha256']:
        for path, expected in original[field].items():
            assert sha(path) == expected
    for path in ['/', '/data', str(root)]:
        fs = os.statvfs(path)
        assert fs.f_bavail * fs.f_frsize - 134217728 >= 64424509440 and fs.f_favail >= 4096
    allocated = int(subprocess.check_output(['du', '-s', '-B1', str(lab)]).split()[0])
    assert allocated + 134217728 <= 4294967296
    build = root / 'recovery-acquisition-build'
    batch = root / 'disclosed-successor-batch'
    assert not build.exists() and not batch.exists(), 'Reconcile original constructor; never overwrite'
    batch.mkdir()
    protocol = batch / 'protocol'
    protocol.mkdir()
    for name in ['build-initramfs.py', 'guest-capture.sh', 'run-batch.py', 'capture_frames.py', 'carrier_control.py', 'guest-acquisition-cases.sh']:
        shutil.copy2(old / 'protocol' / name, protocol / name)
    shutil.copy2(args.runner, protocol / 'run-guest.py')
    retirement = 'Root final acceptance/replay; expanded root removable after seal/consumer check; exact custody review2026-10-18'
    state = dict(owner='root', purpose='Infrastructure-only successor; original product binary and cases preserved',
                 invocation_id=os.environ['INVOCATION_ID'], pid=os.getpid(), started=time.time(),
                 source_revision=args.source_revision, checkpoint_sha256=sha(checkpoint),
                 producer_sha256=sha(__file__), original_plan_sha256=sha(old / 'PLAN.json'),
                 runner_sha256=args.runner_sha256, retirement=retirement, phase='constructing-module-free-archive')
    save(batch / 'CONSTRUCTION-START.json', state)
    command = ['/usr/bin/python3', str(protocol / 'build-initramfs.py'), '--alibi', str(args.candidate.resolve()),
               '--output-dir', str(build), '--admission', str(lab / 'ADMISSION.json'), '--owner', 'root',
               '--retirement', retirement, '--scenario', str(protocol / 'guest-acquisition-cases.sh')]
    with (batch / 'construction.log').open('xb') as stream:
        subprocess.run(command, stdout=stream, stderr=subprocess.STDOUT, check=True)
    manifest = json.loads((build / 'BUILD.json').read_text())
    assert manifest['module_kernel_release'] is None
    assert sha(manifest['archive']) == manifest['archive_sha256']
    # Original first13 terminal attempts are preserved. G09 was never launched.
    cases = []
    for old_case in original['cases'][13:]:
        case = dict(old_case)
        case.update(run_id=str(uuid.uuid4()), output=str(batch / Path(old_case['output']).name))
        cases.append(case)
    assert len(cases) == 38 and cases[0]['case_id'] == 'G09'
    retry = dict(next(c for c in original['cases'] if c['case_id'] == 'acquisition-second-kernel'))
    retry.update(run_id=str(uuid.uuid4()), output=str(batch / 'acquisition-second-kernel-retry'), initramfs=manifest['archive'])
    cases.append(retry)
    plan = dict(original)
    plan.update(directory=str(batch), runner=str(protocol / 'run-guest.py'), cases=cases,
                protocol_hashes={p.name: sha(p) for p in protocol.iterdir()},
                archive_sha256={p: sha(p) for p in {c['initramfs'] for c in cases}},
                kernel_sha256={p: sha(p) for p in {c['kernel'] for c in cases}},
                original_plan=str(old / 'PLAN.json'), original_plan_sha256=sha(old / 'PLAN.json'),
                recovery='38 previously unexecuted cases plus6.8 bootstrap-only infrastructure retry; retain all original outcomes')
    assert len(cases) == 39
    save(batch / 'PLAN.json', plan)
    state.update(phase='constructed-not-launched', finished=time.time(), cases=39,
                 plan=str(batch / 'PLAN.json'), plan_sha256=sha(batch / 'PLAN.json'),
                 next_action='Root registers exact39-case serial producer; constructor never launches QEMU')
    save(batch / 'CONSTRUCTION-COMPLETE.json', state)
    print(json.dumps(state))


if __name__ == '__main__':
    main()
