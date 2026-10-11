#!/usr/bin/env python3
"""Execute predeclared command-substitution cases; never guest qualification."""
import argparse
import datetime
import hashlib
import json
import os
from pathlib import Path
import signal
import subprocess
import time


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def execute(argv, env, prefix, limit=7):
    start = time.monotonic()
    record = {"argv": list(map(str, argv)), "outer_timeout": False}
    try:
        child = subprocess.Popen(argv, env=env, stdout=subprocess.PIPE,
                                 stderr=subprocess.PIPE, start_new_session=True)
        record["pid"] = child.pid
        try:
            stdout, stderr = child.communicate(timeout=limit)
        except subprocess.TimeoutExpired:
            record["outer_timeout"] = True
            os.killpg(child.pid, signal.SIGKILL)
            stdout, stderr = child.communicate(timeout=2)
        record["exit_status"] = child.returncode
        # EOF and parent exit do not by themselves prove no remaining descendants.
        try:
            os.killpg(child.pid, 0)
            record["process_group_remaining"] = True
        except ProcessLookupError:
            record["process_group_remaining"] = False
    except OSError as error:
        stdout, stderr = b"", str(error).encode()
        record.update(exit_status=None, exec_errno=error.errno)
    record["elapsed_seconds"] = time.monotonic() - start
    for suffix, content in [("stdout", stdout), ("stderr", stderr)]:
        path = Path(str(prefix) + "." + suffix)
        path.write_bytes(content)
        record[suffix + "_sha256"] = digest(path)
    return record


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("binary", type=Path)
    parser.add_argument("source_revision")
    parser.add_argument("spec", type=Path)
    parser.add_argument("output", type=Path)
    args = parser.parse_args()
    args.output.mkdir(exist_ok=False)
    spec = json.loads(args.spec.read_text())
    receipt = dict(schema_version=1, source_revision=args.source_revision,
                   binary=str(args.binary.resolve()), binary_sha256=digest(args.binary),
                   spec_sha256=digest(args.spec), runner_sha256=digest(Path(__file__)),
                   started_at=datetime.datetime.now(datetime.timezone.utc).isoformat(),
                   invocation_id=os.environ.get("INVOCATION_ID"),
                   owner="root", classification="local CLI substitution, not guest fault trial",
                   records=[], retirement="Retain minimal oracle/output evidence through review 2026-10-18")
    (args.output / "START.json").write_text(json.dumps(receipt, indent=2) + "\n")
    for case in spec["cases"]:
        out = args.output / case["id"]
        out.mkdir()
        commands = out / "commands"
        commands.mkdir()
        fixtures = []
        for name, fixture in case["commands"].items():
            path = commands / name
            path.write_text(fixture["content"])
            path.chmod(int(fixture["mode"], 8))
            fixtures.append(dict(name=name, sha256=digest(path), mode=fixture["mode"]))
        env = dict(os.environ, PATH=str(commands.resolve()))
        probes = {}
        for name in ("dmesg", "journalctl"):
            probes[name] = execute([str((commands / name).resolve())], env, out / (name + "-oracle"))
        result = execute([str(args.binary.resolve()), "check", "--json", "kernel-scream"], env, out / "alibi")
        try:
            finding = json.loads((out / "alibi.stdout").read_text())
            result["verdict"] = finding["findings"][0]["verdict"]
            result["allowed_verdict"] = result["verdict"] in case["allowable_verdicts"]
            result["receipt_version"] = finding["receipt_version"]
            result["innocence_established"] = finding["innocence_established"]
        except (ValueError, KeyError, IndexError) as error:
            result["receipt_error"] = str(error)
        # Files are scoped to this run; fixture bytes/modes remain in hashed spec.
        for name in case["commands"]:
            (commands / name).unlink()
        commands.rmdir()
        receipt["records"].append(dict(case=case["id"], fixtures=fixtures,
                                       oracle_probes=probes, result=result))
        (args.output / "PROGRESS.json").write_text(json.dumps(receipt, indent=2) + "\n")
    receipt["finished_at"] = datetime.datetime.now(datetime.timezone.utc).isoformat()
    with (args.output / "COMPLETE.json").open("x") as output:
        json.dump(receipt, output, indent=2)
        output.write("\n")
    print(json.dumps({"output": str(args.output), "cases": len(receipt["records"])}))


if __name__ == "__main__":
    main()
