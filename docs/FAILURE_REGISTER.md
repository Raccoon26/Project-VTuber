# Failure register

모든 항목은 DEV-00에서 발생했다. 최종 재검증 명령과 결과는 TEST_RUNS에 연결한다. 사실·추정·미해결을 구별하며 동일 실패에 맹목적인 버전 변경/리팩터링을 반복하지 않는다.

| ID / 모듈 | 증상·재현 조건 | 확인된 원인/증거 | 수정 또는 미수정 이유 | 회귀·재발 방지 / 검증 |
|---|---|---|---|---|
| F00-001 환경 | sandbox CIM query/브랜치 쓰기 실패 | Access denied / cannot lock ref; sandbox가 해당 작업 제한 | 지정된 읽기 조사·branch 생성만 escalation | 실제 CIM 결과/branch 확인. 장치 없음으로 오판 금지 |
| F00-002 AI runtime | `python --version` 실행 실패 | WindowsApps alias를 실제 Python처럼 발견 | 동작하는 기존 Python3.12.14를 명시 경로로 사용 | Python unittest+Rust 실제 process 테스트 통과; launch script 버전 확인 |
| F00-003 frontend test deps | 최초 npm audit moderate1/critical2 | Vitest3.2.4 및 mocker/tinypool advisory | advisory 수정 버전5.0.5 및 지원 Vite8.3.5 고정 | npm audit 0, build/contract/UI 재검증. force upgrade 안 함 |
| F00-004 Windows resource | Cargo build에서 icon.ico missing | tauri-build Windows resource가 기본 icon을 요구 | 자체 기하학 icon 생성 스크립트/asset 추가 | release build 통과. CI에도 asset 포함 |
| F00-005 TS config | process 이름을 찾을 수 없음 | Playwright config의 Node 타입 누락 | @types/node22.19.0 exact 추가 | tsc/build 통과; 타입 오류 무시 옵션 금지 |
| F00-006 Tauri package alignment | desktop build 시작 시 version mismatch | Rust tauri2.12.5와 JS api2.10.1 minor 불일치 | npm API/CLI2.12.4 + lockfile | native release build 통과; cargo major 범위 해석 시 lock 필수 |
| F00-007 UI locator | 언어 selector getByLabel exact timeout | role tree에는 combobox 있으나 label text 계산에 nested option text 영향; explicit accessible name 필요 | selector aria-label 명시 | 동일 Playwright 시험 3언어 통과; 접근성 name 고정 |
| F00-008 Windows linker | `LNK1181 dbghelp.lib` release 실패 | 다른 VS linker 경로 선택, 실제 SDK엔 파일 존재. 자동 탐지의 SDK 검색 경로가 불완전 | env script에서 존재 검증한 MSVC+SDK LIB/INCLUDE 경로 구성 | 같은 release command 통과. 라이브러리 재설치/임의 버전 downgrade 안 함 |
| F00-009 Windows UI capture | FrameArrived/window capture timed out | helper capture timeout만 확인, 원인 미확정. 앱 자체 실패 증거 아님 | 규정된 1회 재선택/재시도 후 중단, 실제 WebView CDP로 native smoke 대체 | native IPC/스크린샷 통과. OS file picker UI는 미실행 |
| F00-010 GitHub connector | get_repo argument binding 실패 | 도구가 InvalidActionArgumentsError, 기대 schema 확인 불가 | 저장소/규칙/PR 작업은 기존 인증을 이용한 official CLI fallback | CLI repo/branch rules read 통과; token 로그/파일 노출 금지 |
| F00-011 approval service | GitHub Actions commit 조회 미실행 | automatic approval reviewer 사용량 한도 오류; unsafe 판정 아님 | 사용자 이어서 요청 후 동일 escalation 절차 재시도 | 조회 성공. sandbox 우회로 재실행하지 않음 |
| F00-012 dependency inventory | offline cargo metadata가 Android 패키지 다운로드를 요구 | 플랫폼 필터 없는 metadata가 미설치 타 OS dependency까지 해석 | 실제 배포 대상 x86_64-pc-windows-msvc filter 추가 | offline inventory 성공: npm85/Rust250. Windows 목록이며 전 OS audit로 주장하지 않음 |
| F00-013 native test harness | 새 WebView profile로 launch 직후 page missing: about:blank | 앱 navigation 전 CDP 연결됨. 이후 실제 목록은 tauri.localhost로 전환됨 | 고정 sleep 대신 실제 target URL 준비 조건을 최대15초 기다림 | 같은 native harness 재실행. 앱의 저장/Worker 실패와 분리 |

## 보류 결함/기술 부채 (재현 또는 범위 제한)
- D00-001 Storage: 손상된 project 하나의 `open` 오류가 `list` 전체로 전파된다. fail-closed를 유지한 최소 구현의 UX 제한. DEV-01에서 개별 오류 항목/복구 UI로 분리; 원본 성공 위장 금지.
- D00-002 Runtime: job 상태는 메모리뿐이며 강제 app 종료 시 자식 트리 관리/재개를 보장하지 않는다. 현재 dummy는 짧은 CPU 프로세스. 실제 장시간 provider 전 OS job object/checkpoint가 필요하다.
- D00-003 Export: Live2D authoring/권리와 실제 VBridger preset writer 미검증. 생성 기능을 비활성화한 이유이며 버그 수정으로 해결됐다고 기록하지 않는다.

새 오류에는 ID/DEV/모듈/증상/재현/증거/검증된 원인 또는 미확정/수정/기각 이유/회귀/예방/결과를 추가한다. 외부 도구의 실패를 제품 테스트 pass로 바꾸지 않는다.

## DEV-01 실패 및 해결

| ID / 모듈 | 재현·증상 | 근본 원인 / 확인 범위 | 수정 | 회귀·검증 |
|---|---|---|---|---|
| F01-001 test environment | 변경 전 baseline UI assertion 성공 후 종료 정지; 재실행은 ui-test.log 점유 오류 | Playwright/Vite 자식과 원래 PowerShell이 남아 로그를 점유한 것은 확인. sandbox 종료 지연의 세부 원인은 미확정 | 해당 baseline 프로세스 트리만 종료; 동일 suite를 승인된 실행 환경에서 재실행 | dev-01-baseline-clean.log 전체 통과. 코드/버전 변경으로 숨기지 않음 |
| F01-002 authoring tool | Python patch read_text가 일본어 포함 소스에서 UnicodeDecodeError | Windows 기본 cp949로 UTF-8 파일을 읽음. 쓰기 이전 실패로 원본 변화 없음 | 모든 읽기/쓰기 encoding=utf-8 명시 | 이후 patch/format/build 통과 |
| F01-003 SQLite integration | SQL parameter u64 ToSql 컴파일 오류 | rusqlite0.41의 u64 ToSql 부재 | 상한이 검증된 revision/time을 i64 parameter로 변환 | cargo check/SQL 재구축 테스트 통과 |
| F01-004 index durability | SQLite 재구축에서 access_denied, 두 회귀 실패 | read-only File handle에 sync_all 호출. Windows FlushFileBuffers에 쓰기 권한 필요 | index 임시 파일을 read/write handle로 열어 sync_all | 같은 sqlite_missing... 재현 후 전체28 Rust 통과; index 실패에도 revision 보존 시험 |
| F01-005 test UI | Node에서 JSON import needs type json | Playwright의 ESM JSON import attribute 누락 | with {type:json} 추가 | recovery UI와 기존 UI 2개 통과 |

D00-001은 list v2의 per-project 결과 및 명시 복구로 해결했고 정상 항목 독립 사용을 Rust/native에서 검증했다. D00-002의 메모리 전용 job은 journal로 해결했다. 전체 OS 자식 트리 종료/실제 AI checkpoint 재개 부분은 DEV-28에 남으며 해결로 표시하지 않는다. F00-002/006/008은 기존 실제 Python 경로·lockfile·SDK 환경을 재사용했고 F00-009 화면 캡처를 반복하지 않았다. native harness는 F00-013의 URL 준비 조건을 유지한다.

### F01-006 — native restart harness readiness

DEV-01 native 재실행의 두 번째 launch에서 CDP ECONNRESET이 관찰되었다(artifacts/dev-01-native-final.log). HTTP endpoint 응답 뒤 실제 WebSocket 연결을 별도 수행해 준비 조건 사이에 단절을 허용한 것이 확인되었다. 직전 프로세스의 같은 포트 endpoint 잔존은 가능한 원인이며 확정하지 않는다. 재시작마다 OS가 제안한 빈 loopback 포트를 사용하고 실제 CDP connect 성공 및 tauri.localhost navigation을 bounded poll로 확인하도록 수정했다. 앱 데이터 코드는 바꾸지 않았다. 같은 native 전체 흐름 재실행은 dev-01-native-final-retry.log에서 통과했다.


## DEV-02 failures and evidence

| ID | Reproduction / symptom | Confirmed cause | Fix / verification |
|---|---|---|---|
| F02-001 baseline isolation | First baseline compile saw newly-added module/dependency mismatch | Edited source before baseline process finished; not a DEV-01 defect | Preserve log; extract exact5d38dcb into E/.tools/dev02-baseline, run full suite successfully (dev-02-baseline-isolated.log) |
| F02-002 reference fixture writing | psd-tools save failed on Japanese/Korean legacy name | reference writer attempted macroman encode; original project files untouched | ASCII independent fixture plus separate Rust luni Unicode fixture; log reference-fixtures.log/2.log |
| F02-003 PSD groups/padding | Independent valid grouped PSD rejected | Bottom-to-top divider ordering and2-byte trailing zero padding differed from initial assumption | Bounded zero padding≤3 and group marker balance; independent groups.psd + expected RGBA fixture regression |
| F02-004 reference pixel comparator | RGB reference and RGBA output byte arrays differed | Test helper compared different channel layouts | Normalize RGB reference alpha255, rerun independent numeric tests; production parser unchanged |
| F02-005 direct cargo environment | Two existing journal tests unwrapped missing Python env | Direct cargo shell did not set VTUBER_PYTHON, unlike scripts/test.ps1 | Explicit actual Python3.12 path; same full Rust regression passed (dev-02-rust-full2.log) |
| F02-006 UI tooling result | sky.click reported coordinate input geometry unavailable | Error response confirmed; underlying click effect initially unknown | Fresh window list showed dialog closed; independent native IPC/source hash/reload checks passed. No repeated blind click. No claim helper geometry root cause known |

Initial source test compile/lint errors (unused React import, constant-chunk lint) were corrected with TypeScript build/clippy; no dependency version changes were used to hide errors. The first reference package needed attrs, installed only in E with explicit version.

F02-007 UI mock regression: existing simulated Tauri fixture omitted currentWindow/currentWebview and event-plugin unregisterListener. Adding native drop subscription exposed that omission (suite3/UI2 logs); actual native app worked. Mock now represents event metadata/cleanup and subscription setup catches synchronous failures. Added source UI test; full final suite reruns all3UI tests.
F02-008 bounded-memory compatibility: the first new RGBA16 buffer160MiB cap accidentally reduced accepted8-bit canvases below the old40M limit. Windows measurements showed40M RGB8 uses≈445.5MB peak. Keep original64MiB input/40M canvas/160MiB decoded caps and set separate320MiB working cap; final40M succeeds and40,005,000 pixels refuses. No original resize or quality reduction was used.

F02-009 legacy PSD names: review identified UTF-8 lossy decoding of Pascal names when no luni block exists. A MacRoman Café fixture demonstrates the difference. Decode bounded Pascal bytes with encoding_rs0.8.42 MACINTOSH, preserve authoritative UTF16BE luni, and warn for region-dependent non-ASCII legacy fallback. Original bytes never changed. Unit/name bounds and all previous PNG/PSD/UI/native regressions are rerun before merging the new head.

F02-010 review hardening: global unknown extension count previously had only the64MiB file bound, allowing many repeated diagnostics. A crash was not reproduced; this was a code-review resource risk. Added4096-block budget,1MiB total layer-name budget and deduplicated warning keys. Regression verifies8 repeated blocks produce one diagnostic and4097 blocks fail image_too_large. No source mutation or Normal-mode fallback was introduced.

## DEV-03 failures and review hardening

| ID | Reproduction/evidence | Cause and correction | Regression/result |
|---|---|---|---|
| F03-001 | baseline UI assertions passed but sandbox runner remained; dev-03-baseline.log | Remaining Playwright/Vite tree confirmed, environment-specific termination cause unknown (F01-001). Stop only exact baseline tree; rerun unchanged source with authorized execution | dev-03-baseline-clean.log full pass; no version changes |
| F03-002 | install1 failed after55%; direct venv succeeds; install2 with8-process budget succeeds | Windows venv redirectors add processes. Four-process budget was insufficient in observed run; initial generic exception alone does not prove every causal detail. Increase bounded budget to8, keep512MiB | actual installation/native; no global Python modification |
| F03-003 | smoke-repro traces reject gethostname, then platform ver fallback | overly broad Python audit blocked local host-name and trusted stdlib OS query. Cache platform.uname before guard and allow only local gethostname | actual ONNX three vectors and denied socket attempt; Python2 log pass |
| F03-004 | native-runtime1 rejected-consent assertion could not see code | Playwright wraps page.evaluate rejection objects. Harness now returns explicit success/error envelope then asserts outside page | native-runtime2 pass, product consent code unchanged |
| F03-005 | review identified venv redirector spawning before OS assignment | Direct venv launcher could create descendant before ownership. Always start trusted base bootstrap, await assigned stdin, then spawn venv child | actual descendant timeout and RAMlimit tests pass; no claim malicious bootstrap sandbox |
| F03-006 | review found report was read as arbitrary JSON and corrupt installation looked absent | Add version/job/checksum report envelope, typed artifact/content-address revalidation and blocked install state | report_and_artifact_tampering_and_corrupt_installation_are_not_certified passes |

Test-authoring corrections: groups-composite.png fixture name was nonexistent (use groups-merged.png); Windows handle constant is PROCESS_SYNCHRONIZE; JSON-import schema_version is widened number in TypeScript (assert1 then literal typed copy). Compile errors occurred before affected tests, retained in logs and rerun. No dependency roulette or source resets. F00 Python/SDK/Tauri versions, UTF8 handling and native CDP readiness retained.

F03-007 release byte audit: Git staging reported CRLF→LF for runtime-lock.json. Because environment receipts bind exact lock bytes, the previous local CRLF environment differs from a clean Git checkout. Normalize the file to repository LF, add a contract assertion preventing CRLF, create a new verified environment generation and rerun final runtime tests/build/native. Existing generations are retained. No wheel version/hash changes or bypass of integrity validation.

F03-008 commit identity: git commit refused because repository identity was unset. No commit was made. Read prior commit authors and reuse their Kawauso / GitHub noreply identity via per-command git -c options; do not write global C-drive configuration.
