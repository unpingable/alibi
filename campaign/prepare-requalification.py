#!/usr/bin/env python3
"""Construct disclosed successor archives and a serial plan; never launch guests.
Root runs this producer through the existing detached user-systemd protocol.
"""
import argparse
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import time
import uuid


def digest(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def save(path, value):
    temporary = path.with_suffix('.new')
    with temporary.open('w') as stream:
        json.dump(value, stream, indent=2)
        stream.write('\n')
        stream.flush()
        os.fsync(stream.fileno())
    temporary.replace(path)


def allocated(path):
    return int(subprocess.check_output(['du', '-s', '-B1', str(path)]).split()[0])


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--candidate', type=Path, required=True)
    parser.add_argument('--source-revision', required=True)
    parser.add_argument('--candidate-sha256', required=True)
    parser.add_argument('--worktree', type=Path, default=Path('/data/git/alibi-requalification-20261011'))
    parser.add_argument('--root', type=Path, default=Path('/data/git/alibi-qualification-evidence-20261011/guest-lab/requalification-20261011'))
    args = parser.parse_args()
    root = args.root.resolve()
    lab = root.parent
    campaign = args.worktree.resolve() / 'campaign'
    assert digest(args.candidate) == args.candidate_sha256
    revision = subprocess.check_output(['git', 'rev-parse', args.source_revision + '^{commit}'], cwd=args.worktree, text=True).strip()
    checkpoint = root / 'CHECKPOINT.json'
    assert checkpoint.is_file()
    checkpoint_data = json.loads(checkpoint.read_text())
    assert not checkpoint_data.get('active_producers'), 'Root must admit exclusive construction before launch'
    initial = allocated(root)
    build_root = root / 'disclosed-builds'
    batch = root / 'disclosed-batch'
    assert not build_root.exists() and not batch.exists(), 'Reconcile original construction; never overwrite/restart'
    for path in ['/', '/data', str(root)]:
        fs = os.statvfs(path)
        assert fs.f_bavail * fs.f_frsize - 1073741824 >= 64424509440
        assert fs.f_favail >= 8192
    assert allocated(lab) + 1073741824 <= 4294967296
    build_root.mkdir()
    batch.mkdir()
    protocol = batch / 'protocol'
    protocol.mkdir()
    retirement = 'Root independent acceptance/replay; expanded trees retire after seal and consumer checks; custody review 2026-10-18'
    names = ['build-initramfs.py', 'guest-capture.sh', 'run-guest.py', 'run-batch.py', 'capture_frames.py', 'carrier_control.py', 'extended-builder-args.json']
    scenarios = {
        'matrix': campaign / 'guest-matrix.sh',
        'route': campaign / 'guest-route-cases.sh',
        'acquisition': campaign / 'guest-acquisition-cases.sh',
        'carrier': campaign / 'guest-carrier-cases.sh',
        'filesystem': campaign / 'guest-filesystem-identity-controls.sh',
        'tun': campaign / 'guest-tun-cases.sh',
        'old-holdouts': campaign / 'holdouts/guest-holdouts.sh',
        'churn': campaign / 'guest-churn-regressions.sh',
    }
    for name in names:
        shutil.copy2(campaign / name, protocol / name)
    for source in scenarios.values():
        shutil.copy2(source, protocol / source.name)
    protocol_hashes = {p.name: digest(p) for p in protocol.iterdir()}
    state = dict(owner='root', purpose='51 disclosed guest requalification construction only', retirement=retirement,
                 source_revision=revision, candidate=str(args.candidate.resolve()), candidate_sha256=args.candidate_sha256,
                 checkpoint=str(checkpoint), checkpoint_sha256=digest(checkpoint), producer_sha256=digest(__file__),
                 invocation_id=os.environ.get('INVOCATION_ID'), pid=os.getpid(), started=time.time(),
                 protocol_hashes=protocol_hashes, initial_allocated_root=initial, phase='constructing', completed=[])
    save(build_root / 'START.json', state)
    common = json.loads((protocol / 'extended-builder-args.json').read_text())['argv']
    e1000 = lab / 'carrier-inputs-cc5d42ed-3283-409e-a11d-15cbe63905e4/e1000.ko'
    old_tun = lab / 'tun-inputs-88d2b6a9-6ae6-47fb-87ca-cdd2bbe4be5a/tun-drop-fixture'
    archives = {}
    for label, scenario in scenarios.items():
        assert allocated(lab) + 134217728 <= 4294967296
        assert allocated(root) - initial + 134217728 <= 1073741824
        options = list(common)
        if label == 'carrier':
            options += ['--module-file', str(e1000)]
        if label == 'tun':
            options += ['--tool', str(old_tun)]
        command = ['/usr/bin/python3', str(protocol / 'build-initramfs.py'), '--alibi', str(args.candidate.resolve()),
                   '--output-dir', str(build_root / label), '--admission', str(lab / 'ADMISSION.json'),
                   '--owner', 'root', '--retirement', retirement, '--scenario', str(protocol / scenario.name)] + options
        state.update(current=label, command=command)
        save(build_root / 'PROGRESS.json', state)
        with (build_root / (label + '.log')).open('wb') as log:
            subprocess.run(command, stdout=log, stderr=subprocess.STDOUT, check=True)
        manifest = json.loads((build_root / label / 'BUILD.json').read_text())
        assert digest(manifest['archive']) == manifest['archive_sha256']
        archives[label] = manifest['archive']
        state['completed'].append(dict(label=label, archive=manifest['archive'], sha256=manifest['archive_sha256']))
        save(build_root / 'PROGRESS.json', state)
    prior_path = lab / 'final-matrix-batch/PLAN.json'
    prior = json.loads(prior_path.read_text())
    mapping = {'matrix-final-build': 'matrix', 'route-final-build': 'route', 'acquisition-final-build': 'acquisition',
               'carrier-final-build': 'carrier', 'filesystem-controls-final-build': 'filesystem'}
    cases = []
    for old in prior['cases']:
        if old['case_id'] in ['G30', 'G36']:
            continue
        case = dict(old)
        label = 'tun' if old['case_id'] in ['G13', 'G14'] else mapping[Path(old['initramfs']).parent.name]
        case.update(initramfs=archives[label], source_revision=revision, run_id=str(uuid.uuid4()),
                    output=str(batch / Path(old['output']).name))
        cases.append(case)
    kernel = str(lab / 'kernel-package/boot/vmlinuz-6.5.0-44-generic')
    for prefix, count, label in [('H', 6, 'old-holdouts'), ('C', 4, 'churn')]:
        for index in range(1, count + 1):
            case_id = f'{prefix}{index:02}'
            cases.append(dict(case_id=case_id, run_id=str(uuid.uuid4()), kernel=kernel,
                              initramfs=archives[label], source_revision=revision, output=str(batch / case_id),
                              nics=1, memory_mib=512, timeout=90 if prefix == 'H' else 180))
    assert len(cases) == 51
    plan = dict(directory=str(batch), runner=str(protocol / 'run-guest.py'), checkpoint=str(checkpoint),
                allocation_root=str(lab), allocation_cap=4294967296, pending_allocation_reserve=268435456,
                retirement=retirement, protocol_hashes=protocol_hashes, cases=cases,
                source_revision=revision, binary_sha256=args.candidate_sha256,
                prior_plan=str(prior_path), prior_plan_sha256=digest(prior_path),
                archive_sha256={p: digest(p) for p in archives.values()},
                kernel_sha256={p: digest(p) for p in {c['kernel'] for c in cases}},
                scope='Disclosed regressions only; fresh sealed holdouts are excluded')
    save(batch / 'PLAN.json', plan)
    state.update(phase='constructed-not-launched', finished=time.time(), plan=str(batch / 'PLAN.json'),
                 plan_sha256=digest(batch / 'PLAN.json'), allocated_root_after=allocated(root),
                 next_action='Root registers exact detached serial producer checkpoint; no launch by constructor')
    save(build_root / 'COMPLETE.json', state)
    print(json.dumps({'plan': state['plan'], 'sha256': state['plan_sha256'], 'cases': len(cases)}))


if __name__ == '__main__':
    main()
