# Trademark Usage Guideline

**Project:** CAD Open Layer
**Version:** v1 (2026-04-25)
**Reference:** Autodesk Trademark Guidelines (R002 in `02-reference-material-log.md`)

---

## 1. 핵심 원칙

다음 명칭/용어는 Autodesk, Inc. 의 등록 또는 미등록 상표:

- **DWG** (file format 명칭이자 상표)
- **AutoCAD** (제품명 + 상표)
- **Autodesk** (회사명 + 상표)
- **TrustedDWG** (Autodesk 인증 표시)
- **DWG TrueView** (Autodesk 제품)
- **RealDWG** (Autodesk SDK 명)
- **DXF** (file format — Autodesk 가 published spec, 상표성 약함)

**원칙:**

| 사용 형태 | 허용 |
|---|---|
| 사실 진술 / 호환성 설명 | 허용 |
| 출처 표시 후 참조 | 허용 |
| 제품명, 회사명, 도메인명, brand element | **금지** |
| Autodesk 와의 연관성 시사 | **금지** |
| Autodesk 로고/스타일 모방 | **금지** |

---

## 2. 사용 금지 (HARD NO)

### 2.1 제품명 / 브랜드

다음 형태로 "DWG" 또는 "AutoCAD" 사용 금지:

| 표현 | 금지 사유 |
|---|---|
| `OpenDWG` | 브랜드 흉내. ODA 가 2010 합의로 사용 중단한 형태. |
| `DWG Open Layer` | "DWG" 가 brand prefix |
| `DWG-Rust` | "DWG" 가 brand prefix |
| `DWG API` | brand 명 |
| `DWG Cloud` | brand 명 |
| `MyDWG` | brand element |
| `dwg.io`, `dwgrs.io`, `opendwg.com` | 도메인에 DWG |
| `AutoCAD-Replacement` | Autodesk 제품명 직접 사용 |
| `RealDWG-Alternative` | Autodesk SDK 명 직접 사용 |

### 2.2 인증 / 호환 강조 과장

| 표현 | 금지 사유 |
|---|---|
| `100% AutoCAD compatible` | 실제로 100% 보장 불가 + Autodesk 제품명 brand 형 |
| `TrustedDWG certified` | TrustedDWG 는 Autodesk 인증 마크. 사칭 |
| `Autodesk-approved` | Autodesk 가 승인하지 않음 |
| `Official DWG library` | 오피셜 아님 |
| `The DWG library` | the 사용은 brand 위치 |

### 2.3 로고 / 시각 요소

- Autodesk 로고 사용 금지
- AutoCAD 로고 사용 금지
- DWG 마크 (Autodesk 의 stylized "DWG") 사용 금지
- Autodesk brand color/font 모방 금지

---

## 3. 사용 허용 (OK)

### 3.1 사실/호환성 설명

다음 형태는 허용:

| 표현 |
|---|
| "Library for reading and writing DXF and DWG files" |
| "Compatible with the DWG file format" |
| "Works with .dwg files" |
| "Supports DXF (Autodesk Drawing Exchange Format)" |
| "Reads files produced by AutoCAD" |
| "Outputs DXF that opens in AutoCAD R2000 and later" |

### 3.2 비교 / 참조

다음 형태는 허용 (출처 명시 시):

| 표현 |
|---|
| "Unlike Autodesk RealDWG SDK, this library is open source" |
| "Provides similar functionality to ezdxf and LibreDWG" |
| "DWG is the native file format of AutoCAD" |

### 3.3 우리 프로젝트 명명

권장:

| 표현 | 평가 |
|---|---|
| **CAD Open Layer** | OK — DWG 미포함, 일반적 용어 |
| `cad-open-layer` (crate name) | OK |
| `@cadopenlayer/core` (npm) | OK |
| `cadopenlayer.dev` (도메인) | OK |
| `CAD-OL` (약어) | OK |

회피:

| 표현 | 사유 |
|---|---|
| `dwg-layer`, `dwg-rust` | DWG 포함 |
| `opencad-trade-mark`, `dwg-cad` | DWG 또는 CAD trade implication |
| `autodesk-alternative` | 회사명 직접 |

---

## 4. 필수 고지문 (Trademark Notice)

다음 고지문을 README, 웹사이트 footer, 문서에 포함:

### 4.1 README 헤더 또는 푸터

```markdown
## Trademark Notice

DWG is a registered trademark of Autodesk, Inc., and AutoCAD is a registered 
trademark of Autodesk, Inc. CAD Open Layer is an independent open-source 
project and is not affiliated with, endorsed by, sponsored by, or otherwise 
connected to Autodesk, Inc. References to "DWG", "DXF", "AutoCAD", or other 
Autodesk product names are made solely for descriptive purposes to indicate 
file format compatibility.
```

### 4.2 웹사이트 footer

```html
<footer>
  <p class="trademark-notice">
    DWG and AutoCAD are trademarks of Autodesk, Inc.
    CAD Open Layer is an independent project not affiliated with Autodesk, Inc.
  </p>
</footer>
```

### 4.3 첫 사용 시 별표

문서/blog post 에서 "DWG" 처음 사용 시:

```
DWG* file format
*DWG is a trademark of Autodesk, Inc.
```

또는 inline:

```
The DWG (a trademark of Autodesk, Inc.) file format ...
```

---

## 5. Marketing Copy 가이드라인

### 5.1 헤드라인 (좋은 예)

```
✓ "Programmable open-source library for CAD file formats"
✓ "Read and write CAD files from any language"
✓ "AI-friendly access to DXF and DWG formats"
✓ "rhwp-style open layer for CAD formats"
```

### 5.2 헤드라인 (나쁜 예)

```
✗ "The DWG library for developers"  (DWG가 brand 위치)
✗ "Replace AutoCAD's RealDWG SDK"  (Autodesk 제품명 직접)
✗ "100% TrustedDWG compatible"  (Autodesk 인증 사칭)
✗ "Autodesk-approved CAD library"  (사실 아님)
```

### 5.3 본문 표현

좋은 예:

```
CAD Open Layer is an open-source Rust library that lets developers read 
and write files in the DXF format (an Autodesk-published exchange format) 
and the DWG file format (a proprietary format created by Autodesk, Inc.). 

This library allows AI agents and developer tools to manipulate CAD drawings 
without requiring an AutoCAD installation or proprietary SDK license.
```

특징:
- DWG/AutoCAD 출처 명확히 (Autodesk, Inc.)
- 기능 설명에 한정
- brand 위치 안 차지

---

## 6. 도메인 / Repo / Package 명명

### 6.1 GitHub repository

| 후보 | 평가 |
|---|---|
| `cad-open-layer` | OK |
| `cadopenlayer` | OK |
| `dxf-rs` | 보통 (DXF는 trademark 약하지만 부정확) |
| `dwg-parser` | **금지** |
| `opendwg-rust` | **금지** |
| `autocad-lib` | **금지** |

### 6.2 도메인

| 후보 | 평가 |
|---|---|
| `cadopenlayer.dev` | OK (.dev 권장) |
| `cad-open-layer.com` | OK |
| `cadlayer.io` | OK |
| `dwg.dev`, `dwg.io` | **금지** (DWG 도메인) |
| `autocadlib.com` | **금지** |

### 6.3 Package names

| 생태계 | 후보 | 평가 |
|---|---|---|
| crates.io | `cad-open-layer` | OK |
| npm | `@cadopenlayer/core` | OK |
| PyPI | `cad-open-layer` | OK |
| crates.io | `dwg-rust` | **금지** |
| npm | `@dwg/parser` | **금지** |

---

## 7. 소셜 미디어

### 7.1 핸들

| 후보 | 평가 |
|---|---|
| `@cadopenlayer` | OK |
| `@cad_ol` | OK |
| `@dwgrust` | **금지** |

### 7.2 게시물

좋은 예:

```
Just released CAD Open Layer v0.1 — an open-source Rust library for 
reading and writing CAD file formats (DXF and DWG). Apache-2.0 licensed.

#opensource #rust #cad

Note: DWG is a trademark of Autodesk, Inc.
```

---

## 8. 변경 알림

Autodesk 가 trademark guideline 을 변경하면 본 정책도 갱신.

- Autodesk 페이지 (R002) 분기별 1회 확인
- 큰 변경 시 변호사 검토 후 본 가이드라인 v 증가

---

## 9. C&D 수령 시 1차 대응 (요약)

상세는 `legal-prep.md` §5.

1. **답장 금지** — 즉시 답하지 않는다
2. **사본 보존** — 모든 첨부 파일 저장
3. **변호사 통보** — 24시간 이내
4. **트리거 분석** — 본 가이드라인 위반 부분 식별
5. **임시 수정** — 분명한 위반 (도메인명, 마케팅 카피) 즉시 시정

---

## 부록: 자주 받는 질문

**Q1. ".dwg" 확장자 파일을 출력해도 되는가?**
A. **OK.** Autodesk 가 .dwg 확장자 사용을 금지하지 않는다고 명시 (R002).

**Q2. README 에 "Compatible with AutoCAD" 라고 써도 되는가?**
A. **OK** — fact 진술. 단 sub-text 에서 "AutoCAD is a trademark of Autodesk" 명시.

**Q3. 광고에 "Open-source AutoCAD alternative" 표현 가능?**
A. **부분적 OK.** "AutoCAD" 가 brand 위치이지만, "alternative" 는 비교 맥락. 단 첨부 disclaimer 필수. 변호사 자문 권장.

**Q4. GitHub repo 이름에 "autocad" 들어가면?**
A. **금지.** Autodesk brand 이름 직접 사용.

**Q5. 블로그 제목 "Building a DWG parser in Rust" 가능?**
A. **OK** — descriptive technical title. 단 내용에 trademark notice 첨부.

**Q6. Logo 디자인에 "DWG" 글자 디자인하면?**
A. **금지.** Autodesk DWG 로고 모방으로 해석 가능.

**Q7. "rhwp for CAD" 표현은?**
A. **OK** — rhwp 는 별개 프로젝트 (Apache-2.0). 본 프로젝트의 비교 reference. CAD 도 일반적 산업 용어.

---

## 10. 책임

- **Founder (hakki):** 본 가이드라인 집행 + 모든 communication 검토
- **변호사 (민후):** 분쟁 발생 시 자문 + 분기별 변경 검토
