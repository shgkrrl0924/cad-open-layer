#!/bin/bash
# Verify all files in tests/corpus/ are registered in legal-package/06-test-corpus-license-log.md
# Runs in CI on every PR.

set -euo pipefail

CORPUS_LOG="legal-package/06-test-corpus-license-log.md"
CORPUS_DIR="tests/corpus"

if [ ! -f "$CORPUS_LOG" ]; then
  echo "ERROR: $CORPUS_LOG does not exist."
  exit 1
fi

if [ ! -d "$CORPUS_DIR" ]; then
  echo "==> No corpus directory yet, skipping check."
  exit 0
fi

VIOLATIONS=0

# Find all files (excluding NDA / private / encrypted directories that are gitignored)
while IFS= read -r filepath; do
  filename=$(basename "$filepath")

  if ! grep -q "$filename" "$CORPUS_LOG"; then
    echo "ERROR: $filepath not registered in $CORPUS_LOG"
    VIOLATIONS=$((VIOLATIONS + 1))
  fi
done < <(find "$CORPUS_DIR" -type f \
  -not -path "*/maket-real/*" \
  -not -path "*/encrypted/*" \
  -not -path "*/private/*" \
  -not -path "*/.git/*" \
  -not -name ".gitkeep" \
  -not -name "README*")

if [ "$VIOLATIONS" -gt 0 ]; then
  echo ""
  echo "==> Corpus log consistency: FAIL ($VIOLATIONS unregistered)"
  echo "    Add corpus entries to $CORPUS_LOG before merging."
  exit 1
fi

echo "==> Corpus log consistency: PASS"
exit 0
