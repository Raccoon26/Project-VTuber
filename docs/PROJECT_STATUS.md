# Project status — DEV-00

2026-10-26 JST. 로컬 기반 구현과 실제 Windows 앱 검증을 수행했다. **GitHub CI/PR/main 병합을 확인하기 전 DEV-00 최종 완료로 취급하지 않는다.** 원격 결과는 최종 PR과 아래 release evidence에 연결한다. 제품 전체 완성이 아니다.

## 완료한 구현/조사
- 깨끗한 initial commit 저장소 조사, 작업 브랜치 분리, CPU/GPU/RAM/disk/tool 확인.
- Tauri2/React/TS/Rust 실행 파일, 로컬 Python JSONL 더미, 진행률·취소·실패·timeout 경계.
- 프로젝트 생성/이름 저장/다시 열기, 불변 revision, PNG 원본 hash 보존 및 변조 감지.
- NVIDIA GPU 진단(추론 지원 판정은 미검증), 방송 profile 10개, VBridger 2D/3D 독립.
- ja/en/ko 기반, 초보/전문가 표시, offline 광고 adapter, 모든 미구현 exporter 차단.
- 22개 모듈, 현재/미래 모델 계약, ADR, A–F 품질 기준, 실패 기록, 62개 요구사항과 DEV00–33 로드맵.
- Live2D 공식/제3자 출력 경로, VBridger VMC 얼굴 한계, 모델 VRAM/권리 및 광고 후보 조사.
- Windows release build와 실제 native WebView→Rust→Python smoke 통과. 합성 원본만 사용했다.

## DEV-00 완료 조건 현황
환경/요구사항/핵심 기술·라이선스 조사/모듈·데이터 설계/실행 기반/저장/Worker/진단/profile/i18n/광고 인터페이스/품질·오류 체계/로드맵은 작성·검증 범위를 [TEST_RUNS](TEST_RUNS.md)에 기록했다. 최종 로컬 회귀(12 Rust/2 Python/4 계약/1 UI), 정적 검사와 문서 audit는 통과했다. GitHub CI·main 병합 검증은 release evidence를 기준으로 판정한다. 정식 모델 생성 및 외부 앱 호환 인증은 DEV-00 완료 조건에 포함된 것처럼 가장하지 않는다.

## 미해결 / 재개 조건
| 항목 | 원인/증거 | 다음 조치·담당 |
|---|---|---|
| 자동 cmo3/moc3 authoring·배포 | 공식 Editor 출력은 확인, 일반 compiler API/제3자 권리·round-trip 미검증 | DEV-11: 권리 확인된 최소 모델+공식/제3자 isolated 실험 |
| VBridger 실제 인증 | commercial 앱/preset fixture/receiver 실험 미실행 | DEV-24: 사용자 보유 앱/version과 권리 fixture 확보 후 얼굴 채널 sweep |
| 전문가급 2D/3D 리깅 | 구현/품질 corpus 없음, 생성 모델만으로 해결 불가 | DEV-04–24/32, blind 전문가 평가 |
| 6GB에서 대형 3D 모델 | upstream 요구16/24/10–29GB, SkinTokens14GB | DEV-03/12: 허용된 경량/offload 후보 실측 또는 적합 장치 승인 |
| Hunyuan/RigAnything 상업 경로 | 지역 제한/비상업 연구 license | 기본 배포 금지; 대체 모델 또는 별도 허가 |
| source importer 범위 | PNG 64MiB/40M만; PSD·색 renderer 미구현 | DEV-02: bounded PSD/full-res color pipeline |
| 손상 항목이 목록 전체를 막음 | fail-closed list가 open 오류를 전파 | DEV-01 우선: 손상 항목 격리+복구 UI |
| 복구/작업 영속성 | revision 보존만, job 상태 메모리, 강제종료 child tree/전원차단 미검증 | DEV-01/28: journal/OS job object/fault injection |
| 광고 상용 연결 | desktop/Japan/지급/DPA 계약 미확인 | DEV-27: 승인 전 disabled |
| 배포 패키지 | 시스템 Python 필요, unsigned exe, installer 미구현 | DEV-30: sidecar/서명/clean-machine 검사 |
| Windows native file picker UI | Windows capture helper 2회 timeout; importer 코어는 실험 완료 | 실제 dialog click 흐름 수동 재검증; native IPC smoke와 구별 |

## 저장 위치
후속 사용자 지침에 따라 신규 작업 데이터/cache/temp는 E에 고정했다. 지침 전 생성한 Cargo registry는 E `.tools/cargo/registry`로 이동했다. 기존 Rust 설치와 공유 npm cache 등 사용자 도구 경로는 무단 삭제하지 않았다. 앱/WebView data도 E에 생성된 것을 확인했다. OS/Codex 자체 로그까지 통제했다고 주장하지 않는다.

## Release evidence
작업 브랜치 `codex/dev-00-foundation`, base `47efb6c0f80a37a58e584c17cca1a4955c977121`. 변경은 [PR #1](https://github.com/Raccoon26/Project-VTuber/pull/1)에 연결되어 있다. [해당 PR의 Checks](https://github.com/Raccoon26/Project-VTuber/pull/1/checks)에서 최종 head의 Windows 테스트·release build·의존성 감사를 확인하고, PR의 merged 상태와 merge commit으로 최종 완료 여부를 판정한다. 문서 작성 시점과 병합 시점은 다르므로 병합 전 상태를 성공으로 기록하지 않는다.

2026-10-26 조회 결과 main은 protected=false, 적용 branch rules 없음, 요구 review 없음이었다. merge 직전에 다시 확인하며, 필수 조건이 없어도 Windows CI 성공을 요구하고 관리자 강제 merge를 쓰지 않는다. 병합 뒤 로컬/원격 main SHA 일치와 구현 commit 포함 여부를 확인한다. [main CI](https://github.com/Raccoon26/Project-VTuber/actions/workflows/ci.yml?query=branch%3Amain)도 별도로 확인한다. 최종 merge SHA 및 검증 결과는 PR과 작업 최종 보고에 남긴다.

DEV-01 시작은 [로드맵의 시작 지침](DEV_ROADMAP.md#dev-01-정확한-시작-지침)을 따른다.
