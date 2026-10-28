# License and redistribution audit

검토 기준일 2026-10-25~26 JST. 코드·weights·데이터·산출물·SDK 계약을 분리한다. 공개 저장소나 무료 다운로드는 재배포 허가와 동의어가 아니다. 본 앱 자체 배포 라이선스는 소유자 결정 전이며 `package.json private=true`, Cargo publish=false다. DEV-00은 개발 기반을 GitHub에 반영하는 작업이지 모델/상용 SDK가 포함된 판매용 배포가 아니다.

| 구성요소 | 확인 범위·근거 | 상업/재배포 상태 | DEV-00 |
|---|---|---|---|
| React / TypeScript / Vite / Vitest / Playwright | 설치 package metadata와 lockfile의 version/integrity/license | 주로 MIT/Apache 계열; 전체 전이 의존성 notice 목록은 배포 전 감사 | 개발/런타임 의존성 포함 |
| Tauri / Rust crates | Cargo.lock exact version/checksum; package license metadata | MIT/Apache 등 각 crate 조건, WebView2 배포 방식 별도 확인 | Rust core 포함 |
| Python | 기존 설치 3.12.14 실행, 표준 라이브러리만 사용 | Python을 직접 번들하지 않음; portable 배포 시 PSF 및 포함 모듈 notices 필요 | executable 경로만 사용 |
| Live2D Cubism | [SDK 공식 계약](https://www.live2d.com/sdk/license/) / [Editor](https://www.live2d.com/en/cubism/about/) | runtime Core/Editor/출판 허가 개별 조건. 편집 도구·라이선스 비용·출시 계약 미확정 | 미번들, 실행 안 함 |
| Inochi2D runtime | [LICENSE](https://github.com/Inochi2D/inochi2d/blob/main/LICENSE) BSD-2-Clause | copyright/license 보존. Creator/Session 및 sample artwork 별도 | 후보만 |
| Ayatsuri2D | [upstream licensing](https://github.com/Yoru-KenomoVT/ayatsuri_2d) | GPL-3.0 app, MIT openmoc3, Apache ABI 등 혼합; 독점형식 관련 권리/통합 의무 미확인 | 통합 보류 |
| SAM2 | [공식 repository](https://github.com/facebookresearch/sam2) | code/weights Apache-2.0 명시, demo font/data 별도 | 미설치 |
| TRELLIS/TRELLIS.2 | [원본](https://github.com/microsoft/TRELLIS), [2](https://github.com/microsoft/TRELLIS.2) | 본체 MIT; renderer/추가 weights별 조건 별도 | 미설치 |
| Hunyuan3D2.1 | [LICENSE 원문](https://github.com/Tencent-Hunyuan/Hunyuan3D-2.1/blob/main/LICENSE) | Community License, EU/UK/한국 제외, 조건부 상업 사용, 추가 상업 조건/배포 notice/AUP. 현재 원문은 1M MAU 기준 추가 허가 조항 포함 | 전세계 기본 번들 Blocked |
| UniRig | [LICENSE](https://github.com/VAST-AI-Research/UniRig/blob/main/LICENSE) MIT | repository 조건 확인; 실제 weight/dependency/data provenance 감사 별도 | 후보만 |
| SkinTokens | [repository](https://github.com/VAST-AI-Research/SkinTokens) MIT 표시 | 배포할 정확한 checkpoints/기반 모델/학습 데이터 조건 별도 검토 | 후보만 |
| RigAnything | [Adobe Research License](https://github.com/Isabella98Liu/RigAnything/blob/main/LICENSE.md) | 비상업 연구 전용, 상업 제품 개발 제외 | 제품 경로 Rejected with Reason, 별도 허가 전 미실행 |
| Blender/addons | [Blender license](https://www.blender.org/about/license/) | Blender GPL, artwork와 앱 라이선스 구별; Python addon 배포 의무 별도 | 미설치/미번들 |
| VBridger/방송 앱 | 각 공식 배포처의 EULA | 외부 상용/무료 앱의 별도 설치·사용 권리; 앱 재배포 허용을 가정하지 않음 | 미번들 |
| 광고 SDK | [MONETIZATION](MONETIZATION.md) | desktop/Japan/DPA 계약 확인 전 보류 | 미설치 |

## 감사 산출물
`package-lock.json`과 `src-tauri/Cargo.lock`이 구현 의존성을 고정한다. `scripts/dependency-inventory.mjs`가 npm metadata와 Windows 타깃 Cargo metadata에서 명칭·버전·라이선스·출처를 출력하며, raw inventory는 `artifacts/dependency-inventory.json`에 저장한다. DEV-00 최종 inventory는 formatter 포함 npm86/Rust250개 항목이다. 생성 목록은 권리 검토 결과가 아니고 license 누락/비표준 항목은 출시 전 개별 검토해야 한다. AI weights가 0개이므로 model checksum을 지어내지 않는다.

개발/컴파일 의존성의 OS별 선택도 기록한다. MIT/Apache라고 전체 제품이 무조건 출시 가능하다는 blanket clearance를 하지 않는다. DEV-29/30에서 실제 배포 binary에 포함되는 dependency closure, full notices, binary-source 대응과 SDK 계약을 잠근다. 출시 차단 항목이 해소되기 전 해당 기능의 enabled/redistributable을 true로 바꾸지 않는다.

## DEV-01 의존성 추가

rusqlite0.41.0(bundled) 및 libsqlite3-sys0.39.1을 Cargo.lock으로 고정했다. [rusqlite 공식 문서](https://docs.rs/rusqlite/0.41.0/rusqlite/)의 API에 따라 로컬 SQLite를 사용한다. rusqlite MIT, SQLite public-domain 표기와 정확한 전이 의존성 metadata는 dependency-inventory로 확인한다. 정식 배포 전 전체 notice/라이선스 closure 감사 의무는 유지하며 유료 DB/외부 DB 서비스는 사용하지 않는다.


## DEV-02 integration

DEV-02: lcms2=6.2.0 MIT and lcms2-sys=4.0.7 MIT (static LittleCMS2.19 family MIT). flate2 existing transitive dependency now explicit (MIT/Apache-2.0); base64=0.22.x MIT/Apache-2.0. Exact checksums remain Cargo.lock. psd-tools1.26.0 MIT plus attrs25.4.0 MIT are E-only independent test tools, not bundled. No Photoshop SDK, paid API, commercial editor or user artwork was acquired or redistributed. Synthetic PSD/reference fixtures contain only generated solid shapes; retain notices for distributed dependencies.
