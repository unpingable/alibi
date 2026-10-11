#!/usr/bin/env python3
"""Decode hash-checked guest frames; corruption/missing fields remain F7 evidence.

Serial capture is transport, never the verdict oracle. No receipt interpretation.
"""
import argparse
import base64
import hashlib
import json
from pathlib import Path
import re

KINDS={'argv','stdout','stderr','resource','metadata'}
BEGIN=re.compile(r'^ALIBI_BLOB_BEGIN ([A-Za-z0-9_-]+) ([A-Za-z0-9_-]+) ([a-z]+) ([0-9]+) ([0-9a-f]{64})$')

def parse(data, run_id):
    records={}; errors=[]; active=None; body=[]
    for line in data.splitlines():
        match=BEGIN.fullmatch(line)
        if match:
            if active is not None:
                errors.append('unterminated frame: '+active[1]+'/'+active[2])
            active=list(match.groups()); body=[]
            continue
        if active is None: continue
        marker='ALIBI_BLOB_END '+' '.join(active[:3])
        if line==marker:
            run,case,kind,size,sha=active
            if run != run_id:
                errors.append('unexpected run identity: '+run)
            elif kind not in KINDS:
                errors.append('unexpected frame kind: '+kind)
            else:
                try:
                    value=base64.b64decode(''.join(body),validate=True)
                    if len(value)!=int(size) or hashlib.sha256(value).hexdigest()!=sha:
                        raise ValueError('size or SHA-256 mismatch')
                    if (case,kind) in records: raise ValueError('duplicate frame')
                    if kind=='metadata':
                        meta=json.loads(value)
                        if meta.get('case_id')!=case or not isinstance(meta.get('exit_status'),int):
                            raise ValueError('metadata case/status mismatch')
                    records[(case,kind)]=value
                except (ValueError,UnicodeError) as exc:
                    errors.append(case+'/'+kind+': '+str(exc))
            active=None; body=[]
        else: body.append(line)
    if active is not None: errors.append('unterminated frame: '+active[1]+'/'+active[2])
    cases=sorted({case for case,kind in records})
    if not cases: errors.append('no complete capture frames')
    for case in cases:
        missing=KINDS-{kind for candidate,kind in records if candidate==case}
        if missing: errors.append(case+': missing '+','.join(sorted(missing)))
    return records,errors

def extract(serial,run_id,destination):
    records,errors=parse(Path(serial).read_text(errors='replace'),run_id)
    destination=Path(destination); destination.mkdir(exist_ok=False)
    manifest={'run_id':run_id,'errors':errors,'captures':[],'interpretation':'Transport validation only; no oracle acceptance'}
    for (case,kind),value in sorted(records.items()):
        directory=destination/case; directory.mkdir(exist_ok=True)
        path=directory/kind
        with path.open('xb') as f:f.write(value)
        manifest['captures'].append({'case_id':case,'kind':kind,'path':str(path),
                                    'bytes':len(value),'sha256':hashlib.sha256(value).hexdigest()})
    (destination/'CAPTURES.json').write_text(json.dumps(manifest,indent=2)+'\n')
    return manifest

if __name__=='__main__':
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--serial',type=Path,required=True)
    parser.add_argument('--run-id',required=True)
    parser.add_argument('--output-dir',type=Path,required=True)
    args=parser.parse_args()
    result=extract(args.serial,args.run_id,args.output_dir)
    print(json.dumps(result))
    raise SystemExit(bool(result['errors']))
