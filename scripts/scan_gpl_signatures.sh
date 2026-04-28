#!/bin/bash
# GPL signature scanner — detects patterns suggesting GPL code contamination.
# Runs in CI on every PR.
#
# 사용법:
#   bash scripts/scan_gpl_signatures.sh
#
# Exit code:
#   0 - clean
#   1 - violations found, commit override 필요
#
# Override:
#   commit 메시지에 "GPL-OVERRIDE: <reason>" 라인 포함 시 통과 (maintainer review 후)

set -euo pipefail

# 검사 대상 (Rust 소스만)
SEARCH_PATHS=("crates/" "tests/" "fuzz/")

# 1. LibreDWG 알려진 함수명 prefix 패턴
LIBREDWG_FN_PATTERNS=(
  "fn dwg_decode_"
  "fn dwg_encode_"
  "fn dwg_section_"
  "fn dwg_object_"
  "fn dwg_handle_"
  "fn bit_read_"
  "fn bit_write_"
  "fn dxf_decode_"
  "fn dxf_encode_"
  "fn decode_R2000"
  "fn decode_R2004"
  "fn decode_R2007"
  "fn decode_R2010"
  "fn encode_R2000"
)

# 2. LibreDWG 자주 등장하는 주석/문자열 패턴
LIBREDWG_COMMENT_PATTERNS=(
  "LibreDWG"
  "GNU General Public License"
  "Free Software Foundation"
  "GPL[0-9]"
  "this is part of LibreDWG"
)

# 3. 다른 GPL CAD 라이브러리 패턴 (필요 시 확장)
OTHER_GPL_PATTERNS=(
  "from gerbv"      # PCB 처리, GPL
  "from libdxf"     # LGPL — 별도 검토
)

VIOLATIONS=0
FINDINGS=()

# Function: search a pattern across SEARCH_PATHS
scan_pattern() {
  local pattern="$1"
  local description="$2"

  for path in "${SEARCH_PATHS[@]}"; do
    [ -d "$path" ] || continue
    if grep -rln -E "$pattern" "$path" 2>/dev/null; then
      FINDINGS+=("Pattern '$description' found")
      VIOLATIONS=$((VIOLATIONS + 1))
    fi
  done
}

echo "==> Scanning for LibreDWG function name patterns"
for pattern in "${LIBREDWG_FN_PATTERNS[@]}"; do
  scan_pattern "$pattern" "$pattern"
done

echo "==> Scanning for GPL-related comments / strings"
for pattern in "${LIBREDWG_COMMENT_PATTERNS[@]}"; do
  scan_pattern "$pattern" "$pattern"
done

echo "==> Scanning for other GPL library traces"
for pattern in "${OTHER_GPL_PATTERNS[@]}"; do
  scan_pattern "$pattern" "$pattern"
done

# Check for GPL-OVERRIDE in last commit message
if [ "$VIOLATIONS" -gt 0 ]; then
  if [ -n "${GITHUB_HEAD_REF:-}" ] || [ -n "${GITHUB_SHA:-}" ]; then
    LAST_MSG=$(git log -1 --format=%B || echo "")
    if echo "$LAST_MSG" | grep -q "GPL-OVERRIDE:"; then
      OVERRIDE_REASON=$(echo "$LAST_MSG" | grep "GPL-OVERRIDE:" | sed 's/.*GPL-OVERRIDE: *//')
      echo ""
      echo "==> GPL-OVERRIDE found in commit message:"
      echo "    $OVERRIDE_REASON"
      echo ""
      echo "==> Findings (suppressed by override, but logged for audit):"
      printf '    - %s\n' "${FINDINGS[@]}"
      echo ""
      echo "==> Override accepted. Maintainer must review."
      exit 0
    fi
  fi

  echo ""
  echo "==> ERROR: GPL contamination scan failed."
  echo "    $VIOLATIONS finding(s):"
  printf '    - %s\n' "${FINDINGS[@]}"
  echo ""
  echo "    If a finding is a false positive (e.g. legitimate use of a"
  echo "    standard DWG specification term), document the override in"
  echo "    your commit message:"
  echo ""
  echo "      GPL-OVERRIDE: <one-line reason>"
  echo ""
  echo "    All overrides are logged for audit."
  exit 1
fi

echo ""
echo "==> GPL contamination scan: PASS"
exit 0
