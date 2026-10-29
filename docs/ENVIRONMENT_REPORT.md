# Environment report

조사 시작 2026-10-25 JST, 검증 진행 2026-10-26 JST. 작업 루트 `E:\Project-VTuber`. 처음 저장소는 `main...origin/main`, clean, README 16bytes만 존재, initial commit `47efb6c`였다. 원격은 `https://github.com/Raccoon26/Project-VTuber.git`. AGENTS.md는 확인한 루트/저장소에서 발견되지 않았다. 사용자 코드를 초기화하거나 삭제하지 않았다.

| 항목 | 실측/확인 | 방법·한계 |
|---|---|---|
| OS | Windows 10 Pro, 10.0.19045, x64 | CIM/RuntimeInformation |
| Shell | PowerShell 7.6.5 | PSVersionTable |
| CPU | AMD Ryzen 7 4800H, 8 cores / 16 logical | Registry + CIM |
| RAM | 42,812,522,496 bytes (약39.87 GiB) | CIM TotalPhysicalMemory, GPU memory와 별도 |
| GPU | NVIDIA GeForce RTX2060, 6144MiB | nvidia-smi query 및 native 앱 |
| Driver | NVIDIA 591.74, Windows32.0.15.9174 | nvidia-smi/CIM |
| Disk 조사시 free | C 약60.6GB, D 약409.8GB, E 약2.785TB (decimal) | Get-PSDrive, 이후 빌드로 변동 |
| Node / npm | 22.15.0 /10.9.2 | 실제 --version |
| Rust 초기 | rustc/cargo PATH 및 .cargo/bin 없음 | 초기 조사 |
| Rust 이후 | 사용자 설치, rustc1.99.1/cargo1.99.1, x86_64-pc-windows-msvc | 실제 --version/rustup show, toolchain pin |
| Python 기본 | WindowsApps alias 실행 실패 | 실행 오류를 정상 설치로 간주하지 않음 |
| Python 검증 | Codex bundled Python3.12.14 | 실제 Worker/단위 테스트 사용 |
| C++ SDK | VS2022와 MSVC/Windows SDK 존재; 이후 다른 VS 설치 경로도 발견 | vswhere, release link 진단; scripts/env.ps1이 완전한 SDK 선택 |
| WebView2 |155.0.4283.62 | 설치 경로, 실제 Tauri 실행 |
| Blender | PATH/기본 설치 위치에서 발견 안 됨 | PC 전체 부재로 단정하지 않음 |
| 방송 앱 | 이 작업에서 실행·연결하지 않음 | 설치 여부 전수조사/상용 구매 안 함 |
| GPU inference | 미검증 | nvidia-smi는 실제 CUDA/PyTorch/ORT tensor 실행 증거가 아님 |

사용한 Python 실행 파일: `C:\Users\cksgu\.cache\codex-runtimes\codex-primary-runtime\dependencies\python\python.exe`. 기존 설치를 읽어 실행하며 새 프로젝트 자료는 여기에 저장하지 않는다. 배포 시 이 사용자별 경로를 하드코딩하지 않고 `VTUBER_PYTHON`으로 공급한다.

## 권한·충돌과 저장 정책
초기 sandbox에서 CIM과 `.git` 쓰기는 접근 거부되어 읽기 전용 하드웨어 진단/작업 브랜치 생성만 정식 escalation으로 수행했다. 오류를 GPU 없음으로 해석하지 않았다. 초기 npm registry 조회도 제한 환경에서 지연되어 허용된 네트워크 실행으로 의존성을 받았다.

사용자 추가 지침 이후 `scripts/env.ps1`과 `.npmrc`로 cache/TEMP/TMP/Cargo target/Playwright/Python cache/앱 데이터/WebView 위치를 E checkout 아래로 지정했다. 이번 작업에서 생성한 C Cargo registry는 정확한 경로를 확인 후 E로 이동했다. 사용자 설치 Rust toolchain/공유 npm cache/Visual Studio는 무단 제거하지 않았다. Windows 자체 진단·Codex 운영 로그의 저장 경로는 이 프로젝트 통제 밖이다.

대형 AI 모델 다운로드0, GPU inference/부하 시험0, 유료 API 호출0, 사용자 이미지 외부 전송0. 공개 개발 의존성/Chromium/GitHub CLI만 다운로드했으며 GitHub CLI는 release checksum을 확인했다. 실제 검증에는 저장소의 합성 PNG만 사용했다. 작업 중 추가 개발 도구 설치와 source build에 따른 disk 증가는 E의 `.tools`, `node_modules`, `src-tauri/target`에 남는다.

## 재현
`git status --short --branch`, `nvidia-smi --query-gpu=name,memory.total,driver_version --format=csv,noheader`, `rustc --version`, `cargo --version`, `node --version`, `npm --version`, 명시 Python `--version`을 기록한다. CIM 실패는 권한 문제와 장치 부재를 분리한다. raw 로그에는 사용자 경로가 들어가므로 ignore된 artifacts에 두고 공개 저장소에는 요약만 포함한다.

## DEV-01 재확인 — 2026-10-27 JST

base main ac7e257 및 CI40314991998 success, clean tree에서 codex/dev-01-project-resilience 생성. 기존 실제 Python3.12.14 경로/Node22.15.0/Rust1.99.1/SDK env를 유지했다. E 드라이브 DriveInfo는 NTFS, 조회 시 가용 2,777,176,973,312 bytes였다. 기존 app-data 프로젝트5개는 구조만 열거하고 테스트에는 새 tempfile/native fixture를 사용했다. 추가한 rusqlite/SQLite 및 캐시/빌드/로그는 E 작업 루트에 있다.

## DEV-03 2026-10-29 observed environment

Ryzen7 4800H8cores/16logical, RAM41,809,104KiB; RTX2060 total6144MiB/free4332MiB at sample, driver591.74. E local NTFS capacity4,000,750,497,792B/free2,769,608,577,024B. Actual Python3.12.14 from existing installation is read-only; new provider environments/caches are E-only. These values change with other applications. CPU ONNX tested; GPU tensor backend unverified. No Rust/Python installation or C cleanup was changed.
