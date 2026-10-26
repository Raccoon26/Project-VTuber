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
| GitHub Windows CI / PR main merge | Pending | 최종 PR/check run/merge SHA로 갱신 |

## 최종 suite
`./scripts/test.ps1 -Python <검증된 절대경로>`는 docs audit, frontend, contract, Python, rustfmt, Rust tests, clippy, UI 순서로 실행하고 실패 시 중단한다. 2026-10-26 JST 최종 로컬 suite는 모든 항목을 통과했다(`artifacts/final-suite.log`). 원격 CI는 별도 결과로 기록한다. 시뮬레이션으로 GPU나 방송 앱을 검증했다고 표시하지 않는다.
