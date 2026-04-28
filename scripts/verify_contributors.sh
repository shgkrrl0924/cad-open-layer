#!/bin/bash
# Verify all commit authors have a contributor declaration.
# Runs in CI on every PR.

set -euo pipefail

DECL_DIR="legal-package/contributors"
BASE_REF="${GITHUB_BASE_REF:-main}"

if [ ! -d "$DECL_DIR" ]; then
  echo "ERROR: $DECL_DIR does not exist. Create it before adding contributors."
  exit 1
fi

# 모든 unique author email in this PR's commits
EMAILS=$(git log --format='%ae' "origin/$BASE_REF..HEAD" 2>/dev/null | sort -u || git log --format='%ae' | sort -u)

if [ -z "$EMAILS" ]; then
  echo "==> No commits to verify."
  exit 0
fi

VIOLATIONS=0

while IFS= read -r email; do
  [ -z "$email" ] && continue

  # Heuristic: GitHub email format "{username}@users.noreply.github.com" → extract username
  # 또는 declaration 파일이 email 또는 username 으로 매칭되는지 확인

  # Check 1: 어떤 declaration 파일이 이 email 을 reference?
  if grep -lr -F "$email" "$DECL_DIR" >/dev/null 2>&1; then
    echo "==> $email: declaration found"
    continue
  fi

  # Check 2: GitHub noreply email pattern
  if [[ "$email" =~ ^([0-9]+\+)?(.+)@users\.noreply\.github\.com$ ]]; then
    username="${BASH_REMATCH[2]}"
    if [ -f "$DECL_DIR/$username.md" ]; then
      echo "==> $email (@$username): declaration found"
      continue
    fi
  fi

  echo "ERROR: Author $email has no contributor declaration in $DECL_DIR/"
  echo "       Submit a declaration PR first using legal-package/04-contributor-declaration-template.md"
  VIOLATIONS=$((VIOLATIONS + 1))
done <<< "$EMAILS"

if [ "$VIOLATIONS" -gt 0 ]; then
  echo ""
  echo "==> Contributor verification: FAIL ($VIOLATIONS missing)"
  exit 1
fi

echo ""
echo "==> Contributor verification: PASS"
exit 0
