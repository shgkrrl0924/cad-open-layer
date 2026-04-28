# Contributor Declaration

**Project:** CAD Open Layer
**Effective:** 모든 contributor 가 첫 commit 전 본 양식 서명 후 `legal-package/contributors/{github-username}.md` 로 commit.

---

## 양식 (template)

아래 내용을 복사하여 새 파일 `legal-package/contributors/{your-github-username}.md` 로 저장. 모든 항목 채우고 마지막에 서명/일자/해시 기록.

```markdown
# Contributor Declaration — {Full Name}

**GitHub username:** @{your-github-username}
**Email:** {your-email}
**Date of declaration:** YYYY-MM-DD
**Project version at time of declaration:** {git rev-parse --short HEAD}

## 본인 진술

본인은 CAD Open Layer 프로젝트에 기여하기 전 다음을 진술한다.

### 1. Identity

본인의 이름과 GitHub 계정은 위에 명시된 바와 같다. 본 진술서는 본인이 직접 작성·서명하였다.

### 2. License agreement

본인은 본 프로젝트가 Apache License 2.0 으로 배포됨을 이해하고, 본인의 모든 기여(commit, PR, comment, documentation, test corpus, design suggestion 등)가 동일 라이선스로 제공됨에 동의한다.

본인의 기여는 본인의 원작이거나, 본인이 합법적으로 사용 권한을 가진 자료에 근거한다.

### 3. No GPL contamination

본인은 다음을 진술한다.

- [ ] 본인은 LibreDWG (https://www.gnu.org/software/libredwg/) 의 소스코드를 본 적 없다.
  - 만약 과거에 본 적이 있다면 다음 필드를 채운다:
    - 마지막 접근일자: ____________
    - 본 영역: ____________
    - 12개월 cooldown 종료일: ____________
- [ ] 본인은 GPLv2 / GPLv3 / AGPL 라이선스의 DWG/DXF/CAD 처리 코드를 본 프로젝트에 포함하지 않을 것을 약속한다.
- [ ] 본인은 GPL 코드를 번역하거나 그 알고리즘을 그대로 옮기지 않을 것을 약속한다.

### 4. No proprietary SDK contamination

본인은 다음을 진술한다.

- [ ] 본인은 Autodesk RealDWG SDK 의 소스코드/헤더/문서(공개 부분 외)를 사용하거나 본 프로젝트에 포함시키지 않는다.
- [ ] 본인은 ODA Teigha SDK / ODA Drawings SDK 의 소스코드 또는 NDA 자료에 접근한 적 없다.
  - 만약 ODA 멤버십 가입 또는 SDK 접근 이력이 있다면 다음 필드를 채운다:
    - ODA 멤버십 시점: ____________
    - 접근 자료: ____________
- [ ] 본인은 Bentley MicroStation SDK 등 다른 상용 CAD vendor 의 비공개 자료를 본 프로젝트에 포함하지 않는다.
- [ ] 본인은 NDA 하에 받은 자료를 본 프로젝트에 포함하지 않는다.

### 5. Independent work

본인은 다음을 진술한다.

- [ ] 본인이 본 프로젝트에 기여한 모든 코드는 본인의 독립 작업이거나, `02-reference-material-log.md` 의 whitelist 자료에 근거한다.
- [ ] 본인이 사용하는 AI 코딩 어시스턴트(Copilot, Codex, Claude Code 등)의 결과물은 본인이 직접 검증·수정 후 commit 한다.
- [ ] AI 어시스턴트가 LibreDWG 등 GPL 코드 패턴을 제안하면 본인이 그것을 인지하고 폐기한다.

### 6. Trademark compliance

본인은 다음을 약속한다.

- [ ] "DWG", "AutoCAD", "Autodesk" 를 본 프로젝트의 제품명/도메인/브랜드 요소로 사용하지 않는다.
- [ ] 위 명칭들을 사용할 경우 사실 진술 ("works with DWG files", "compatible with the DWG file format" 등) 또는 출처 표시(trademark notice) 형태로만 사용한다.
- [ ] `05-trademark-usage-guideline.md` 의 가이드라인을 준수한다.

### 7. Test corpus

본인은 본 프로젝트에 추가하는 모든 테스트 DXF/DWG 파일에 대해:

- [ ] 자체 합성한 파일이거나, 명확한 라이선스로 사용 가능한 파일임을 확인한다.
- [ ] 라이선스 정보를 `06-test-corpus-license-log.md` 에 기록한다.
- [ ] LibreDWG 의 test corpus 등 GPL 라이선스 테스트 파일은 사용하지 않는다.

### 8. Reporting obligation

본인은 다음을 약속한다.

- [ ] 본 프로젝트에서 GPL contamination 또는 IP 위반을 발견하면 즉시 Founder (hakki) 또는 본 프로젝트 변호사에게 보고한다.
- [ ] 본인이 작성한 코드에 위반 가능성이 발견되면 자기 신고한다.
- [ ] 본 진술서 내용 중 사실관계가 변경되면 (예: ODA 멤버십 가입 등) 즉시 본 진술서 갱신한다.

### 9. Renewal

본 진술서는 작성일로부터 12개월간 유효하며, 매년 갱신한다. 갱신 시 동일 양식으로 새 버전 작성.

---

## 서명

상기 진술이 모두 사실임을 확인한다.

**Name:** ____________
**Date:** YYYY-MM-DD
**Signature (typed):** ____________
**Hash of signed declaration:** `{sha256sum of this file content above signature line}`

본 파일을 commit 한 후, commit hash 를 본 줄 아래에 기록:
**Commit hash:** ____________
```

---

## 절차

### Founder (hakki) 의 첫 declaration

본인이 첫 contributor. 본 양식을 채워서 `legal-package/contributors/hakki.md` 로 저장. 첫 commit.

### 향후 contributor 추가 시

1. 새 사람이 PR 송부 전 본 양식 작성
2. PR #1: declaration 추가 (반드시 별도 PR. 다른 코드 변경과 섞지 않음)
3. PR #1 머지 후에야 PR #2: 실제 코드 기여 가능
4. CI `scripts/verify_contributors.sh` 가 commit author 와 declarations/ 의 등록 사실 확인

### 진술 내용 변경 시

- 사실 변경 (예: ODA 멤버십 가입) — 24시간 이내 declaration 갱신
- 갱신은 새 파일 (`hakki-v2.md`) 추가, 이전 버전은 그대로 보존 (이력 추적)

---

## CI 검증 스크립트 예 (`scripts/verify_contributors.sh`)

```bash
#!/bin/bash
# 모든 commit author 가 contributors/ 에 declaration 보유한지 확인

set -e

declare -A AUTHORS
while IFS= read -r author; do
  AUTHORS["$author"]=1
done < <(git log --format='%ae' main..HEAD | sort -u)

VIOLATIONS=0
for author_email in "${!AUTHORS[@]}"; do
  # email → github username 매핑 (gitconfig 또는 별도 매핑 파일)
  username=$(get_github_username "$author_email")
  
  if [ ! -f "legal-package/contributors/$username.md" ]; then
    echo "ERROR: Author $author_email (@$username) has no contributor declaration"
    VIOLATIONS=$((VIOLATIONS + 1))
  fi
done

if [ "$VIOLATIONS" -gt 0 ]; then
  echo "Add declarations to legal-package/contributors/ before merging."
  exit 1
fi

echo "All contributors verified."
```

---

## FAQ

**Q1. 외부 일회성 PR (typo 수정 등) 에도 declaration 필요한가?**
A. 단순 typo, docs 변경, configuration tweak 같은 trivial PR 은 declaration 면제 가능. 단 코드 변경 PR 은 size 무관 declaration 필수.

**Q2. AI 가 작성한 코드는 누구의 declaration 으로 cover 되나?**
A. AI 어시스턴트를 운영하고 결과물을 commit 한 사람. 그 사람의 declaration 으로 cover. AI 결과물 검증 책임도 그 사람.

**Q3. 회사 직원으로 기여하는 경우?**
A. 본인 individual declaration + 회사 측 CLA(Contributor License Agreement) 별도. 회사 CLA 양식은 OSPO 표준 (예: Google CLA, Apache ICLA) 활용.

**Q4. 과거에 LibreDWG 코드 본 적 있는데 12개월 안 됐어요. 기여 가능?**
A. Stage 1 DXF 영역엔 가능 (LibreDWG 와 직접 겹치는 영역 아님). 단 Stage 2 DWG 영역엔 12개월 cooldown 후. 변호사 검토 후 케이스별 결정.

**Q5. Declaration 한 사람이 나중에 LibreDWG 봤어요.**
A. 자기 신고. 그 시점 이후 작성한 코드는 review 대상. 이후 12개월 cooldown 후 DWG 영역 재진입 가능.
