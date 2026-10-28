# DEV-01 schema 및 migration

CharacterProject는 v1 그대로다. DEV-00 sample과 원본·정식 revision을 보존하며 자동 manifest migration을 수행하지 않는다. list API v2는 별도 명령이고 기존 list_projects는 정상 항목 배열을 돌려주는 호환 인터페이스로 남긴다. 신규 UI는 오류 항목을 놓치지 않도록 list_projects_v2/search_projects를 사용한다.

새로운 job journal은 자체 version=1이다. schema나 unknown field를 읽을 수 없으면 recovery_required로 표시하고 원본을 보존한다. 아직 journal 변환기나 과거 state 덮어쓰기를 제공하지 않는다. 최초 DEV-00 실행 데이터에는 journal이 없으므로 비어 있는 jobs 디렉터리를 추가하는 것으로 시작한다.

SQLite는 독립 user_version=1의 버려도 되는 파생 인덱스다. 이전 DB 또는 알 수 없는 DB version을 원본 프로젝트 schema로 오인하지 않는다. DB 내용을 불러와 프로젝트를 만드는 대신 파일 정본을 다시 검사하고 새 DB로 교체한다. 실패한 DB 갱신은 기존 revision을 손상시키지 않는다.

향후 manifest v2가 필요한 경우 구버전 읽기 지원 → 사용자 선택 → 새 디렉터리/새 파일에서 변환 → 참조·원본 hash 검사 → 검증된 사본 전환 순서를 따른다. 구버전과 백업을 유지하고 실패 시 전환하지 않는다. 미래 schema가 있는 프로젝트를 v1 과거 버전으로 복구하는 우회는 차단한다.

현재 검증: DEV-00 sample 읽기, unknown field 거부, 미래 schema 격리/복구 차단, 잘못된 ID/번호 검사, SQLite version999 및 손상 DB 재구축. 실제 v1→v2 migration은 필요하지 않아 미구현이며 수행했다고 주장하지 않는다.


## DEV-02 integration

DEV-02 does not batch-convert existing projects. Selecting a PSD in an empty project creates a validated in-memory v2 copy then publishes it as the next immutable revision, preserving the v1 file. The v1 JSON schema and sample remain byte-for-byte unchanged. Existing PNG projects save as v1. User-selected recovery may create a new v1 or v2 revision from its validated source; previous data is preserved. Old DEV-01 binaries safely reject v2 instead of treating PSD bytes as PNG.
