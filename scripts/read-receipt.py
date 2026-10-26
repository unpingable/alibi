#!/usr/bin/env python3
"""Bounded Receipt V2 validator/digest; explanation strings are never parsed."""
import argparse
import json
import sys

ACCUSATIONS = {"kernel-scream", "silent-killer", "memrot", "network-choke-point"}
SOURCE_NAMES = {"kernel-log", "mounts", "fs-usage", "edac", "net", "routes", "conntrack"}
VERDICTS = {"CLEAR", "DEGRADED", "UNKNOWN", "NOT_APPLICABLE"}


def require(condition, message):
    if not condition:
        raise ValueError(message)


def enum(value, choices, label):
    require(type(value) is str and value in choices, f"invalid {label}")
    return value


def records(value, label):
    require(type(value) is list, f"{label} must be an array")
    require(all(type(item) is dict for item in value), f"invalid {label} record")
    return value


def unique(items, key, label):
    names = [item[key] for item in items]
    require(all(type(name) is str for name in names), f"invalid {label} name")
    require(len(set(names)) == len(names), f"duplicate {label}")


def digest(receipt, expected, process_exit=None):
    require(type(receipt) is dict, "receipt must be an object")
    require(receipt.get("tool") == "alibi" and type(receipt.get("receipt_version")) is int
            and receipt["receipt_version"] == 2, "expected an Alibi Receipt V2")
    require(type(receipt["host"]) is str and bool(receipt["host"]), "invalid host")
    require(type(expected) in (list, set, tuple) and bool(expected), "expected accusations required")
    require(all(type(name) is str and name in ACCUSATIONS for name in expected), "invalid expected accusation")
    require(len(set(expected)) == len(expected), "duplicate expected accusation")
    verdict = enum(receipt["verdict"], VERDICTS, "aggregate verdict")
    findings = records(receipt["findings"], "findings")
    unique(findings, "accusation", "accusation")
    require({f["accusation"] for f in findings} == set(expected), "selected accusations differ from request")
    require(type(receipt["envelope"]) is dict, "invalid envelope")
    source_rows = records(receipt["envelope"]["sources"], "sources")
    unique(source_rows, "name", "source")
    for source in source_rows:
        enum(source["name"], SOURCE_NAMES, "source name")
        enum(source["status"], {"ok", "unavailable", "unsupported", "malformed"}, "source status")
        enum(source["completeness"], {"complete", "partial", "none"}, "completeness")
        require(source["status"] == "ok" or source["completeness"] != "complete", "failed source claims completeness")
    sources = {s["name"]: s for s in source_rows}
    result = []
    for finding in findings:
        fv = enum(finding["verdict"], VERDICTS, "finding verdict")
        for field in ("indications", "evidence", "skipped"):
            require(type(finding[field]) is list and all(type(x) is str for x in finding[field]), f"invalid {field}")
        require((fv == "DEGRADED") == bool(finding["indications"]), "indications contradict verdict")
        roles = records(finding["sources"], "finding sources")
        require(bool(roles), "empty finding sources")
        unique(roles, "name", "finding source")
        inadequate = []
        required = 0
        for role in roles:
            enum(role["name"], SOURCE_NAMES, "finding source name")
            enum(role["role"], {"required", "supporting"}, "source role")
            if role["role"] == "required":
                required += 1
                source = sources.get(role["name"])
                if source is None or source["status"] != "ok" or source["completeness"] != "complete":
                    inadequate.append(role["name"])
        require(required > 0, "no required source")
        require(fv not in {"CLEAR", "NOT_APPLICABLE"} or not inadequate, "exoneration/applicability lacks required acquisition")
        result.append({"accusation": finding["accusation"], "verdict": fv,
                       "inadequate_required_sources": inadequate})
    values = {f["verdict"] for f in result}
    aggregate = next(v for v in ("DEGRADED", "UNKNOWN", "CLEAR", "NOT_APPLICABLE") if v in values)
    require(verdict == aggregate, "aggregate verdict contradicts findings")
    innocence = aggregate in {"CLEAR", "NOT_APPLICABLE"}
    require(type(receipt["innocence_established"]) is bool and receipt["innocence_established"] == innocence,
            "innocence flag contradicts verdict")
    mtti = receipt["mtti_ms"]
    require((type(mtti) is int and mtti >= 0) if innocence else mtti is None, "MTTI contradicts verdict")
    if process_exit is not None:
        require(type(process_exit) is int and process_exit == {"CLEAR": 0, "NOT_APPLICABLE": 0, "DEGRADED": 1, "UNKNOWN": 2}[aggregate],
                "process exit contradicts receipt")
    return {"host": receipt["host"], "verdict": verdict,
            "selected_findings_only": True, "findings": result}


def no_duplicate_keys(pairs):
    result = {}
    for key, value in pairs:
        require(key not in result, "duplicate JSON object key")
        result[key] = value
    return result


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--expected", required=True, help="comma-separated requested accusation names")
    parser.add_argument("--exit-status", type=int, help="original Alibi process exit status")
    args = parser.parse_args()
    try:
        receipt = json.load(sys.stdin, object_pairs_hook=no_duplicate_keys)
        print(json.dumps(digest(receipt, args.expected.split(","), args.exit_status), indent=2))
    except (ValueError, KeyError, TypeError, RecursionError) as error:
        print(f"unusable receipt: {error}", file=sys.stderr)
        sys.exit(2)
