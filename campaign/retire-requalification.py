#!/usr/bin/env python3
"""Exact successor cleanup: default read-only preflight; root separately authorizes --execute."""
import argparse
import datetime
import hashlib
import json
import os
from pathlib import Path
import shutil
import stat
import subprocess
import uuid

R = Path('/data/git/alibi-qualification-evidence-20261011/guest-lab/requalification-20261011')
W = Path('/data/git/alibi-requalification-20261011')
INVENTORY = R / 'storage-retirement-inventory-0a7910ff-d616-4aca-8f07-3ac6f42bdb7b.json'


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def capacity():
    output = {}
    for path in ['/', '/data', str(R)]:
        fs = os.statvfs(path)
        output[path] = dict(available_bytes=fs.f_bavail * fs.f_frsize, free_blocks=fs.f_bfree,
                            available_blocks=fs.f_bavail, block_size=fs.f_frsize, free_inodes=fs.f_favail,
                            reserve_met=fs.f_bavail * fs.f_frsize >= 64424509440)
    return output


def allocated(path):
    return int(subprocess.check_output(['du', '-s', '-B1', str(path)]).split()[0])


def save(path, data, exclusive=False):
    with path.open('x' if exclusive else 'w') as stream:
        json.dump(data, stream, indent=2)
        stream.write('\n')
        stream.flush()
        os.fsync(stream.fileno())


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--execute', action='store_true')
    parser.add_argument('--proposal', type=Path)
    parser.add_argument('--proposal-sha256')
    args = parser.parse_args()
    prior = json.loads(INVENTORY.read_text())
    targets = [Path(item['path']) for item in prior['cleanup_candidates']] + [R / 'fresh-holdout-build/root']
    assert len(targets) == 13 and len(set(targets)) == 13
    for path in targets:
        assert path.is_dir() and not path.is_symlink() and path.resolve().is_relative_to(R)
    if args.execute:
        assert args.proposal and args.proposal_sha256 and sha(args.proposal) == args.proposal_sha256
        proposal = json.loads(args.proposal.read_text())
        assert proposal['producer_sha256'] == sha(__file__)
        assert [x['path'] for x in proposal['objects']] == [str(p) for p in targets]
    checkpoint = json.loads((R / 'CHECKPOINT.json').read_text())
    assert checkpoint['active_producers'] == []
    verified = []
    def verify(path, expected, receipt):
        assert sha(path) == expected, path
        verified.append(dict(path=str(path), sha256=expected, receipt=str(receipt)))
    for manifest in R.rglob('BUILD.json'):
        data = json.loads(manifest.read_text())
        verify(data['archive'], data['archive_sha256'], manifest)
    for receipt in R.glob('candidate*.receipt.json'):
        value = json.loads(receipt.read_text())['binary']
        verify(value['path'], value['sha256'], receipt)
    frozen = json.loads((R / 'FINAL-FREEZE.json').read_text())
    verify(frozen['binary']['path'], frozen['binary']['sha256'], R / 'FINAL-FREEZE.json')
    copies = W / 'campaign/fresh-holdouts-20261011'
    seal = json.loads((copies / 'SEAL.json').read_text())
    for value in seal['inputs']:
        verify(value['path'], value['sha256'], copies / 'SEAL.json')
        verify(copies / Path(value['path']).name, value['sha256'], copies / 'SEAL.json')
    for copy in copies.iterdir():
        original = R / 'holdouts' / copy.name
        if original.exists():
            verify(copy, sha(original), original)
    terminal_chains = []
    for batch_name in ['disclosed-successor-batch', 'fresh-holdout-batch']:
        batch = R / batch_name
        completion = json.loads((batch / 'COMPLETE.json').read_text())
        assert completion['phase'] == 'terminal'
        verify(batch / 'PLAN.json', completion['plan_sha256'], batch / 'COMPLETE.json')
        for record in completion['records']:
            assert record['terminal_exists']
            verify(record['terminal'], record['terminal_sha256'], batch / 'COMPLETE.json')
        terminal_chains.append(dict(path=str(batch / 'COMPLETE.json'), sha256=sha(batch / 'COMPLETE.json'), cases=len(completion['records'])))
    original = json.loads((R / 'disclosed-batch/INDETERMINATE.json').read_text())
    assert len(original['records']) == 14
    for record in original['records'][:13]:
        assert record['terminal_exists']
        verify(record['terminal'], record['terminal_sha256'], R / 'disclosed-batch/INDETERMINATE.json')
    for execution in R.rglob('EXECUTION.json'):
        # Guest executions have qemu_pid. Producer records use other fields.
        data = json.loads(execution.read_text())
        if 'qemu_pid' in data:
            assert (execution.parent / 'TERMINAL.json').is_file(), execution
    def inside(name):
        return any(name == str(path) or name.startswith(str(path) + '/') for path in targets)
    mounts = [line for line in Path('/proc/self/mountinfo').read_text().splitlines() if inside(line.split()[4])]
    assert not mounts
    references = []
    unreadable = []
    for process in Path('/proc').iterdir():
        if not process.name.isdigit() or int(process.name) == os.getpid():
            continue
        try:
            links = [process / 'cwd', process / 'exe'] + list((process / 'fd').iterdir())
            for link in links:
                try:
                    name = os.readlink(link)
                    if inside(name):
                        references.append(dict(pid=process.name, path=name))
                except OSError:
                    pass
        except PermissionError:
            unreadable.append(process.name)
        except OSError:
            pass
    assert not references
    backing = []
    for entry in Path('/sys/block').glob('loop*/loop/backing_file'):
        try:
            name = entry.read_text().strip()
            if inside(name):
                backing.append(dict(path=str(entry), backing=name))
        except OSError:
            pass
    assert not backing
    groups = {}
    candidates = set()
    for current, directories, files in os.walk(R, followlinks=False):
        for name in files:
            path = Path(current) / name
            info = path.lstat()
            if stat.S_ISREG(info.st_mode):
                key = (info.st_dev, info.st_ino)
                groups.setdefault(key, []).append((str(path), info.st_nlink, info.st_blocks * 512))
                if inside(str(path)):
                    candidates.add(key)
    hardlinks = []
    for key in candidates:
        names = groups[key]
        assert len(names) == names[0][1], ('unresolved aliases', names)
        if len(names) > 1:
            hardlinks.append(dict(paths=[x[0] for x in names], retained=[x[0] for x in names if not inside(x[0])]))
    qmp = []
    for custody in (R / 'disclosed-successor-batch').glob('*/CONTROL-CUSTODY.json'):
        data = json.loads(custody.read_text())
        terminal = json.loads((custody.parent / 'TERMINAL.json').read_text())
        for key in ['socket', 'directory']:
            path = Path(data[key])
            assert not path.exists() and not path.is_symlink()
        assert terminal['carrier_control']['directory_removed']
        qmp.append(dict(custody=str(custody), sha256=sha(custody), directory=data['directory'], absent=True))
    record = dict(schema_version=1, time=datetime.datetime.now(datetime.timezone.utc).isoformat(),
                  owner='root', execution_lane='lab_feasibility', producer_sha256=sha(__file__),
                  state='PREFLIGHT_ONLY_NO_DELETION', inventory=str(INVENTORY), inventory_sha256=sha(INVENTORY),
                  objects=[dict(path=str(p), allocated_before=allocated(p)) for p in targets],
                  verified_inputs=verified, terminal_chains=terminal_chains, mounts=mounts,
                  live_references=references, uninspectable_process_fd_directories=unreadable,
                  loop_backing=backing, hardlinks=hardlinks, qmp_runtime_retirement=qmp,
                  before_capacity=capacity(), allocated_R_before=allocated(R),
                  would_reclaim_unique_file_blocks=sum(groups[k][0][2] for k in candidates if all(inside(x[0]) for x in groups[k])),
                  preserved='Source worktree/commits, candidates1+2, exact archives, original failed fixtures, protocols, receipts, sealed input originals and copied source remain intact. Parent artifacts outsideR untouched.',
                  custody='Root final acceptance/replay review2026-10-18; no indefinite retention default.',
                  limits=['Visible process namespace/readable descriptors only; preflight is not a global absence theorem.',
                          'Filesystem deltas include concurrent tenants; do not attribute them solely to cleanup.',
                          'Per-object totals include directory blocks; unique file blocks account for retained hardlinks.',
                          'No SSH/agent operations; no parent campaign cleanup.'])
    if not args.execute:
        output = R / ('cleanup-proposal-' + str(uuid.uuid4()) + '.json')
        save(output, record, True)
    else:
        output = R / 'STORAGE-CLOSEOUT.json'
        record.update(state='PRECONDITIONS_VERIFIED', authorized_proposal=str(args.proposal), authorized_proposal_sha256=args.proposal_sha256)
        save(output, record, True)
        for path in targets:
            shutil.rmtree(path)
            assert not path.exists()
        record.update(state='EXACT_SCOPED_RETIREMENT_COMPLETE', completed_at=datetime.datetime.now(datetime.timezone.utc).isoformat(),
                      after_capacity=capacity(), allocated_R_after=allocated(R), deleted_paths=[str(p) for p in targets],
                      retired_path_allocated_sum=sum(x['allocated_before'] for x in record['objects']))
        save(output, record)
    print(json.dumps(dict(receipt=str(output), sha256=sha(output), state=record['state'], objects=len(targets))))


if __name__ == '__main__':
    main()
