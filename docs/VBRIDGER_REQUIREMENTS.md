# VBridger mandatory compatibility

VBridger 호환성은 선택적 장식 기능이 아니라 출시 요구사항이다. 다만 DEV-00에 프로그램 내 실동작은 **미실행**이다. 설치 탐색은 PATH/알려진 도구 위치 범위이며 PC 전체의 부재를 단정하지 않는다. 사용자 계정·구매·실행/트래킹 권한 없이 설치를 가장하지 않는다.

## 독립 profiles
| ID | 제작물 | 방송 시 연결 | DEV-00 상태 |
|---|---|---|---|
| vbridger-vts | Cubism runtime bundle + 모델별 input/output mapping + creator preset 참조 | VBridger → VTS WebSocket API → Live2D parameters | profile/정적 validator 구현; exporter/실동작 미검증 |
| vbridger-vmc | 대상 receiver용 ARKit 표정 모델 + 출력명/clip name 매핑 | VBridger → VMC 얼굴 데이터 → VSeeFace/Warudo/VNyan 등 | profile 구현; 전송·receiver별 실동작 미검증 |

[VBridger 공식 설명](https://store.steampowered.com/app/1898830/VBridger/)에서 VMC는 얼굴만 지원하며 머리 회전·안구 회전·전신 제어는 지원하지 않는다고 명시한다. 전신은 다른 트래커/receiver 기능을 검증해야 한다. VRM이 존재한다는 사실만으로 ARKit 표정이나 모든 VMC 기능이 생기지 않는다.

## 표정과 파라미터 계약
검사할 의미 채널: jaw.open/forward/left/right, mouth.close/funnel/pucker, 좌우 smile/frown/stretch/dimple/press, upper/lower lip 이동·말림, cheek.puff/squint, 좌우 blink/wide/squint, brow 움직임, tongue(대상 지원 시), 표정 조합과 비대칭. 이것은 우리 제품의 **의미 요구 목록**이며 VBridger의 공식 고정 ID 목록이라고 주장하지 않는다.

VBridger output 이름 → VTS input parameter → 모델의 Live2D parameter ID의 세 계층을 저장한다. min/neutral/max, 반전, sensitivity/curve/delay, clamp, 표정 우선순위, neutral 보정, 좌우 규약, preset hash와 버전을 보존한다. VTS API의 InputParameterList/Live2DParameterList로 실제 목록과 범위를 대조한다. 하나의 parameter에 여러 플러그인이 동시에 쓰는 충돌도 검사한다. [공식 VTS API](https://github.com/DenchiSoft/VTubeStudio).

기본 preset과 creator 설정은 분리한다. [매뉴얼 1.06](https://cdn.cloudflare.steamstatic.com/steam/apps/1898830/manuals/VBridger_Manual_1.06.pdf)은 기본 앱이 제작자가 만든 설정을 로드할 수 있고, Editor DLC가 custom 설정/출력 편집에 쓰인다고 설명한다. `.vbridger` 저장 파일은 확인됐지만 공개되고 안정적인 writer schema/자동화 API는 확인하지 못했다. DEV-00에서 생성하지 않으며 임의 JSON을 그 확장자로 저장하지 않는다. 자동 파일 생성은 권리 확인된 실제 fixture round-trip 후 DEV-11/24에서 판단한다.

## 정적 최소 실험
`validateBindings`는 필수 semantic 누락, 중복 target/semantic, NaN/무한대/역전 범위/neutral 범위를 검사한다. `tests/contracts.test.ts`는 성공·누락·역전·중복 synthetic mapping을 실행한다. 이 결과는 VBridger preset 인증도, 모델의 실제 변형 품질 검증도 아니다. 공식 parameter guide의 완전한 버전 고정 fixture 확보는 미해결이다.

## 실제 검증 절차 / 통과 조건
1. 사용자 보유 VBridger(필요 시 Editor DLC), VTS 또는 선택한 VMC receiver의 정확한 버전을 기록한다. 우리가 상용 앱을 재배포하지 않는다.
2. 라이선스 허용된 최소 모델과 preset을 준비하고 hash/출처를 고정한다. VTS API 허용은 사용자가 수행한다.
3. neutral, 각 채널 단독 0/0.5/1, 좌우 비대칭, blink+smile+jaw, yaw/pitch와 표정 조합을 녹화하고 값/이미지를 보존한다.
4. 설정 저장→앱 재시작→다시 로드 후 mapping/curve/delay와 표정이 동일한지 확인한다. missing channel, 중복 writer, 비활성 input의 복구 안내를 확인한다.
5. VMC는 receiver별 clip 이름 및 좌우·범위 매핑, 표정 toggle, 누락/비지원 채널 보고를 검증한다. head/eye/body를 VBridger 지원으로 인증하지 않는다.
6. 정적 검사와 실제 실행 증거가 모두 해당 export hash에 연결될 때만 Gate D 해당 profile에 pass를 준다. 한 receiver의 pass를 다른 앱에 복사하지 않는다.

차단 조건: 상용 프로그램/테스트 모델 미확보, 공개 preset 스키마 미확인, 버전별 output convention 미고정. 해결 경로: 권리 확인된 설정/테스트 데이터 확보→허용된 로컬 연결→수동·자동 실측. 제품 핵심 무료 정책은 외부 사용자가 선택한 상용 companion의 구매 비용까지 무료화한다는 뜻이 아니다.
