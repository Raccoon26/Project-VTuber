# Test execution ledger

환경: ENVIRONMENT_REPORT의 Windows 장치, 2026-10-25~26 JST. 모든 신규 로그·브라우저 캐시·임시 파일은 E 프로젝트의 artifacts/.tools. raw 로그는 사용자 경로를 포함할 수 있어 Git ignore하며 여기에는 검증 가능한 요약을 남긴다. CI artifact는 합성 테스트 로그만 포함한다.

| 명령/검사 | 상태 | 증거 위치 / 범위 |
|---|---|---|
| `npm run build` | Pass | `artifacts/frontend-build.log`; TS+Vite production build |
| `npm test` | Pass (4 tests) | `artifacts/contracts-test.log`; export/semantic/i18n/offline adapter |
| Python3.12 `-m unittest discover -s workers -v` | Pass (2 tests) | `artifacts/python-test.log`; protocol fixtures |
| `cargo test --manifest-path src-tauri/Cargo.toml --locked` | Pass (12 tests) | `artifacts/rust-test.log`; 실제 Python, crash/oversize/timeout 추가 포함 |
| `cargo clippy --manifest-path src-tauri/Cargo.toml --locked --all-targets -- -D warnings` | Pass | `artifacts/clippy.log`; 최종 suite에서 재실행 |
| `npm run test:ui` | Pass (1 test) | `artifacts/ui-test.log`, `artifacts/browser-preview.png`; 3언어/preview/좁은 화면 |
| `npm run desktop:build` | Pass | `artifacts/desktop-build.log`; Windows release exe, 초기 실패는 FAILURE_REGISTER |
| `cargo run --manifest-path src-tauri/Cargo.toml --example seed_fixture --locked` | Pass | `artifacts/seed-fixture.log`; 실제 Rust PNG importer, 합성64×64 |
| `node scripts/native-smoke.mjs` | Pass | `artifacts/native-smoke.log`, `artifacts/native-app.png`; mock 없는 native IPC/실제 Python/GPU |
| `npm audit` | Pass, 0 vulnerabilities (교체 후) | 최종 `artifacts/npm-audit.log`; 초기3개 advisory 수정 기록 |
| `node scripts/audit-docs.mjs` | Pass | 20개 필수 상세 문서, 62개 요구사항, 34개 DEV, evidence 경로 |
| `node scripts/dependency-inventory.mjs` | Pass | npm86/Rust250(Windows). 라이선스 미지정 root vtuber-desktop은 출시 전 소유자 결정 필요 |
| `npm run format:check` / `cargo fmt --check` | Pass | TypeScript/CSS/설정 및 Rust 포맷 일관성 |
| Windows computer-use screenshot | Failed, 1회 재시도도 timeout | F00-009; 실제 app 결과와 구별 |
| Windows native file picker 직접 클릭 | Not run | importer 코어/fixture는 실행했으나 native dialog 조작 증거 없음 |
| Live2D/VBridger/방송 앱 runtime | Not run | 상용 앱/권리 있는 preset·테스트 모델 미확보 |
| 실제 AI GPU 추론/3D/자동 rigging | Not run | 모델 미설치, 자원·권리/승인 조건 미충족 |
| expert fidelity/dance/RTL/clean-machine installer | Not run | 후속 DEV와 corpus 필요 |
| GitHub Windows CI | 원격 실행 결과 참조 | [PR #1 Checks](https://github.com/Raccoon26/Project-VTuber/pull/1/checks); 최종 head에서 suite/release build/audit 성공을 병합 조건으로 요구 |
| PR main merge / main CI | 원격 상태 참조 | [PR #1](https://github.com/Raccoon26/Project-VTuber/pull/1)의 merged 상태·SHA와 [main workflow](https://github.com/Raccoon26/Project-VTuber/actions/workflows/ci.yml?query=branch%3Amain); 로컬 결과와 구별 |

## 최종 suite
`./scripts/test.ps1 -Python <검증된 절대경로>`는 docs audit, frontend, contract, Python, rustfmt, Rust tests, clippy, UI 순서로 실행하고 실패 시 중단한다. 2026-10-26 JST 최종 로컬 suite는 모든 항목을 통과했다(`artifacts/final-suite.log`). 원격 CI는 별도 결과로 기록한다. 시뮬레이션으로 GPU나 방송 앱을 검증했다고 표시하지 않는다.

## DEV-01 실행 기록 (2026-10-27 JST)

시작 base: ac7e2573ed4a2086940842a01aecb7746fd72028, DEV-00 main CI 40314991998 success. Python3.12.14의 ENVIRONMENT_REPORT 절대 경로를 사용했다. 기존 .tools/app-data의 5개 프로젝트는 구조만 조회했고 destructive test에 사용하지 않았다.

| 명령/검사 | 결과 | 증거/범위 |
|---|---|---|
| scripts/test.ps1 -Python <실제3.12> 기준선 | Pass (재실행) | artifacts/dev-01-baseline-clean.log; 최초 실행은 UI 종료 정지 F01-001 |
| cargo test --manifest-path src-tauri/Cargo.toml --locked | Pass 28 tests + ignored helper1 | artifacts/dev-01-rust-full.log; helper는 상위 테스트가 실제 child로 호출 |
| npm run test:ui | Pass 2 | artifacts/dev-01-ui-tests2.log; 기존 UI + simulated IPC recovery/취소/키보드/3언어 |
| npm run build / cargo clippy --locked --all-targets -- -D warnings | Pass | artifacts/dev-01-ui-build3.log / dev-01-clippy.log |
| npm run desktop:build | Pass | artifacts/dev-01-release-build.log; 실제 Windows release |
| node scripts/native-recovery.mjs | Pass | artifacts/dev-01-native.log / native-dev01-recovery.png; 실제 app/IPC/Python/GPU + kill/restart/recovery/hash |
| 전체 최종 suite / 최종 release / audit | 아래 최종 증거 참조 | docs/evidence/dev-01-local.json에 최종 실행 후 hash 기록 |
| GitHub PR/main CI | 원격 검증 결과 참조 | PROJECT_STATUS release evidence; 로컬 검사와 구분 |

### 필수 실패 주입 20항목

| # | 상황 | 결과·검증 방식 |
|---|---|---|
| 1 | manifest 작성 도중 종료 | Pass 실제 child partial-write checkpoint에서 kill, 기존 bytes 동일 |
| 2 | 임시 파일만 생성 | Pass empty checkpoint kill, 이전 revision 열기 |
| 3 | 공개 직전 실패 | Pass before_publish kill 및 StorageFull 주입, 이전 정본 유지 |
| 4 | 공개 직후 실패 | Pass after_publish kill, 새 revision 검증·기존 파일 보존 |
| 5 | 파일 접근 거부 | Pass Windows share_mode(0) 배타 handle로 실제 읽기 거부; 정상 항목 격리. 실제 사용자 ACL 변경 안 함 |
| 6 | 디스크 부족 | Pass StorageFull 오류 주입 + journal 쓰기 실패; 실제 드라이브 소진은 미실행 |
| 7 | JSON 일부 손상 | Pass 잘린 JSON 거부/이력 보존 |
| 8 | 최신 revision 손상 | Pass 자동 rollback 없음, 선택 복구로 새 번호 |
| 9 | 과거 revision 손상 | Pass 최신 정상 열기 유지, 이력에서 과거 오류 |
| 10 | blob 손상 | Pass 참조 버전 복구 후보 제외, 원본 없는 초기 버전과 구별 |
| 11 | SQLite 삭제 | Pass 다음 검사 시 재구축 |
| 12 | SQLite 손상 | Pass 새 DB로 재구축, 파일 정본 불변 |
| 13 | index/파일 불일치 | Pass stale/version999/manual add/move 재스캔 |
| 14 | 두 프로세스 동시 저장 | Pass barrier 동시 시작, 하나만 성공; 다른 쪽 busy/conflict |
| 15 | 오래된 revision | Pass 기존 stale save + recovery token 충돌 |
| 16 | 잠금 경합 | Pass 다른 process 사용 중 안내, owner kill 후 OS lock 재획득 |
| 17 | Unicode | Pass 일본어/한국어 디렉터리·프로젝트명·검색 |
| 18 | 링크/경로 | Pass 실제 Windows junction 거부/외부 target 변경0, 기존 ../escape 시험 |
| 19 | pending 잔존 | Pass 실제 kill 잔존 pending 무시·보존, 이후 저장 성공 |
| 20 | 앱 강제 종료/재시작 | Pass 실제 native app kill→interrupted→선택 복구→재열기/hash 일치 |

일반 process kill은 전원 차단 검사가 아니다. 전원 차단, 네트워크 FS/NTFS 외, 전체 child process tree 회수, 장시간 AI checkpoint, 실제 디스크 고갈, OS 파일 선택창 클릭은 미실행이다. 모든 failure hook은 cfg(test)만 사용한다.

### 최종 로컬 증거

전체 suite: artifacts/dev-01-final-suite.log 통과(Rust28 + helper1, Python2, 계약4, UI2, docs78/23, format/build/clippy). release: dev-01-release-final.log 통과. native 최종 재검증: dev-01-native-final-retry.log 통과; 직전 CDP 연결 오류는 F01-006에 기록했다. npm audit 취약점0, dependency inventory npm86/Rust257(Windows)이다. 마지막 변경은 native harness 연결 준비 및 문서이며 최종 format/docs audit로 확인한다.


## DEV-02 — 2026-10-28 JST (validation in progress)

All commands dot-source scripts/env.ps1. Actual Python is the existing3.12 path in ENVIRONMENT_REPORT. No original user artwork or heavy AI model is used. New data/builds/cache/reference dependencies are on E.

| Command / method | Result | Evidence / scope |
|---|---|---|
| scripts/test.ps1 -Python <actual3.12>, original5d38dcb extraction | Pass | artifacts/dev-02-baseline-isolated.log (28 Rust/helper1,4 contracts,2 Python,2 UI); first mixed-source attempt is invalid baseline F02-001 |
| cargo test --locked image_tests | Pass through image-tests5.log | PNG modes/16/ICC/transparent RGB, PSD layers/hash/recovery/Lr16, independent compression/group reference, white matte/alpha |
| cargo test --locked with VTUBER_PYTHON | Pass | artifacts/dev-02-rust-full2.log; earlier direct command without Python env failed two journal tests F02-005 |
| cargo clippy --locked --all-targets -- -D warnings | Pass before last additions | artifacts/dev-02-clippy2.log; final suite still required |
| npm run build / npm run test:ui | Pass | artifacts/dev-02-drop-build.log / dev-02-ui.log; original2 UI tests (mock recovery separate from native) |
| npm run desktop:build | Pass | artifacts/dev-02-release.log; final rebuilt labels still to check |
| node scripts/native-input.mjs + computer-use native dialog selection | Pass | artifacts/dev-02-native-input.log/.json/.png; actual OS dialog→PSD→automatic project→preview→expert layer→3locale→reload→original bytes/hash. Computer-use click returned unknown geometry error, independently verified action result F02-006 |
| cargo build --release --example inspect_image; python scripts/measure-image-input.py | Pass / bounded refusal | artifacts/dev-02-performance.json/.log; actual Windows peak working set via GetProcessMemoryInfo, includes process overhead; decoder/preview times separate |
| independent psd-tools1.26.0/attrs25.4.0 E-only reference | Pass | artifacts/dev-02-reference-check4.log; self-created solid-shape PSD composite matches exact RGBA. Unicode legacy-name generation initially failed F02-002 |

Measured4000² RGB8: decode145ms/preview6ms, process wall187–195ms, peak181,014,528B.4096² RGB16: decode211–238ms/preview6ms, wall260–286ms, peak240,410,624B.초기5000² 거부는 기존40M 지원 범위를 좁혀서 작업 버퍼만320MiB로 조정했다. 최종8000×5000은 peak445,517,824B/wall431ms,8001×5000은 peak≤4,476,928B로 거부했다. Process cold start/file cache affects wall time; these are synthetic examples, not universal performance guarantees. Native cached8² source preview≈3.6ms on this machine, source hash still verified.

Not run / not certified: licensed Photoshop runtime comparison, physical monitor/P3/grayICC accuracy, OS cross-window file drop gesture, true disk exhaustion/power loss/non-NTFS durability, exhaustive hostile ICC fuzzing, actual AI generation or checkpoint resume, five-person beginner study. Final CI/PR/main and final suite will be recorded separately.


### DEV-02 final validation update (chronological intermediate results)

- artifacts/dev-02-suite-final.log: full suite Pass (before final40M budget/test additions), Rust37/helper1, contracts4, Python2, UI3. All requested checks passed.
- artifacts/dev-02-mid-cancel.log: actual concurrent4000² decode cancellation passed; subsequent independent parse succeeds. No automatic import resume is claimed.
- artifacts/dev-02-performance-40m.log and dev-02-performance.json: actual40M8-bit support retained; decode/preview/time/peak memory measured independently from original bytes. Layer benchmark1/128/1024 succeeds (27–39ms,peak≤7,589,888B);1025 refuses with no success report.
- artifacts/dev-02-native-recovery.log: actual final release DEV-00/01 workflow, Python progress/cancel, GPU query, app kill/restart, recovery/index/source/history checks Pass.
- artifacts/dev-02-npm-audit.log:0 vulnerabilities; dev-02-dependencies.log:86npm/265Rust Windows inventory. Static color MIT notices in third_party/IMAGE_INPUT_NOTICES.md.
- UI mock missing currentWindow/unregisterListener caused initial recovery crash/unhandled rejection; real Tauri metadata and event cleanup were added to mock, and subscription errors are caught without whole-app crash. Final UI rerun required after mock fix.


### Final acceptance (DEV-02)

`./scripts/test.ps1 -Python <actual3.12>`: Pass in artifacts/dev-02-acceptance-suite.log,40 Rust tests +1 subprocess helper,4contract/2Python/3UI, format/tsc/Vite/rustfmt/clippy/doc coverage. `npm run desktop:build`: Pass in dev-02-release-40m.log. `node scripts/native-recovery.mjs`: Pass in dev-02-native-recovery-final.log; `node scripts/native-input.mjs` with actual OS file selection: Pass in dev-02-native-input-final.log (screenshot and hash evidence). No mock native pass substituted for OS dialog. Final cached8² preview≈12.4ms includes source verification; previous3.6ms was an earlier sample, not a guarantee.

Synthetic D65 Display P3 ICC conversion numeric sample passes within2 levels (dev-02-p3.log); physical monitor/P3 full profile conformance remains unverified. Cancellation during4000² decode and retry succeeds (dev-02-mid-cancel.log). Final performance source hash unchanged for every repeat; original/preview dimensions are distinct. Tests reference current log hashes in docs/evidence/dev-02-local.json.

Final resource-only packaging: dev-02-release-notices.log Pass; installed notices/IMAGE_INPUT_NOTICES.md hash matches checked-in notice. Final coverage audit27 documents/94 requirements in dev-02-docs-final.log. Source and runtime behavior remain the accepted40M build; subsequent GitHub CI verifies the exact committed tree.
