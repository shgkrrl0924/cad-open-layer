# ODA Contact / No-NDA Log

**Project:** CAD Open Layer
**Purpose:** ODA(Open Design Alliance) 와의 모든 접촉, 자료 접근, NDA 시그너처 가능 시점을 기록.
**Why:** 변호사 의견서가 권고 — Series A 실사 시 "ODA 멤버십 미가입 + NDA 미체결 + 공개 자료만 사용" 입증 자료.

---

## 현재 상태 (2026-04-25 baseline)

### Membership Status

- **ODA Member:** **NO**
- **ODA Membership Application Submitted:** No
- **ODA SDK Downloaded:** No
- **ODA NDA Signed:** No
- **Last Verified:** 2026-04-25

### Material Access Status

- **ODA Public Specification PDF (R001 = R003 in reference log):** Listed in reference log but **NOT YET ACCESSED** in Stage 1. 변호사 검토 후 Stage 2 에 사용 시작 예정.
- **ODA Member-only Materials:** **NEVER ACCESSED**
- **ODA Webinar Recordings (Member-only):** **NEVER ACCESSED**
- **ODA Forum (Member-only):** **NEVER ACCESSED**

### Communication Status

- **Email exchange with ODA:** None
- **Phone call with ODA:** None
- **In-person meeting:** None
- **Conference attendance (BIM Show, ODA Annual Meeting, etc.):** None

---

## Stage 1 정책

**Stage 1 동안 ODA 와의 모든 접촉을 보류한다.**

- 멤버십 가입 안 함
- SDK 다운로드 안 함  
- NDA 체결 안 함
- 자료 요청 안 함
- ODA 공개 PDF 도 미사용 (Stage 2 시작 전 변호사 재검토 후 결정)

이 정책의 이유:
1. Stage 1 은 **DXF-first**. DWG 바이너리 처리 없음. ODA 자료 불필요.
2. ODA 멤버십 가입 시 NDA 조항이 **clean-room 독립 구현 주장과 충돌 가능**.
3. Series A 실사 시 "ODA 자료 미접근" 이 가장 안전한 입증.

---

## Stage 2 진입 시 절차 (DWG R&D 시작 시점)

DWG 바이너리 처리에 ODA 공개 PDF 가 사실상 불가피한 시점이 오면:

### Step 1: 변호사 재검토 요청

1. 민후 법무법인에 다음 명시:
   - ODA 공개 PDF URL: https://www.opendesign.com/files/guestdownloads/OpenDesign_Specification_for_.dwg_files.pdf
   - 사용 의도: DWG 사양 학습 (사실/아이디어 수준)
   - 사용 안 할 것: 표/문장/다이어그램의 표현 복제
2. 변호사 의견서 → 본 로그에 첨부

### Step 2: 자료 접근

1. 공식 사이트에서 PDF 다운로드 (Wayback Machine 캡처)
2. PDF 메타데이터 + 다운로드 일자 + 다운로더 기록
3. `02-reference-material-log.md` 의 R003 항목 status 업데이트
4. **본 파일 `Material Access Status` 업데이트**

### Step 3: 사용 절차

1. 공개 PDF 의 사실/아이디어만 학습
2. 학습 노트는 본인 머릿속 + summary (raw 인용 금지)
3. 코드 작성 시 PDF 의 표현 모방 금지 — 자체 명명, 자체 구조
4. commit 메시지에 "ODA public spec referenced" 명시 (단 직접 인용 안 함)

### Step 4: 멤버십 결정 (별도)

ODA 멤버십 가입 여부는 별도 결정. 가입 시:

1. **NDA 약관 변호사 검토 필수** — clean-room 주장과 충돌하는 조항 없는지
2. 가입 후 멤버 자료 접근하는 사람과 **clean-room 코드 작성하는 사람 분리** 권장
3. 본 로그에 가입 일자, 멤버십 등급, 접근 자료 범위 모두 기록

**현 시점 판단:** Stage 1 종료 후 매출 검증 단계에서 멤버십 비용/benefit 재평가. 무리 가입 안 함.

---

## 접촉 이력 (live append-only)

### 2026-04-25 — Baseline

- ODA 멤버십 미가입
- ODA 공개 PDF 미사용
- ODA 와 일체 communication 없음
- 본 로그 시작

### (이후 접촉 발생 시 여기에 append)

```
### YYYY-MM-DD — {접촉 종류}
- **유형:** Email / Phone / Meeting / Material download / NDA review / Membership inquiry
- **상대:** {ODA 측 contact name + 직책}
- **이니시에이터:** {hakki 또는 그쪽}
- **목적:** {요청 내용}
- **결과:** {합의 / 보류 / 자료 수령 / etc.}
- **자료 첨부:** {이메일 thread / 다운로드 파일 hash / etc.}
- **변호사 통보 여부:** Yes / No (Yes 시 일자)
- **본 정책 §Stage 2 §Step 1~3 준수 여부:** {반영 항목}
```

---

## NDA 시그너처 절차 (만약 발생 시)

NDA 체결을 검토하게 되면:

1. **시그너처 전 변호사 검토 필수** — 24시간 이내
2. 검토 항목:
   - clean-room 독립 구현 주장에 영향 있는 조항?
   - 내부 정보 사용 범위 제한?
   - 본 프로젝트 contributor 들에게 NDA 효력 미치는지?
   - 종료 시 자료 폐기 조항?
3. 변호사 OK 후 시그너처
4. 시그너처 사본 본 로그에 첨부 (sensitive 부분 redact)
5. NDA 하 자료에 접근하는 contributor 별 declaration 갱신 (`04-contributor-declaration-template.md` 의 §4 항목)

**현 시점 입장:** ODA NDA 체결 안 한다.

---

## ODA 와 분리된 활동 OK

다음은 ODA 와 무관하므로 자유롭게 가능:

- ODA 의 공개 보도자료, 블로그 포스트 읽기 (사용자 시점 정보)
- ODA 와 Autodesk 의 2010 합의 등 공개 법적 문서 학습
- ODA 멤버십 정보 페이지 read-only 확인 (가입 결정용)
- ODA 가 publish 한 industry whitepaper (공개) 읽기

---

## 검증 (Series A 실사 대비)

본 로그가 IP audit 시 핵심 입증 자료. 갖춰야 할 것:

- [ ] ODA 멤버십 미가입 사실 명시
- [ ] ODA NDA 미체결 사실 명시
- [ ] ODA 자료 접근 시 (Stage 2) 변호사 사전 검토 기록
- [ ] 모든 ODA 관련 communication 기록
- [ ] 사용 자료가 모두 공개 PDF 임 (member-only 아님) 입증

---

## 책임

- **Founder (hakki):** ODA 와의 모든 접촉 보고 + 본 로그 유지 1차 책임
- **변호사 (민후):** Stage 2 진입 시 재검토 + NDA 검토
- **Contributor:** ODA 자료 접근 시 본인 declaration §4 항목 업데이트
