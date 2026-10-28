# Localization

현재 ja/en/ko의 모든 화면·오류·Gate 이름은 `src/i18n.ts`의 같은 key 집합으로 관리한다. locale 값은 ISO language code, 데이터 ID/parameter ID는 번역하지 않는다. VBridger의 output 이름을 언어에 따라 바꾸면 mapping이 깨지므로 표시 이름과 기술 ID를 분리한다.

새 언어 추가: `Locale`에 코드 추가 → 영어 catalog와 동일한 `Record<MessageKey,string>` 작성 → selector 옵션 → key parity 테스트 → 화면/오류/긴 문장 검토. 누락/알 수 없는 오류는 해당 언어의 일반 복구 문구로 fallback한다. 영어가 아닌 error key를 코드에서 직접 표시하지 않는다. 전문가 모드만 보조 진단 코드를 보여 준다.

DEV-00은 브라우저 locale 자동 감지/영구 저장/ICU 복수형 라이브러리까지 구현하지 않았다. DEV-26에서 날짜·숫자·복수형·인수 escape를 Intl/ICU 기반으로 정리하고 오류/도움말/튜토리얼/품질 설명 catalog를 확장한다. 본 단계의 기술 개발 문서는 한국어 중심이며 제품 도움말 전체 번역 완료로 보지 않는다.

RTL 확장 계획: locale metadata에 direction을 추가하고 `<html dir>`로 전달한다. margin/padding inline-start/end, 긴 문장 wrapping, bidirectional parameter ID는 `bdi`/LTR code isolate 사용, 아이콘의 방향 의미 검토. 현재 3개 언어는 LTR이며 RTL **실동작은 미검증**이다.

검증: TS key 타입 + parity 테스트, Playwright 3언어 전환, 800px에서 가로 overflow 확인. DEV-26은 pseudolocale 2배 길이/RTL, 200% zoom, 키보드, screen reader, 독립 번역 검수를 수행한다. 색상만으로 pass/fail을 전달하지 않는다.

## DEV-01

저장 상태 7종, 이력/메타데이터 preview, 차이/보존/제외 항목, 명시 확인/취소, 잠금·공간·권한·index 오류, 작업 중단/재시도 설명을 ja/en/ko에 추가했다. checkbox Space와 언어 전환은 UI 테스트로 검사한다. 전문가 모드의 UUID/오류 코드는 번역하지 않으며 개인 파일 경로는 노출하지 않는다. RTL/전체 screen reader 실사용은 후속 검증이다.


## DEV-02 integration

DEV-02 adds input/cancel/ICC/precision/layer/unsupported-feature messages in en/ja/ko. Beginner preview avoids color setup dialogs; expert details expose original precision/profile/layer records. Source path is omitted from diagnostics. Locale parity remains mandatory.
