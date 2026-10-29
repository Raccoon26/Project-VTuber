# Requirements and coverage audit

기존94개 ID와 원문 요구를 유지하고 DEV-03 REQ-095–110을 추가한다. Verified는 해당 증거 범위이며 GPU·캐릭터 제작·방송 인증을 뜻하지 않는다. 원격 최종 완료는 PR/main CI 별도 확인이다. 정본과 루트 JSON mirror를 자동 검사한다.

| ID | 요구사항 | 우선 | 담당 DEV | 모듈 | 검증 | 현재 상태 | 미해결 위험 | 테스트·증거 |
|---|---|---|---|---|---|---|---|---|
| REQ-001 | 제작 단계에서 리깅 완료, 방송 추적 앱과 분리 | P0 | DEV-00 | M01 | architecture 검토 | Verified | 실제 생성은 후속 | docs/ARCHITECTURE.md |
| REQ-002 | CPU/GPU/RAM 로컬 핵심 제작, 유료 API 필수 금지 | P0 | DEV-03, DEV-12 | M04 | offline 실제 모델 inference | Partially Implemented | 실제 소형 ONNX CPU 진단 구현; 캐릭터 제작 AI/GPU는 후속 | tests/contracts.test.ts, src-tauri/src/runtime_tests.rs, workers/test_runtime.py |
| REQ-003 | 이미지·모델 무단 외부 전송 금지 | P0 | DEV-00, DEV-29 | M04 | network traffic/권한 감사 | Partially Implemented | 전체 OS traffic 측정 미실행 | docs/SECURITY.md |
| REQ-004 | 첫 다운로드 뒤 offline, 검증된 CPU/경량 대안 | P0 | DEV-03, DEV-31 | M04 | 모델 설치 후 disconnected 실행 | Partially Implemented | 설치 후 실제 CPU 추론 및 Python socket 거부 시험; 물리적 네트워크 단절/전체 OS traffic 미검증 | src-tauri/src/worker.rs, src-tauri/src/runtime_tests.rs, workers/test_runtime.py |
| REQ-005 | GPU/VRAM/backend 진단과 지원 불명 상태 | P0 | DEV-00, DEV-03 | M05 | 장치 query와 실제 tensor smoke 분리 | Partially Implemented | managed ONNX CPU 실연산 검증; RTX2060 장치 감지는 CUDA 검증과 분리 | src-tauri/src/gpu.rs, src-tauri/src/runtime_tests.rs, workers/test_runtime.py |
| REQ-006 | 대형 AI 모델 설치·버전·무결성·권리 관리 | P0 | DEV-03 | M04 | consent/download/hash/offline fixtures | Partially Implemented | 소형 모델 registry/승인/무결성/격리 설치 기반; 대형 외부 모델 미설치 | docs/MODEL_STRATEGY.md, src-tauri/src/runtime_tests.rs, workers/test_runtime.py |
| REQ-007 | 원본 파일 불변 및 해시 검사 | P0 | DEV-00, DEV-02, DEV-01 | M03 | 입력/저장 bytes 동일 및 변조 거부 | Verified | 지원 PNG/PSD 원본 bytes/hash 보존; 악의적 동일 사용자 race 완전 방어 제외 | src-tauri/src/storage.rs |
| REQ-008 | 원해상도·색 정보·손실 압축/임의 축소 금지 | P0 | DEV-00, DEV-02 | M03 | 원본 byte 보존과 ICC/render 비교 | Partially Implemented | DEV-02 RGB ICC/16-bit 표시 사본 지원; 모든 profile/monitor 인증은 미검증 | src-tauri/src/storage.rs |
| REQ-009 | 원본/generated/manual 구분과 수정 가능 중간 결과 | P0 | DEV-01, DEV-06, DEV-14 | M02 | provenance DAG/revision round-trip | Partially Implemented | 원본·preview·진단 artifact 분리; 실제 캐릭터 master/수정 승인 흐름은 후속 | docs/MODEL_CONTRACT.md |
| REQ-010 | 시각·구조 비교 및 얼굴/눈/의상/색/머리/장식 충실도 | P0 | DEV-04, DEV-20 | M15 | 같은 조건의 render+전문가 평가 | Planned | 전문가급 실증 없음 | docs/QUALITY_GATES.md |
| REQ-011 | 측후면 추정의 불확실성 표시, 완벽 복원 보장 금지 | P0 | DEV-06, DEV-13 | M08 | inferred mask/turntable review | Planned | 관찰 불가능 정보 | docs/MODEL_STRATEGY.md |
| REQ-012 | 품질 부족 모델 정식 완성/export 금지 | P0 | DEV-00, DEV-20 | M15 | unknown Gate/export 차단 | Partially Implemented | 실제 평가 엔진 후속 | tests/contracts.test.ts |
| REQ-013 | 정밀·비대칭 얼굴과 눈꺼풀/눈동자 | P0 | DEV-09, DEV-17 | M12 | 단독/복합 face sweep | Planned | style별 topology | docs/DEV_ROADMAP.md |
| REQ-014 | 입술·웃음·턱·볼·입꼬리 복합 표현 | P0 | DEV-09, DEV-17 | M12 | 조합 변형·관통 평가 | Planned | corrective 상호작용 | docs/DEV_ROADMAP.md |
| REQ-015 | 얼굴 회전 중 표정 유지 | P0 | DEV-09, DEV-17 | M12 | yaw/pitch와 표정 교차 테스트 | Planned | 2D 깊이 추정 | docs/DEV_ROADMAP.md |
| REQ-016 | 휴머노이드 전신 skeleton/skinning/관절 보정 | P0 | DEV-15, DEV-16 | M11 | joint/weight/humanoid validation | Planned | 생성 topology 부적합 | docs/TECH_RESEARCH.md |
| REQ-017 | 춤·회전·점프/어깨/허리/골반/다리/발접지 | P0 | DEV-18 | M13 | 고정 dance battery | Planned | 전문가 평가 필요 | docs/QUALITY_GATES.md |
| REQ-018 | 머리·옷 물리/충돌·메시 관통 최소화 | P0 | DEV-10, DEV-19 | M14 | dt/penetration stress | Planned | 대상 physics 차이 | docs/DEV_ROADMAP.md |
| REQ-019 | 방송 실시간 성능에 적합한 출력 최적화 | P0 | DEV-23, DEV-31 | M18 | 앱별 fps/RAM/VRAM/frame budget | Planned | 품질/LOD tradeoff | docs/DEV_ROADMAP.md |
| REQ-020 | 제작 시작 시 2D/3D 및 대상 방송 앱 선택 | P0 | DEV-00, DEV-25 | M01 | UI profile 선택 | Partially Implemented | 초보자 전체 제작 미구현 | tests/ui/workspace.spec.ts |
| REQ-021 | VTube Studio 호환 출력, editable/runtime 구분 | P0 | DEV-11, DEV-24 | M18 | Editor 재편집+VTS 실행 | Blocked | 공식 자동 authoring/권리 미확인 | docs/TECH_RESEARCH.md |
| REQ-022 | VBridger 필수 독립 2D profile/고급 parameters | P0 | DEV-00, DEV-11, DEV-24 | M19 | 정적 mapping+VTS 실동작 | Partially Implemented | 공식 preset fixture 미확보 | tests/contracts.test.ts |
| REQ-023 | VBridger VMC/VRM 실제 범위·설정 보존 | P0 | DEV-00, DEV-24 | M19 | receiver별 얼굴 출력/재시작 | Partially Implemented | head/eye/body는 VBridger VMC 미지원 | docs/VBRIDGER_REQUIREMENTS.md |
| REQ-024 | VSeeFace VRM0/VSFAvatar 및 표정·물리 | P0 | DEV-23, DEV-24 | M19 | 실제 앱 load/표정/춤 | Planned | VRM1 미지원 | docs/COMPATIBILITY_MATRIX.md |
| REQ-025 | Warudo/VNyan 대상별 표정·물리·확장 | P0 | DEV-23, DEV-24 | M19 | 각 앱/version runtime | Planned | custom shader/blueprint 손실 | docs/COMPATIBILITY_MATRIX.md |
| REQ-026 | Inochi Session/PrprLive/기타 주요 앱 조사·인증 | P0 | DEV-11, DEV-24 | M19 | 앱별 format/parameter/runtime | Partially Implemented | 현재 버전/유통 일부 미확인 | docs/COMPATIBILITY_MATRIX.md |
| REQ-027 | VRChat/Unity/GLB/FBX 출력과 제한 | P0 | DEV-23, DEV-24 | M18 | validator+Unity SDK build/test | Planned | 계정·SDK·플랫폼 제한 | docs/COMPATIBILITY_MATRIX.md |
| REQ-028 | 공통 master 보존·Export Profile별 손실 기록 | P0 | DEV-00, DEV-23 | M18 | export 전후 master hash | Partially Implemented | master 생성 미구현 | src/contracts.ts |
| REQ-029 | 파일 열기만으로 완전 호환 인증 금지 | P0 | DEV-00, DEV-24 | M19 | per-feature/version 증거 검사 | Partially Implemented | runtime 증거 없음 | tests/contracts.test.ts |
| REQ-030 | 초보자 6단계와 전문가 모드 분리 | P0 | DEV-00, DEV-25 | M01 | 초보자 usability study | Partially Implemented | 모드·기반 화면만 구현 | src/main.tsx |
| REQ-031 | 친숙한 오류 설명/복구 경로·자동 수정 | P0 | DEV-00, DEV-21, DEV-28, DEV-01 | M16 | error UX/repair rollback | Partially Implemented | 선택 복구 UI 구현; AI 자동 repair는 후속 | src/i18n.ts |
| REQ-032 | 핵심 무료·최하단 배너 최대1개·강제광고 금지 | P0 | DEV-00, DEV-27 | M21 | UI/계약/광고 수량 검사 | Partially Implemented | 현재 배너0 | src/contracts.ts |
| REQ-033 | 공급업체 교체·desktop/Japan 계약·직접 스폰서 | P0 | DEV-00, DEV-27 | M21 | 계약/SDK 지원 audit | Partially Implemented | 미계약, 일본·지급 미확인 | docs/MONETIZATION.md |
| REQ-034 | 광고 실패/offline에도 제작 정상·GPU/데이터 격리 | P0 | DEV-00, DEV-27 | M21 | network timeout/권한/traffic | Partially Implemented | offline null adapter만 구현 | tests/contracts.test.ts |
| REQ-035 | 일본어·영어·한국어 UI/오류/quality/i18n | P0 | DEV-00, DEV-26 | M20 | key parity/언어 전환 | Partially Implemented | 튜토리얼·전체 도움말 후속 | tests/contracts.test.ts |
| REQ-036 | 새 언어·RTL·긴 문자열 고려 | P0 | DEV-26 | M20 | pseudolocale/RTL/zoom | Planned | RTL 실동작 미검증 | docs/LOCALIZATION.md |
| REQ-037 | 시작 시 저장소·OS·CPU/GPU/RAM/disk/tool 조사 | P0 | DEV-00 | M22 | 로컬 명령 증거 | Verified | 설치 부재는 탐색 범위 한정 | docs/ENVIRONMENT_REPORT.md |
| REQ-038 | 기존 코드·사용자 변경 보존, main 직접 변경 금지 | P0 | DEV-00 | M02 | git diff/status/branch 확인 | Verified | DEV-03 별도 브랜치; 사용자 cleanup 파일 hash 보존 및 미실행 | docs/ENVIRONMENT_REPORT.md |
| REQ-039 | 기술 선택 비교·ADR·22개 모듈 경계 | P0 | DEV-00 | M01 | architecture/decision coverage audit | Verified | 후속 실험으로 교체 가능 | docs/ARCHITECTURE.md |
| REQ-040 | 공개 schema·모델별 adapter·2D/3D 독립 | P0 | DEV-00, DEV-03 | M04 | schema/contract tests | Partially Implemented | CharacterProject v1/v2 유지; ModelManifest v1/Worker v2 추가, 실제 캐릭터 adapter는 후속 | docs/MODEL_CONTRACT.md |
| REQ-041 | Live2D/Inochi/Ayatsuri 편집·runtime·권리 조사 | P0 | DEV-00, DEV-11 | M18 | primary evidence+실험 계획 | Partially Implemented | 실제 변환 미실행 | docs/TECH_RESEARCH.md |
| REQ-042 | 복수 로컬3D 모델 비교·동일 조건 실험 설계 | P0 | DEV-00, DEV-12 | M10 | source/license/VRAM 및 benchmark | Partially Implemented | 품질 실측 미실행 | docs/MODEL_STRATEGY.md |
| REQ-043 | 자동 rigging 후보와 춤 후처리 구조 | P0 | DEV-00, DEV-15, DEV-18 | M11 | license review/pose battery | Partially Implemented | RigAnything 상업 경로 제외 | docs/TECH_RESEARCH.md |
| REQ-044 | 오류ID/재현/근본원인/수정/회귀 기록 | P0 | DEV-00 | M22 | failure register audit | Verified | 불명 원인은 불명 표시 | docs/FAILURE_REGISTER.md |
| REQ-045 | 반복 실패는 증거→가설→최소 실험 순서 | P0 | DEV-00 | M22 | 재현/회귀 기록 | Verified | 일회성 tooling 실패도 구분 | docs/FAILURE_REGISTER.md |
| REQ-046 | 테스트 명령·환경·결과·로그·미실행 구분 | P0 | DEV-00 | M22 | test ledger | Verified | 최종 CI 결과 별도 | docs/TEST_RUNS.md |
| REQ-047 | Gate A–F 네 상태와 사람 품질 검토 | P0 | DEV-00, DEV-20 | M15 | schema/validator/전문가 review | Partially Implemented | 모델 gates 모두 unverified | docs/QUALITY_GATES.md |
| REQ-048 | Tauri React TS Rust 실행 가능한 Windows 기반 | P0 | DEV-00 | M01 | frontend/native build+실행 | Verified | clean-machine installer는 후속 | src-tauri/src/main.rs |
| REQ-049 | Python interface/dummy progress/cancel/failure | P0 | DEV-00 | M04 | 실제 subprocess 단위·통합 | Verified | 기존 dummy v1 유지; 신규 v2만 OS Job Object 보호, 전체 legacy/AI checkpoint는 DEV-28 | src-tauri/src/worker.rs |
| REQ-050 | CharacterProject 생성·저장·다시 열기·예제 | P0 | DEV-00, DEV-01 | M02 | round-trip/conflict/reopen | Verified | v1 저장/다시 열기/손상 격리/선택 복구 검증; 전원 차단 제외 | src-tauri/src/storage.rs, src-tauri/src/resilience_tests.rs |
| REQ-051 | 테스트 실행 스크립트/CI/의존성 lock | P0 | DEV-00 | M22 | clean npm ci/cargo locked | Verified | DEV-02 main CI40696324294 success; DEV-03 해당 PR/main CI 별도 확인 | scripts/test.ps1 |
| REQ-052 | 잘못된 이미지·metadata·폭탄·경로 공격 | P0 | DEV-00, DEV-02, DEV-29 | M03 | malformed/limit/path regressions | Partially Implemented | 전면 fuzz/TOCTOU 미해결 | src-tauri/src/storage.rs |
| REQ-053 | AI 체크포인트·plugin·출처·license·checksum | P0 | DEV-03, DEV-29 | M04 | manifest audit/quarantine | Partially Implemented | 합성 ONNX와 고정 wheel 출처/license/hash 검사; 외부 checkpoint/plugin 미허용 | docs/SECURITY.md, src-tauri/src/runtime_tests.rs, workers/test_runtime.py |
| REQ-054 | Worker crash/OOM/cancel·저장 실패·강제종료 복원 | P0 | DEV-00, DEV-28, DEV-01 | M22 | fault injection/soak | Partially Implemented | 신규 provider 실제 RAM 한도/timeout/descendant kill 시험; GPU OOM/전원 차단 미검증 | src-tauri/src/worker.rs, docs/JOB_PERSISTENCE.md, src-tauri/src/runtime_tests.rs, workers/test_runtime.py |
| REQ-055 | 불필요한 관리자 권한 없이 설치/실행 | P0 | DEV-00, DEV-30 | M01 | 일반 사용자 실행/clean install | Partially Implemented | portable interpreter 번들 후속 | docs/SECURITY.md |
| REQ-056 | 필수 문서/원본 요구사항/DEV00–33 로드맵 | P0 | DEV-00 | M22 | 문서 coverage audit | Verified | 최종 링크/상태 audit 필요 | scripts/audit-docs.mjs |
| REQ-057 | 각 요구사항 ID/우선/DEV/모듈/검증/상태/위험/테스트 | P0 | DEV-00 | M22 | 요구사항 ledger validator | Verified | 제품 요구 Verified와 구분 | docs/requirements.json |
| REQ-058 | 모든 DEV 시작/종료 누락·회귀 감사 | P0 | DEV-00, DEV-33, DEV-01 | M22 | CI와 PR checklist | Implemented | 후속 DEV 지속 준수 필요 | docs/DEV_ROADMAP.md |
| REQ-059 | DEV 후 commit/push/PR/CI/조건충족 merge/main 검증 | P0 | DEV-00, DEV-01 | M22 | GitHub checks/protection/merge SHA | Implemented | DEV-00~02 PR/main 검증 완료; DEV-03은 해당 PR와 main CI 성공 후 완료 | docs/PROJECT_STATUS.md |
| REQ-060 | 실패·충돌·권한·필수 리뷰 미충족 시 강제 merge 금지 | P0 | DEV-00, DEV-01 | M22 | GitHub rules/checks+head 검증 | Implemented | 관리자 bypass 사용 금지 | docs/PROJECT_STATUS.md |
| REQ-061 | 승인 없는 대형 모델/장시간GPU/유료/외부 전송 금지 | P0 | DEV-00, DEV-03 | M04 | 작업/모델 inventory audit | Verified | 명시 개발 준비로 약26.8MiB 고정 CPU wheels만 설치; 대형 모델/GPU 부하/유료/사용자 데이터 전송 없음 | docs/ENVIRONMENT_REPORT.md |
| REQ-062 | 후속 지침: 신규 출력·cache·temp·앱 데이터는 E | P0 | DEV-00 | M02 | env/path/filesystem 확인 | Partially Implemented | 지침 전 C cache 이력·OS 자체 로그 제외 | scripts/env.ps1 |
| REQ-063 | 손상 프로젝트 개별 격리 및 전역 오류 분리 | P0 | DEV-01 | M02 | 개별 상태/정상 목록/전역 열거 오류 시험 | Verified | 검사 한도·공유 폴더 경합은 명시 오류 | src-tauri/src/resilience_tests.rs |
| REQ-064 | revision JSON/schema/ID/번호/원본 hash/path 검사 | P0 | DEV-01 | M02 | 과거/최신 손상·미래 schema·blob 변조 시험 | Verified | 복원 불가 원본은 정상 복구 후보 아님 | src-tauri/src/resilience_tests.rs |
| REQ-065 | 사용자 선택·차이 확인·취소 후 새 revision 복구 | P0 | DEV-01 | M01 | UI 확인 차단/취소/명시 복구 및 stale token | Verified | 화면은 저장 메타데이터 preview, 렌더 아님 | tests/ui/recovery.spec.ts |
| REQ-066 | 복구 전후 원본·정상/손상 이력 보존 | P0 | DEV-01 | M02 | recover kill before/after publish와 bytes/hash 비교 | Verified | 전원 차단 수준은 미검증 | src-tauri/src/resilience_tests.rs |
| REQ-067 | SQLite 파생 검색 인덱스와 없음/손상/구버전/stale 재구축 | P0 | DEV-01 | M02 | 파일 재검사/SQL search/manual add/move/cache fail | Verified | snapshot는 순간적으로 오래될 수 있어 open 재검증 | src-tauri/src/resilience_tests.rs |
| REQ-068 | 프로젝트 OS 잠금·동시 저장·stale revision 충돌 | P0 | DEV-01 | M02 | Windows 실제 두 프로세스 경쟁/owner kill/lock 재획득 | Verified | 로컬 NTFS 검증; 타 FS/네트워크 미검증 | src-tauri/src/resilience_tests.rs |
| REQ-069 | 20개 실패 주입 사례와 실제/모의/미실행 구분 | P0 | DEV-01 | M22 | 테스트 매트릭스와 실제 데이터 보존 assertion | Verified | StorageFull은 주입; 실제 디스크 소진·전원 차단 미실행 | docs/TEST_RUNS.md |
| REQ-070 | queued/running/terminal/interrupted/recovery_required 영속화 | P0 | DEV-01 | M04 | journal 저장 실패·손상·owner kill·restart·실제 Python | Verified | 신규 v2 작업 tree 회수 시험; legacy dummy 전체 tree 및 AI checkpoint 재개는 DEV-28 | src-tauri/src/resilience_tests.rs |
| REQ-071 | job 기록과 프로젝트 revision 분리 | P0 | DEV-01 | M04 | progress/complete/cancel 후 manifest 번호 불변 | Verified | 장시간 journal 압축/retention은 후속 | src-tauri/src/resilience_tests.rs |
| REQ-072 | CharacterProject v1 및 미래 schema 안전 거부 | P0 | DEV-01 | M02 | DEV-00 sample/unknown field/future schema 격리 | Verified | DEV-02 PSD v2 별도 revision/기존 v1 보존; DEV-03 project migration 없음 | docs/MIGRATION_STRATEGY.md |
| REQ-073 | 복구·충돌·중단 안내 3언어 및 키보드 조작 | P0 | DEV-01 | M20 | key parity/ko-ja-en 전환/checkbox Space/cancel | Verified | 전문가/초보자 전체 사용성 평가는 후속 | tests/ui/recovery.spec.ts |
| REQ-074 | DEV-00 모든 구현 기능 회귀 보존 | P0 | DEV-01 | M22 | 기존 suite와 native-smoke를 확장 suite에 포함 | Verified | 기존 미구현 AI/export를 완료로 바꾸지 않음 | scripts/native-recovery.mjs |
| REQ-075 | DEV-01 reparse/접근 거부/경로 안전성 | P0 | DEV-01 | M02 | Windows 실제 junction/배타 handle 및 기존 경로 검사 | Verified | 악의적 동일 사용자 TOCTOU 완전 방어 제외 | src-tauri/src/resilience_tests.rs |
| REQ-076 | 기존62+신규 요구 전수 감사 및 증거 문서 동기화 | P0 | DEV-01 | M22 | ID 불변/중복/정본-루트 mirror/문서 링크 audit | Verified | Verified는 해당 시험 범위에만 한정 | scripts/audit-docs.mjs |
| REQ-077 | 실제 Tauri 종료·재시작·복구 검증 | P0 | DEV-01 | M01 | native Windows IPC와 OS process kill/rehash | Verified | DEV-01 native 복구, DEV-02 실제 OS PSD picker 검증; DEV-03 runtime native는 합성 seed helper 사용 | scripts/native-recovery.mjs |
| REQ-078 | DEV-01 PR/CI/보호 조건/main 병합 및 후속 CI | P0 | DEV-01 | M22 | PR checks/merge SHA/remote-local tree/main CI | Verified | DEV-01 PR#2 / main CI40502328646 success; proof applies to that merge only | docs/PROJECT_STATUS.md |
| REQ-079 | 캐릭터 재창조 금지 및 Observed/Inferred/Unknown/User Approved 근거 추적 | P0 | DEV-02, DEV-04, DEV-06, DEV-12, DEV-20, DEV-32 | M03 | 원본 anchor/provenance 및 승인 없는 관찰 영역 변경 비교 | Partially Implemented | 의미 특징 인식·AI 정체성 비교는 후속 | docs/CHARACTER_IDENTITY_POLICY.md |
| REQ-080 | 초보자 최소 클릭 입력·미리보기·자동 제작 UX | P0 | DEV-02, DEV-25, DEV-32 | M01 | 파일 선택부터 preview까지 사용자 흐름/실사용자 검사 | Partially Implemented | 전체 자동 제작·초보자 실사용자 평가는 후속 | src/SourcePreview.tsx |
| REQ-081 | 방송용 모델 자동 완성 및 품질·대상 앱 검증 후 출력 | P0 | DEV-04, DEV-24, DEV-32 | M18 | 원본→rig→quality→대상 runtime 종단 검증 | Planned | 진단 연산은 실제 캐릭터 AI/rig/export가 아님; 완성 모델 생성 미구현 | docs/CHARACTER_IDENTITY_POLICY.md |
| REQ-082 | PNG 유형·16-bit·ICC·투명도 원본 보존 입력 | P0 | DEV-02 | M03 | 유형별 bytes/hash/pixel 검사 | Verified | 색 렌더 범위는 COLOR_MANAGEMENT 참조 | src-tauri/src/image_tests.rs |
| REQ-083 | PSD 원본 및 레이어 이름·순서·위치·표시·불투명도·mask·clipping 보존 | P0 | DEV-02, DEV-05, DEV-07 | M03 | PSD 구조/byte range 및 독립 fixture 비교 | Verified | 고급 레이어 재합성 미지원 | docs/PSD_SUPPORT_MATRIX.md |
| REQ-084 | 원본/미리보기/작업/생성/승인 데이터 분리 | P0 | DEV-02, DEV-06, DEV-14 | M03 | source hash와 derivative 부모 참조 검사 | Partially Implemented | 원본/preview/진단 생성 artifact 분리; 캐릭터 생성 및 사용자 승인 편집은 후속 | docs/IMAGE_IMPORT.md |
| REQ-085 | 색 프로필 보존 및 표시용 ICC 변환·16-bit 작업 정밀도 | P0 | DEV-02 | M03 | RGB ICC 변환과 preview 정밀도 고지 검사 | Verified | RGB ICC/synthetic P3 numerical path tested; monitor/gray ICC certification remains unverified | docs/COLOR_MANAGEMENT.md |
| REQ-086 | Alpha 경계 및 blend 미지원 처리의 명시성 | P0 | DEV-02, DEV-08 | M17 | transparent RGB/반투명/저장 composite 기준 비교 | Partially Implemented | 레이어 compositor 미구현 | src-tauri/src/image_tests.rs |
| REQ-087 | 고해상도 입력 자원 한도·취소·CPU 미리보기 | P0 | DEV-02 | M03 | Windows 시간/peak RAM/취소 후 자원 측정 | Verified | 40M RGB8 measured; independent decoded160MiB/working320MiB limits, true OS power-loss excluded | docs/IMAGE_IMPORT.md |
| REQ-088 | 손상 PSD·오프셋·폭탄·계층·레이어·ICC·읽기 변경 방어 | P0 | DEV-02, DEV-29 | M03 | 잘린 파일/한도/구조 mutation과 경로 검사 | Partially Implemented | bounded parsing/mutation/reparse checks; exhaustive fuzz/hostile same-user race remains DEV-29 | src-tauri/src/image_tests.rs |
| REQ-089 | PNG v1 보존 및 PSD 별도 v2 불변 revision·복구 호환 | P0 | DEV-02 | M02 | v1 bytes 보존/PSD reopen/recover/index 시험 | Verified | 기존 프로젝트 무단 변환 금지 | src-tauri/src/image_tests.rs |
| REQ-090 | 입력·색·미지원 안내 3언어와 native preview | P0 | DEV-02 | M20 | catalog parity/UI/native 입력 확인 | Verified | actual OS PSD dialog and3locale preview/reload tested; OS drop gesture not run | src/SourcePreview.tsx |
| REQ-091 | 합성 PNG·PSD corpus 및 독립 기준/검증 범위 구분 | P0 | DEV-02 | M22 | fixture별 expected 픽셀/구조/hash 검증 | Verified | Photoshop 원본 reference 없음 | tests/fixtures/source |
| REQ-092 | DEV-00·01 저장·복구·Worker·프로필 전체 회귀 | P0 | DEV-02 | M22 | 전체 suite/native recovery | Verified | DEV-02 최종42Rust/helper1,4contract,2Python,3UI 근거 유지; DEV-03 회귀는 TEST_RUNS 별도 | docs/TEST_RUNS.md |
| REQ-093 | DEV-02 전체 요구 감사·문서·비용·E 경로·사용자 수정 보호 | P0 | DEV-02 | M22 | 78 ID 보존/mirror/status/path audit | Verified | cleanup 파일은 ignore된 사용자 로컬 파일로 보존 | docs/PROJECT_STATUS.md |
| REQ-094 | DEV-02 PR·CI·보호 조건·main 병합 후 검증 | P0 | DEV-02 | M22 | GitHub PR/main SHA/CI/tree 검증 | Verified | PR#3 merge c07a6fe6c9f15a3b76e18179974a7db86ba8ab29 및 CI40696324294 success 시작 시 확인 | docs/PROJECT_STATUS.md |
| REQ-095 | 로컬 Runtime Manager·버전 adapter·전용 환경 | P0 | DEV-03 | M04 | 실제 환경 설치/기존 dummy 공존 | Verified | Windows Python3.12 x64 CPU 경로만 검증 | src-tauri/src/runtime_tests.rs, docs/evidence/dev-03-local.json |
| REQ-096 | ModelManifest 버전·hash·권리·지역·지원 형식 검증 | P0 | DEV-03 | M04 | 잘못된 manifest/license/region 거부 | Verified | 외부 모델 추가는 새 권리 검토와 앱 업데이트 필요 | src-tauri/src/runtime_tests.rs, docs/evidence/dev-03-local.json |
| REQ-097 | 모델 설치 정보와 명시적 사용자 동의 | P0 | DEV-03 | M01/M04 | UI checkbox 및 stale token/미승인 process 차단 | Verified | 개발/CI prepare 명령은 별도 명시 실행 | tests/ui/runtime.spec.ts, docs/evidence/dev-03-local.json |
| REQ-098 | HTTPS 출처·redirect·크기·hash·부분 다운로드 안전성 | P0 | DEV-03 | M04 | mock stream cancel/disconnect/hash/size/redirect 시험 | Verified | 실제 디스크 소진/원격 서버 장애는 모의 주입 | workers/test_runtime.py, docs/evidence/dev-03-local.json |
| REQ-099 | 격리 로딩 및 불변 설치 generation·실패 복구 | P0 | DEV-03 | M04/M22 | native 설치 취소/kill 후 기존 active 및 재실행 | Verified | 불완전 환경은 보존, 재개 대신 새 generation; 자동 삭제 없음 | scripts/native-runtime.mjs, docs/evidence/dev-03-local.json |
| REQ-100 | GPU 감지와 실제 CPU 연산 기반 backend 지원 구분 | P0 | DEV-03 | M05 | 실제 ORT CPU probe 및 NVIDIA 장치 정보 별도 기록 | Verified | CUDA/PyTorch/DirectML 실행 미검증·미설치 | src-tauri/src/runtime_tests.rs, docs/evidence/dev-03-local.json |
| REQ-101 | RAM·디스크·backend 사전 검사 및 실행 자원 한도 | P0 | DEV-03 | M04/M05 | 자원 부족 mock/실제512MiB Job Object allocation 실패 | Verified | GPU offload/fallback 미허용;6GB 대형 모델 성능 보장 없음 | src-tauri/src/runtime_tests.rs, docs/evidence/dev-03-local.json |
| REQ-102 | 실제 합성 ONNX 추론과 설치 후 로컬 반복 실행 | P0 | DEV-03 | M04 | 3개 입력의 실제 출력 및 Python socket 차단 | Verified | 물리적 offline/네이티브 DLL 전체 네트워크 감사 미실행 | workers/test_runtime.py, docs/evidence/dev-03-local.json |
| REQ-103 | Worker v2 strict protocol 및 기존 v1 호환 | P0 | DEV-03 | M04/M22 | 잘못된 ID/version/progress/terminal/큰 출력/crash | Verified | 임의 Provider 코드 실행 금지 | src-tauri/src/runtime_tests.rs, docs/evidence/dev-03-local.json |
| REQ-104 | Staging·결과 검증·원본 provenance·보고서 무결성 | P0 | DEV-03 | M03/M04 | 형식/math/source/hash/내용주소 결과 변조 거부 | Verified | 무결성 checksum은 동일 사용자 악의적 재작성에 대한 서명이 아님 | src-tauri/src/runtime_tests.rs, docs/evidence/dev-03-local.json |
| REQ-105 | Job Journal·취소·비정상 중단·재시도 | P0 | DEV-03 | M22 | 실제 취소/descendant timeout/앱 kill/restart | Verified | AI checkpoint 자동 재개/legacy 전체 tree는 DEV-28 | scripts/native-runtime.mjs, docs/evidence/dev-03-local.json |
| REQ-106 | 추론 전후 PNG·PSD 및 저장 이력 불변 | P0 | DEV-03 | M03/M04 | 실제CPU 전후 원본/revision hash 대조 | Verified | source hash는 캐릭터 정체성 검증이 아님 | src-tauri/src/runtime_tests.rs, docs/evidence/dev-03-local.json |
| REQ-107 | 초보자 AI 준비·설치·결과·오류 3언어 UI | P0 | DEV-03 | M01/M20 | mock consent/실제 native 설치·결과·3locale | Verified | 초보자5명/전체 screen reader 평가는 DEV-25/26 | tests/ui/runtime.spec.ts, docs/evidence/dev-03-local.json |
| REQ-108 | E 전용 환경·신뢰 코드·공급망·권한 경계 | P0 | DEV-03 | M04/M22 | 고정wheel·환경inventory·경로·변조·실행 allowlist 감사 | Verified | venv/-I/Job Object는 OS 파일/네트워크 sandbox가 아님 | workers/test_runtime.py, docs/evidence/dev-03-local.json |
| REQ-109 | 실측 자원·DEV00~02 회귀·94개 요구 전수 감사 | P0 | DEV-03 | M22 | 전체 suite/release/native 및 evidence hash | Verified | 측정은 이 장치/합성 모델 범위; 제품 품질 게이트 미검증 | docs/TEST_RUNS.md, docs/evidence/dev-03-local.json |
| REQ-110 | DEV-03 PR·CI·보호 조건·main 병합 후 검증 | P0 | DEV-03 | M22 | 원격 PR head/main SHA·tree·CI 비교 | Implemented | 필수 CI/보호/리뷰 미충족 시 병합 금지 | docs/PROJECT_STATUS.md |
