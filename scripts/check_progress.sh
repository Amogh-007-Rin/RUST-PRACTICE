#!/usr/bin/env bash
set -euo pipefail

# check_progress.sh
# Parses the curriculum-map checkboxes in the root README.md and reports
# completion. Use it by ticking checkboxes in README.md as you finish modules
# and capstones.

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
README="$ROOT/README.md"

if [ ! -f "$README" ]; then
  echo "error: $README not found" >&2
  exit 1
fi

MOD_TOTAL="$(grep -c '^- \[[ x]\] \[Module ' "$README" || true)"
MOD_DONE="$(grep -c '^- \[x\] \[Module ' "$README" || true)"
CAP_TOTAL="$(grep -c '^- \[[ x]\] \[Capstone ' "$README" || true)"
CAP_DONE="$(grep -c '^- \[x\] \[Capstone ' "$README" || true)"

echo "${MOD_DONE}/${MOD_TOTAL} modules complete, ${CAP_DONE}/${CAP_TOTAL} capstones complete."

if [ "$MOD_DONE" -eq "$MOD_TOTAL" ] && [ "$CAP_DONE" -eq "$CAP_TOTAL" ]; then
  echo "All done. You're job-ready — go get it."
fi
