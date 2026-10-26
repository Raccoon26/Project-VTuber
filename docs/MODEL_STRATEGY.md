# Local AI model strategy

## 선택 원칙
평가 순서는 권리/보안 → 실행 가능성 → 원본 충실도 → 리깅 적합성 → 성능이다. 무료·최신·데모가 보기 좋다는 이유만으로 채택하지 않는다. 6GB GPU에 맞추기 위해 원본을 자동 축소하거나 미지원 모델을 지원한다고 표시하지 않는다. 원본은 보존하고 모델 입력 proxy가 필요하면 별도 artifact로 만들고 해상도·변환 이유를 기록한다.

DEV-00 설치 모델은 **없음**. CPU dummy만 실행한다. [후보와 upstream VRAM](TECH_RESEARCH.md)을 보면 TRELLIS/TRELLIS.2/Hunyuan2.1/SkinTokens의 표준 구성은 현재 GPU 조건을 만족하지 않는다. offload·양자화·CPU 대안은 개별 실측 전까지 unavailable/unverified다.

## ModelManifest와 설치 상태
`id`, exact version/commit, official URL, artifact bytes/SHA-256, code license, weights license, dependency licenses, output-use restrictions, allowed territories, runtime/Python/torch/CUDA compatibility, backend capabilities, VRAM/RAM/disk estimate, verified hardware, expected inputs/outputs, preprocessing loss, network requirement, reviewer/evidence를 기록한다. URL만으로 신뢰하지 않는다.

상태: discovered → legal_review → user_approved_download → downloading.partial → hash_verified → quarantined_load_test → installed → enabled. 실패는 partial/quarantine에서 멈추고 원인 기록. 모델 변경/삭제는 진행 중 job과 reference count 확인, 기존 프로젝트 재현용 hash pin 유지. 자동 모델 다운로드 및 runtime install은 DEV-00에 없다.

## 로컬 runtime matrix 설계
Windows CUDA(Pytorch), CPU(검증된 lightweight segmentation/기하 처리), ONNX CPU/DirectML, 향후 Linux CUDA/ROCm/macOS Metal을 서로 다른 검증 대상으로 둔다. GPU 제품명과 VRAM은 자원 정보이며 backend 지원 판정에는 provider의 실제 tensor/operator smoke와 peak allocation 측정이 필요하다. 설치가 없으면 absent, 검사 실패면 unknown, 검증된 실패 조건이면 unsupported로 구분한다.

## 공정한 비교 실험
권리 확보된 12개 캐릭터 corpus: 반신/전신, 대칭/비대칭, 복잡한 머리, 얇은 장식, 긴 의상, 피부/털, 투명 소품, 다양한 해상도. 원본 bytes/ICC/alpha/hash를 고정하고 단일 정면 트랙과 추가 view 트랙을 분리한다. 비교 seed 3개 이상, 동일 입력 framing/배경, 허용 preprocessing과 proxy를 명시한다. 시간 제한 없이 품질 우선으로 실행하되 취소·VRAM 제한을 적용한다.

기록: 얼굴 landmark/눈 형태·색·장식·실루엣 비교, 사용자/전문가 blind rating, 8방향 turntable, 후면 추정 표시, topology manifold/관절 edge flow, texture texel density/seams, normalized skin weights, 춤 deformation, VRAM/RAM peak, 실패율, wall time, 모델·dependency hashes. SSIM/PSNR은 동일 투영/색 조건의 참고값이며 합격을 대체하지 않는다.

## 보안·승인 조건
pickle 기반 checkpoint의 임의 코드 실행 위험을 피하고 가능한 safetensors를 우선한다. 임의 remote code나 pip install script를 신뢰하지 않는다. 의존성·model card·LICENSE 별도 감사 후 제한된 실행 환경에서 로드한다. 대형 weights 다운로드/장시간 GPU 부하/유료 서비스/이미지 전송은 별도 권한 확인이 필요하다. 핵심 제작에 cloud API를 필수로 넣지 않는다. 광고는 모델 관리자에 접근할 수 없다.
