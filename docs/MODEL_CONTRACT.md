# Common model contract

## 구현된 schema v1
정의: `schemas/character-project.schema.json`, `src-tauri/src/model.rs`, `src/contracts.ts`. 예제: `samples/character-project.json`.

`CharacterProject`는 schema_version=1, 정규화된 UUID, 1부터 증가하는 revision, 1–120자 name, dimension(2d/3d), 유효한 target_profile, source 0–1개, A–F 미검증 Gate, `master_status=not_generated`를 가진다. 현재 importer는 PNG만 허용한다. v1에서 완료 master나 pass Gate를 주입하면 Rust가 거부한다. 문서상 목표를 저장 데이터의 구현으로 오인하지 않는다.

SourceAsset는 sha256/bytes/width/height/format/provenance=original을 가진다. 물리 경로는 외부 입력 대신 코어가 해시로 만든다. 원본의 색 정보는 재인코딩하지 않은 blob 자체에 보존한다. source path/개인 파일명은 프로젝트 manifest와 Worker에 넣지 않는다.

## v2 이후 설계 (미구현)
`ArtifactRef`: SHA-256, original/generated/manual, parentHashes, 모델 ID+버전+가중치 hash, seed, 설정, 작성 도구, 좌표계/단위/색 공간, observed/inferred/unknown, confidence map. seed만 같다고 GPU 연산 결과의 bitwise 재현성을 보장하지 않는다.

2D Master: layer graph(픽셀 원좌표·Z 순서·mask·blend mode), texture 원본/작업본, mesh topology, deformers, keyforms, clipping, 표정 조합 보정, physics graph. 3D Master: 실제 topology/UV/material/texture, meter 단위 right-handed 기준 좌표계, skeleton(rest pose/inverse bind), normalized weights, corrective shapes, semantic face channels, colliders/springs. GLB/VRM은 export 사본이며 모든 편집 이력의 정본이 아니다.

SemanticExpression는 언어 독립 ID(`jaw.open`, `eye.left.blink`, `mouth.left.smile`), 좌우/중앙, min/neutral/max, 조합 관계, target parameter+곡선 목록을 보존한다. 좌우는 캐릭터 기준이다. 범위 변환/미러링/좌표계 변환은 exporter가 명시한다. VBridger 출력명과 Live2D 모델 파라미터 ID는 서로 다른 공간이며 별도 mapping이 필요하다. 3D는 receiver가 소비하는 blendshape clip 이름까지 기록한다.

## Worker protocol v1 (구현)
stdin 한 줄: `{"protocol":1,"job_id":"uuid","provider":"dummy"}`. stdout는 동일 protocol/job_id, kind(progress/completed), progress(0–100)의 JSONL. stdout에 로그를 섞지 않는다. unknown keys, 역행 진행률, 중복 terminal, 잘못된 job, 비정상 종료를 실패로 분류한다. 사용자가 지정한 실행 파일/임의 Python 코드를 IPC로 받지 않는다. stderr는 현재 버려 개인 경로 노출을 피하고, 구조화 진단 수집은 DEV-28 범위다.

차후 handshake는 provider protocol range, capabilities, 모델 hash, backend, limit을 포함해야 한다. 아티팩트는 Worker가 staging에 쓰고 코어가 경로·size·hash·schema를 검증한 뒤 정식 프로젝트에 승격한다. 원본과 source blob은 Worker write scope에서 제외한다.

## Export/검증 계약
ExportProfile는 대상 앱/버전, 포맷 버전, 지원 표정·물리·재질, companion 도구, 라이선스 상태, 필요한 semantic mapping, 손실 목록, static/runtime 증거를 가진다. DEV-00 profile implemented=false. export 승인에는 각 Gate evidence가 같은 원본·마스터·export hash에 결합되어야 한다. 모델 수정 시 이전 인증은 무효화된다.

## schema 변경 규칙
미래 schema는 조용히 읽지 않는다. 입력 크기 한도와 deny_unknown_fields를 유지한다. migration은 새 디렉터리에 작성→검증→사용자 선택으로 전환한다. 구버전 fixture를 CI에 남기고 원본을 덮어쓰지 않는다. uint64 revision의 JS 안전정수 범위를 벗어나지 않도록 현재 최대 99,999,999로 제한한다.

## DEV-01 추가 DTO (project v1 변경 없음)

ProjectList v2 = version:2 + healthy CharacterProject[] + issues[{id,status,error,recoverable revision[]}] + index_warning. RevisionReport는 revision/project|null/error|null을 가진다. RecoveryPreview는 selected/latest|null/latest_revision/token/changed_fields다. recover_project는 id/revision/token을 받고 새 CharacterProject를 반환한다. token은 preview 이후 manifest 변경을 감지하며 원본 hash는 실행 시 다시 확인한다.

Job에 queued/interrupted/recovery_required 상태를 추가했다. 별도 journal record v1은 sequence/updated_ms/Job을 저장한다. 프로젝트 revision이나 Gate는 Worker 진행률로 바뀌지 않는다. job_history는 실제 기록에서 읽으며 손상 항목은 복구 확인 필요로 표시한다. 계약은 src/contracts.ts 및 Rust 타입에 함께 정의한다. 자세한 schema 호환 방침은 MIGRATION_STRATEGY.md.
