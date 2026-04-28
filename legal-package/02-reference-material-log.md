# Reference Material Log

**Project:** CAD Open Layer
**Format:** Live append-only log. Stage 1 시작 후 매 신규 자료 접근 시 즉시 추가.
**Verification:** CI `scripts/check_corpus_log_consistency.sh` 가 신규 자료 PR 와 본 로그 동기화 검증.

---

## 사용 규칙

### 추가 시점
- 새 외부 문서/PDF/논문/코드 저장소 처음 보는 순간
- 같은 자료 다른 버전을 새로 보는 경우 별도 항목
- 자료 다운로드 또는 북마크 추가 시 30분 이내 기록

### 필수 필드
| 필드 | 의미 |
|---|---|
| `id` | 단조 증가 정수 (R001, R002, …) |
| `title` | 자료 정확한 제목 |
| `source_url` | 영구 URL (Wayback Machine 캡처 권장) |
| `license` | 라이선스 명시 또는 "Public Domain" 또는 "Proprietary, descriptive use only" |
| `accessed_date` | YYYY-MM-DD |
| `accessed_by` | contributor name |
| `purpose` | 한 문장으로 무엇을 위해 본 자료인가 |
| `usage` | "Reference / Copy / Algorithm reference / Verification only" |
| `whitelist_section` | `01-clean-room-policy.md` 의 §3.x 중 어느 카테고리 |

---

## 등록 자료 (live)

### R001
- **id:** R001
- **title:** AutoCAD 2018 DXF Reference
- **source_url:** https://help.autodesk.com/view/OARX/2018/ENU/?guid=GUID-235B22E0-A567-4CF6-92D3-38A2306D73F3
- **license:** Public, Autodesk official documentation
- **accessed_date:** 2026-04-25
- **accessed_by:** hakki
- **purpose:** DXF group code 사양 학습. Entity 종류별 group code 매핑.
- **usage:** Reference (사실/사양)
- **whitelist_section:** §3.1 공식 공개 사양

### R002
- **id:** R002
- **title:** Autodesk Trademark Guidelines (DWG)
- **source_url:** https://www.autodesk.com/company/legal-notices-trademarks/trademarks/dwg-trademarks
- **license:** Public, Autodesk official trademark guidance
- **accessed_date:** 2026-04-25
- **accessed_by:** hakki
- **purpose:** DWG 상표 사용 기준 확인 — `05-trademark-usage-guideline.md` 의 근거.
- **usage:** Reference
- **whitelist_section:** §3.1

### R003
- **id:** R003
- **title:** Open Design Alliance — Open Design Specification for .dwg files
- **source_url:** https://www.opendesign.com/files/guestdownloads/OpenDesign_Specification_for_.dwg_files.pdf
- **license:** ODA 공개 PDF (이용조건 별도 검토 필요 — see R003-LICENSE-NOTE)
- **accessed_date:** 2026-04-25 (preliminary, full review 변호사 자문 후)
- **accessed_by:** hakki
- **purpose:** DWG 바이너리 포맷 사양 학습 (Stage 2 R&D 시점에 사용 예정)
- **usage:** TBD — 변호사 검토 후 결정. 현재 미사용.
- **whitelist_section:** §3.2 (제한적 — 사용 전 별도 검토 필수)
- **note:** **현재 STAGE 1 에선 미사용.** Stage 2 DWG R&D 시점에 사용 시작 전 본 항목 status update + 변호사 ratification 필수.

### R004
- **id:** R004
- **title:** rhwp project (GitHub)
- **source_url:** https://github.com/edwardkim/rhwp
- **license:** Apache License 2.0
- **accessed_date:** 2026-04-23
- **accessed_by:** hakki
- **purpose:** 프로젝트 구조, 패키징 방식 참고. CAD Open Layer 의 영감 / 비교점.
- **usage:** Reference (프로젝트 구조만 — algorithm 코드 직접 참조 안 함)
- **whitelist_section:** §3.4

### R005
- **id:** R005
- **title:** ezdxf documentation
- **source_url:** https://ezdxf.readthedocs.io/
- **license:** MIT (project license 확인됨)
- **accessed_date:** 2026-04-25
- **accessed_by:** hakki
- **purpose:** DXF 사양 검증 (Autodesk 공식 문서와 cross-reference).
- **usage:** Verification only — algorithm 코드 직접 참조 안 함
- **whitelist_section:** §3.4

### R006
- **id:** R006
- **title:** Computational Geometry: Algorithms and Applications (de Berg et al., 3rd ed.)
- **source_url:** ISBN 978-3-540-77973-5 (학술 교과서)
- **license:** Copyrighted, fair use under educational study
- **accessed_date:** 2026-04-25
- **accessed_by:** hakki
- **purpose:** Bentley-Ottmann sweep, DCEL, planar graph face extraction 학술 알고리즘 학습.
- **usage:** Algorithm reference (학술 자산, 자체 Rust 구현)
- **whitelist_section:** §3.3

### R007
- **id:** R007
- **title:** rstar crate documentation
- **source_url:** https://docs.rs/rstar/
- **license:** Apache-2.0 / MIT (dual)
- **accessed_date:** 2026-04-25
- **accessed_by:** hakki
- **purpose:** R-tree spatial index. cad-geometry 의 SpatialIndex 구현에 사용.
- **usage:** Library dependency (link)
- **whitelist_section:** §3.4

### R008
- **id:** R008
- **title:** slotmap crate documentation
- **source_url:** https://docs.rs/slotmap/
- **license:** Zlib OR Apache-2.0 OR MIT
- **accessed_date:** 2026-04-25
- **accessed_by:** hakki
- **purpose:** DCEL key 자료구조. cad-geometry 의 Dcel 구현.
- **usage:** Library dependency (link)
- **whitelist_section:** §3.4

### R009
- **id:** R009
- **title:** proptest crate documentation
- **source_url:** https://docs.rs/proptest/
- **license:** Apache-2.0 OR MIT
- **accessed_date:** 2026-04-25
- **accessed_by:** hakki
- **purpose:** Property-based testing. tests/property/ 의 random plan generator.
- **usage:** Library dependency (link, dev only)
- **whitelist_section:** §3.4

---

## 명시적 비참조 (explicit non-access)

본 프로젝트가 **의도적으로 접근하지 않은** 자료. 향후 분쟁 시 입증 자료.

### NR001
- **title:** LibreDWG (GNU GPLv3+)
- **source_url:** https://www.gnu.org/software/libredwg/, https://github.com/LibreDWG/libredwg
- **license:** GPLv3 or later (Apache-2.0 비호환)
- **status:** **NOT ACCESSED** (영구 금지)
- **enforcement:** `scripts/scan_gpl_signatures.sh` 가 commit 시 자동 검증
- **note:** 본 프로젝트의 어떤 contributor 도 LibreDWG 소스코드를 열어보거나 다운로드한 사실 없음. 향후 동일 정책.

### NR002
- **title:** Autodesk RealDWG SDK
- **source_url:** https://www.autodesk.com/developer-network/platform-technologies/realdwg
- **license:** Proprietary, NDA 가입 후 라이선스
- **status:** **NOT ACCESSED**
- **note:** 본 프로젝트는 RealDWG SDK 가입/다운로드/사용한 사실 없음.

### NR003
- **title:** ODA Teigha SDK / ODA Drawings SDK
- **source_url:** https://www.opendesign.com/products/drawings
- **license:** Proprietary, ODA 멤버십 + NDA
- **status:** **NOT ACCESSED**
- **note:** 본 프로젝트는 ODA 멤버십 미가입. SDK 다운로드/사용 사실 없음.

### NR004
- **title:** Bentley MicroStation SDK
- **status:** **NOT ACCESSED**

### NR005
- **title:** AutoCAD ARX SDK 비공개 부분
- **status:** **NOT ACCESSED**
- **note:** Autodesk 가 공개한 일부 ObjectARX 헤더는 R001 에 포함. 비공개 SDK 부분은 별개.

---

## 자료 변경 이력

| 일자 | 항목 | 변경 | 사유 |
|---|---|---|---|
| 2026-04-25 | INDEX | 초기 9개 항목 + 5 NR 항목 등록 | Stage 1 빌드 직전 baseline |
| (추가 시 여기에 기록) | | | |

---

## 향후 추가 시 체크리스트

신규 자료 등록 시:
- [ ] 자료가 `01-clean-room-policy.md` whitelist 에 부합?
- [ ] 라이선스 명확히 확인?
- [ ] Apache-2.0 호환 또는 fair use 명확?
- [ ] 사용 목적 명시?
- [ ] R001~R0NN 다음 ID 할당?
- [ ] Wayback Machine 캡처 (URL 영속성)?
- [ ] commit 메시지에 본 로그 업데이트 명시?
