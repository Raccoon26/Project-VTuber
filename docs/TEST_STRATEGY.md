# Test strategy

검증은 단위/프로세스 통합/브라우저 화면/실제 desktop/실제 외부 앱/전문가 품질로 구분한다. 앞 단계의 pass를 뒷 단계에 상속하지 않는다. 테스트 명령·환경·결과·로그는 [TEST_RUNS](TEST_RUNS.md), 실제 실패는 [FAILURE_REGISTER](FAILURE_REGISTER.md)에 기록한다.

| 계층 | 현재 보호하는 행동 | 자동화/fixture | 후속 |
|---|---|---|---|
| Rust storage | 생성·revision/reopen·stale save·pending·원본 hash·malformed 입력 | tempfile 기반 합성 PNG, source 파일 불변 비교 | disk-full/power-loss/lock/PSD fuzz |
| Rust model | sample schema, 미래 schema·위조 Gate 거부 | 공개 JSON sample | migration/unknown extensions |
| Rust worker | 실제 Python 완료/cancel/single-job/실행 실패/프로토콜 오류/crash/oversize/timeout | Python subprocess, 짧은 합성 스크립트 | GPU OOM/process tree/resources |
| Python | 요청 schema와 이벤트 순서, malformed/oversize 거부 | 표준 unittest, fake sleep | adapter contract suites |
| TypeScript | export fail-closed, 독립 VBridger profiles, 정적 매핑, i18n parity, 광고 offline | Vitest | schema generation/변환/quality engine |
| Browser UI | preview에서 저장 불가, ja/en/ko, 2D/3D profiles, exporter 미지원, 800px layout | Playwright Chromium | visual goldens/keyboard/RTL |
| Native desktop | WebView→실제 Rust storage→실제 Python 및 GPU query | `scripts/native-smoke.mjs`, 실제 Windows exe, E fixture | native file picker 및 restart/close tree |
| External compatibility | 현재 미실행 | 계획: 권리 fixture+앱/version matrix | DEV-11/24 |
| Visual/rig quality | 현재 미실행 | 계획: corpus+pose battery+expert rubric | DEV-20/32 |

## 실행 규칙
PowerShell `scripts/test.ps1`은 첫 실패에 멈춘다. 선택적 `-SkipUI`는 이유가 있을 때만 사용하고 UI 미실행을 기록한다. 기본 CI는 skip하지 않는다. `npm ci`, `cargo --locked`, toolchain pin을 지킨다. 의존성 vulnerability audit도 별도 실행한다. 모든 완료 보고에는 정확한 CI head SHA를 확인한다.

새 모델은 다운로드를 테스트에 숨기지 않는다. 기본 CI는 네트워크 AI 호출/모델 weights/GPU 없이 실행되며 합성 fixtures만 사용한다. GPU/외부 앱 시험은 별도 opt-in 환경·승인·version fixture로 수행한다. 시각 golden은 이미지 생성/리깅 품질 합격 기준이 아니라 renderer 회귀 감지 도구다.

## Native test 재현
`VTUBER_DATA_DIR`를 비어 있는 **E 테스트 폴더**로 지정하고 `WEBVIEW2_USER_DATA_FOLDER`도 그 안으로 지정한다. `cargo run --example seed_fixture --manifest-path src-tauri/Cargo.toml --locked`로 합성 프로젝트를 준비한다. `WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS=--remote-debugging-port=9223`을 테스트 프로세스에만 주고 release exe 실행 후 `node scripts/native-smoke.mjs`를 실행한다. 루프백 debugger는 테스트 종료 시 프로세스를 닫아 없앤다. 제품 기본에는 debugger가 없다.

이 테스트의 source import는 같은 Rust ProjectStore 예제로 수행하며 **OS 파일 선택창 클릭을 검증했다고 보고하지 않는다**. UI create/save/reload/open/Worker/GPU는 mock 없이 실제 native commands를 통과한다. 하드웨어 이름 assertion은 이 개발 장치 전용이라 CI 표준 suite에서 실행하지 않는다.

## 회귀/실패 처리
재현 → 기대/실제 비교 → 증거 수집 → 가설 → 최소 실험 → 근본 원인 수정 → 회귀 → 관련 기능 → 기록 갱신. 같은 수정이 실패하면 가설을 바꾼다. 버전 변경은 보안 advisory/지원 matrix 같은 증거로 정당화하고 lockfile/환경/실패 register에 남긴다. CI를 통과시키려고 테스트를 지우거나 성공을 강제하지 않는다.
