#!/usr/bin/env python3
"""Root-registered detached constructor only; never launches a guest or the helper."""
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import time
import uuid

ROOT = Path('/data/git/alibi-qualification-evidence-20261011/guest-lab/requalification-20261011')
LAB = ROOT.parent
FLOOR = 64424509440
ENVELOPE = 134217728
SOURCE = '7e04d3cb076a48f2e131d6284a16f6bbb836ff58'
BINARY = '0760ea3529ac3c3d831038a71c46f82b6bd3d750fd4dbc52866399962e593b06'
FREEZE = '9973ed36810bfe84217933179723fb840dc2a574c939f49bef37f5c7f46e2ddd'
SEAL = '20b0be9e4ee3c355ff805becbc7d20b5d43cdacc004f4ad69a7e6eead3090a39'
EXECUTION = '6e5ac50d4f1e4cd0b3499723c47f5d1c164f362b11ca783fd898cd118624e3ce'
PROTOCOL_HASHES = {
    'build-initramfs.py': '385e34bc6e507a0f7a5bb8fea6a00768ce991db580ebc93e431532ec57768293',
    'guest-capture.sh': 'b9dcb4f9a11671019a1ed6cccec36882244e284a39939145b7e3b0668c76f8ff',
    'run-batch.py': '0136d387633b173cb10137d0ea2d2f1541f3209c950fc7a74a48996b40a40278',
    'run-guest.py': 'dedbf5ddb1881ec89f0a2bbd2d3c10c01bbc1e81ae4205de8a5e2f27acc50d62',
    'capture_frames.py': 'f5d560338b5114ad76d56351527ddc1303f4eb95e93be5523d840f7fdbfce639',
    'carrier_control.py': '477405fe725392117b8784e34a6feab061848ba6ddd4fdc63d3e5f0dfd5fea22',
}


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def save(path, value):
    with path.open('x') as stream:
        json.dump(value, stream, indent=2)
        stream.write('\n')
        stream.flush()
        os.fsync(stream.fileno())


def capacity():
    result = {}
    for name in ['/', '/data', str(ROOT)]:
        fs = os.statvfs(name)
        available = fs.f_bavail * fs.f_frsize
        assert available - ENVELOPE >= FLOOR and fs.f_favail >= 4096
        result[name] = dict(available_bytes=available, free_inodes=fs.f_favail)
    allocated = int(subprocess.check_output(['du', '-s', '-B1', str(LAB)]).split()[0])
    assert allocated + ENVELOPE <= 4294967296
    return dict(filesystems=result, aggregate_allocated_bytes=allocated,
                admitted_new_envelope_bytes=ENVELOPE, aggregate_cap=4294967296)


def main():
    checkpoint = ROOT / 'CHECKPOINT.json'
    active = json.loads(checkpoint.read_text()).get('active_producers', [])
    assert len(active) == 1 and active[0].get('kind') == 'archive-construction'
    producer = active[0]
    assert Path(producer['script']).resolve() == Path(__file__).resolve()
    assert producer['script_sha256'] == sha(__file__) and producer['source_revision'] == SOURCE
    invocation = os.environ.get('INVOCATION_ID')
    assert invocation
    actual = subprocess.check_output(['systemctl', '--user', 'show', producer['unit'],
                                      '-p', 'InvocationID', '--value'], text=True).strip()
    assert actual == invocation
    if producer.get('invocation_id'):
        assert producer['invocation_id'] == invocation
    freeze_path = ROOT / 'FINAL-FREEZE.json'
    assert sha(freeze_path) == FREEZE
    freeze = json.loads(freeze_path.read_text())
    assert freeze['product_source_revision'] == SOURCE
    candidate = Path(freeze['binary']['path'])
    assert sha(candidate) == BINARY == freeze['binary']['sha256']
    hidden = ROOT / 'holdouts'
    assert sha(hidden / 'SEAL.json') == SEAL
    assert sha(hidden / 'EXECUTION-SEAL-V2.json') == EXECUTION
    for item in json.loads((hidden / 'SEAL.json').read_text())['inputs']:
        assert sha(item['path']) == item['sha256']
    addendum = json.loads((hidden / 'EXECUTION-SEAL-V2.json').read_text())
    for key in ['helper_receipt', 'helper_source', 'original_helper_source', 'helper_binary']:
        assert sha(addendum[key]['path']) == addendum[key]['sha256']
    helper = Path(addendum['helper_binary']['path'])
    assert helper.name == 'tun-drop-fixture'
    old = ROOT / 'disclosed-successor-batch'
    original = json.loads((old / 'PLAN.json').read_text())
    assert (old / 'COMPLETE.json').is_file()
    for name, expected in PROTOCOL_HASHES.items():
        assert sha(old / 'protocol' / name) == expected
    kernel = LAB / 'kernel-package/boot/vmlinuz-6.5.0-44-generic'
    assert sha(kernel) == original['kernel_sha256'][str(kernel)]
    build = ROOT / 'fresh-holdout-build'
    batch = ROOT / 'fresh-holdout-batch'
    assert not build.exists() and not batch.exists(), 'Reconcile original producer; never duplicate or overwrite'
    admitted = capacity()
    batch.mkdir()
    protocol = batch / 'protocol'
    protocol.mkdir()
    for name in PROTOCOL_HASHES:
        shutil.copy2(old / 'protocol' / name, protocol / name)
    shutil.copy2(hidden / 'guest-sealed.sh', protocol / 'guest-sealed.sh')
    retirement = 'Root final holdout acceptance/replay; exact inputs retained; expanded root retires after seal/consumer checks; review2026-10-18'
    state = dict(owner='root', purpose='Exact ten previously sealed once-only fresh holdout cases',
                 invocation_id=invocation, unit=producer['unit'], pid=os.getpid(), cwd=os.getcwd(),
                 started=time.time(), source_revision=SOURCE, binary_sha256=BINARY,
                 producer_sha256=sha(__file__), checkpoint_sha256=sha(checkpoint),
                 freeze_sha256=FREEZE, case_seal_sha256=SEAL, execution_seal_sha256=EXECUTION,
                 helper_sha256=sha(helper), storage=admitted, retirement=retirement,
                 phase='constructing-fresh-holdout-archive')
    save(batch / 'CONSTRUCTION-START.json', state)
    try:
        # Existing matching kernel modules and exact GNU tools; no overlay or QMP case.
        modules = LAB / 'matching-modules-51279060-4abd-4eaf-8f50-3ac57507374a/modules'
        module_names = ['veth', 'libcrc32c', 'nf_defrag_ipv4', 'nf_defrag_ipv6',
                        'nf_conntrack', 'nfnetlink', 'nf_tables', 'nft_ct']
        tools = ['/usr/sbin/mke2fs', '/usr/sbin/losetup', '/usr/bin/stat',
                 '/usr/bin/setpriv', '/usr/bin/unshare', '/usr/sbin/nft',
                 '/usr/bin/bash', '/usr/bin/dd', str(helper)]
        command = ['/usr/bin/python3', str(protocol / 'build-initramfs.py'), '--alibi', str(candidate),
                   '--output-dir', str(build), '--admission', str(LAB / 'ADMISSION.json'),
                   '--owner', 'root', '--retirement', retirement,
                   '--scenario', str(protocol / 'guest-sealed.sh'), '--kernel-release', '6.5.0-44-generic']
        for tool in tools:
            command += ['--tool', tool]
        for name in module_names:
            command += ['--module-file', str(modules / (name + '.ko'))]
        save(batch / 'CONSTRUCTION-INPUTS.json', dict(command=command,
             tools={p: sha(p) for p in tools}, modules={str(modules / (n + '.ko')): sha(modules / (n + '.ko')) for n in module_names}))
        capacity()  # immediate recheck before the material allocation
        with (batch / 'construction.log').open('xb') as stream:
            subprocess.run(command, stdout=stream, stderr=subprocess.STDOUT, check=True)
        manifest = json.loads((build / 'BUILD.json').read_text())
        assert sha(manifest['archive']) == manifest['archive_sha256']
        assert sha(build / 'root/bin/alibi') == BINARY
        assert sha(build / 'root/bin/tun-drop-fixture') == sha(helper)
        assert sha(build / 'root/bin/sh') == sha('/usr/bin/dash')
        cases = [dict(case_id=f'Q{n:02}', run_id=str(uuid.uuid4()), kernel=str(kernel),
                      initramfs=manifest['archive'], source_revision=SOURCE,
                      output=str(batch / f'Q{n:02}'), nics=1, memory_mib=512,
                      timeout=180, carrier_control=False, nic_model='virtio-net-pci') for n in range(1, 11)]
        plan = dict(directory=str(batch), runner=str(protocol / 'run-guest.py'),
                    checkpoint=str(checkpoint), allocation_root=str(LAB), allocation_cap=4294967296,
                    pending_allocation_reserve=ENVELOPE, retirement=retirement,
                    source_revision=SOURCE, binary_sha256=BINARY, cases=cases,
                    protocol_hashes={p.name: sha(p) for p in protocol.iterdir()},
                    archive_sha256={manifest['archive']: manifest['archive_sha256']},
                    kernel_sha256={str(kernel): sha(kernel)}, freeze_sha256=FREEZE,
                    original_case_seal_sha256=SEAL, execution_seal_sha256=EXECUTION,
                    construction_complete=str(batch / 'CONSTRUCTION-COMPLETE.json'))
        save(batch / 'PLAN.json', plan)
        state.update(phase='constructed-not-launched', finished=time.time(), cases=10,
                     plan=str(batch / 'PLAN.json'), plan_sha256=sha(batch / 'PLAN.json'),
                     build_receipt=str(build / 'BUILD.json'), build_receipt_sha256=sha(build / 'BUILD.json'),
                     next_action='Root registers exact ten-case serial producer and launches existing run-batch.py; no tuning')
        save(batch / 'CONSTRUCTION-COMPLETE.json', state)
        print(json.dumps(state))
    except BaseException as error:
        state.update(phase='construction-failed', finished=time.time(), error=repr(error),
                     next_action='Root reconciles this original terminal; preserve inputs and failure; no automatic restart')
        save(batch / 'CONSTRUCTION-FAILED.json', state)
        raise


if __name__ == '__main__':
    main()
