#!/usr/bin/env bash

set -e

echo "Getting target list from rustc..."
TARGETS=$(rustc --print target-list)

echo ""
echo "Target Endianness:"
echo "-------------------"

for target in $TARGETS; do
  # Try to get target spec JSON (nightly required)
  endian=$(rustc +nightly -Z unstable-options --print target-spec-json --target "$target" 2>/dev/null)

  printf "%-40s %s\n" "$target" "$endian"
done
