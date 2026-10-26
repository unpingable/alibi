"""Smoke tests for the documented agent consumer: python3 tests/read_receipt_test.py."""
import copy
import importlib.util
import json
import sys
import subprocess
from pathlib import Path
import unittest

sys.dont_write_bytecode = True

ROOT = Path(__file__).resolve().parents[1]
SPEC = importlib.util.spec_from_file_location("consumer", ROOT / "scripts/read-receipt.py")
CONSUMER = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(CONSUMER)


class ReceiptConsumerTests(unittest.TestCase):
    def setUp(self):
        self.receipt = json.loads((ROOT / "tests/golden/receipt_v2.json").read_text())

    def test_mixed_verdicts_preserved(self):
        result = CONSUMER.digest(self.receipt, sorted(CONSUMER.ACCUSATIONS))
        self.assertEqual(result["verdict"], "DEGRADED")
        findings = {f["accusation"]: f for f in result["findings"]}
        self.assertEqual(findings["memrot"]["verdict"], "UNKNOWN")
        self.assertEqual(findings["memrot"]["inadequate_required_sources"], ["edac"])
        self.assertEqual(findings["network-choke-point"]["verdict"], "NOT_APPLICABLE")

    def test_missing_source_cannot_be_adequate(self):
        self.receipt["envelope"]["sources"] = []
        with self.assertRaises(ValueError):
            CONSUMER.digest(self.receipt, sorted(CONSUMER.ACCUSATIONS))

    def test_incomplete_success_is_not_adequate(self):
        self.receipt["envelope"]["sources"][0]["completeness"] = "partial"
        with self.assertRaises(ValueError):
            CONSUMER.digest(self.receipt, sorted(CONSUMER.ACCUSATIONS))

    def test_unusable_receipts_fail(self):
        for field, value in [("receipt_version", 999), ("tool", "other"), ("findings", [])]:
            receipt = copy.deepcopy(self.receipt)
            receipt[field] = value
            with self.assertRaises(ValueError):
                CONSUMER.digest(receipt, sorted(CONSUMER.ACCUSATIONS))

    def test_cli_rejects_unusable_and_deep_json_without_traceback(self):
        for raw in ["[]", '"x"', "[" * 2000 + "]" * 2000,
                    '{"tool":"alibi","tool":"other"}']:
            completed = subprocess.run([sys.executable, str(ROOT / "scripts/read-receipt.py"),
                "--expected", "kernel-scream"], input=raw, text=True,
                capture_output=True, timeout=5, check=False)
            self.assertEqual(completed.returncode, 2)
            self.assertNotIn("Traceback", completed.stderr)
            self.assertFalse(completed.stdout)

    def test_negative_controls(self):
        def set_field(path, value):
            r = copy.deepcopy(self.receipt)
            item = r
            for key in path[:-1]:
                item = item[key]
            item[path[-1]] = value
            return r
        cases = [[], None, 1,
            set_field(["receipt_version"], 2.0),
            set_field(["receipt_version"], True),
            set_field(["verdict"], "CLEAR"),
            set_field(["innocence_established"], True),
            set_field(["mtti_ms"], 0),
            set_field(["findings", 0, "verdict"], "HEALTHY"),
            set_field(["findings", 0, "sources"], []),
            set_field(["findings", 0, "sources", 0, "role"], "optional"),
            set_field(["findings", 0, "indications"], ["contradictory"]),
            set_field(["envelope", "sources", 0, "status"], "absent"),
            set_field(["envelope", "sources", 0, "completeness"], True),
            set_field(["findings"], self.receipt["findings"] * 2),
            set_field(["envelope", "sources"], self.receipt["envelope"]["sources"] * 2),
            set_field(["findings", 0, "sources"], self.receipt["findings"][0]["sources"] * 2),
        ]
        missing = copy.deepcopy(self.receipt)
        del missing["findings"][0]["evidence"]
        cases.append(missing)
        for r in cases:
            with self.subTest(receipt=r), self.assertRaises((ValueError, KeyError, TypeError)):
                CONSUMER.digest(r, sorted(CONSUMER.ACCUSATIONS))
        with self.assertRaises(ValueError):
            CONSUMER.digest(self.receipt, ["kernel-scream"])
        with self.assertRaises(ValueError):
            CONSUMER.digest(self.receipt, sorted(CONSUMER.ACCUSATIONS), 0)
        CONSUMER.digest(self.receipt, sorted(CONSUMER.ACCUSATIONS), 1)
        with self.assertRaises(ValueError):
            json.loads('{"tool":"alibi","tool":"other"}', object_pairs_hook=CONSUMER.no_duplicate_keys)


if __name__ == "__main__":
    unittest.main()
