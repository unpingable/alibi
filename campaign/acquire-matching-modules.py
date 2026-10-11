#!/usr/bin/env python3
"""Campaign-only bounded exact-package module acquisition; no installation/load."""
import hashlib,json,os,subprocess,sys,tarfile,time
from pathlib import Path

out=Path(sys.argv[1]).resolve(); spec=json.loads((out/'INPUT.json').read_text())
def sha(path):return hashlib.sha256(Path(path).read_bytes()).hexdigest()
def save(name,value):
    p=out/name
    with p.open('w') as f:json.dump(value,f,indent=2);f.write('\n');f.flush();os.fsync(f.fileno())
def phase(name,**kw):save('PROGRESS.json',dict(phase=name,at=time.time(),**kw))
modules=out/'modules'; modules.mkdir(exist_ok=False)
received={}; package_receipts=[]; wanted=set(spec['wanted']); builtin=set()
started=time.time()
try:
    save('EXECUTION.json',dict(pid=os.getpid(),proc_identity=Path('/proc/self/stat').read_text(),invocation_id=os.environ.get('INVOCATION_ID'),source_sha256=sha(__file__),started=started))
    for p in ('/','/data'):
        fs=os.statvfs(p)
        if fs.f_bavail*fs.f_frsize-268435456<64424509440:raise RuntimeError('host reserve gate: '+p)
    # Aggregate grant is recorded by root; independently refuse the stated4GiB cap.
    allocated=int(subprocess.check_output(['du','-s','-B1',str(out.parent)],text=True).split()[0])
    if allocated+268435456>4294967296:raise RuntimeError('campaign4GiB gate')
    for package in spec['packages']:
        if wanted<=set(received):break
        phase('DOWNLOAD',package=package['Package'])
        path=out/(package['Package']+'.deb')
        subprocess.run(['curl','--fail','--location','--proto','=https','--max-time','120','--retry','1','--output',str(path),'https://archive.ubuntu.com/ubuntu/'+package['Filename']],check=True)
        if path.stat().st_size!=int(package['Size']) or sha(path)!=package['SHA256']:raise RuntimeError('package integrity mismatch')
        package_receipts.append(dict(name=package['Package'],path=str(path),sha256=sha(path),version=package['Version'],bytes=path.stat().st_size))
        for scan in range(4):
            phase('SELECTIVE_EXTRACTION',package=package['Package'],scan=scan,wanted=sorted(wanted),received=sorted(received))
            producer=subprocess.Popen(['dpkg-deb','--fsys-tarfile',str(path)],stdout=subprocess.PIPE)
            assert producer.stdout
            with tarfile.open(fileobj=producer.stdout,mode='r|') as archive:
                for member in archive:
                    basename=Path(member.name).name
                    normalized=basename.removesuffix('.zst').removesuffix('.xz').removesuffix('.gz').removesuffix('.ko').replace('-','_')
                    is_metadata=basename in ('modules.builtin','config-'+spec['release'])
                    if not member.isfile() or not (is_metadata or (normalized in wanted and normalized not in received)):continue
                    if member.size>33554432:raise RuntimeError('unexpected member size')
                    source=archive.extractfile(member); assert source
                    data=source.read()
                    if is_metadata:
                        dest=out/basename
                        if not dest.exists():dest.write_bytes(data)
                        if basename=='modules.builtin':
                            builtin.update(Path(line).name.removesuffix('.ko').replace('-','_') for line in data.decode().splitlines())
                        continue
                    dest=modules/basename
                    if sum(v['bytes'] for v in received.values())+len(data)>67108864:raise RuntimeError('selected module budget')
                    with dest.open('xb') as f:f.write(data)
                    received[normalized]=dict(path=str(dest),archive_member=member.name,sha256=sha(dest),bytes=len(data),package=package['Package'],package_sha256=package['SHA256'])
            producer.stdout.close()
            if producer.wait()!=0:raise RuntimeError('dpkg extraction failed')
            before=set(wanted)
            for name,item in received.items():
                deps=subprocess.check_output(['modinfo','-F','depends',item['path']],text=True).strip()
                item['depends']=[d.replace('-','_') for d in deps.split(',') if d]
                item['vermagic']=subprocess.check_output(['modinfo','-F','vermagic',item['path']],text=True).strip()
                wanted.update(d for d in item['depends'] if d not in builtin)
            if wanted==before:break
    missing=wanted-set(received)-builtin
    if missing:raise RuntimeError('missing dependency modules: '+','.join(sorted(missing)))
    save('TERMINAL.json',dict(disposition='VERIFIED_SELECTIVE_MODULE_CUSTODY',elapsed_seconds=time.time()-started,packages=package_receipts,modules=received,builtins=sorted(builtin),input_sha256=sha(out/'INPUT.json'),owner='root campaign agent',retirement=spec['retirement'],next_action='Root builds successor immutable archive; real guest insertion must confirm symbol compatibility. No host module loaded.'))
    phase('TERMINAL',disposition='VERIFIED_SELECTIVE_MODULE_CUSTODY')
except Exception as exc:
    save('TERMINAL.json',dict(disposition='ACQUISITION_FAILED',error=repr(exc),elapsed_seconds=time.time()-started,packages=package_receipts,modules=received,next_action='Reconcile original acquisition; do not silently restart.'))
    raise
