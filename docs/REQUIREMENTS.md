# Requirements and coverage audit

기준: 이 채팅의 DEV-00 명세 전체 및 E 드라이브 저장 추가 지침. 이전 대화·별도 로드맵은 제공되지 않아 존재한다고 추정하지 않는다. 전문 제작 품질은 장기 목표이며 DEV-00의 dummy 성공과 구별한다. 원본 요구를 아래 추적 항목으로 정규화했다.

기계 판독 정본: `requirements.json`. 상태는 Planned / Implemented / Verified / Blocked / Partially Implemented / Rejected with Reason. Verified는 명시된 좁은 검증 범위에만 적용한다. P0는 필수 요구이며 장기 DEV에 배정된 P0가 DEV-00 제품 기능 완료를 뜻하지 않는다.

| ID | 요구사항 | 우선 | 담당 DEV | 모듈 | 검증 | 현재 상태 | 미해결 위험 | 테스트·증거 |
|---|---|---|---|---|---|---|---|---|
| REQ-001 | 제작 단계에서 리깅 완료, 방송 추적 앱과 분리 | P0 | DEV-00 | M01 | architecture 검토 | Verified | 실제 생성은 후속 | docs/ARCHITECTURE.md |
| REQ-002 | CPU/GPU/RAM 로컬 핵심 제작, 유료 API 필수 금지 | P0 | DEV-03, DEV-12 | M04 | offline 실제 모델 inference | Partially Implemented | dummy만 구현 | tests/contracts.test.ts |
| REQ-003 | 이미지·모델 무단 외부 전송 금지 | P0 | DEV-00, DEV-29 | M04 | network traffic/권한 감사 | Partially Implemented | 전체 OS traffic 측정 미실행 | docs/SECURITY.md |
| REQ-004 | 첫 다운로드 뒤 offline, 검증된 CPU/경량 대안 | P0 | DEV-03, DEV-31 | M04 | 모델 설치 후 disconnected 실행 | Planned | 현재 CPU dummy만 검증 | src-tauri/src/worker.rs |
| REQ-005 | GPU/VRAM/backend 진단과 지원 불명 상태 | P0 | DEV-00, DEV-03 | M05 | 장치 query와 실제 tensor smoke 분리 | Partially Implemented | NVIDIA만 감지; 추론 미검증 | src-tauri/src/gpu.rs |
| REQ-006 | 대형 AI 모델 설치·버전·무결성·권리 관리 | P0 | DEV-03 | M04 | consent/download/hash/offline fixtures | Planned | weights 미설치 | docs/MODEL_STRATEGY.md |
| REQ-007 | 원본 파일 불변 및 해시 검사 | P0 | DEV-00, DEV-02 | M03 | 입력/저장 bytes 동일 및 변조 거부 | Verified | PNG 지원 범위 | src-tauri/src/storage.rs |
| REQ-008 | 원해상도·색 정보·손실 압축/임의 축소 금지 | P0 | DEV-00, DEV-02 | M03 | 원본 byte 보존과 ICC/render 비교 | Partially Implemented | bytes 보존; 색 renderer 미구현 | src-tauri/src/storage.rs |
| REQ-009 | 원본/generated/manual 구분과 수정 가능 중간 결과 | P0 | DEV-01, DEV-06, DEV-14 | M02 | provenance DAG/revision round-trip | Partially Implemented | 현재 원본만 존재 | docs/MODEL_CONTRACT.md |
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
| REQ-031 | 친숙한 오류 설명/복구 경로·자동 수정 | P0 | DEV-00, DEV-21, DEV-28 | M16 | error UX/repair rollback | Partially Implemented | 복구·repair UI 후속 | src/i18n.ts |
| REQ-032 | 핵심 무료·최하단 배너 최대1개·강제광고 금지 | P0 | DEV-00, DEV-27 | M21 | UI/계약/광고 수량 검사 | Partially Implemented | 현재 배너0 | src/contracts.ts |
| REQ-033 | 공급업체 교체·desktop/Japan 계약·직접 스폰서 | P0 | DEV-00, DEV-27 | M21 | 계약/SDK 지원 audit | Partially Implemented | 미계약, 일본·지급 미확인 | docs/MONETIZATION.md |
| REQ-034 | 광고 실패/offline에도 제작 정상·GPU/데이터 격리 | P0 | DEV-00, DEV-27 | M21 | network timeout/권한/traffic | Partially Implemented | offline null adapter만 구현 | tests/contracts.test.ts |
| REQ-035 | 일본어·영어·한국어 UI/오류/quality/i18n | P0 | DEV-00, DEV-26 | M20 | key parity/언어 전환 | Partially Implemented | 튜토리얼·전체 도움말 후속 | tests/contracts.test.ts |
| REQ-036 | 새 언어·RTL·긴 문자열 고려 | P0 | DEV-26 | M20 | pseudolocale/RTL/zoom | Planned | RTL 실동작 미검증 | docs/LOCALIZATION.md |
| REQ-037 | 시작 시 저장소·OS·CPU/GPU/RAM/disk/tool 조사 | P0 | DEV-00 | M22 | 로컬 명령 증거 | Verified | 설치 부재는 탐색 범위 한정 | docs/ENVIRONMENT_REPORT.md |
| REQ-038 | 기존 코드·사용자 변경 보존, main 직접 변경 금지 | P0 | DEV-00 | M02 | git diff/status/branch 확인 | Verified | 초기 README만 존재 | docs/ENVIRONMENT_REPORT.md |
| REQ-039 | 기술 선택 비교·ADR·22개 모듈 경계 | P0 | DEV-00 | M01 | architecture/decision coverage audit | Verified | 후속 실험으로 교체 가능 | docs/ARCHITECTURE.md |
| REQ-040 | 공개 schema·모델별 adapter·2D/3D 독립 | P0 | DEV-00, DEV-03 | M04 | schema/contract tests | Partially Implemented | 현재 schema는 최소 v1 | docs/MODEL_CONTRACT.md |
| REQ-041 | Live2D/Inochi/Ayatsuri 편집·runtime·권리 조사 | P0 | DEV-00, DEV-11 | M18 | primary evidence+실험 계획 | Partially Implemented | 실제 변환 미실행 | docs/TECH_RESEARCH.md |
| REQ-042 | 복수 로컬3D 모델 비교·동일 조건 실험 설계 | P0 | DEV-00, DEV-12 | M10 | source/license/VRAM 및 benchmark | Partially Implemented | 품질 실측 미실행 | docs/MODEL_STRATEGY.md |
| REQ-043 | 자동 rigging 후보와 춤 후처리 구조 | P0 | DEV-00, DEV-15, DEV-18 | M11 | license review/pose battery | Partially Implemented | RigAnything 상업 경로 제외 | docs/TECH_RESEARCH.md |
| REQ-044 | 오류ID/재현/근본원인/수정/회귀 기록 | P0 | DEV-00 | M22 | failure register audit | Verified | 불명 원인은 불명 표시 | docs/FAILURE_REGISTER.md |
| REQ-045 | 반복 실패는 증거→가설→최소 실험 순서 | P0 | DEV-00 | M22 | 재현/회귀 기록 | Verified | 일회성 tooling 실패도 구분 | docs/FAILURE_REGISTER.md |
| REQ-046 | 테스트 명령·환경·결과·로그·미실행 구분 | P0 | DEV-00 | M22 | test ledger | Verified | 최종 CI 결과 별도 | docs/TEST_RUNS.md |
| REQ-047 | Gate A–F 네 상태와 사람 품질 검토 | P0 | DEV-00, DEV-20 | M15 | schema/validator/전문가 review | Partially Implemented | 모델 gates 모두 unverified | docs/QUALITY_GATES.md |
| REQ-048 | Tauri React TS Rust 실행 가능한 Windows 기반 | P0 | DEV-00 | M01 | frontend/native build+실행 | Verified | clean-machine installer는 후속 | src-tauri/src/main.rs |
| REQ-049 | Python interface/dummy progress/cancel/failure | P0 | DEV-00 | M04 | 실제 subprocess 단위·통합 | Verified | OS 강제종료 트리 회수 후속 | src-tauri/src/worker.rs |
| REQ-050 | CharacterProject 생성·저장·다시 열기·예제 | P0 | DEV-00 | M02 | round-trip/conflict/reopen | Verified | 손상 목록 격리 후속 | src-tauri/src/storage.rs |
| REQ-051 | 테스트 실행 스크립트/CI/의존성 lock | P0 | DEV-00 | M22 | clean npm ci/cargo locked | Implemented | 원격 CI 최종 확인 필요 | scripts/test.ps1 |
| REQ-052 | 잘못된 이미지·metadata·폭탄·경로 공격 | P0 | DEV-00, DEV-02, DEV-29 | M03 | malformed/limit/path regressions | Partially Implemented | 전면 fuzz/TOCTOU 미해결 | src-tauri/src/storage.rs |
| REQ-053 | AI 체크포인트·plugin·출처·license·checksum | P0 | DEV-03, DEV-29 | M04 | manifest audit/quarantine | Planned | 실제 모델0 | docs/SECURITY.md |
| REQ-054 | Worker crash/OOM/cancel·저장 실패·강제종료 복원 | P0 | DEV-00, DEV-28 | M22 | fault injection/soak | Partially Implemented | 실제 GPU OOM/전원 차단 미실행 | src-tauri/src/worker.rs |
| REQ-055 | 불필요한 관리자 권한 없이 설치/실행 | P0 | DEV-00, DEV-30 | M01 | 일반 사용자 실행/clean install | Partially Implemented | portable interpreter 번들 후속 | docs/SECURITY.md |
| REQ-056 | 필수 문서/원본 요구사항/DEV00–33 로드맵 | P0 | DEV-00 | M22 | 문서 coverage audit | Verified | 최종 링크/상태 audit 필요 | scripts/audit-docs.mjs |
| REQ-057 | 각 요구사항 ID/우선/DEV/모듈/검증/상태/위험/테스트 | P0 | DEV-00 | M22 | 요구사항 ledger validator | Verified | 제품 요구 Verified와 구분 | docs/requirements.json |
| REQ-058 | 모든 DEV 시작/종료 누락·회귀 감사 | P0 | DEV-00, DEV-33 | M22 | CI와 PR checklist | Implemented | 후속 DEV 지속 준수 필요 | docs/DEV_ROADMAP.md |
| REQ-059 | DEV 후 commit/push/PR/CI/조건충족 merge/main 검증 | P0 | DEV-00 | M22 | GitHub checks/protection/merge SHA | Implemented | 원격 merge 전까지 완료 아님 | docs/PROJECT_STATUS.md |
| REQ-060 | 실패·충돌·권한·필수 리뷰 미충족 시 강제 merge 금지 | P0 | DEV-00 | M22 | GitHub rules/checks+head 검증 | Implemented | 관리자 bypass 사용 금지 | docs/PROJECT_STATUS.md |
| REQ-061 | 승인 없는 대형 모델/장시간GPU/유료/외부 전송 금지 | P0 | DEV-00, DEV-03 | M04 | 작업/모델 inventory audit | Verified | 현재 AI 다운로드0/GPU inference0 | docs/ENVIRONMENT_REPORT.md |
| REQ-062 | 후속 지침: 신규 출력·cache·temp·앱 데이터는 E | P0 | DEV-00 | M02 | env/path/filesystem 확인 | Partially Implemented | 지침 전 C cache 이력·OS 자체 로그 제외 | scripts/env.ps1 |

## 감사 결과 해석
출시 기능을 삭제한 항목은 없다. 특정 기술 경로(RigAnything의 상업 제품 이용, AdSense software 삽입)를 기각한 이유는 LICENSE_AUDIT/MONETIZATION에 있고 요구사항 자체는 유지한다. Live2D 자동 authoring과 고급 리깅은 아직 해결했다고 표시하지 않는다. 현재 구현·검증은 TEST_RUNS/TECH_SPIKES를 따른다. 시작/종료 audit에서 상태를 바꿀 때 evidence와 담당 DEV를 함께 갱신한다.
