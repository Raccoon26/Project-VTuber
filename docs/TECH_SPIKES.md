# Minimal technical spikes

관찰과 예상 계획을 분리한다. 대형 모델·상용 SDK·사용자 작품을 사용하지 않았다. 실행 로그와 명령은 TEST_RUNS에 연결한다.

| Spike | 최소 질문·방법 | 관찰 결과 | 해석/다음 조건 |
|---|---|---|---|
| S00-01 原本 보존 | 합성 PNG import→save→reopen→bytes/hash 비교→tamper | Rust 시험 통과, 입력/보존 blob 동일, 변조 거부 | 지원 PNG의 lossless 보존 근거. PSD/ICC 렌더 충실도는 아님 |
| S00-02 트랜잭션 | 새 immutable revision/stale revision/pending/손상 manifest | 저장 충돌과 잘못된 최신 파일 거부, pending 무시 | 프로세스 수준 보호. 실제 전원차단 내구성은 미실행 |
| S00-03 Worker | 실제 Python stdout progress/terminal/cancel/invalid/missing | process boundary 시험 통과; dummy 1.8초 작업 | GPU 생성과 무관. 큰 모델 취소/OOM 미실행 |
| S00-04 desktop feasibility | Windows release build + native WebView 실제 IPC | exe 생성, create/save/reload/open/job/cancel/GPU 통과 | 실행 가능한 기반. installer/깨끗한 PC 인증 아님 |
| S00-05 VBridger static | synthetic semantic↔target 범위/중복/누락 validator | 올바른 mapping 허용, 결함 mapping 거부 | 공식 preset/VBridger 실행 인증 아님. 실제 앱은 미실행 |
| S00-06 6GB feasibility preflight | 실제 GPU query와 upstream 최소 VRAM 대조 | RTX2060 6144MiB; 주요 비교 후보 표준 요구보다 작음 | 추론 benchmark 미실행. 낮은 품질 CPU 대안을 있다고 주장하지 않음 |
| S00-07 2D output due diligence | 공식 Editor export/SDK와 제3자 Ayatsuri license/readme 비교 | 공식 Editor output 존재, 제3자는 cmo3/moc3 주장·혼합 license | compiler/권리/round-trip 아직 미검증. DEV-11 isolated fixture 필요 |
| S00-08 ads | desktop 허용 정책/SDK 소개와 offline adapter 비교 | AdSense software 경로 기각, Pubfinity 계약 보류, adapter null 시험 pass | 실제 공급업체/Japan/지급 확인 전 미연결 |
| S00-09 localization | 3언어 key parity + browser/native 전환 | 자동 검사 통과, 실제 일본어 native screenshot 확인 | RTL/긴 문자열 전체/사용성은 후속 |

## 다음 실험의 재현 조건
Live2D: 허용된 최소 원화/레이어→deformer2개/좌우 eye/jaw/입 복합/physics fixture→공식 Editor editable import/edit/export→VTS 실제 parameter sweep. 제3자 도구는 별도 sandbox checkout, exact commit/license/dependency 기록, 동일 fixture 비교. 모든 output과 예상 parameter hash를 남긴다. 배포 권리 확인 전 제품에 포함하지 않는다.

VBridger: 권리 있는 preset/model, 앱+Editor DLC 필요성 확인, VTS API 사용자 허용, receiver별 VMC 얼굴 output 명칭 확인, restart/preset save/load round-trip. 정확한 테스트 순서는 VBRIDGER_REQUIREMENTS를 따른다.

3D/rigging: MODEL_STRATEGY의 12개 corpus/seed/리소스 측정 절차를 사용한다. 얼굴/장식/후면/UV/어깨/골반/무릎/발/옷/머리 physics를 분리 평가한다. 현재 모델 outputs가 없으므로 PSNR/SSIM/전문가 점수·속도 수치를 생성하지 않는다.
