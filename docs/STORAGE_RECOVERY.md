# DEV-01 저장·복구 계약

정본은 기존 `projects/<uuid>/<revision:08>.json`과 `sources/<sha256>.png`다. CharacterProject v1 및 원본 bytes는 유지한다. DEV-01은 기존 프로젝트를 자동 변환하거나 과거 버전으로 조용히 대체하지 않는다.

## 읽기와 진단

`list_projects_v2`는 version=2, healthy[], issues[], index_warning을 반환한다. issues는 recoverable/unrecoverable/access_denied/project_busy/schema_unsupported/uninspectable과 오류 코드·복구 가능한 revision 번호를 가진다. 루트 디렉터리 열거 실패는 전역 오류다. 한 항목의 실패만 격리한다. 정식 revision을 최대 10,000개 검사하고 manifest당 1MiB를 넘으면 명시적으로 검사를 중단한다. `.pending`은 정식 이력이 아니다.

`project_history`는 각 revision의 JSON, schema, ID/번호, 필드, 원본 존재·크기·SHA-256 및 경로를 검사한다. 최신 revision이 정상이어도 이력 화면에서 과거 손상을 확인할 수 있다. 원본을 참조하지 않는 초기 버전만 살아 있다면 원본 없는 상태로의 복구임을 미리보기에서 표시한다. 손상된 blob을 복원했다고 주장하지 않는다. 미래 schema가 최신인 프로젝트는 과거 버전으로 덮어 우회하지 않고 복구를 거부한다.

## 선택과 확정

미리보기는 선택한 모델 메타데이터, 최신 번호·읽을 수 있는 최신 데이터, 변경 필드, 전체 manifest 내용 해시 기반 token을 반환한다. 최신 데이터가 읽히지 않으면 정확한 차이를 모른다고 표시한다. 현재 캐릭터 렌더 미리보기는 없으며 원본 개수·크기 등 저장 데이터 미리보기다.

사용자 확인 후 동일 프로젝트 OS 잠금 아래에서 token과 선택한 원본을 재검증한다. 가장 큰 정식 revision 번호 다음에 새 JSON을 공개한다. 기존 정상·손상 이력이나 blob은 수정/삭제하지 않는다. 생성 후 다시 열고 hash를 검증해야 성공 응답을 반환한다. 실패하면 성공으로 표시하지 않으며, 공개 직후 실패했다면 새 파일은 남을 수 있으므로 다시 검사한다. 복구 취소는 아무 revision도 쓰지 않는다. token 뒤에 정식 파일이 변경되면 충돌이다.

## 잠금·공개 경계

`.write.lock`을 읽기/쓰기 및 Windows share read/write(delete 제외)로 열고 Rust `File::try_lock`을 사용한다. 잠금 파일 존재가 잠금 여부를 뜻하지 않는다. Windows에서는 `LockFileEx`에 대응하며 handle 종료 후 OS가 해제한다. PID로 판단하거나 lock 파일을 삭제하지 않는다. 일반 save/import는 잠금 획득 → 현재 revision/원본 검사 → 예상 revision 비교 → 변경 검증 → 공개 순서를 지킨다. 오래된 writer는 revision_conflict, 동시 소유자는 project_busy다.

새 파일은 create_new pending → bytes 쓰기 → sync_all → hard_link(no overwrite) → 자신의 pending 정리 순서다. 프로세스 종료 때 남은 pending은 보존하고 무시한다. 원자적인 파일 공개와 전원 차단 후 디렉터리 엔트리 내구성은 별개다. 로컬 Windows NTFS의 실제 프로세스 종료·동시 writer를 시험했으며, 전원 차단·네트워크 공유·NTFS 외 파일시스템은 미검증이다. 같은 사용자 권한의 악의적인 경로 교체를 완전히 방어하는 handle-relative sandbox는 DEV-29 범위다.

공식 근거: [Rust File lock](https://doc.rust-lang.org/std/fs/struct.File.html#method.try_lock), [Windows LockFileEx](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-lockfileex).

## SQLite 파생 인덱스

rusqlite 0.41.0 bundled SQLite를 사용한다. `projects/index.sqlite`, user_version=1, id/name/dimension/target/revision/status/checked_at을 저장한다. 원본 bytes는 넣지 않는다. 목록/검색 요청마다 파일 정본을 검사한 결과로 새 임시 DB를 만든다. transaction commit·close·쓰기 가능한 handle의 sync_all 후 교체한다. `.index.lock`이 교체를 직렬화한다. DB 없음/손상/과거·미래 버전/오래된 내용은 이 방식으로 재구축한다. 원본 manifest를 수정하는 migration과는 다르다.

검색은 검증 결과를 바탕으로 SQL parameter binding을 사용한다. 인덱스 실패 시 경고와 파일 기반 목록/검색을 제공한다. index update는 revision commit과 원자적이지 않으며 파일 저장 성공을 취소하지 않는다. 다른 프로세스의 저장과 스캔이 교차하면 인덱스는 순간적으로 오래될 수 있다. 다음 요청에서 재검사하며, 실제 open/recover는 항상 파일을 다시 검증한다. 전체 스캔 비용과 Unicode case-fold의 SQLite/JS 차이는 향후 최적화 대상이다.

## 실패 주입 범위

`resilience_tests`의 child helper는 테스트 바이너리에만 포함된다. checkpoint 환경 변수나 journal 실패 marker는 release 코드에 들어가지 않는다. 데이터는 tempfile의 합성 fixture뿐이다. 실제 권한 없는 공유 handle과 junction, kill, 동시 프로세스를 시험한다. 디스크 부족은 StorageFull 오류를 주입하며 실제 E 드라이브를 채우지 않는다. 전원 차단을 시험했다고 표시하지 않는다.
