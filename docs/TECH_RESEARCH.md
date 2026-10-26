# Technical research — 2026-10-25 JST

공식 문서/개발자 저장소의 명시와 이 프로젝트의 실험 결과를 구별한다. 여기의 논문 데모·upstream 주장은 프로젝트 품질 검증이 아니다. 라이선스는 출시 허가 판단을 위한 기술 검토이며, 계약상 불확실성은 해결 전 배포를 막는다. 변경 가능한 웹 문서는 도입 시 commit과 파일 SHA-256을 다시 고정한다.

## Desktop / runtime 후보
Tauri 2의 Windows 요구사항은 MSVC/Windows SDK/WebView2다. 현재 모두 존재하며 실제 빌드로 확인한다. React/TypeScript는 접근성·번역·상태 화면, Rust는 프로젝트 IO/프로세스 관리, Python은 모델별 어댑터 역할이다. Electron(일관된 Chromium, 큰 배포), Qt/PySide(네이티브 UI/라이선스 선택), 단일 Python 앱(빠른 실험, UI/모델 장애 결합)을 비교하여 ADR-001/002로 Tauri를 채택했다. [공식 Tauri 요구사항](https://v2.tauri.app/start/prerequisites/).

PyTorch는 연구 모델 호환성 후보, ONNX Runtime은 변환 가능한 모델의 CPU/CUDA/DirectML provider 후보다. 단순 설치 여부와 실제 연산 backend 가용성은 다르다. OpenCV는 마스크/기하 처리 후보이며 색 관리·PSD 원본 파서의 대체물이 아니다. 현재 Worker는 Python 표준 라이브러리만 쓰므로 모델별 의존성 충돌을 만들지 않는다. [PyTorch 설치](https://pytorch.org/get-started/locally/), [ORT providers](https://onnxruntime.ai/docs/execution-providers/).

## Live2D / 2D 경로
| 후보 | 확인된 사실 | 미확인·위험 | 판단 |
|---|---|---|---|
| Cubism Editor | 공식 runtime export에서 moc3 및 model3.json, texture 등 출력 | 일반 공개 SDK가 임의 편집 마스터를 cmo3/moc3로 컴파일하는 authoring API라는 근거 없음; 자동 일괄 제작·Editor 원격 API 별도 확인 | 사용자 설치 Editor 경유의 공식 경로를 기준선으로 조사 |
| Cubism SDK | Editor 제작 모델을 앱에서 표시·제어하는 runtime | Core와 framework의 라이선스가 다름; 편집 도구 SDK 이용/출판 계약 조건 확인 필요 | compiler라고 취급하지 않음; 번들 안 함 |
| Inochi2D | 공개 INP/INX 생태계, BSD-2-Clause runtime, 레이어 메시 변형 | VTS는 Inochi 파일을 그대로 여는 경로가 아님; physics/표정 의미 변환 필요 | 독립적인 공개 2D 경로 후보 |
| Ayatsuri2D | upstream 0.5.7 및 이전 release가 cmo3/moc3 입출력과 VTS 지원을 주장 | GPL-3.0 앱, MIT openmoc3, Apache C ABI 등 서로 다른 조건. 제3자 구현·형식 정확성·권리·대상 버전 실제 검증 없음 | isolated spike 후보, 출시 exporter로 승인하지 않음 |

공식 경로 근거: [Editor export](https://docs.live2d.com/en/cubism-editor-manual/export-moc3-motion3-files/), [SDK 개요](https://www.live2d.com/en/sdk/about/), [SDK 계약](https://www.live2d.com/sdk/license/). Inochi 근거: [runtime](https://github.com/Inochi2D/inochi2d), [Creator releases](https://github.com/Inochi2D/inochi-creator/releases). 제3자 경로: [Ayatsuri repository](https://github.com/Yoru-KenomoVT/ayatsuri_2d), [releases](https://github.com/Yoru-KenomoVT/ayatsuri_2d/releases).

결론: `cmo3` 편집 가능 파일과 `moc3` runtime 파일은 별개다. 우리 마스터 JSON을 임의 확장자로 바꾸지 않는다. 독점 포맷이 불가능하다고 단정하지도, 제3자 README만 보고 가능·합법·고품질을 확정하지도 않는다. DEV-11에 최소 geometry/deformer/physics/비대칭 표정 fixture의 생성→Editor 재편집→VTS 동작을 비교한다.

## VBridger
매뉴얼 1.06은 VTS API 연결과 preset/custom 설정을 설명한다. 현재 공식 판매 페이지는 ARKit 호환 VRM에 VMC 얼굴 데이터 출력을 명시하며 출력명과 VRM blendshape clip 이름의 일치를 요구한다. 동시에 VMC로 머리 회전·안구 회전·몸 제어를 보내지 못한다고 명시한다. 오래된 매뉴얼의 2D 설명만으로 3D 불가능이라 결론 내리면 안 된다. [공식 매뉴얼](https://cdn.cloudflare.steamstatic.com/steam/apps/1898830/manuals/VBridger_Manual_1.06.pdf), [현재 제품 설명](https://store.steampowered.com/app/1898830/VBridger/). 상세 검증은 [VBRIDGER_REQUIREMENTS](VBRIDGER_REQUIREMENTS.md).

## 이미지→3D 후보 비교
| 후보 | upstream 실행 조건 | 결과/토폴로지/텍스처 | 권리·Windows | RTX2060 6GB 결론 |
|---|---|---|---|---|
| TRELLIS 원본 | Linux 테스트, NVIDIA ≥16GB | structured latents→mesh/gaussian 등; 애니메이션용 edge flow 별도 검증 | 본체 MIT, 의존 라이선스 별도; native Windows 검증 안 됨 | 표준 구성 기준 부족, 실행 안 함 |
| TRELLIS.2 4B | Linux 테스트, NVIDIA ≥24GB | O-Voxel, PBR 속성; open/non-manifold 형상 처리 주장은 rig-ready 보증 아님 | 본체/모델 MIT 명시, nvdiffrast/nvdiffrec 별도 | 부족, 실행 안 함 |
| Hunyuan3D 2.1 | shape 10GB / texture 21GB / 전체29GB, Python3.10/PyTorch2.5.1 CUDA12.4 테스트 | shape와 PBR texture 분리; 캐릭터 얼굴·UV seam·관절 품질 실측 필요 | Windows/macOS/Linux 지원 명시. Community license EU/UK/한국 제외 등 제한 | 표준 구성 부족 + 전세계 배포 후보 차단 |

출처: [TRELLIS](https://github.com/microsoft/TRELLIS), [TRELLIS.2](https://github.com/microsoft/TRELLIS.2), [Hunyuan3D 2.1](https://github.com/Tencent-Hunyuan/Hunyuan3D-2.1), [Hunyuan license](https://github.com/Tencent-Hunyuan/Hunyuan3D-2.1/blob/main/LICENSE).

원본 얼굴·눈·의상·장식·후면 정확도, 애니메이션 특화 성능, 산출 texture 유효 해상도는 모두 **우리 데이터로 미측정**이다. 메모리 offload/양자화 fork를 공식 CPU 대안으로 표시하지 않는다. 측면·후면은 관찰 자료가 없으면 추정으로 표기한다. 최종 topology 정리·UV/texture 보존·facial blendshape 생성은 생성 모델 뒤의 별도 단계다.

## 자동 리깅 후보
| 후보 | 방법/근거 | 라이선스/자원 | 다음 판단 |
|---|---|---|---|
| UniRig | 뼈대 예측과 skinning 단계 분리 | 저장소 MIT; 정확한 체크포인트/부속 의존성 별도 audit. README의 60GB는 특정 **학습** 설명이며 추론 최소값으로 오용 금지 | 추론 VRAM 미측정, 휴머노이드 naming/가중치/Blender 수정 실험 |
| SkinTokens | 하나의 sequence로 뼈대와 compressed weights; Qwen3 기반 | 저장소 MIT, upstream 추론 NVIDIA ≥14GB, Python≥3.11/CUDA≥12.1 | 이 GPU 기준 부족. 실제 weights·학습 데이터·종속 구성 권리 재확인 |
| RigAnything | autoregressive 자동 리깅 연구 | Adobe Research License는 비상업 연구만 허용, 상업 제품 개발도 제외 | 광고 수익 제품 개발 경로에 포함하지 않음. 별도 허가 전 코드/weights 실행·번들 금지 |
| Blender/Rigify 및 기하 기반 baseline | 명시적 뼈대/스키닝·shape correction·물리 후처리 | Blender GPL/추가 addon별 audit | 사용자 설치 별도 프로세스로 재현 가능한 수정 레시피 검토 |

근거: [UniRig](https://github.com/VAST-AI-Research/UniRig), [UniRig license](https://github.com/VAST-AI-Research/UniRig/blob/main/LICENSE), [SkinTokens](https://github.com/VAST-AI-Research/SkinTokens), [RigAnything license](https://github.com/Isabella98Liu/RigAnything/blob/main/LICENSE.md), [Blender license](https://www.blender.org/about/license/).

춤 품질은 자동 skeleton 완성으로 판정할 수 없다. 위팔 180°·허리 ±90°·깊은 squat·골반 회전·점프 착지·빠른 반전 시 어깨 부피, weight bleeding, 관절 뒤집힘, 발 미끄러짐, 의상/신체 관통을 검사한다. corrective blendshape/보조 bone/weight 재분배/충돌체를 독립 modifier로 보존한다.

## segmentation / 복원
SAM2는 mask 후보 생성에 사용 가능하고 공개 코드·체크포인트 Apache-2.0 명시가 있다. 의미적 레이어 분할이나 가려진 부분 복원까지 해결한 모델로 보지 않는다. OpenCV mask cleanup, alpha 경계 검토, 원본 픽셀/생성 픽셀 분리 후 품질 비교가 필요하다. 이미지 복원 모델은 아직 선정하지 않았다. [SAM2](https://github.com/facebookresearch/sam2).

## 광고
직접 스폰서 정적 배너와 Pubfinity 후보를 비교했다. 데스크톱 앱에 일반 AdSense 웹 코드를 삽입하는 경로는 정책상 채택하지 않는다. Pubfinity는 Windows 앱용 banner SDK를 공개하지만 Tauri/Win32 허용, 일본 inventory·지급 조건·데이터 처리 계약은 확인 전이다. Microsoft 기존 UWP 광고 플랫폼은 종료 공지가 있어 새 기반으로 채택하지 않는다. 상세 출처와 계약 체크는 [MONETIZATION](MONETIZATION.md).
