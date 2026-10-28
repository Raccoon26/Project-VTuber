# Security and privacy

## 현재 경계
사용자의 제작 이미지/모델은 업로드하지 않는다. 앱에는 HTTP 클라이언트·광고 SDK·분석 SDK·외부 plugin loader가 없다. Tauri UI CSP는 self/IPC만 허용하며 shell/fs/network plugin 권한을 부여하지 않는다. 브라우저 preview에서는 실제 파일 작업을 사용할 수 없다. 개발 의존성 다운로드/GitHub 코드 push와 제품 이미지 전송을 구분한다.

현재 신뢰 대상은 설치된 Python/OS, 우리 Worker, 앱의 저장 루트다. `-I`와 별도 프로세스는 임의 악성 코드를 차단하는 OS sandbox가 아니다. 파일 symlink/reparse 체크는 보편적인 TOCTOU 방어를 보장하지 않는다. 같은 사용자 권한의 공격자가 저장 중 경로를 바꿀 수 있는 공유 폴더는 지원하지 않는다. 플러그인 실행을 아직 열지 않는 이유다.

| 위협 | DEV-00 방어/시험 | 남은 조치 |
|---|---|---|
| 잘못된 PNG/PSD | PNG signature/decoder/CRC, PSD/APNG 거부, decoder 한도, byte/pixel 제한 | PSD parser 격리·fuzz/색/레이어 기능 DEV-02 |
| 초대형/압축 폭탄/metadata | 입력64MiB, 40M pixels, 출력160MiB, decoder192MiB; 임의 축소 없음 | 전체 peak RAM·ICC decompress·worst-case fuzz/OS limit DEV-02 |
| 경로 탐색 | UUID 정규화, 코어 생성 hash 이름, root/dir/file reparse 거부 | handle 기반 race 방어와 multi-process lock DEV-01/29 |
| 원본 손상 | source bytes 별도 복사, hash 재검사, 덮어쓰기 없음 | 디스크 오류/전원 차단/백업 UX DEV-01/28 |
| 저장 중 취소/강제 종료 | immutable revision, sync 후 publish, pending 무시 | OS power-loss 내구성/복구 UI DEV-28 |
| Worker crash/악성 출력 | bounded JSONL, request/job validation, monotonic progress, timeout, kill/wait | 실제 모델 자식 프로세스 트리 job object / OS RAM/CPU 제한 DEV-03/28 |
| GPU OOM | 실제 GPU 모델 실행 비활성 | provider preflight, allocation tracking, 단계별 unload/retry DEV-03 |
| 모델 파일 변조/pickle | 모델 미포함; manifest hash/license gate 설계 | 서명/허용 출처·safetensors·quarantine load DEV-03/29 |
| 광고의 파일/식별자 접근 | offline adapter만 존재; 프로젝트 인수 없음 | 별도 origin/process, allowlist, timeout, 동의/traffic audit DEV-27 |
| plugin 권한 확대 | plugin system 미제공 | capability 선언·서명·승인·IPC schema 검증 DEV-29 |

## 저장 위치 정책
사용자 추가 지침: 이 작업이 관리하는 모든 신규 데이터·캐시·임시 파일은 E 드라이브. `. scripts/env.ps1`로 TEMP/TMP, npm cache, Cargo cache/target, Playwright, Python cache, 앱/WebView 데이터를 작업 루트 아래로 지정한다. C/D에 설치된 도구는 읽어 실행할 수 있지만 프로젝트 출력 위치로 쓰지 않는다. Windows/Codex 자체의 운영 로그·업데이트는 이 앱이 통제하는 저장소가 아니며 그 위치까지 통제했다고 주장하지 않는다.

앱 기본 데이터는 실행 파일 옆 `vtuber-data`; 명시한 절대경로 `VTUBER_DATA_DIR`로 바꿀 수 있다. 배포 후 설치 위치가 쓰기 불가이면 관리자 권한을 요구하지 않고 사용자가 쓸 수 있는 위치 선택을 구현해야 한다(DEV-30). 현재는 개발 실행 지침의 writable E 위치를 사용한다.

## 진단과 공개
오류는 기계 코드와 번역된 복구 설명으로 반환한다. raw path/이미지/모델/토큰은 로그·telemetry에 넣지 않는다. 자동 crash upload 없음. 공유 진단 번들은 DEV-28에서 내용 preview와 명시 동의 후에만 생성/공유한다. 원격 저장소에는 코드·문서·합성 fixture만 포함하며 실제 캐릭터 입력·대형 모델·로컬 경로가 포함된 raw 로그는 ignore한다.

보안 이슈는 재현 가능한 합성 입력으로 기록하고 실제 비밀을 넣지 않는다. 수정은 원인 증거→최소 patch→회귀→관련 기능 재검증 순서다. dependency audit는 SBOM/라이선스 감사의 대체물이 아니다.

## DEV-01 추가 방어 및 남은 범위

프로젝트별 OS lock을 사용하며 lock 파일을 PID로 삭제하지 않는다. Windows handle은 delete sharing을 허용하지 않는다. 진단 중 source/manifest/reparse를 검사하고 복구 후보의 원본 hash를 재확인한다. SQLite는 fresh DB를 parameter binding으로 구성하고 extension loader를 활성화하지 않는다. journal/manifest 크기와 개수 제한을 둔다.

OS share-denied handle과 junction은 합성 경로로 시험했다. 관리자 권한으로 사용자 폴더 ACL을 변경하지 않는다. 파일 경로를 검사한 뒤 handle을 여는 TOCTOU 및 동일 사용자에 의한 hardlink/상위 경로 교체의 완전 방어는 아직 보장하지 않는다. 공유·공격 가능한 저장 루트는 지원 범위에서 제외한다. 프로젝트 목록/진단에는 UUID와 기계 오류만 반환하고 원래 사용자의 개인 파일 경로를 넣지 않는다.


## DEV-02 integration

DEV-02 adds bounded signature-based PSD reading; no smart-object external path or executable payload is opened. PSD offsets/record counts/channel sizes/nesting/decoded allocations have independent limits. Raw/RLE/ZIP/predictor errors fail explicitly; no black replacement pixels. Windows selected-source handle denies write/delete sharing; reparse ancestors and modified/length checks protect cooperative workflows. LittleCMS2.19 vendored source was examined for the CubeSize pre-multiplication overflow fix; no broad absence-of-vulnerabilities claim. PNG metadata decompression is still within the decoder budget; native ICC fuzzing/OS-isolated parser remain DEV-29 work.
