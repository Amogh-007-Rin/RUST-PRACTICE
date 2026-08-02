#!/usr/bin/env bash
set -euo pipefail

# new_module.sh <number> <slug>
# Scaffolds modules/module-XXX-slug/ from the template in scripts/templates/.
# Example: ./scripts/new_module.sh 042 the-tokio-runtime

if [ "$#" -ne 2 ]; then
  echo "usage: $0 <number> <slug>" >&2
  echo "example: $0 042 the-tokio-runtime" >&2
  exit 1
fi

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
NUM="$1"
SLUG="$2"

case "$NUM" in
  ''|*[!0-9]*) echo "error: <number> must be a non-negative integer, got '$NUM'" >&2; exit 1 ;;
esac

NUM3="$(printf '%03d' "$((10#$NUM))")"
TARGET="$ROOT/modules/module-${NUM3}-${SLUG}"

if [ -e "$TARGET" ]; then
  echo "error: $TARGET already exists" >&2
  exit 1
fi

echo "scaffolding $TARGET"
cp -R "$ROOT/scripts/templates/module" "$TARGET"

# Fill in the number/slug in file names and content.
for f in "$TARGET"/exercises/Cargo.toml \
         "$TARGET"/solutions/Cargo.toml; do
  sed -i "s/__XXX__/$NUM3/g" "$f"
done

for f in "$TARGET"/exercises/tests/*.rs \
         "$TARGET"/solutions/tests/*.rs; do
  [ -e "$f" ] && sed -i "s/__XXX__/$NUM3/g" "$f"
done

# Test files import the crate by package name; fix the module number in them.
for f in "$TARGET"/exercises/tests/*.rs \
         "$TARGET"/solutions/tests/*.rs; do
  [ -e "$f" ] && sed -i "s/module_XXX_exercises/module_${NUM3}_exercises/g; s/module_XXX_solutions/module_${NUM3}_solutions/g" "$f"
done

# Rename test files so they carry the module number.
for d in exercises solutions; do
  if [ -f "$TARGET/$d/tests/module_XXX.rs" ]; then
    mv "$TARGET/$d/tests/module_XXX.rs" "$TARGET/$d/tests/module_${NUM3}.rs"
  fi
done

# README placeholders.
sed -i "s/__XXX__/$NUM3/g; s/__SLUG__/$SLUG/g" "$TARGET/README.md"

echo "done."
echo "next: edit $TARGET/README.md and fill in $TARGET/exercises/ (TODOs + tests) and $TARGET/solutions/"
