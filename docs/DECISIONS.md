# Architecture decisions

상태: Accepted = DEV-00 범위의 결정, Provisional = 실험 후 재검토, Rejected = 이유가 있는 기각. 기능 요구사항을 기각한 것은 아니다.

| ADR | 결정·상태 | 이유와 대안 | 위험·교체 방법 |
|---|---|---|---|
| 001 | Tauri 2 + React/TS + Rust, Accepted | Windows WebView2 설치 확인, 최소 명령 IPC. Electron은 대안이나 현재 별도 Chromium 배포 불필요. Qt는 Python UI 통합 가능하지만 웹 i18n/타입 자산 재사용 적음 | WebView 버전 차이; UI와 코어 DTO 유지해 shell 교체 가능 |
| 002 | Python 별도 프로세스, Accepted | 모델 환경 충돌/크래시를 UI에서 분리. 임베디드 CPython·HTTP 서버는 초기 복잡도/공격면 때문에 기각 | 인터프리터 설치 필요; 고정 인터프리터 sidecar는 DEV-30. 프로토콜 유지해 backend 교체 |
| 003 | 원본 blob + immutable JSON revision, Accepted | 손실 없는 bytes 및 저장 전후 비교. 단일 mutable JSON 덮어쓰기는 기각 | hard-link FS 요구; journaling DB로 publication 구현 교체 가능, blob 주소는 보존 |
| 004 | SQLite는 재구축 가능한 인덱스, Provisional | 작은 초기 프로젝트는 파일만으로 검증 가능 | DEV-01에서 성능/복구 실험; 단일 정본 정책은 유지 |
| 005 | 모델별 adapter/환경/라이선스 분리, Accepted | 모델 폐기·지역 제한·CUDA 충돌이 제품 전체를 막지 않음 | 배포 매트릭스 증가; provider ID/manifest를 기준으로 교체 |
| 006 | Live2D 자체 exporter 출시 보류, Provisional | 공식 Editor 출력 경로와 SDK runtime을 혼동하지 않음. Ayatsuri2D는 검증 대상이나 아직 프로젝트 실험·법적 검토 없음 | DEV-11에 권한/편집가능성/파라미터 round-trip 증거 필요. Inochi 경로는 독립 제공하되 VTS 요구를 대체하지 않음 |
| 007 | VBridger-VTS와 VBridger-VMC 독립, Accepted | VMC 얼굴 출력은 전신/머리/안구 회전 지원과 다름 | preset/version별 매핑 필요; profile migration 지원 |
| 008 | unknown은 unsupported/pass와 구별, Accepted | 장치 감지·파일 열기·더미 완료는 품질 인증이 아님 | 검증 UI 복잡도; 상태 enum과 evidence로 일관성 유지 |
| 009 | 광고 기본 비활성, adapter만 구현, Accepted | 계약/개인정보 확인 전 외부 SDK 실행 금지 | 후속 별도 origin/process와 allowlist; 핵심 처리와 독립 교체 |
| 010 | png 크기 한도 초과 시 거부, Accepted | 임의 다운스케일 대신 자원 한도를 명확히 표시 | 대형 일러스트 요구는 유지. DEV-02 tiled decoder/metadata 제한 검증 후 한도 확장 |
| 011 | 후보 모델 즉시 번들 금지, Accepted | RigAnything 비상업 연구 제한, Hunyuan 지역 제한, 6GB 장치의 실제 추론 미검증 | 합법적 대체 후보·상용 허가·경량 구현을 비교, 무단 제한 해제 금지 |
| 012 | 고정 Node/Rust 도구+lockfile+Windows CI, Accepted | 실환경의 Tauri JS/Rust minor 불일치와 테스트 의존성 취약점 발견 | 의존성 변경 시 audit/build 회귀; 자동 force upgrade 기각 |

외부 기술 근거는 [TECH_RESEARCH](TECH_RESEARCH.md), 라이선스 근거는 [LICENSE_AUDIT](LICENSE_AUDIT.md)에 모은다. 최신 버전이라는 이유만으로 모델 품질을 승인하지 않는다.
