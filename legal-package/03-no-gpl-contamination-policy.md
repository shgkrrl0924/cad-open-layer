# No-GPL-Contamination Policy

**Project:** CAD Open Layer
**Version:** v1 (2026-04-25)
**Effective:** Stage 1 빌드 첫 commit 시점부터

---

## 1. 배경

CAD Open Layer 는 **Apache License 2.0** 으로 배포된다.

Apache Software Foundation 의 호환성 가이드에 따르면, **Apache-2.0 코드를 GPLv3 프로젝트에 포함하는 것은 가능하나 GPLv3 코드를 Apache-2.0 프로젝트에 포함할 수는 없다**. (https://www.apache.org/licenses/GPL-compatibility.html)

따라서 본 프로젝트는 GPLv3 / GPLv2 / AGPL 라이선스 코드를 **어떤 형태로도 포함, 링크, 번역, 모방할 수 없다.** 

특별히 **LibreDWG (GPLv3 or later)** 는 DWG 처리 분야에서 가장 많이 알려진 GPL 라이브러리이고 본 프로젝트의 도메인과 직접 겹치므로 가장 엄격히 차단한다.

---

## 2. 정의

### 2.1 "GPL 코드"
다음 라이선스 중 하나로 배포되는 모든 소스코드, 바이너리, 문서, 그리고 그것들의 일부:

- GNU General Public License v2 (GPLv2)
- GNU General Public License v3 (GPLv3)
- GNU Affero General Public License (AGPLv3)
- 위 라이선스들의 "or any later version" 변형

### 2.2 "Contamination" (오염)
GPL 코드가 본 프로젝트에 다음 형태 중 하나로 들어오는 것:

- **Direct copy:** 코드 복사 붙여넣기 (모든 양, 단 한 줄도 금지)
- **Translation:** 다른 언어로 번역 후 사용 (예: LibreDWG C 코드를 Rust 로 그대로 옮김)
- **Structural mimicry:** 함수명, 구조체명, 파일 구조, 모듈 구성을 실질적으로 동일하게 따라함
- **Algorithm porting:** 알고리즘의 step-by-step 구현이 GPL 원본과 실질적으로 동일
- **Linking:** GPL 라이브러리를 dependency 로 추가
- **Dynamic invocation:** GPL 바이너리를 subprocess 로 실행하여 결과만 받음 (간접 contamination)
- **Embedding:** GPL 자료(테스트 파일, 예제 데이터 포함)를 repo 에 포함

### 2.3 "Reference contamination"
직접 코드 contamination 은 아니지만 분쟁 시 불리한 패턴:

- 같은 사람이 GPL 코드를 학습한 직후 같은 기능 구현
- GPL 코드의 주석/문서를 보고 그 표현을 모방
- GPL 프로젝트의 public 함수 시그너처를 그대로 차용

---

## 3. 절대 금지 행위

### 3.1 LibreDWG 관련

다음 행위 모두 금지:

| 행위 | 금지 여부 |
|---|---|
| LibreDWG GitHub repo 다운로드 | **금지** |
| LibreDWG source code 열람 | **금지** |
| LibreDWG 함수명 검색 | **금지** |
| LibreDWG documentation 읽기 (코드 implementation 부분) | **금지** |
| LibreDWG 의 사용자용 문서/README 읽기 | 허용 (사용자 시점, 외부 정보) |
| LibreDWG 라이선스 페이지 읽기 | 허용 (라이선스 자체) |
| LibreDWG 의 존재/기능 범위 인지 | 허용 (시장 분석) |

### 3.2 다른 GPL 코드

위와 동일 원칙이 다음 프로젝트에도 적용:

- **gerbv** (GPL, PCB 처리)
- **libdxf** (LGPL — 별도 검토)
- 기타 CAD/벡터 그래픽 GPL 프로젝트

신규 발견 시 이 리스트에 추가 + `02-reference-material-log.md` 의 NR 섹션에 등록.

---

## 4. 허용되는 GPL 관련 행위

### 4.1 메타 정보 (안전)

- "LibreDWG 가 GPL 라이선스다" 라는 사실 인지
- "LibreDWG 가 DWG 처리한다" 라는 시장 정보 인지
- LibreDWG 가 ezdxf 와 다르다는 비교 인지
- LibreDWG 의 별점/사용자 수 정도의 메타 정보

### 4.2 라이선스 분석 (안전)

- LibreDWG 의 LICENSE 파일 자체 확인
- "GPLv3 or later" 라는 라이선스 사실 확인
- Copyright holder 확인

### 4.3 우회 (불안전 — 금지)

- "GPL 코드 안 보고 GPL 알고리즘 학습한다" — 불가능. GPL 알고리즘은 GPL 코드에서만 추출됨.

---

## 5. 검증 메커니즘

### 5.1 자동 검증 (CI)

`scripts/scan_gpl_signatures.sh` (Stage 1 첫 commit 전 작성):

```bash
#!/bin/bash
# GPL signature scanner
# Detects patterns suggesting LibreDWG / GPL code contamination

set -e

# 1. LibreDWG 알려진 함수명 패턴 (prefix 기반)
LIBREDWG_PATTERNS=(
  "dwg_decode_"
  "dwg_encode_"
  "dwg_section_"
  "dwg_object_"
  "bit_read_"
  "bit_write_"
  "dxf_decode_"
  "dxf_encode_"
  "dwg_handle_"
  "decode_R[0-9]"  # decode_R2000, decode_R2004, etc
  "encode_R[0-9]"
)

# 2. LibreDWG 자주 등장하는 주석 패턴
LIBREDWG_COMMENT_PATTERNS=(
  "GPL"
  "LibreDWG"
  "Free Software Foundation"
  "GNU General Public License"
)

# 3. 알려진 magic constants (LibreDWG 가 사용하는 특정 byte sequence)
# (예: AC1015 R2000 decoder 의 특정 magic bit pattern)

VIOLATIONS=0

for pattern in "${LIBREDWG_PATTERNS[@]}"; do
  if git diff --cached --name-only | xargs grep -l -E "$pattern" 2>/dev/null; then
    echo "WARNING: Pattern '$pattern' detected in staged files"
    VIOLATIONS=$((VIOLATIONS + 1))
  fi
done

for pattern in "${LIBREDWG_COMMENT_PATTERNS[@]}"; do
  if git diff --cached --name-only | xargs grep -l "$pattern" 2>/dev/null; then
    echo "WARNING: GPL-related comment '$pattern' detected"
    VIOLATIONS=$((VIOLATIONS + 1))
  fi
done

if [ "$VIOLATIONS" -gt 0 ]; then
  echo "ERROR: GPL contamination check failed. $VIOLATIONS violation(s) found."
  echo "Review staged changes. If false positive, document override in commit message:"
  echo "  GPL-OVERRIDE: <reason>"
  exit 1
fi

echo "GPL contamination check passed."
exit 0
```

### 5.2 PR 리뷰

리뷰어 체크리스트:
- [ ] 새 코드의 작성자가 contributor declaration 보유?
- [ ] 알고리즘 구조가 LibreDWG 등 알려진 GPL 구현과 실질적으로 다른가?
- [ ] 함수명/구조체명이 GPL 프로젝트 명명 컨벤션과 우연한 유사성 정도인가?
- [ ] commit 메시지에 reference material 언급?

### 5.3 Audit (Series A 실사 대비)

- 매 분기 1회 외부 IP audit (변호사 또는 OSPO 컨설턴트)
- repo 전체 GPL contamination scan
- contributor 별 declaration 유효성 확인
- 의심 영역 spot check

---

## 6. 위반 발견 시 절차

### 6.1 자동 검출 (CI 실패)

1. 해당 commit pre-merge block
2. 작성자에게 알림
3. 패턴이 false positive 면 commit 메시지에 `GPL-OVERRIDE: <사유>` 추가 후 재시도
4. 진짜 contamination 이면 4단계 절차

### 6.2 진짜 contamination 발견

1. **즉시 격리** — 의심 commit revert, 의심 영역 freeze
2. **변호사 통보** — 24시간 이내
3. **재구현 계획** — 다른 contributor 가 reference 자료만 보고 같은 기능 재구현
4. **교차 검증** — 새 구현이 GPL 원본과 다른지 변호사 + 외부 reviewer 확인
5. **이력 기록** — 본 문서 끝의 "Incident Log" 에 추가

### 6.3 의도적 위반

- 즉시 contributor 자격 박탈
- 해당 영역 코드 전체 제거 후 다른 사람이 재구현
- 변호사 자문 + 필요 시 법적 대응

---

## 7. 신규 자료 검토 절차

새로운 GPL 의심 자료 발견 시:

1. 자료 접근 **하지 말고** 본 정책 작성자(hakki) 또는 변호사에게 보고
2. 라이선스 확인 (LICENSE 파일, README, 공식 페이지)
3. GPL/LGPL/AGPL 인 경우 → blacklist 추가
4. MIT/Apache/BSD 인 경우 → whitelist 검토 후 `02-reference-material-log.md` 등록
5. 모호한 경우 → 변호사 자문

---

## 8. 동시 오염 시나리오 (특별 주의)

### 8.1 같은 사람 시나리오

**Risk:** Founder 가 1년 전 LibreDWG 코드를 본 적이 있는 경우.

**대응:**
- Founder 가 GPL 코드 본 적 있다면 본 정책 §6 절차로 자기 신고
- 해당 영역 (예: Stage 2 DWG R&D) 은 다른 contributor 가 처음부터 작성
- "냉각 기간" 12개월 — GPL 코드 본 후 1년 경과 시 같은 사람 작성 허용 (보수적 해석)

### 8.2 AI 코드 어시스턴트 시나리오

**Risk:** GitHub Copilot, Codex, Claude 가 학습 데이터에 LibreDWG 포함 + 그 패턴 제안.

**대응:**
- 모든 AI 제안 코드 검증 (§6.2)
- 수상한 코드 (LibreDWG 함수명 닮음) 폐기 + 자체 작성
- AI 도구 사용 사실은 commit 메시지 또는 별도 log 에 기록

---

## 9. 책임 및 검토

- **Founder (hakki):** 본 정책 집행 1차 책임. 위반 발견 시 즉시 변호사 통보.
- **변호사 (민후 법무법인):** 정책 유효성 검토 + 위반 발생 시 자문.
- **재검토 주기:** 매 6개월 또는 GPL 라이선스 환경 중대 변화 시.

---

## 10. Incident Log

(violations or audit findings 기록 — append-only)

| 일자 | 유형 | 영향 | 조치 | 결과 |
|---|---|---|---|---|
| (없음) | | | | |

---

## 부록: GPLv3 와 Apache-2.0 의 비호환성 요약

**왜 안 되는지 한 줄:** GPL 은 derivative work 가 GPL 라이선스 유지를 강제. Apache-2.0 은 그 강제를 받아들일 수 없음. 한 codebase 에 둘 섞이면 결국 전체가 GPL 이 됨 → Apache-2.0 약속 위반.

따라서 단 **한 줄의 GPL 코드** 도 Apache-2.0 프로젝트에 들어오면:
1. 그 라인 자체 위법
2. 더 큰 문제: 그 라이브러리 전체가 사실상 GPL 라이선스로 강제 해석 가능
3. 모든 사용자(우리 라이브러리 쓰는 다른 회사) 가 GPL 영향 받음 → 우리 비즈니스 모델 붕괴

**No GPL = No Compromise.**
