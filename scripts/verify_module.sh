#!/usr/bin/env bash
set -euo pipefail

# verify_module.sh <number>
# Runs cargo fmt --check, cargo clippy -D warnings, and cargo test scoped to a
# single module's two crates (module-XXX-exercises and module-XXX-solutions).
# Example: ./scripts/verify_module.sh 042

if [ "$#" -ne 1 ]; then
  echo "usage: $0 <number>" >&2
  echo "example: $0 042" >&2
  exit 1
fi

NUM="$1"
case "$NUM" in
  ''|*[!0-9]*) echo "error: <number> must be a non-negative integer, got '$NUM'" >&2; exit 1 ;;
esac

NUM3="$(printf '%03d' "$((10#$NUM))")"
EXERCISES="module-${NUM3}-exercises"
SOLUTIONS="module-${NUM3}-solutions"

# If the module has no crates (worksheet-style modules), just report it.
if ! cargo metadata --no-deps --format-version 1 2>/dev/null | grep -q "\"$EXERCISES\""; then
  echo "module-${NUM3}: no crate packages found (worksheet-style module?) — nothing to verify."
  exit 0
fi

echo "==> cargo fmt --check (${EXERCISES}, ${SOLUTIONS})"
cargo fmt --check -p "$EXERCISES" -p "$SOLUTIONS"

echo "==> cargo clippy -- -D warnings (${EXERCISES}, ${SOLUTIONS})"
cargo clippy -p "$EXERCISES" -p "$SOLUTIONS" -- -D warnings

echo "==> cargo test (${EXERCISES})"
cargo test -p "$EXERCISES"

echo "==> cargo test (${SOLUTIONS})"
cargo test -p "$SOLUTIONS"

echo "module-${NUM3}: all checks passed."
