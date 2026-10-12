#!/usr/bin/env python3
"""Package one Linux x86_64 GNU executable; never publish or install it."""
import argparse
import gzip
import hashlib
import io
import json
import re
from pathlib import Path
import subprocess
import tarfile
import tomllib

QUALIFIED_SOURCE = '7e04d3cb076a48f2e131d6284a16f6bbb836ff58'
QUALIFIED_BINARY = '0760ea3529ac3c3d831038a71c46f82b6bd3d750fd4dbc52866399962e593b06'
ROOT = Path(__file__).resolve().parent.parent


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--binary', required=True, type=Path)
    parser.add_argument('--output', required=True, type=Path)
    parser.add_argument('--glibc-baseline', choices=['2.31'], help='Verify the compatibility build ABI and use distinct asset naming')
    args = parser.parse_args()
    version = tomllib.loads((ROOT/'Cargo.toml').read_text())['package']['version']
    if not re.fullmatch(r'[0-9]+\.[0-9]+\.[0-9]+(?:-[A-Za-z0-9.-]+)?', version):
        parser.error('unsupported package version')
    archive_name = 'alibi-' + version + '-x86_64-unknown-linux-gnu' + ('-glibc' + args.glibc_baseline if args.glibc_baseline else '')
    binary = args.binary.read_bytes()
    if binary[:6] != b'\x7fELF\x02\x01' or binary[18:20] != b'\x3e\x00':
        parser.error('expected a little-endian ELF64 x86_64 binary')
    reported = subprocess.check_output([str(args.binary.resolve()), '--version'], text=True, timeout=10).strip()
    if reported != 'alibi ' + version:
        parser.error('binary version does not match Cargo.toml')
    revision = subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=ROOT, text=True).strip()
    # Packaging does not silently qualify changed product source.
    correspondence = subprocess.run(['git', 'diff', '--quiet', QUALIFIED_SOURCE, '--',
                                     'src', 'tests', 'Cargo.toml', 'Cargo.lock'], cwd=ROOT)
    if correspondence.returncode not in (0, 1):
        parser.error('cannot establish source correspondence')
    unchanged_product = correspondence.returncode == 0
    binary_sha = hashlib.sha256(binary).hexdigest()
    required_glibc = None
    if args.glibc_baseline:
        elf = subprocess.check_output(['readelf', '-W', '-V', str(args.binary)], text=True)
        versions = re.findall(r'Name: GLIBC_(\d+(?:\.\d+)+)', elf)
        if not versions or 'GLIBC_PRIVATE' in elf:
            parser.error('cannot establish public glibc requirements')
        required_glibc = max(versions, key=lambda v: tuple(map(int, v.split('.'))))
        if tuple(map(int, required_glibc.split('.'))) > (2, 31):
            parser.error('binary exceeds glibc 2.31 baseline')
    source = dict(version=version, stage='experimental', repository='https://github.com/unpingable/alibi',
                  repository_revision=revision, qualified_product_revision=QUALIFIED_SOURCE,
                  product_source_matches_qualified=unchanged_product,
                  binary_sha256=binary_sha, exact_guest_qualified_binary=binary_sha == QUALIFIED_BINARY,
                  qualification=('QUALIFIED_WITH_LIMITATIONS; see docs/requalification-20261011/QUALIFICATION.md'
                                 if unchanged_product else 'CHANGED_SOURCE; historical baseline QUALIFIED_WITH_LIMITATIONS does not qualify this build'),
                  packaging='Dynamic Linux x86_64 GNU binary. No static or universal Linux compatibility claim.',
                  note='Check product_source_matches_qualified and binary identity separately; this package does not establish new host qualification.')
    if args.glibc_baseline:
        source.update(compatibility_baseline='glibc 2.31', maximum_required_glibc_symbol=required_glibc,
                      note='New older-glibc build; see docs/linux-compatibility.md for its bounded runtime checks. Not the original guest-qualified binary.')
    entries = {'alibi': binary, 'SOURCE.json': (json.dumps(source, indent=2)+'\n').encode()}
    for name, path in [('LICENSE', ROOT/'LICENSE'), ('NOTICE', ROOT/'NOTICE'),
                       ('INSTALL.md', ROOT/'docs/install.md')]:
        entries[name] = path.read_bytes()
    args.output.mkdir(parents=True, exist_ok=True)
    archive = args.output/(archive_name+'.tar.gz')
    with archive.open('xb') as stream:
        with gzip.GzipFile(fileobj=stream, mode='wb', filename='', mtime=0) as compressed:
            with tarfile.open(fileobj=compressed, mode='w') as tar:
                for member, data in sorted(entries.items()):
                    info = tarfile.TarInfo(archive_name+'/'+member)
                    info.size = len(data)
                    info.mode = 0o755 if member == 'alibi' else 0o644
                    info.mtime = 0
                    tar.addfile(info, io.BytesIO(data))
    digest = hashlib.sha256(archive.read_bytes()).hexdigest()
    with (args.output/'SHA256SUMS').open('x') as stream:
        stream.write(digest+'  '+archive.name+'\n')
    print(json.dumps(source, indent=2))


if __name__ == '__main__':
    main()
