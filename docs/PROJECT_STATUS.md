# Project status — DEV-03

2026-10-29 JST. 시작 main/원격 main c07a6fe6c9f15a3b76e18179974a7db86ba8ab29, PR#3 merged 및 main CI40696324294 success 확인. clean tree에서 codex/dev-03-local-ai-runtime 생성. 기존94 요구 ID 유지, 신규095–110, 총110개. 사용자 cleanup 스크립트는 기존 hash를 보존하고 실행/변경/삭제하지 않았다. 신규 task 데이터는 E에 둔다.

구현: 실제 ONNX CPU 진단, 고정 Registry/ModelManifest, 명시 동의·HTTPS/hash/크기 제한, 전용 Python generation과 격리 smoke, v2 감독기/Windows Job Object, strict protocol, 자원 사전 검사, 결과 staging/검증/원본 재확인, 기존 journal과3언어 UI. 원본 PNG/PSD v1/v2와 master/gates는 변경하지 않는다. 216-byte 합성 가중치는 학습된 캐릭터 모델이 아니다.

검증된 기반: 깨끗한 DEV-02 전체 기준선 및 native recovery 통과. 실제 CPU 연산/PNG·PSD 원본 보존, 취소/kill/restart, 모델 변조 차단, 실제 RAM 한도 및 descendant 종료 시험 통과. 최종 acceptance 기록은 TEST_RUNS의 DEV-03 절과 docs/evidence/dev-03-local.json을 기준으로 한다. 코드 존재만으로 요구를 Verified로 승격하지 않는다.

제한: RTX2060 6144MiB 장치는 확인했으나 CUDA/DirectML/PyTorch 추론은 미검증·미설치. AI 캐릭터 분석/생성/rig/export/VBridger 실동작 미구현·미검증. Python audit guard는 OS 네트워크 sandbox가 아니며 물리적 offline/전체 DLL traffic 미검증. 같은 사용자 악성 변경 race/전원 차단/legacy dummy 전체tree/AI checkpoint는 DEV-28/29. Python3.12는 기존 실제 설치가 필요하며 깨끗한 PC용 bootstrap은 DEV-30.

최종 완료는 DEV-03 PR의 exact head CI·필수 리뷰/보호/충돌 검사 후 안전한 main 병합과 그 merge SHA의 main CI 성공을 모두 확인해야 한다. PR/merge SHA는 해당 변경 자체에 미리 쓸 수 없으므로 원격 PR 기록·최종 보고·로컬 release-verification 증거를 최종 권위로 사용한다. 강제/관리자 bypass는 금지한다.

DEV-04는 DEV_ROADMAP 마지막 시작 계약을 따른다. 아래 DEV-00~02 기록은 당시의 범위이며 현재 미지원 상태는 위 내용과 각 전용 문서가 우선한다.

---

# Project status — DEV-02

2026-10-28 JST. 시작 main 5d38dcb817e7f9d53c373ab584c280dc251154ce / CI40502328646 success, clean tree. 작업 브랜치 codex/dev-02-source-fidelity. 기존78개 ID 유지, 신규16개(079–094), 총94개. 사용자 artifacts/plan-c-cleanup.py SHA256 AB3D3C8D67157D1341E1FB25E71633A83516C98023E43849F80F24459BECB1A8를 확인하고 변경/실행/삭제하지 않았다. C 정리는 범위 밖이다.

구현: 원본 PNG/PSD capture/hash/immutable publication, known v1/v2, 별도 RGB16/ICC/sRGB8 preview, bounded PSD 구조/압축, 취소, native drop authority, 3언어/전문가 세부정보/자동 프로젝트 생성. 기존 저장·복구·index·job 구조를 유지한다. IMAGE_IMPORT/PSD_SUPPORT_MATRIX/COLOR_MANAGEMENT가 실제 지원 경계다.

기준선: 최초 실행은 작업 중 변경이 섞여 기준선으로 무효 처리했다. E 전용 DEV-01 추출본의 전체 suite가 통과했다. 새 이미지 tests 및 독립 psd-tools synthetic composite/codecs가 통과했다. 실제 native OS PSD 선택창→preview→layers→3언어→reload/hash가 통과했다. Windows 측정에서4000²8-bit peak181,014,528 bytes,4096²16-bit peak240,410,624 bytes;초기160MiB 작업 사본 한도를320MiB로 조정하여 기존40M 8-bit PNG 범위를 유지했다. 최종8000×5000은 peak445,517,824B/wall431ms,8001×5000은 image_too_large로 거부했다. 최종 로컬 acceptance suite는42 Rust/helper1,4contracts,2Python,3UI 및 format/build/clippy/docs audit를 통과했다. release40M, actual native recovery 및 native PSD file dialog/preview/reload/hash도 통과했다. 원격 PR/main CI 성공 전 최종 완료로 표시하지 않는다.

미해결: Photoshop-equivalent editable compositor/고급 mask·effects·smart object 렌더링, monitor/grayICC/customgAMA/P3 인증, PSB32·CMYK, 타일/격리decoder/fuzz/wholeprocesscleanup는 명시 범위 제한이다. AI 생성·rig·방송 호환 및 캐릭터 의미 비교는 후속 DEV이다. 신규 permanent principles079–081을 DEV04–24/32에 연결했다.

DEV-03는 DEV_ROADMAP 마지막 시작 지침을 따른다. 최종 PR 병합 및 그 SHA의 main CI 성공 전 DEV-02는 최종 완료가 아니다.

---

## 이전 DEV-01 기록

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


## DEV-02 release evidence

Local evidence: docs/evidence/dev-02-local.json; TEST_RUNS final acceptance. Final PR URL/checks/main merge SHA are remote release authority. No force/admin bypass is authorized. Tested tree must equal merged main tree, CI must succeed for that merge SHA, and local/remote main must match before completion is reported. Original CharacterProject v1 sample/schema and user cleanup script were preserved.

[DEV-02 PR #3](https://github.com/Raccoon26/Project-VTuber/pull/3), [exact-head checks](https://github.com/Raccoon26/Project-VTuber/pull/3/checks), [main workflow](https://github.com/Raccoon26/Project-VTuber/actions/workflows/ci.yml?query=branch%3Amain) are the final remote release evidence. Merge/main CI status must be read from GitHub; local implementation alone is not completion.

Final review fix: legacy Pascal layer names now use explicit MacRoman decoding with authoritative UTF16BE luni; uncertain regional fallback has3locale warning. Global extension count and total name bytes are bounded. Final42-test suite/release/native recovery and OS PSD input rerun are recorded in TEST_RUNS.

Actual40M native preview responsiveness was measured:972ms to displayed copy, max sampled rAF gap100.1ms/54 samples, cached3.8ms. Original8000×5000 remains unchanged while displayed copy1000×625. This is a synthetic machine-specific sample; five-person beginner/OS drop studies remain unverified.

## DEV-03 final local acceptance

Full53Rust/12Python/5contract/4UI suite, Windows release, expanded actual native runtime including installation cancel/kill, prior native recovery and40M preview all passed. Detailed scope/measurements/failures: TEST_RUNS and evidence/dev-03-local.json. New095–109 are Verified only within their stated tests/limits;110 remains Implemented until remote release. Original94IDs preserved; future character AI/rig/VBridger/GPU gates remain unverified. No uncommitted user file was staged and cleanup hash remains unchanged.

Remote release authority: [DEV-03 branch PR lookup](https://github.com/Raccoon26/Project-VTuber/pulls?q=is%3Apr+head%3Acodex%2Fdev-03-local-ai-runtime) and [main workflow](https://github.com/Raccoon26/Project-VTuber/actions/workflows/ci.yml?query=branch%3Amain). Final report supplies actual PR/merge SHA after checks. DEV-04 must verify these remote records rather than inferring completion from this local paragraph.
