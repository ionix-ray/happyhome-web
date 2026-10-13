#!/usr/bin/env bash
# Coverage gate — Rule 5 (TDD) companion.
#
# Asserts 100% line coverage on the SEO domain module (src/domain/seo.rs).
# Usage: ./tools/coverage_gate.sh
set -euo pipefail
cd "$(dirname "$0")/.."

TARGET="${1:-src/domain/seo.rs}"
OUT="$(mktemp -t happyhome-cov-XXXXXX.json)"
trap 'rm -f "$OUT"' EXIT

cargo llvm-cov --json --summary-only > "$OUT"

python3 - "$OUT" "$TARGET" <<'PY'
import json, sys

report_path, target = sys.argv[1], sys.argv[2]
data = json.load(open(report_path))
files = data["data"][0]["files"]

match = next((f for f in files if f["filename"].endswith(target)), None)
if match is None:
    print(f"FAIL: {target} not found in coverage report")
    sys.exit(1)

lines = match["summary"]["lines"]
funcs = match["summary"]["functions"]
regions = match["summary"]["regions"]

print(f"file      : {target}")
print(f"lines     : {lines['covered']}/{lines['count']} ({lines['percent']:.2f}%)")
print(f"functions : {funcs['covered']}/{funcs['count']} ({funcs['percent']:.2f}%)")
print(f"regions   : {regions['covered']}/{regions['count']} ({regions['percent']:.2f}%)")

if lines["percent"] < 100.0 or funcs["percent"] < 100.0:
    print(f"FAIL: {target} must be at 100% line+function coverage")
    sys.exit(1)
print(f"PASS: {target} at 100% line coverage")
PY
