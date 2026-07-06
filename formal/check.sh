#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
MODEL="$ROOT_DIR/formal/z3/tlm-field-group-contract.smt2"

if ! command -v z3 >/dev/null 2>&1; then
  echo "z3 is required for formal checks" >&2
  exit 1
fi

actual="$(z3 "$MODEL" | grep -E '^(sat|unsat|unknown)$' | tr '\n' ' ')"
expected="sat sat sat sat unsat unsat unsat "

if [[ "$actual" != "$expected" ]]; then
  echo "unexpected z3 result sequence" >&2
  echo "expected: $expected" >&2
  echo "actual:   $actual" >&2
  exit 1
fi

echo "[ok] z3 formal/z3/tlm-field-group-contract.smt2: $actual"
