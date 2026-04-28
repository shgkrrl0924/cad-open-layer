# Legal Package — CAD Open Layer

**Purpose:** Stage 1 빌드 시작 전 필수 운영 정책 문서 + 로그 템플릿. 법률 의견서(2026-04-25, 민후 법무법인) 권고에 따른 IP 리스크 완화 패키지.

**Status:** v1 (Stage 1 빌드 시작 전 변호사 최종 검토 후 ratify)

## 구성

| # | 파일 | 종류 | 목적 |
|---|---|---|---|
| 01 | `01-clean-room-policy.md` | Policy | 독립 구현 절차 정의. 가장 중요한 정책 |
| 02 | `02-reference-material-log.md` | Log (live) | 참조자료 출처·일자·접근자 기록. 매 신규 자료 접근 시 업데이트 |
| 03 | `03-no-gpl-contamination-policy.md` | Policy | LibreDWG 및 GPL 코드 접근 금지 룰 |
| 04 | `04-contributor-declaration-template.md` | Template | 기여자 진술서 양식. 각 contributor 가 첫 commit 전 서명 |
| 05 | `05-trademark-usage-guideline.md` | Guideline | DWG, AutoCAD, Autodesk 명칭 사용 기준 |
| 06 | `06-test-corpus-license-log.md` | Log (live) | 테스트 DXF 파일 라이선스 추적 |
| 07 | `07-oda-contact-log.md` | Log (live) | ODA 와의 접촉/자료 접근 범위 기록 |

## CI/Audit 통합

- `01`, `03`: `scripts/scan_gpl_signatures.sh` 가 자동 검증
- `02`: 신규 외부 자료 추가 시 commit 강제 (pre-commit hook)
- `04`: `scripts/verify_contributors.sh` 가 commit author 등록 여부 확인
- `06`: `tests/corpus/` 에 새 파일 추가 시 동기화 검증
- `07`: ODA 접촉 발생 시 commit 강제

## 변호사 검토 절차

1. 본 패키지 v1 작성 (현재 단계)
2. 민후 법무법인 송부 → 검토 (1주)
3. 피드백 반영 → v2
4. 변호사 ratification 서면 받기
5. Stage 1 빌드 첫 commit 전 GitHub 에 push
6. 향후 변경 시 변호사 재검토 후 v 증가

## Series A 실사 제출용

본 패키지 7개 문서 + 1쪽 의견서 SUMMARY (legal-prep.md 의 SUMMARY 섹션) = **IP 실사 증거 패키지**.

투자자 측 변호사가 이 패키지 받으면 일반적으로 통과 가능한 수준.
