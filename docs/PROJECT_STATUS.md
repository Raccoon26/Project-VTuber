# Project status — DEV-01

2026-10-27 JST. DEV-00은 PR #1, merge ac7e2573ed4a2086940842a01aecb7746fd72028, main CI 40314991998 성공을 다시 확인했다. DEV-01은 저장·복구 기반 확장이다. **로컬 검증과 GitHub main 최종 완료를 구분하며, 해당 PR 병합 및 main CI 성공 전 최종 완료로 취급하지 않는다.**

## 구현과 검증

- list v2가 정상/손상/권한/잠금/미지원 schema/검사 불가를 분리한다. 전역 루트 오류는 숨기지 않는다.
- 원본 hash까지 확인한 revision 이력, 변경 미리보기, 확인/취소, stale token 검사, 새 revision 복구. 정상·손상 과거 파일은 보존한다.
- 프로젝트별 OS lock, 예상 revision 검증, 불변 파일 공개. 실제 2프로세스 경합 및 owner kill을 시험했다.
- SQLite 파생 검색/상태 index. missing/corrupt/old/stale/manual add/move 재구축. index 실패가 파일 저장을 취소하지 않는다.
- job별 immutable journal, queued/running/terminal/interrupted/recovery_required. 재시작 시 기록을 검사하며 자동 재개하지 않는다.
- CharacterProject v1, 기존 sample, 독립 VBridger 프로필, PNG bytes, 더미 Worker, GPU 진단, 3언어, offline 광고, exporter 차단을 유지한다.
- Rust28/계약4/Python2/UI2와 실제 Windows app kill/restart/recovery를 검사했다. 최종 증거/로그는 TEST_RUNS 및 evidence/dev-01-local.json을 기준으로 한다.
- 기존62 + 신규16 = 78 요구사항을 감사하고 전체 DEV00–33 계획을 유지했다.

## 남은 위험과 후속 처리

| 항목 | 현재 한계 | 담당/재검증 |
|---|---|---|
| 전원 차단·타 FS | NTFS 프로세스 kill과 구별; disk flush가 전원 내구성 전체를 보장하지 않음 | DEV-28 fault rig/타 FS 실험 |
| 악의적인 같은 사용자 race | reparse 검사와 OS lock은 cooperative writer 보호, 완전한 handle-relative sandbox 아님 | DEV-29 |
| 대형 프로젝트 성능 | full history/source hash scan, 10,000 revision 한도, UI 작업 취소는 향후 | DEV-02/28 profiling/증분 검사 |
| journal retention | append-only 기록, 한도 초과 시 명시 오류 | DEV-03/28 안전한 compaction/archival |
| 전체 Worker tree/AI checkpoint | OS Job Object 미구현, interrupted는 orphan 없음의 보증 아님 | DEV-03/28 |
| 캐릭터 제작 기능 | AI 생성·2D/3D rig·export·전문가 품질 미구현 | DEV-04–24/32 |
| Live2D/VBridger | 자동 출력 권리·실제 preset/runtime 인증 미확인 | DEV-11/24 |
| 원본 포맷/색 | PNG64MiB/40M, PSD·색 renderer 없음 | DEV-02 |
| 6GB GPU/모델 권리 | 대형 모델 표준 요구 미달, Hunyuan 지역/RigAnything 비상업 제한 | DEV-03/12/29 |
| 배포/광고 | Python 별도 필요, unsigned exe, 광고 계약 없음 | DEV-27/30 |
| 사용자 검증 | OS file picker/전체 screen reader/RTL 미실행 | DEV-02/26 |

## Release evidence

작업 브랜치 codex/dev-01-project-resilience. 원격 PR URL/최종 head checks/merge 결과는 아래에 연결하며, 병합 시 보호 규칙·필수 리뷰·충돌을 확인하고 관리자 강제 옵션을 쓰지 않는다. 병합 후 main CI 성공, 로컬/원격 SHA 일치, 테스트한 tree와 병합 tree 동일성을 확인한다. 신규 작업 파일/cache/temp/합성 데이터는 E 프로젝트에 둔다. 실제 사용자 데이터와 raw 로컬 로그는 GitHub에 올리지 않는다.

[DEV-01 PR #2](https://github.com/Raccoon26/Project-VTuber/pull/2), [최종 head 검사](https://github.com/Raccoon26/Project-VTuber/pull/2/checks), [병합 후 main CI](https://github.com/Raccoon26/Project-VTuber/actions/workflows/ci.yml?query=branch%3Amain)를 완료 판정의 원격 증거로 사용한다. PR의 merged 상태와 merge SHA, 그 SHA의 main CI success를 모두 확인한다. merge SHA는 자신의 commit 안에 미리 쓸 수 없으므로 PR 기록과 최종 보고 및 로컬 release-verification 증거에 남긴다.

DEV-02는 DEV_ROADMAP의 정확한 시작 지침과 STORAGE_RECOVERY/MIGRATION_STRATEGY를 읽고 baseline을 실행한 뒤 PSD/고해상도/ICC 계약을 설계한다.
