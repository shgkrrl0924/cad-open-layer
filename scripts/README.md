# Scripts

CI 자동화 + clean-room 정책 강제 스크립트.

| Script | 목적 | 호출 시점 |
|---|---|---|
| `scan_gpl_signatures.sh` | LibreDWG 등 GPL 코드 패턴 검출 | 모든 PR |
| `verify_contributors.sh` | commit author 의 declaration 등록 확인 | 모든 PR |
| `check_corpus_log_consistency.sh` | tests/corpus/ 의 파일이 license log 에 등록 확인 | 모든 PR |

## 위반 시 commit override

False positive 발생 시:

```
git commit -m "feat: ...

GPL-OVERRIDE: pattern 'decode_R2000' is part of standard DWG R2000 spec terminology, not LibreDWG-specific"
```

`GPL-OVERRIDE: ` 줄이 있으면 maintainer review 후 머지. 단 모든 override 는 audit log 에 남음.
