# Clean-Room Implementation Policy

**Project:** CAD Open Layer
**Version:** v1 (2026-04-25)
**Author:** hakki (Founder)
**Reviewed by:** [TBD - 민후 법무법인 검토 대기]
**Effective:** Stage 1 빌드 첫 commit 시점부터

---

## 1. 정의

**Clean-Room Implementation:** 폐쇄형 또는 호환 비호환 라이선스 코드(특히 Autodesk RealDWG SDK, ODA Teigha SDK, LibreDWG GPLv3)의 소스코드를 참조하지 않은 상태로, 공개된 사양·문서·합법 취득 샘플 파일에 근거하여 독립적으로 작성하는 구현 방식.

**핵심 목적:** Apache-2.0 라이선스로 배포 가능한 독립 저작물 보장 + 향후 IP 분쟁 시 입증 가능한 절차 증거.

---

## 2. 적용 범위

본 정책은 다음에 적용된다.

- CAD Open Layer 의 모든 crate 코드 (`crates/cad-*`)
- 모든 기여자(contributor) — 본인, 향후 채용 엔지니어, 외부 PR 송부자
- 모든 외부 자료 참조 행위 (PDF, GitHub, 논문, 강의자료, 블로그)
- 자동화된 코드 생성/제안 도구 (Copilot, Codex, Claude Code 등)의 결과물도 본인이 검증 후 본 정책 준수 책임

---

## 3. 허용되는 자료 (whitelist)

다음 자료에 한정하여 참조 가능. **모든 신규 자료는 `02-reference-material-log.md` 에 기록 후 사용.**

### 3.1 공식 공개 사양

- **Autodesk DXF Reference (모든 버전).** Autodesk가 공식 공개. 무료. R12부터 R2024까지 PDF 제공.
- **Autodesk DWG Trademark Guidelines.** 공식 페이지.
- **AutoLISP / ObjectARX Reference.** Autodesk 공식 공개 부분 (SDK 본체 제외).

### 3.2 ODA 공개 자료 (제한적)

- **ODA Open Design Specification PDF (공개 버전).** 단 사용 전 라이선스 조건·재배포 가부 확인 필수. 표현(문장, 표, 다이어그램)은 복제하지 않고 사실·아이디어 수준에서만 참조.
- ODA 멤버십 자료, ODA SDK, ODA NDA 자료는 **사용 금지** (별도 절차 후 변호사 재검토 시까지).

### 3.3 학술 자료

- 컴퓨터 그래픽스, 계산 기하학 교과서 및 논문 (CCBY 등 공개 라이선스 또는 합법 구매)
- Bentley-Ottmann 알고리즘
- DCEL 자료구조
- Polygon clipping (Vatti, Greiner-Hormann 등)
- 평면 그래프 face extraction

### 3.4 호환 라이선스 오픈소스 (참조용)

- **rhwp** (Apache-2.0): 프로젝트 구조, 패키징 방식만 참고. 알고리즘/구현 코드 직접 참조 금지.
- **ezdxf** (MIT): DXF 사양 검증, 테스트 파일 라이선스 호환 확인. 알고리즘 코드 참조 금지.
- **Rust 생태계 crates** (각 라이선스 확인): rstar, slotmap, proptest, etc.

### 3.5 합법 취득 샘플 파일

- 본인이 자체 생성한 합성 DXF 파일
- 공개 도면 데이터셋 (예: 정부 GIS 공개 데이터, 학교 졸업작품 공개 도면)
- Maket 등 파트너로부터 NDA 후 받은 anonymized 파일 (encrypted storage, repo 비포함)

---

## 4. 금지되는 자료 (blacklist)

다음 자료는 어떤 contributor 도 접근/참조 금지.

### 4.1 폐쇄/상용 SDK

- **Autodesk RealDWG SDK** (소스, 문서, 헤더, 컴파일된 바이너리 disassembly 모두 금지)
- **ODA Teigha / ODA Drawings SDK** 
- **Bentley MicroStation SDK**
- 기타 상용 CAD vendor 의 비공개 SDK

### 4.2 GPL 계열 코드

- **LibreDWG (GPLv3)** — 4.3 에서 별도 정책
- 기타 GPLv2 / GPLv3 / AGPL 라이선스 CAD 처리 라이브러리
- Apache-2.0 호환되지 않는 모든 copyleft 라이선스

### 4.3 NDA 자료

- ODA 멤버십 통해 취득한 자료 (가입 시점부터)
- 산업 파트너가 NDA 하에 공유한 spec
- 외부 자문/컨설팅에서 받은 비공개 자료

### 4.4 의심 출처 자료

- 출처 불명 leak 자료
- 토렌트 등 비합법 경로 자료
- "내부 직원에게 들은" 비공식 정보

---

## 5. 절차

### 5.1 신규 자료 접근 전

1. 자료가 위 3장 whitelist 에 포함되는지 확인
2. 의심 시 본 정책 추가 또는 변호사 자문 후 결정
3. 접근 시 `02-reference-material-log.md` 에 기록:
   - 자료명
   - 출처 URL
   - 라이선스/이용조건
   - 접근 일자
   - 접근자 (contributor name)
   - 사용 목적

### 5.2 코드 작성 시

1. 모든 구현은 Rust 로 직접 작성. 다른 언어 코드 번역/이식(porting) 금지.
2. 함수명/구조체명/변수명은 자체 명명. 참조 자료의 명명을 그대로 차용 금지.
3. 알고리즘 구조도 가능한 한 자체 설계. 단 표준 알고리즘 (Bentley-Ottmann, DCEL 등 학계 공유 자산) 은 학술 자료 인용 후 자체 작성.
4. 각 commit 메시지에 참조한 자료 명시 (가능 시):
   ```
   feat(extract): wall pair detection
   
   References: docs/algorithms.md §1, Autodesk DXF R2000 Reference §LINE
   ```

### 5.3 코드 리뷰 시

리뷰어는 다음 체크:
- [ ] 새 코드가 contributor declaration 보유한 사람의 작성인가
- [ ] 참조 자료가 reference log 에 등록되어 있는가
- [ ] 명명 컨벤션이 LibreDWG/RealDWG 등 금지 자료의 명명을 모방하는 흔적 있는가
- [ ] 알고리즘 구조가 금지 자료와 실질적으로 유사하지 않은가

### 5.4 외부 PR 처리

1. PR 송부자가 `04-contributor-declaration-template.md` 서명 완료해야 머지 가능
2. PR 본문에 참조 자료 명시 의무
3. 의심 시 변경 거부 + 사유 기록

---

## 6. 자동화된 코드 생성 도구 (AI Coding Assistant) 정책

### 6.1 허용 조건

GitHub Copilot, Codex, Claude Code 등 사용 시:
- **본인이 결과물의 모든 부분 검증 후 책임진다**
- 도구가 실제로 LibreDWG 등을 학습 데이터에 포함했을 가능성 인지
- 도구가 제안한 코드가 의심스럽게 친숙하면 (특정 함수 시그너처, 변수명) → 폐기 후 자체 작성

### 6.2 검증 절차

AI 가 제안한 코드를 commit 전:
1. Function name 검색 — 알려진 LibreDWG 함수명과 일치 시 폐기
2. Comment / docstring 검색 — 알려진 LibreDWG 주석 패턴과 일치 시 폐기
3. 알고리즘 구조 — LibreDWG 와 같은 함수 분할/필드 순서면 재구성

### 6.3 차단 패턴 (CI 자동 검증)

`scripts/scan_gpl_signatures.sh` 에 다음 패턴 등록:
- LibreDWG 의 알려진 public 함수명 prefix (`dwg_`, `dxf_decode_*`, `bit_*`, `decode_*`)
- LibreDWG 주석에 자주 등장하는 시그너처
- LibreDWG 의 특정 magic number 처리 패턴

발견 시 commit 거부.

---

## 7. 위반 시 조치

### 7.1 기여자 위반

- 1차: 위반 commit revert + 재작성 요구 + 경고
- 2차: contributor 자격 정지 + 문제 코드 영역 다른 contributor 가 재구현
- 3차: 영구 자격 박탈

### 7.2 의도적 위반

- 즉시 자격 박탈
- 해당 영역 코드 전체 다른 사람이 재구현
- 변호사 통보

### 7.3 모르고 위반

- 위반 사실 발견 즉시 자기 신고 (보호 조치)
- revert + 재작성 후 재진입 가능
- 이력 기록

---

## 8. 검토 및 갱신

- 본 정책은 **최초 변호사 검토 후 ratify**
- 매 6개월 또는 중대 변화 시 (Stage 2 DWG 진입, 신규 contributor 영입 등) 재검토
- 변경 시 변호사 재검토 → 새 버전 발행 → 모든 contributor 재서명

---

## 9. 책임자

- **Founder (hakki):** 본 정책 유지·집행 1차 책임
- **변호사 (민후 법무법인):** 정책 유효성 검토 + Series A 실사 시 의견 제공

---

## 부록 A: 의심 시 의사결정 트리

```
신규 자료 접근하려 할 때
        │
        ▼
┌────────────────────────┐
│ Whitelist (§3) 에 있나? │  yes ──> 접근 OK, log 작성
└──────────┬─────────────┘
           │ no
           ▼
┌────────────────────────┐
│ Blacklist (§4) 에 있나? │  yes ──> 접근 금지
└──────────┬─────────────┘
           │ no
           ▼
┌────────────────────────┐
│ 라이선스 명확하나?      │  no  ──> 접근 보류, 변호사 문의
└──────────┬─────────────┘
           │ yes
           ▼
┌────────────────────────┐
│ Apache-2.0 호환?        │  no  ──> 접근 금지
└──────────┬─────────────┘
           │ yes
           ▼
       접근 OK + log + 본 정책에 추가 검토
```
