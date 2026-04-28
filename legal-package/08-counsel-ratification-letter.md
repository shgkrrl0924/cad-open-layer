# 법률의견 확인서 (Counsel Ratification Letter)

**수신:** hakki (shgkrrl0924@gmail.com)
**작성자:** 법무법인 [●], 담당 변호사 [●], 대한변호사협회 등록번호 [●]
**작성일:** 2026-04-26
**제목:** CAD Open Layer 프로젝트의 clean-room 구현, 오픈소스 라이선스 분리 및 상표 사용 정책에 관한 법률의견 확인서

---

## 1. 확인 목적

본 확인서는 hakki가 추진 중인 CAD Open Layer 프로젝트와 관련하여, 당 법무법인이 제공한 법률 검토 결과 및 프로젝트의 IP 리스크 완화 계획을 확인하기 위한 것이다.

CAD Open Layer 프로젝트는 Apache License 2.0 기반의 Rust/WebAssembly 오픈소스 라이브러리로서, 개발자 및 AI 에이전트가 CAD 파일 포맷, 특히 DXF 및 향후 DWG 파일을 프로그래밍 방식으로 읽고 쓸 수 있도록 하는 것을 목적으로 한다.

본 확인서는 투자 실사, 파트너십 검토, 내부 컴플라이언스 검토 및 Stage 1 개발 착수 여부 판단을 위한 참고자료로 제공된다.

## 2. 검토 자료

당 법무법인은 본 확인서 작성과 관련하여 아래 자료를 검토하였다.

1. CAD Open Layer 프로젝트 개요서
2. SSAFY-unknown-design-20260423-172821.md 설계 문서
3. Clean-room implementation 계획
4. No-GPL-contamination 정책 초안
5. ODA 공개 사양 문서 사용 계획
6. Autodesk DXF 공개 문서 사용 계획
7. LibreDWG 비참조 정책
8. 제품명, 문서, README, 마케팅 문구 초안
9. 테스트 코퍼스 라이선스 관리 계획
10. C&D 수령 시 대응 프로세스 초안

## 3. 전제 사실 (10개)

[원문 §3 모든 전제 그대로 적용]

## 4. 확인 사항 요약

### 4.1 Clean-room 구현 절차
한국 저작권법상 호환성 확보 목적의 독립 구현은 허용 가능. 8개 절차 (Reference Material Log, 구현자별 자료 접근 기록, GPLv3 코드 접근 금지, NDA/SDK 자료 접근 금지, 자체 생성 테스트 파일 우선, 외부 테스트 파일 라이선스 기록, commit log 유지, contributor declaration 징구) 갖추는 것이 적절.

### 4.2 ODA 자료 및 DWG 구현
- ODA 공개 PDF 참고 시 이용조건/저작권 고지/접근 경로 기록
- ODA 멤버십·SDK·NDA 자료는 별도 검토 전 접근 금지
- ODA 표현물 복제 금지
- DWG 바이너리 구현은 별도 legal gate 후

### 4.3 LibreDWG / GPLv3 분리
- LibreDWG 코드 repo 비포함
- 번역/이식 금지 + 함수명/구조체명/알고리즘 모방 금지
- 사양/학습/참고 자료로도 사용 금지
- contributor declaration 으로 확인
- 테스트 파일도 라이선스 확인된 것만

### 4.4 "DWG" 상표 및 문구
**허용:** "compatible with the DWG file format", "works with DWG files", "supports reading and writing .dwg and .dxf files", + 상표 고지문 첨부.

**금지:** OpenDWG, DWG API, DWG Cloud, DWG Open Layer, dwgopenlayer.com, dwg-rust, Official DWG-compatible logo, RealDWG replacement, TrustedDWG compatible 등 브랜드 위치 사용.

**권장 고지문:** 
> DWG is the native file format for Autodesk AutoCAD software and is a trademark of Autodesk, Inc.
> CAD Open Layer is an independent project and is not affiliated with, endorsed by, sponsored by, or certified by Autodesk, Inc.

### 4.5 C&D 대응 체계
6개 절차: 직접 회신 금지 → 법률대리인 전달 → 문제 식별 → 증거 보존 → 임시 수정 (권리 침해 인정 표현 금지) → 청구 근거별 분리 대응 (저작권/상표/계약/NDA/영업비밀/부정경쟁) → takedown 시 mirror·통신 계획.

## 5. 결론적 확인

8개 조건 (Stage 1 DXF-first / DWG 별도 게이트 / GPL·SDK·NDA 미사용 / 독립 작성 / 기록 보존 / "DWG" 설명적 사용만 / Apache-2.0 + GPLv3 비혼입 / 외부 고객 파일 비공개) 충족 시 **Stage 1 DXF-first 개발 착수에 중대한 즉시 중단 사유 없음**으로 확인.

단, 본 확인서는:
- 제3자 청구 무발생 보증 아님
- 미국·EU·기타 해외법 / 특허 / 영업비밀 / 수출통제 / 개인정보 / 보안규제 / 세무·투자계약 별도 검토 대상

## 6. 제한 및 유보 8개

[원문 §6 그대로 적용]

## 7. 권장 후속 조치 — repository 운영 문서

```
/legal-package/01-clean-room-policy.md            ✓ 작성 완료
/legal-package/02-reference-material-log.md       ✓ 작성 완료
/legal-package/03-no-gpl-contamination-policy.md  ✓ 작성 완료
/legal-package/04-contributor-declaration-template.md  ✓ 작성 완료
/legal-package/05-trademark-usage-guideline.md    ✓ 작성 완료
/legal-package/06-test-corpus-license-log.md      ✓ 작성 완료
/legal-package/07-oda-contact-log.md              ✓ 작성 완료 (변호사 권장 oda-sdk-and-nda-access-log.md 와 동일 목적)
/legal-package/cd-response-playbook.md            ⚠ TODO (변호사 §4.5 6단계 절차 기반)
```

7/8 완료. 마지막 cd-response-playbook.md 만 추가 작성 권장.

## 8. 결재 / 효력

- **변호사 결재 (수기 서명 또는 디지털 서명):** [●]
- **법무법인 직인:** [●]
- **수신 확인 (hakki):** [●]
- **유효 기간:** 작성일로부터 12개월. Stage 2 진입 또는 중대 사실 변동 시 재검토.

---

## 운영상 확인 (hakki 기록)

이 확인서를 받음으로써 **Stage 1 빌드 착수 가능 (Pre-Stage-1 블로커 B 해제).**

남은 블로커:
- ✓ B (Legal): 본 확인서로 해제
- ⏳ A (Demand): Maket Patrick 콜 후 PoC 진행 중 (단 Stage 1 빌드는 PoC 결과 안 기다리고 병행 가능 — 변호사 의견서가 "착수 OK" 이므로)
- ⏳ C (Funding): Series A 라운드는 Stage 1 빌드 진행하며 동시 진행

**다음 단계:** 본 확인서를 cad-open-layer/legal-package/ 디렉토리에 복사 + Series A 실사 패키지의 핵심 증거 자료로 보관.
