#!/usr/bin/env python3
"""Descriptive measurements from sealed guest batches; no semantic PASS scoring."""
import argparse
import hashlib
import json
from pathlib import Path
import re
import statistics


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def distribution(values):
    if not values:
        return {'n': 0}
    values = sorted(values)
    return dict(n=len(values), minimum=values[0], median=statistics.median(values),
                p95=values[max(0, (95 * len(values) + 99) // 100 - 1)], maximum=values[-1])


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('batches', nargs='+', type=Path)
    parser.add_argument('--output', required=True, type=Path)
    parser.add_argument('--include-invocation', action='append', default=[], type=Path,
                        help='Independently confirmed wrapped human invocation capture directory')
    args = parser.parse_args()
    result = dict(schema_version=1, scope='descriptive execution measurements only',
                  caveat='Boots and CLI invocations are not independent qualification cases; oracle eligibility is separate.',
                  batches=[], guests=[], cli=[], groups=[],
                  explicitly_included_invocations=[str(p.resolve()) for p in args.include_invocation])
    included = {p.resolve() for p in args.include_invocation}
    observed_included = set()
    seen = set()
    for batch in args.batches:
        sealed = batch / 'COMPLETE.json'
        complete = json.loads(sealed.read_text())
        result['batches'].append(dict(path=str(batch), complete_sha256=sha(sealed)))
        for row in complete['records']:
            terminal_path = Path(row['terminal'])
            if sha(terminal_path) != row['terminal_sha256']:
                raise ValueError('terminal seal mismatch: ' + str(terminal_path))
            terminal = json.loads(terminal_path.read_text())
            if terminal['run_id'] in seen:
                raise ValueError('duplicate guest run identity')
            seen.add(terminal['run_id'])
            start_path = terminal_path.parent / 'START.json'
            if sha(start_path) != terminal['start_sha256']:
                raise ValueError('start identity mismatch')
            start = json.loads(start_path.read_text())
            identity = dict(run_id=terminal['run_id'], source_revision=start['source_revision'],
                            kernel_sha256=start['kernel_sha256'], case_id=start['case_id'],
                            directory=str(terminal_path.parent))
            result['guests'].append(dict(**identity, elapsed_seconds=terminal['elapsed_seconds'],
                execution_disposition=terminal['execution_disposition'], scenario_exit_status=terminal['scenario_exit_status'],
                transport_errors=terminal['capture_errors'], qemu_maxrss_kib=terminal['child_maxrss_kib']))
            for metadata_path in sorted((terminal_path.parent / 'captures').glob('*/metadata')):
                directory = metadata_path.parent
                argv = (directory / 'argv').read_bytes().split(b'\0')
                try:
                    parsed = json.loads((directory / 'stdout').read_text())
                except (ValueError, UnicodeError):
                    parsed = None
                is_receipt = isinstance(parsed, dict) and parsed.get('tool') == 'alibi' and parsed.get('receipt_version') == 2
                direct = any(Path(a.decode(errors='replace')).name == 'alibi' for a in argv if a)
                explicit = directory.resolve() in included
                if explicit:
                    observed_included.add(directory.resolve())
                if not direct and not is_receipt and not explicit:
                    continue
                metadata = json.loads(metadata_path.read_text())
                measurement = dict(**identity, invocation=directory.name,
                    metadata_sha256=sha(metadata_path), exit_status=metadata['exit_status'],
                    stream_sha256={name: sha(directory / name) for name in ['argv', 'stdout', 'stderr', 'resource']},
                    wall_ms=round(1000 * (metadata['ended_uptime_seconds'] - metadata['started_uptime_seconds']), 3))
                try:
                    receipt = parsed
                    if not is_receipt:
                        raise ValueError('not an Alibi V2 receipt')
                    measurement.update(receipt_version=receipt.get('receipt_version'),
                        interrogation_ms=receipt.get('interrogation_complete_ms'),
                        emitted_verdict=receipt.get('verdict'),
                        findings=[dict(accusation=f['accusation'], verdict=f['verdict']) for f in receipt.get('findings', [])])
                except (ValueError, UnicodeError):
                    measurement['unparsed_output'] = True
                resource = (directory / 'resource').read_text(errors='replace')
                for label, key in [('Maximum resident set size (kbytes)', 'maxrss_kib'),
                                   ('User time (seconds)', 'user_seconds'), ('System time (seconds)', 'system_seconds')]:
                    match = re.search(re.escape(label) + r':\s*([0-9.]+)', resource)
                    if match:
                        measurement[key] = float(match[1])
                result['cli'].append(measurement)
    if observed_included != included:
        raise ValueError('explicit invocation absent from verified guest captures')
    for source, kernel in sorted({(x['source_revision'], x['kernel_sha256']) for x in result['cli']}):
        rows = [x for x in result['cli'] if (x['source_revision'], x['kernel_sha256']) == (source, kernel)]
        result['groups'].append(dict(source_revision=source, kernel_sha256=kernel, invocations=len(rows),
            wall_ms=distribution([x['wall_ms'] for x in rows]),
            interrogation_ms=distribution([x['interrogation_ms'] for x in rows if x.get('interrogation_ms') is not None]),
            maxrss_kib=distribution([x['maxrss_kib'] for x in rows if 'maxrss_kib' in x]),
            unparsed_outputs=sum(bool(x.get('unparsed_output')) for x in rows),
            exits={str(code): sum(x['exit_status'] == code for x in rows) for code in sorted({x['exit_status'] for x in rows})}))
    with args.output.open('x') as stream:
        json.dump(result, stream, indent=2)
        stream.write('\n')
    print(json.dumps(dict(guests=len(result['guests']), cli_invocations=len(result['cli']), output=str(args.output))))


if __name__ == '__main__':
    main()
