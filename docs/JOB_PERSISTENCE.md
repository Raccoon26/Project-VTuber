# DEV-01 작업 영속화

프로젝트 revision과 분리된 `VTUBER_DATA_DIR/jobs/<job_uuid>/<sequence:08>.json`에 append-only 기록한다. record version=1, sequence, updated_ms, Job을 보존한다. Job의 ID/project_id/status/progress/provider/error 계약은 unknown field를 거부한다. record 최대64KiB, job당 정식 record10,000개, history10,000개를 한도로 명시적으로 오류 처리한다.

상태는 queued → running → completed/cancelled/failed다. interrupted는 이전 실행 중단, recovery_required는 기록 손상·저장 실패로 결과 신뢰가 불가능한 상태다. completed는 실제 Worker 종료 코드와 terminal protocol 검사를 통과하고 완료 record를 공개한 뒤에만 UI에서 관찰된다. progress100만으로 완료가 아니다.

`.runtime.lock`의 OS lock을 실행 동안 보유한다. 같은 데이터 루트에서는 한 번에 하나의 더미 Worker만 허용한다. 프로젝트마다 병렬 GPU 작업을 지원한다는 뜻이 아니다. 시작 전에 queued를 저장하고, running/progress/terminal을 순서대로 기록한다. 기록 실패 시 Worker에 취소 신호를 보내고 recovery_required를 표시한다. terminal record를 쓰지 못하면 다음 시작 시 마지막 안전한 상태만 남을 수 있다.

시작 시 lock 획득이 가능할 때만 남아 있는 queued/running을 interrupted로 새 기록한다. 다른 프로세스가 lock을 소유하면 그 작업을 중단으로 변경하지 않는다. 손상된 최신 journal은 이전 완료 기록으로 조용히 대체하지 않으며 recovery_required를 반환한다. 원본 journal을 덮어쓰지 않는다. 별도 창의 UI는 작업 이력 새로고침으로 외부 작업 상태를 확인한다.

중단·실패 작업은 원래 프로젝트를 다시 열어 원본 검증 후 사용자가 새 더미 작업을 실행할 수 있다. 기존 job ID/기록을 재사용하거나 자동 재개하지 않는다. 이미지·checkpoint 경로는 아직 Worker에 전달하지 않는다. DEV-03은 provider protocol을 유지하고 별도 artifact/checkpoint manifest를 도입해야 한다.

Windows Job Object를 사용하지 않으므로 앱 강제 종료 시 모든 자식·손자 프로세스 종료를 보장하지 않는다. OS lock 해제는 소유 앱의 종료 증거이며 잔존 Worker가 없다는 증거가 아니다. 실제 AI 재개·프로세스 트리 회수·journal retention/압축은 DEV-03/28에서 검증한다. 현재 더미는 짧은 CPU 작업이고 결과 파일을 생성하지 않는다.
