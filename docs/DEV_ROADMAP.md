# DEV-00–33 roadmap

기간보다 정확성·원본 품질·재현 가능한 검증을 우선한다. 아래 순서는 DEV-00 조사 결과를 반영한 의존 순서이며 사용자 기능을 삭제하지 않는다. 모델 권장은 사용자가 제시한 GPT-6 Astra / GPT-6.1 Sol, High를 기준으로 하는 작업 배정 지침이다. 실제 계정에서 제공되는 모델 이름/추론 설정은 실행 시 확인하며 자동 변경하지 않는다. 가장 어려운 설계/권리/교차 검증은 Astra Extra High를 재검토한다.

공통 시작 절차: 이전 DEV의 main/CI/요구사항/실패 등록부/환경 감사. 공통 종료 절차: 해당 테스트→회귀→Gate evidence→요구사항 감사→문서→branch commit/push→PR→CI/필수 리뷰/보호 규칙→안전한 merge→main hash 확인. 표의 완료 조건은 이 공통 종료 절차까지 포함한다. 차단 시 후속 처리는 기능을 성공처럼 표시하거나 무조건 다음 DEV로 넘기는 것이 아니라 명시된 증거·재개 조건을 PROJECT_STATUS에 기록하는 것이다.

| DEV | 목표 | 권장 모델/추론 | 선행 | 구현 범위 | 주요 위험 | 품질 기준 | 자동 테스트 | 실제 앱 검증 | 완료 조건 / 미해결 처리 |
|---|---|---|---|---|---|---|---|---|---|
| 00 | 제작 기반·위험 제거 | Astra High | 없음 | Tauri/Rust/Python dummy, 저장, 계약, 문서/CI | 독점 출력·권리·6GB 제한 | 구현/미검증 구별, 원본 무손실 | 코어/Worker/계약/UI | Windows native create/save/reopen/job | 기반 실행+조사+main; 외부 AI/호환 실험은 명시 보류 |
| 01 | 프로젝트 내구성 | Astra High | 00 main 검증 | SQLite 재구축 인덱스, recovery UI, migration, 손상 항목 격리, 다중 프로세스 lock | FS/DB 이중 커밋, 잠금 경합 | 기존 원본/revision 손상 0 | fault injection, conflict, Unicode path, migration | 강제 종료/다시 열기/복구 선택 | 손상 프로젝트가 다른 목록을 막지 않음; 실패 시 파일 정본 유지 |
| 02 | 원본·PSD 안전 입력 | Sol High | 01 | PNG/PSD layered import, ICC/16-bit/alpha, bounded/tiled decode, source preview | 압축 폭탄·레이어 blending 손실 | Gate A, 원해상도·색 증거 | malformed/fuzz/metadata/color fixtures | 큰 원화와 PSD 비교 | 안전한 입력 범위 명시; 지원 외 포맷은 거부·보존 |
| 03 | 로컬 AI 실행 관리 | Astra High | 01,02 | hardware/backend probe, model manifest/download consent/hash, venv/sidecar, resource budget | CUDA/DirectML 충돌, pickle/OOM | 검증된 backend만 enabled | fake GPU, hash mismatch, offline install, process limits | 소형 허용 모델 CPU/GPU 연산 | adapter 하나의 안전 실행; 대형 모델은 승인/VRAM 충족까지 보류 |
| 04 | 원화 의미 분석 | Sol High | 02,03 | 얼굴/신체/의상/장식 landmarks 및 confidence | 애니메이션 스타일 편향 | 특징 누락/불확실성 표시 | 권리 확보 corpus metrics | 원본 위 overlay 검토 | 수정 가능한 분석; 저신뢰 영역 사용자 검토 |
| 05 | 레이어 분리 | Astra High | 04 | SAM adapter, alpha/mask/part graph, 수동 수정 | 머리/장식 경계·반투명 혼합 | 원본 픽셀 보존, seam 평가 | mask round-trip, alpha edge, full-res | 복잡 원화 layer toggle | 원본+생성 provenance; 부족 시 mask edit와 미완료 표시 |
| 06 | 가림 영역 복원 | Astra Extra High | 05 | 눈/입/머리/의상 숨은 부분 후보, inferred mask | hallucination·캐릭터 변경 | 원본 영역 불변, 추정 영역 표시 | protected-pixel diff, seed provenance | 펼친 레이어 비교/선택 | 검토 가능한 후보; 미승인 추정은 master 승격 금지 |
| 07 | 2D 마스터 메시 | Astra High | 05,06 | part mesh/deformer/keyform graph, 독립 Inochi adapter 실험 | topology·라이선스·단계 결합 | invertible edit graph, 원화 좌표 보존 | topology/reference/serialization | mesh edit/undo/저장 | 수정 가능한 2D master; 실패 exporter와 분리 유지 |
| 08 | 2D 비교 미리보기 | Sol High | 07 | alpha/clipping/deformer renderer, 원본 비교 | 색 공간·render drift | 픽셀/색/레이어 순서 일관 | golden images, clipping stress | 원본/현재/fixed pose toggle | 기준 renderer 검증; 색 관리 미지원 명시 |
| 09 | 고급 2D 얼굴 | Astra Extra High | 07,08 | 비대칭 눈/입/턱/볼/눈동자, 조합 corrective | 파라미터 상호작용 | facial pose battery Gate C | 단독·쌍·삼중 channel sweep | VBridger semantic preview | 검토된 face channels; 실패 조합 국소 repair |
| 10 | 2D 전신·물리 | Astra High | 09 | body deformer, hair/clothes springs/colliders | 격렬 움직임 파손 | 몸 회전/춤/physics 안정 | extrema, jitter, dt stability | 전신 motion preview | 몸/옷 분리 검증; 안정되지 않은 물리 export 차단 |
| 11 | 2D 출력 실현 | Astra Extra High | 07–10 | 공식 Cubism 경로/제3자 isolated spike, Inochi export, VBridger preset 보존 | cmo3/moc3 권리/버전/자동화 | Editor 재편집+VTS parameter fidelity | round-trip geometry/physics/maps | Cubism/VTS/Inochi Session 직접 실행 | 권리+실동작 동시 충족; 미해결 VTS는 Blocked 유지 |
| 12 | 3D 후보 benchmark | Astra High | 03,04 | 복수 image→3D adapters, corpus/seed/resource 기록 | 6GB/OOM/지역 제한 | 공정 조건·충실도·합법성 | schema/resource/failure harness | 승인된 장치에서 후보 실행 | 근거 기반 후보 선정; 자원 없으면 미측정 |
| 13 | 측면·후면 추정 | Astra Extra High | 12 | multi-view constraints, inferred provenance/수정 | 보이지 않는 정보 오판 | 정면 identity와 추정 표시 | view consistency/feature tests | 8방향 turntable human review | 불확실성 포함 결과; 완벽 복원 주장 금지 |
| 14 | topology·재질 | Astra High | 12,13 | 실제 mesh retopology/UV/texture/재질 분리, Blender recipe | 원본 질감 손실, 관절 edge flow | 정면 충실도+편집 가능 mesh | manifold/UV/seam/render diff | 원본-3D 투영 비교 | rig 입력 topology 승인; 손실 시 이전 artifact 유지 |
| 15 | 자동 skeleton | Astra High | 14 | UniRig/SkinTokens 검증 adapter, humanoid naming/rest pose | 자유 skeleton→humanoid mismatch | bone hierarchy/방향/관절 위치 | containment, cycles, missing bones | skeleton overlay/manual edit | 수정 가능한 skeleton; 저신뢰 joint 검토 |
| 16 | skinning·몸 solver | Astra Extra High | 15 | weight 정규화, joint limits, helper bones | 어깨/골반 weight bleed | volume 보존·극한 자세 | weight sums, joint sweeps | 팔 올림/깊은 squat/허리 회전 | 기본 pose battery 통과; 국소 skin repair |
| 17 | 3D 얼굴 solver | Astra Extra High | 14,16 | ARKit semantic blendshapes, jaw/cheek/lips/asymmetry | identity 변화·충돌 | 고개 회전 중 복합 표정 | blendshape combinations/render tests | face-only turntable/표정 조합 | VMC receiver용 의미/이름 계약; 미구현 channel 경고 |
| 18 | 춤/관절 보정 | Astra Extra High | 16,17 | corrective shapes/IK/foot contacts, dance battery | twist collapse/발 미끄러짐 | 빠른 반전/점프 안정 | 30/60/120Hz pose replay | 권리 확보 춤 클립 실행 | expert review Gate C; 실패 pose 원인 기록 |
| 19 | 3D 머리·의상 물리 | Astra High | 18 | springs/colliders/cloth authoring/앱별 mapping | 관통·수치 폭주 | frame-rate 독립 안정·충돌 기준 | timestep/penetration/stress | 격렬 motion+의상 preview | 대상 physics별 차이 보고; 미지원 효과 master 보존 |
| 20 | 통합 품질 평가 | Astra High | 08–10,17–19 | Gate engine/evidence store, image/pose metrics+human review | 점수로 예술성 오판 | A–F 상태·근거·hash 묶음 | threshold boundary/stale evidence | 전문가 baseline과 blind review | 측정 가능 기준 calibration; 애매하면 unverified |
| 21 | 자동 부분 재작업 | Astra High | 20 | defect localization, repair proposal/rollback/budget | 무한 재생성·원본 drift | 변경 부위만 수정/이전 결과 보존 | repair loop limit, rollback, provenance | 수정 전후 비교와 승인 | 개선 증거 없으면 되돌림; 실패 register 유지 |
| 22 | 통합 preview | Sol High | 08,14,20 | 2D/3D viewer, split/overlay/pose timeline | GPU memory·UI 응답성 | 원본/추정/문제 위치 명확 | visual regression, cancel render | 6GB 장치 큰 모델 뷰 | 편집/검토 가능; LOD 원본과 구분 |
| 23 | 3D export profiles | Astra High | 17–20 | VRM0/1, GLB, FBX/Unity recipes, loss report | shader/physics/axis loss | 마스터 불변·유효 포맷 | validators, round-trip, limits | Blender/Unity/VRM viewer | profile별 산출물; unsupported 기능 차단 |
| 24 | 방송 앱 인증 | Astra Extra High | 11,23 | VTS/VBridger/VSeeFace/Warudo/VNyan/Inochi/Prpr/VRChat suite | 버전·preset·앱 확장 편차 | Gate D per app/version/hash | param/format fixtures+replay | 각 대상에서 load/표정/몸/physics/restart | 실제 증거 있는 앱만 인증; 미설치 대상 unverified |
| 25 | 초보자 제작 흐름 | Sol High | 20–24 | upload→2D/3D→target→auto→review→export, expert 분리 | 기술 용어/막힌 복구 UX | Gate E 초보자 실험 | UI navigation/error/keyboard | 5명 이상 formative 사용성 | 핵심 경로 이해 가능; 실패 UX 수정 후 재검증 |
| 26 | 전체 i18n·접근성 | Sol High | 25 | ja/en/ko 도움말·튜토리얼·오류·quality, RTL/long text | 번역 의미/param명 변경 | 키보드·screen reader·긴 문장 | parity/pseudolocale/RTL/snapshot | 각 언어 리뷰+200% zoom | 필수 문구 누락 없음; 미검수 번역 표시 |
| 27 | 광고 adapter | Astra High | 25,26 | 계약 확인된 하단1배너, sponsor/offline/failover | SDK 개인정보·desktop 허가 | 핵심 기능 독립·원본 접근0 | timeout/offline/권한 격리 | 네트워크 traffic audit/일본 계약 | 계약 승인 전 disabled 유지; provider 교체 가능 |
| 28 | 복구·안정성 | Astra High | 01,03,21 | crash/OOM/restart/checkpoint/job-object, 진단 번들 | 전원 차단·자식 orphan | Gate F 데이터 손상0 | kill/fault/disk full/soak | 강제 종료/저장 장치 부족/복원 | 증거 있는 복구 경로; 데이터 손상 release blocker |
| 29 | 보안·권리 최종 감사 | Astra Extra High | 23,24,27,28 | sandbox/plugin scopes/SBOM/model license/출처 | 악성 assets/체크포인트/지역 제한 | unresolved critical·권리 위반0 | fuzz/audit/hash/permission tests | clean environment attack corpus | blocked dependency 제거/허가 후 재검증 |
| 30 | Windows 배포 | Sol High | 26,28,29 | portable/installer, Python sidecar, signature/update rollback, data dir 선택 | 관리자 요구·WebView/runtime 누락 | 재현 가능한 설치/오프라인 core | package smoke/SBOM/signature | 깨끗한 Windows 장치 설치/제거 | installer/portable 실증; 서명·계약 필요 시 승인 요청 |
| 31 | 장치·성능 매트릭스 | Astra High | 30 | CPU/저VRAM/고VRAM/driver/version profiles | 성능 목표 vs 품질 손실 | 지원 범위 정직, fallback 측정 | peak RAM/VRAM/latency/soak | 실제 저사양/다양한 장치 | 표기된 지원만 보증; 미검증 OS/device 분리 |
| 32 | 전문가 품질 beta | Astra Extra High | 24–31 | 권리 확보 캐릭터 corpus, 고급 수작업 비교, 사용자 수정 반복 | 전문가급 목표 미달 | 얼굴/전신/춤 blind assessment | 전체 regression + corpus gates | 전문가/초보자 동시 beta | 합의 기준 충족 전 정식 완성 주장 금지 |
| 33 | 출시 준비·감사 | Astra Extra High | 32 | 요구사항 전수 audit, 문서/tutorial/support, release evidence | 누락/권리/호환성 회귀 | 필수 requirements Verified, blocker0 | clean build/full suite/upgrade rollback | 전체 6단계와 대상 앱 인증 재실행 | 승인된 배포+main/태그 검증; 미해결이면 release 보류 |

## DEV-01 정확한 시작 지침
1. DEV-00 PR의 merge SHA와 main CI를 확인하고 `docs/PROJECT_STATUS.md`, `docs/FAILURE_REGISTER.md`, `docs/TEST_RUNS.md`를 읽는다.
2. `E:\Project-VTuber`에서 `scripts/env.ps1`을 dot-source한다. 기존 도구는 읽고 신규 cache/temp/data는 E에 둔다. `VTUBER_PYTHON` 실제 실행 파일을 확인한다.
3. `scripts/test.ps1` baseline을 실행한다. `samples/character-project.json`의 schema v1을 보존한다.
4. 우선순위: 손상된 하나의 프로젝트가 목록 전체를 막는 문제 → revision 선택 복구 → 작업 상태 영속화 → SQLite는 파생 index → 다중 writer 및 파일 race 최소화.
5. DEV-01에는 대형 모델 다운로드/추론이나 자동 리깅 구현을 끼워 넣지 않는다. DEV-00의 수동 외부 앱 미검증은 그대로 남긴다. 사용자 기능은 축소하지 않는다.

## DEV-01 결과와 DEV-02 정확한 시작 지침

DEV-01은 immutable v1 저장, 선택 복구, OS lock, SQLite 파생 인덱스, job journal을 연결한다. main PR/CI 및 시험 증거는 PROJECT_STATUS/TEST_RUNS 기준으로 확인한다. 자동 AI 재개와 전체 process tree 정리는 DEV-28에 남는다.

DEV-02 시작: (1) DEV-01 merge SHA/main CI와 clean tree 확인, (2) E env 및 실제 Python 경로 설정, (3) 전체 suite + native-recovery 기준선, (4) storage.import_png의 bounded read/decoder/source publication 계약 검토, (5) PSD·ICC·16bit·alpha 합성 corpus와 원본 hash 기대값부터 작성한다. 새 importer는 소스 bytes를 재압축하거나 manifest v1을 조용히 확장하지 않는다. PSD metadata/layer가 새 schema를 요구하면 MIGRATION_STRATEGY의 새 사본 검증 절차를 먼저 설계한다. raw 원본 보존과 색 관리 preview의 정확성을 별도로 평가하고 decoder 자원 한도·원해상도 처리 실패를 명시한다.
