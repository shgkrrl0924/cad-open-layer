# Test Corpus License Log

**Project:** CAD Open Layer
**Format:** Live append-only log. 모든 테스트 DXF/DWG 파일 등록.
**Verification:** CI `scripts/check_corpus_log_consistency.sh` 가 `tests/corpus/` 의 모든 파일이 본 로그에 등록되어 있는지 자동 검증.

---

## 사용 규칙

### 추가 시점

- `tests/corpus/` 또는 `tests/golden/` 에 새 파일 추가하기 전 본 로그에 항목 추가
- 같은 commit 안에서 (corpus 추가 commit + 로그 등록 commit 분리 안 함)

### 필수 필드

| 필드 | 의미 |
|---|---|
| `id` | 단조 증가 (T001, T002, …) |
| `filename` | 정확한 파일 경로 |
| `dxf_version` | AC1015 등 |
| `source` | 자체 합성 / 공개 데이터셋 / 파트너 NDA |
| `license` | "Synthetic, Apache-2.0", "MIT (ezdxf)", "NDA-restricted (Maket)" 등 |
| `creator` | 누가 만들었는지 |
| `created_date` | YYYY-MM-DD |
| `purpose` | 어떤 알고리즘 / 회귀 테스트 용도 |
| `apache_compatible` | yes / no — Apache-2.0 라이선스로 배포 가능한가 |
| `repo_inclusion` | "public repo OK" / "encrypted, .gitignored" |
| `entity_count` | 대략 entity 수 |
| `notes` | 특이사항, anonymization 처리 여부 등 |

---

## 등록 corpus

### T001 — small_floorplan_simple_r2000.dxf

- **id:** T001
- **filename:** `tests/corpus/synthetic/small_floorplan_simple_r2000.dxf`
- **dxf_version:** AC1015 (R2000)
- **source:** 자체 합성 (제공된 파일)
- **license:** Synthetic, Apache-2.0 compatible
- **creator:** hakki
- **created_date:** 2026-04-25 (또는 그 이전)
- **purpose:** Layer 1 (parser) 부트스트랩 검증. 기본 entity (LINE, LWPOLYLINE, INSERT, TEXT, DIMENSION) 처리 확인.
- **apache_compatible:** yes
- **repo_inclusion:** public repo OK
- **entity_count:** ~수십 entity (정확 수치 빌드 후 측정)
- **notes:** Layer 구조 명확 (WALLS, DOORS, WINDOWS, DIMENSIONS, TEXT, GRID). Layer color codes 정상.

### T002 — medium_multifamily_floorplan_synthetic_r2000.dxf

- **id:** T002
- **filename:** `tests/corpus/synthetic/medium_multifamily_floorplan_synthetic_r2000.dxf`
- **dxf_version:** AC1015 (R2000)
- **source:** 자체 합성 (제공된 파일)
- **license:** Synthetic, Apache-2.0 compatible
- **creator:** hakki
- **created_date:** 2026-04-25 (또는 그 이전)
- **purpose:** Layer 2/3 알고리즘 검증. 복수 방, T/L junction, dimension chain, room detection.
- **apache_compatible:** yes
- **repo_inclusion:** public repo OK
- **entity_count:** ~수백 entity
- **notes:** Synthetic data — 사용자/회사 식별 정보 없음.

---

## 향후 추가 예정 (placeholder)

### G001 — small_floorplan_simple_r2000.golden.json

- **id:** G001
- **filename:** `tests/corpus/golden/small_floorplan_simple_r2000.golden.json`
- **format:** JSON (golden snapshot)
- **source:** 자체 합성 (T001로부터 derive)
- **license:** Synthetic, Apache-2.0 compatible
- **creator:** hakki
- **created_date:** 2026-04-25
- **purpose:** T001의 extract_floorplan 결과 골든. 회귀 검증용.
- **apache_compatible:** yes
- **repo_inclusion:** public repo OK
- **notes:** 자동 생성된 결과물 스냅샷.

### G002 — medium_multifamily_floorplan_synthetic_r2000.golden.json

- **id:** G002
- **filename:** `tests/corpus/golden/medium_multifamily_floorplan_synthetic_r2000.golden.json`
- **format:** JSON (golden snapshot)
- **source:** 자체 합성 (T002로부터 derive)
- **license:** Synthetic, Apache-2.0 compatible
- **creator:** hakki
- **created_date:** 2026-04-25
- **purpose:** T002의 extract_floorplan 결과 골든. 회귀 검증용.
- **apache_compatible:** yes
- **repo_inclusion:** public repo OK
- **notes:** 자동 생성된 결과물 스냅샷.

### T003 — TBD: 작은 single-room test

- **purpose:** Wall extraction 단위 테스트 (4-wall rectangular room)
- **creator:** hakki (자체 합성)
- **status:** Stage 1 Day 2 작성 예정

### T004 — TBD: T-junction layout

- **purpose:** Wall topology graph 검증
- **status:** Stage 1 Day 6 작성 예정

### T005 — TBD: Curved wall (Stage 1.5)

- **purpose:** Curved wall handling (defer 가능)
- **status:** Stage 2

### T006~T015 — Maket NDA samples (PoC scope call 후)

- **filename:** `tests/corpus/maket-real/*.dxf` (encrypted, gitignored)
- **dxf_version:** TBD
- **source:** Maket Inc. (NDA via Patrick Murphy)
- **license:** **NDA-restricted, NOT for redistribution**
- **creator:** Maket pipeline (anonymized)
- **created_date:** TBD
- **purpose:** PoC 4-week 검증 (8/10 no-redraw)
- **apache_compatible:** **NO**
- **repo_inclusion:** **NEVER public.** `.gitignore` 에 `tests/corpus/maket-real/` 등록.
- **storage:** Local encrypted (LUKS or VeraCrypt) on hakki machine. NDA expires after PoC OR per Maket terms.
- **notes:** anonymized by Maket. 본인은 anonymization 절차 검증 책임 있음.

---

## 명시적 비참조 (NOT used)

다음 corpus 는 의도적으로 사용하지 않음.

### NT001 — LibreDWG test corpus
- **source:** https://github.com/LibreDWG/libredwg/tree/master/test
- **license:** GPLv3
- **status:** **NEVER USED**
- **rationale:** GPLv3 라이선스. Apache-2.0 호환 안 됨. `03-no-gpl-contamination-policy.md`.

### NT002 — Autodesk RealDWG sample files
- **license:** Proprietary
- **status:** **NEVER USED**

### NT003 — ODA Drawings SDK examples
- **license:** ODA NDA
- **status:** **NEVER USED**

---

## 추가 절차 (신규 corpus 추가 시)

1. 라이선스 확인 — 명시 없으면 사용 안 함
2. 출처 합법성 확인 — 토렌트, 비공식 공유 금지
3. 본 로그에 항목 추가 (위 필수 필드 모두)
4. 테스트 파일 commit (public repo OK 인 경우만)
5. NDA 자료는 별도 encrypted storage + .gitignore 등록 + 본 로그에 명시
6. CI consistency check 통과 확인

---

## CI 검증 (`scripts/check_corpus_log_consistency.sh`)

```bash
#!/bin/bash
# tests/corpus/ 의 모든 파일이 본 로그에 등록되어 있는지 확인

set -e

CORPUS_LOG="legal-package/06-test-corpus-license-log.md"
CORPUS_DIR="tests/corpus"

VIOLATIONS=0

# .gitignored 디렉토리 (NDA samples) 제외
find "$CORPUS_DIR" -type f \
  -not -path "*/maket-real/*" \
  -not -path "*/.git/*" |
while read -r filepath; do
  filename=$(basename "$filepath")
  if ! grep -q "$filename" "$CORPUS_LOG"; then
    echo "ERROR: $filepath not registered in $CORPUS_LOG"
    VIOLATIONS=$((VIOLATIONS + 1))
  fi
done

if [ "$VIOLATIONS" -gt 0 ]; then
  echo "Add corpus entries before merging."
  exit 1
fi

echo "Corpus log consistency check passed."
```

---

## NDA Corpus 저장 절차 (Maket 케이스)

1. Maket 으로부터 anonymized 파일 받음 (Patrick 통해)
2. **즉시 `tests/corpus/maket-real/` 로 이동** (가깝게 두지 않음)
3. `.gitignore` 에 해당 디렉토리 등록 (already done)
4. **로컬 encrypted volume 에만 저장**:
   - LUKS (Linux) / VeraCrypt (Windows)
   - 암호 별도 password manager
5. 본 로그에 entry 추가 (필명 / 일자 / NDA reference)
6. Pre-PoC: 새 contributor 에게 NDA 적용 시 별도 NDA 동의 받음
7. PoC 종료 후 (또는 NDA 만료 시) **secure deletion**:
   - `shred -u` 또는 OS 별 secure delete
   - 본 로그에 deletion 일자 기록

---

## 라이선스 호환성 빠른 참조

| 라이선스 | Apache-2.0 코드 + 라이선스 파일에 포함 가능? | Test corpus 로 사용 가능? |
|---|---|---|
| Public Domain (CC0) | yes | yes |
| Apache-2.0 | yes | yes |
| MIT | yes | yes |
| BSD-3-Clause | yes | yes |
| ISC | yes | yes |
| Unlicense | yes | yes |
| LGPLv2.1+ | dynamic link OK, 코드 포함 별도 검토 | corpus 만 OK (명확하면) |
| GPLv2 | **no** | **no** |
| GPLv3 | **no** | **no** |
| AGPL | **no** | **no** |
| CC-BY (4.0) | yes (with attribution) | yes |
| CC-BY-SA | corpus는 OK, 코드는 별도 검토 | yes (with attribution) |
| Proprietary (no license) | **no** | **no** |
| NDA-restricted | **NEVER public** | yes (encrypted, gitignored) |
