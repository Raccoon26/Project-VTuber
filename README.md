# Project VTuber AI

2D 캐릭터 일러스트로부터 편집 가능한 2D 또는 실제 3D 모델을 만들고, 제작 단계에서 리깅을 완성해 방송 앱으로 내보내는 로컬 데스크톱 도구를 개발합니다.

**DEV-01 저장·복구 기반입니다. 실제 AI 생성·자동 리깅·모델 내보내기는 아직 구현되지 않았습니다.** CPU 더미 Worker는 저장·작업 진행·취소·오류 처리만 검증합니다. 현재 PNG를 원본 bytes 그대로 보존하고 프로젝트를 저장/다시 열 수 있습니다.

## Windows 개발 실행

필수: Node.js 22.15.0, Rust 1.99.1 MSVC, Visual Studio C++ 도구와 Windows SDK, WebView2, 정상 실행 가능한 Python 3.12. Python 모델 패키지/CUDA Toolkit/Blender/상용 방송 앱은 DEV-00 실행에 필요 없습니다.

이 사용자 환경에서는 모든 신규 파일/캐시/임시 파일/앱 데이터를 **E:\Project-VTuber** 아래에 둡니다. 먼저 PowerShell에서:

```powershell
Set-Location E:\Project-VTuber
. ./scripts/env.ps1
$env:VTUBER_PYTHON = '설치된 Python 3.12 실행 파일의 절대 경로'
npm ci
./scripts/dev.ps1 -Python $env:VTUBER_PYTHON
```

현재 이 컴퓨터의 검증된 Python 경로는 `docs/ENVIRONMENT_REPORT.md`에 있습니다. 기존 C/D 드라이브의 도구를 읽어 실행하는 것과 그곳에 작업 데이터를 저장하는 것은 구분합니다. `scripts/env.ps1`은 사용자 HOME/CODEX_HOME이나 시스템 전역 설정을 바꾸지 않습니다.

개발 화면만 확인할 때는 `npm run dev`를 사용합니다. 브라우저 preview에서는 파일 생성/저장이 차단되며 실제 동작은 Tauri desktop에서 수행합니다.

## 검증 및 빌드

```powershell
. ./scripts/env.ps1
npx playwright install chromium
./scripts/test.ps1 -Python $env:VTUBER_PYTHON
npm run desktop:build
```

마지막 명령은 installer 없이 Windows release 실행 파일을 만듭니다. 결과는 `src-tauri/target/release/vtuber-desktop.exe`, Worker는 옆의 `workers/dummy.py`입니다. 실행할 때 같은 환경의 `VTUBER_PYTHON`이 필요합니다. standalone Python 번들·서명된 installer는 DEV-30 범위입니다. 첫 의존성 설치에는 네트워크가 필요하며 AI 모델은 자동 다운로드하지 않습니다.

## 사용

프로젝트 이름/2D 또는 3D/대상 앱 선택 → 프로젝트 생성 → PNG 선택 → 더미 작업 → 취소 또는 완료 확인 → 저장된 프로젝트 다시 열기. 이름 저장은 새 revision을 추가합니다. 원본 해상도/색 정보가 들어 있는 파일 bytes는 재압축하지 않습니다. 지원 한도는 64MiB/40M pixels이며 초과 입력을 몰래 축소하지 않습니다. 모든 품질 Gate는 미검증, 모든 exporter는 비활성입니다.

기본 데이터는 실행 파일 옆 `vtuber-data`; 개발 실행에서는 `.tools/app-data`를 씁니다. WebView 캐시도 E 경로입니다. 백업은 앱을 닫고 데이터 루트의 프로젝트 디렉터리 전체를 복사하세요. revision/source 일부만 지우면 복원이 불가능할 수 있습니다. 깨진 최신 revision을 조용히 이전 것으로 바꾸지 않습니다. 손상 항목의 진단 → 이력 선택 → 미리보기 → 확인 체크 → 새 복구 버전 만들기로 복구합니다. 기존 손상 파일도 보존합니다.

## 문서

- [현재 상태와 차단 항목](docs/PROJECT_STATUS.md), [DEV-00–33 로드맵](docs/DEV_ROADMAP.md)
- [78개 요구사항 추적](docs/REQUIREMENTS.md), [아키텍처](docs/ARCHITECTURE.md), [ADR](docs/DECISIONS.md), [공통 계약](docs/MODEL_CONTRACT.md)
- [기술 조사](docs/TECH_RESEARCH.md), [최소 실험](docs/TECH_SPIKES.md), [모델 전략](docs/MODEL_STRATEGY.md), [권리 감사](docs/LICENSE_AUDIT.md)
- [호환성 표](docs/COMPATIBILITY_MATRIX.md), [VBridger 필수 조건](docs/VBRIDGER_REQUIREMENTS.md)
- [품질 게이트](docs/QUALITY_GATES.md), [테스트 전략](docs/TEST_STRATEGY.md), [실행 이력](docs/TEST_RUNS.md), [실패 기록](docs/FAILURE_REGISTER.md)
- [실제 환경](docs/ENVIRONMENT_REPORT.md), [보안](docs/SECURITY.md), [다국어](docs/LOCALIZATION.md), [수익화](docs/MONETIZATION.md)

각 DEV는 별도 브랜치에서 검증한 뒤 PR/CI/보호 규칙을 확인해 main에 병합합니다. 미병합은 최종 완료가 아닙니다. 사용자 작품·AI weights·개인 진단 로그는 Git에 올리지 않습니다.

## DEV-01 저장·복구 사용

목록의 검색과 새로고침은 파일 검사와 SQLite 인덱스 재구축을 수행합니다. 정상 프로젝트는 손상 항목과 별도로 사용할 수 있습니다. 복구 미리보기는 저장 데이터 기준이며 캐릭터 렌더링은 아닙니다. 선택한 버전 이후 변경이 복구 상태에서 빠진다는 안내를 확인하세요. 원본 없는 초기 버전으로 복구해도 기존 blob을 삭제하지 않습니다.

앱이 중단되면 마지막 기록을 확인해 interrupted 또는 recovery_required로 표시하며 자동 완료/재개하지 않습니다. 원래 프로젝트를 열어 검증한 뒤 새 더미 작업을 시작할 수 있습니다. `.write.lock`/`.runtime.lock` 파일은 삭제하지 마세요. 잠금 소유 프로세스가 닫히면 OS가 해제합니다.

[저장/복구 세부 계약](docs/STORAGE_RECOVERY.md), [작업 기록](docs/JOB_PERSISTENCE.md), [schema 호환성](docs/MIGRATION_STRATEGY.md). 실제 앱 검증은 E 환경에서 `node scripts/native-recovery.mjs`를 사용하며 별도의 합성 fixture만 생성합니다.
