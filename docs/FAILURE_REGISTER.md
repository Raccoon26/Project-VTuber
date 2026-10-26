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
