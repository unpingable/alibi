#!/usr/bin/env python3
"""Package one Linux x86_64 GNU executable; never publish or install it."""
import argparse
import gzip
import hashlib
import io
import json
from pathlib import Path
import subprocess
import tarfile

QUALIFIED_SOURCE = '7e04d3cb076a48f2e131d6284a16f6bbb836ff58'
QUALIFIED_BINARY = '0760ea3529ac3c3d831038a71c46f82b6bd3d750fd4dbc52866399962e593b06'
NAME = 'alibi-0.1.0-x86_64-unknown-linux-gnu'
ROOT = Path(__file__).resolve().parent.parent


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--binary', required=True, type=Path)
    parser.add_argument('--output', required=True, type=Path)
    args = parser.parse_args()
    binary = args.binary.read_bytes()
    if binary[:6] != b'\x7fELF\x02\x01' or binary[18:20] != b'\x3e\x00':
        parser.error('expected a little-endian ELF64 x86_64 binary')
    revision = subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=ROOT, text=True).strip()
    # Packaging does not silently qualify changed product source.
    subprocess.run(['git', 'diff', '--exit-code', QUALIFIED_SOURCE, '--',
                    'src', 'tests', 'Cargo.toml', 'Cargo.lock'], cwd=ROOT, check=True)
    binary_sha = hashlib.sha256(binary).hexdigest()
    source = dict(version='0.1.0', stage='experimental', repository='https://github.com/unpingable/alibi',
                  repository_revision=revision, qualified_product_revision=QUALIFIED_SOURCE,
                  binary_sha256=binary_sha, exact_guest_qualified_binary=binary_sha == QUALIFIED_BINARY,
                  qualification='QUALIFIED_WITH_LIMITATIONS; see docs/requalification-20261011/QUALIFICATION.md',
                  packaging='Dynamic Linux x86_64 GNU binary. No static or universal Linux compatibility claim.',
                  note='A CI rebuild has source correspondence; it is not the original guest-tested executable unless its hash matches.')
    entries = {'alibi': binary, 'SOURCE.json': (json.dumps(source, indent=2)+'\n').encode()}
    for name, path in [('LICENSE', ROOT/'LICENSE'), ('NOTICE', ROOT/'NOTICE'),
                       ('INSTALL.md', ROOT/'docs/release-v0.1/INSTALL.md')]:
        entries[name] = path.read_bytes()
    args.output.mkdir(parents=True, exist_ok=True)
    archive = args.output/(NAME+'.tar.gz')
    with archive.open('xb') as stream:
        with gzip.GzipFile(fileobj=stream, mode='wb', filename='', mtime=0) as compressed:
            with tarfile.open(fileobj=compressed, mode='w') as tar:
                for name, data in sorted(entries.items()):
                    info = tarfile.TarInfo(NAME+'/'+name)
                    info.size = len(data)
                    info.mode = 0o755 if name == 'alibi' else 0o644
                    info.mtime = 0
                    tar.addfile(info, io.BytesIO(data))
    digest = hashlib.sha256(archive.read_bytes()).hexdigest()
    with (args.output/'SHA256SUMS').open('x') as stream:
        stream.write(digest+'  '+archive.name+'\n')
    print(json.dumps(source, indent=2))


if __name__ == '__main__':
    main()
