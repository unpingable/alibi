#!/usr/bin/env python3
"""Run deterministic characterization with an unchanged library; no host faults."""
import hashlib, json, os, pathlib, shutil, subprocess, sys
repo = pathlib.Path(__file__).resolve().parents[3]
out = pathlib.Path(sys.argv[1]).resolve()
out.mkdir(parents=True, exist_ok=False)
source = (repo / 'src/collect/mod.rs').read_text()
start = source.index('pub fn collect_facts_for(')
end = source.index('\nfn not_requested', start)
extracted = source[start:end]
(out / 'ordered.rs').write_text(extracted)
shutil.copyfile(pathlib.Path(__file__).with_name('driver.rs'), out / 'driver.rs')
env = dict(os.environ, TEMPORAL_RESULTS=str(out))
target = pathlib.Path(env['CARGO_TARGET_DIR'])
subprocess.run(['cargo', 'build', '--lib', '--locked'], cwd=repo, env=env, check=True)
subprocess.run(['rustc', '--edition=2021', str(out / 'driver.rs'), '--extern', f'alibi={target}/debug/libalibi.rlib', '-L', f'dependency={target}/debug/deps', '-o', str(out / 'driver')], check=True)
with (out / 'stdout.txt').open('w') as f:
    subprocess.run([str(out / 'driver')], env=env, stdout=f, stderr=subprocess.STDOUT, check=True)
manifest = {'source': subprocess.check_output(['git','rev-parse','HEAD'],cwd=repo,text=True).strip(), 'product_base':'5aef9cd85486cb15b7d044ab44acc08c9aebeef2','scope':'deterministic substitutes and real evaluators, not real kernel behavior','files':{p.name:hashlib.sha256(p.read_bytes()).hexdigest() for p in out.iterdir() if p.is_file()}}
(out / 'MANIFEST.json').write_text(json.dumps(manifest,indent=2)+'\n')
