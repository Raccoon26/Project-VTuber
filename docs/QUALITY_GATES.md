# Quality gates

네 상태: `pass`, `fail`, `unverified`, `not_applicable`. 미실행은 pass가 아니다. 비해당은 이유/적용 범위/검토자를 남긴다. 아래 기준은 제품 목표이며 아직 전부 자동화되지 않았다. DEV-00 앱은 모든 모델 Gate를 미검증으로 표시한다. 저장 단위 테스트 통과를 완성 캐릭터의 Gate A–F 통과로 옮기지 않는다.

| Gate | 입력/검증 | 실패·미검증 기준 | 최종 증거/담당 |
|---|---|---|---|
| A 원본 보존 | 최초 입력 bytes SHA-256, 보존 blob, 이후 reopen/job 전후 hash, 해상도·색 메타데이터 | 한 byte라도 변경·해시 미기록은 fail; 입력 없으면 unverified | 원본/보존 hash, decoder 검사 로그; DEV-02 |
| B 이미지 충실도 | 동일 투영·조명·색 공간의 원본/결과 비교, 얼굴·눈·머리·의상·장식·실루엣·질감/아티팩트 | 핵심 특징 손실·승인 없는 축소/압축 fail; 보이지 않은 후면은 inferred | 양쪽 이미지/차이맵, 사람 평가+정량 참고; DEV-20 |
| C 리깅 품질 | 표정 단독/조합/비대칭, blink/jaw/lips, yaw/pitch, 전신/춤, physics | 관절 뒤집힘·메시 찢어짐·NaN·심각 관통 fail; 전문성 기준 미설정이면 unverified | 고정 pose battery 영상/측정, 전문가 검토; DEV-20 |
| D 호환성 | 파일 validator + 실제 대상 앱의 표정/전신/물리/재시작 | 포맷 열기만으로 pass 금지; target version/실동작 없으면 unverified | 앱/프로필/모델 hash별 결과; DEV-24 |
| E 초보자 사용성 | 처음 쓰는 사람이 6단계 제작/복구를 도움 없이 수행 | 필수 경로 설명 없음, 회복 불가, 미지원 기능 오표시 fail | 최소 5명 formative test, 성공률/실패 위치/시간; DEV-25 |
| F 안정성 | 저장·복구·취소·Worker crash/OOM·오프라인·메모리·강제 종료 | 원본 손상·완료 오판·반복 장애 fail | fault injection + soak + clean machine; DEV-28/31 |

## 판정 레코드
각 record: gate, status, requirement IDs, source/master/export hashes, tool/provider/target version, method(automated/manual/runtime/synthetic), measured values, thresholds version, evidence paths+hashes, reviewer, timestamp. 모델/의존성/대상 앱 변경 시 관련 Gate는 다시 unverified. visual 기준은 전문가 panel calibration 후 수치화하며 지금 임의 SSIM 점수 하나로 전문가급을 주장하지 않는다.

## DEV 완료 게이트와 제품 게이트
DEV-00의 합격 범위는 실행 가능한 기반·위험 조사·안전한 더미 흐름·테스트·문서·main 병합이다. 제품 Gate B/C/D/E의 미검증은 본래 후속 연구 범위로 남긴다. 필수 DEV-00 테스트/빌드/CI 실패나 PR 미병합은 DEV-00 완료를 막는다. 기술적으로/권리상 막힌 장기 요구사항은 담당 DEV와 재검증 조건을 보존한다.

## 자동 repair
수정 후보는 새 revision/artifact에만 저장한다. repair 전후 같은 pose/image 조건으로 A–D를 재검사하고 실패 시 되돌린다. confidence가 낮은 후면·얼굴 수정은 사람이 확인한다. 자동 수정 횟수 제한과 실패 원인 등록은 DEV-21에서 구현한다.
