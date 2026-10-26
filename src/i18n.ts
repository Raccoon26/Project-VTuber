export const en = {
  title: "Character workshop",
  subtitle: "Preserve the original. Build with evidence.",
  foundation: "DEV-00 · Foundation preview",
  notice:
    "This build tests project storage and a dummy worker. It does not generate or rig a character.",
  language: "Language",
  mode: "Workspace",
  beginner: "Beginner",
  expert: "Expert",
  step1: "1 · Create a project",
  step2: "2 · Preserve your illustration",
  step3: "3 · Test the workflow",
  step4: "4 · Review & export",
  name: "Project name",
  dimension: "Model type",
  target: "Broadcast application",
  create: "Create project",
  save: "Save name",
  open: "Open a saved project",
  choose: "Choose a project",
  import: "Choose PNG illustration",
  sourceHint:
    "PNG only in DEV-00 · Up to 64 MiB / 40 million pixels. Originals are copied without resizing or recompression.",
  sourceEmpty: "No illustration added",
  sourcePreserved: "Original preserved",
  revision: "Saved revision",
  start: "Run dummy workflow",
  cancel: "Cancel task",
  idle: "Ready for a workflow test",
  running: "Testing workflow…",
  completed: "Dummy workflow completed. No model was generated.",
  cancelled: "Task cancelled. Your source is safe.",
  failed: "Task failed. Your saved project is preserved.",
  export: "Export model",
  exportBlocked:
    "Export is unavailable: generation, rigging and application compatibility are not yet verified.",
  gates: "Quality gates",
  unverified: "Unverified",
  pass: "Pass",
  fail: "Fail",
  not_applicable: "Not applicable",
  A: "Original integrity",
  B: "Visual fidelity",
  C: "Rig quality",
  D: "Application compatibility",
  E: "Beginner usability",
  F: "Stability",
  gpu: "Device diagnostics",
  detect: "Check GPU",
  unknown: "Not detected; GPU capability is unknown",
  inference:
    "AI inference support has not been tested. The dummy worker runs on CPU.",
  driver: "Driver",
  details: "Technical details",
  offline: "Local workspace · No image uploads · No advertising connection",
  browser:
    "Browser preview only. Open the desktop app to create and save real projects.",
  busy: "Please wait…",
  error:
    "The operation could not be completed. Retry after checking the project and available storage.",
  invalid_name: "Enter a project name between 1 and 120 characters.",
  invalid_profile: "Choose an application that matches the model type.",
  storage_error:
    "Could not save or read the project. Check free space and folder permissions; your previous saved revision is retained.",
  invalid_project:
    "The project is damaged or unsupported. Preserve the folder and restore from a verified revision; do not overwrite it.",
  schema_unsupported:
    "This project needs another application version. The original data has not been changed.",
  unsafe_path:
    "This location cannot be used safely. Choose a regular local file.",
  revision_conflict:
    "A newer revision exists. Reopen the project before saving.",
  source_changed:
    "The stored original differs from its recorded checksum. Stop editing and recover the original from your backup.",
  source_exists:
    "An original is already preserved. Create another project to use a different illustration.",
  image_too_large:
    "This illustration exceeds the current safe input limit. The original will not be resized. Use a supported source or wait for large-image support.",
  invalid_image:
    "This is not a valid supported PNG. Export a non-animated PNG from your illustration editor and try again.",
  source_required: "Choose an illustration before starting the workflow.",
  python_unavailable:
    "Python could not start. Configure VTUBER_PYTHON with a Python 3.12 executable and restart the app.",
  job_busy: "Another task is running. Cancel it or wait until it finishes.",
  job_missing: "This task is no longer active. Refresh the workspace.",
  worker_failed:
    "The worker stopped unexpectedly. Your original is preserved; retry the task.",
  worker_timeout: "The worker took too long and was stopped. Retry the task.",
  worker_protocol:
    "The worker returned invalid data and was stopped. Check that the worker version matches the application.",
} as const;
export type MessageKey = keyof typeof en;
export type Locale = "en" | "ja" | "ko";
export const ja: Record<MessageKey, string> = {
  title: "キャラクター工房",
  subtitle: "原画を守り、検証を重ねて制作。",
  foundation: "DEV-00 · 開発基盤プレビュー",
  notice:
    "このビルドでは保存とダミー処理を検証します。キャラクター生成・リギングは未実装です。",
  language: "言語",
  mode: "ワークスペース",
  beginner: "初心者",
  expert: "エキスパート",
  step1: "1 · プロジェクト作成",
  step2: "2 · 原画の保存",
  step3: "3 · 処理のテスト",
  step4: "4 · 確認と書き出し",
  name: "プロジェクト名",
  dimension: "モデル形式",
  target: "配信アプリ",
  create: "プロジェクトを作成",
  save: "名前を保存",
  open: "保存済みプロジェクトを開く",
  choose: "プロジェクトを選択",
  import: "PNGイラストを選択",
  sourceHint:
    "DEV-00はPNGのみ · 最大64 MiB / 4,000万画素。原画の縮小・再圧縮は行いません。",
  sourceEmpty: "イラスト未選択",
  sourcePreserved: "原画を保存済み",
  revision: "保存リビジョン",
  start: "ダミー処理を実行",
  cancel: "処理をキャンセル",
  idle: "テストを開始できます",
  running: "処理をテスト中…",
  completed: "ダミー処理が完了しました。モデルは生成されていません。",
  cancelled: "キャンセルしました。原画は保護されています。",
  failed: "処理に失敗しました。保存済みプロジェクトは保護されています。",
  export: "モデルを書き出す",
  exportBlocked:
    "生成・リギング・アプリ互換性が未検証のため、書き出しは利用できません。",
  gates: "品質チェック",
  unverified: "未検証",
  pass: "合格",
  fail: "不合格",
  not_applicable: "対象外",
  A: "原画の整合性",
  B: "外観の忠実度",
  C: "リギング品質",
  D: "アプリ互換性",
  E: "初心者の使いやすさ",
  F: "安定性",
  gpu: "デバイス診断",
  detect: "GPUを確認",
  unknown: "未検出：GPU性能は不明です",
  inference: "AI推論は未検証です。ダミー処理はCPUで動作します。",
  driver: "ドライバー",
  details: "技術情報",
  offline: "ローカル作業 · 画像送信なし · 広告接続なし",
  browser:
    "ブラウザーの表示確認用です。保存にはデスクトップアプリを起動してください。",
  busy: "処理中…",
  error:
    "処理を完了できませんでした。保存先と空き容量を確認して再試行してください。",
  invalid_name: "1〜120文字のプロジェクト名を入力してください。",
  invalid_profile: "モデル形式に対応するアプリを選んでください。",
  storage_error:
    "保存・読み込みに失敗しました。空き容量とアクセス権を確認してください。以前の保存履歴は保持されます。",
  invalid_project:
    "プロジェクトが破損しているか非対応です。フォルダーを保持し、検証済み履歴から復元してください。",
  schema_unsupported:
    "別バージョンのアプリが必要です。データは変更されていません。",
  unsafe_path:
    "安全に使用できない場所です。通常のローカルファイルを選んでください。",
  revision_conflict:
    "新しい保存履歴があります。開き直してから保存してください。",
  source_changed:
    "保存した原画のハッシュが一致しません。編集を止め、バックアップから復元してください。",
  source_exists:
    "原画は既に保存済みです。別の原画は新規プロジェクトで使用してください。",
  image_too_large:
    "現在の安全な入力上限を超えています。原画は縮小しません。対応する原画か大画像対応機能をご利用ください。",
  invalid_image:
    "対応するPNGではありません。編集ソフトから静止PNGを書き出して再試行してください。",
  source_required: "処理の前にイラストを選んでください。",
  python_unavailable:
    "Pythonを起動できません。VTUBER_PYTHONにPython 3.12の実行ファイルを設定して再起動してください。",
  job_busy: "別の処理が実行中です。完了を待つかキャンセルしてください。",
  job_missing: "処理は終了しています。画面を更新してください。",
  worker_failed:
    "処理が異常終了しました。原画は保持されています。再試行してください。",
  worker_timeout: "時間がかかったため処理を停止しました。再試行してください。",
  worker_protocol:
    "処理データが不正なため停止しました。ワーカーとアプリのバージョンを確認してください。",
};
export const ko: Record<MessageKey, string> = {
  title: "캐릭터 제작실",
  subtitle: "원본을 지키고, 검증을 거쳐 완성합니다.",
  foundation: "DEV-00 · 개발 기반 미리보기",
  notice:
    "이 버전은 프로젝트 저장과 더미 작업을 시험합니다. 캐릭터 생성과 리깅은 아직 구현되지 않았습니다.",
  language: "언어",
  mode: "작업 모드",
  beginner: "초보자",
  expert: "전문가",
  step1: "1 · 프로젝트 만들기",
  step2: "2 · 원본 일러스트 보존",
  step3: "3 · 작업 흐름 시험",
  step4: "4 · 결과 확인 및 내보내기",
  name: "프로젝트 이름",
  dimension: "모델 유형",
  target: "방송 프로그램",
  create: "프로젝트 만들기",
  save: "이름 저장",
  open: "저장된 프로젝트 열기",
  choose: "프로젝트 선택",
  import: "PNG 일러스트 선택",
  sourceHint:
    "DEV-00은 PNG만 지원 · 최대 64 MiB / 4천만 픽셀. 원본을 축소하거나 재압축하지 않습니다.",
  sourceEmpty: "일러스트가 없습니다",
  sourcePreserved: "원본 보존됨",
  revision: "저장 버전",
  start: "더미 작업 실행",
  cancel: "작업 취소",
  idle: "작업 흐름을 시험할 수 있습니다",
  running: "작업 흐름 시험 중…",
  completed: "더미 작업 완료. 모델은 생성되지 않았습니다.",
  cancelled: "작업을 취소했습니다. 원본은 안전하게 보존됩니다.",
  failed: "작업이 실패했습니다. 저장된 프로젝트는 보존됩니다.",
  export: "모델 내보내기",
  exportBlocked:
    "생성·리깅·방송 프로그램 호환성이 아직 검증되지 않아 내보낼 수 없습니다.",
  gates: "품질 검사",
  unverified: "미검증",
  pass: "통과",
  fail: "실패",
  not_applicable: "비해당",
  A: "원본 무결성",
  B: "이미지 충실도",
  C: "리깅 품질",
  D: "프로그램 호환성",
  E: "초보자 사용성",
  F: "안정성",
  gpu: "장치 진단",
  detect: "GPU 확인",
  unknown: "감지되지 않음: GPU 성능을 확인할 수 없습니다",
  inference:
    "AI 추론 지원 여부는 미검증입니다. 더미 작업은 CPU에서 실행됩니다.",
  driver: "드라이버",
  details: "기술 정보",
  offline: "로컬 작업 · 이미지 외부 전송 없음 · 광고 연결 없음",
  browser:
    "브라우저 화면 미리보기입니다. 프로젝트 생성과 저장은 데스크톱 앱에서 이용하세요.",
  busy: "처리 중…",
  error:
    "작업을 완료하지 못했습니다. 프로젝트와 저장 공간을 확인하고 다시 시도하세요.",
  invalid_name: "프로젝트 이름을 1~120자로 입력하세요.",
  invalid_profile: "모델 유형에 맞는 방송 프로그램을 선택하세요.",
  storage_error:
    "프로젝트를 저장하거나 읽지 못했습니다. 저장 공간과 폴더 권한을 확인하세요. 이전 저장본은 유지됩니다.",
  invalid_project:
    "프로젝트가 손상되었거나 지원되지 않습니다. 폴더를 보존하고 검증된 저장본에서 복원하세요.",
  schema_unsupported:
    "다른 앱 버전이 필요한 프로젝트입니다. 원본 데이터는 변경되지 않았습니다.",
  unsafe_path:
    "안전하게 사용할 수 없는 위치입니다. 일반 로컬 파일을 선택하세요.",
  revision_conflict:
    "더 최근에 저장된 버전이 있습니다. 프로젝트를 다시 열고 저장하세요.",
  source_changed:
    "보존된 원본의 해시가 기록과 다릅니다. 편집을 멈추고 백업에서 원본을 복원하세요.",
  source_exists:
    "원본이 이미 보존되어 있습니다. 다른 일러스트는 새 프로젝트에서 사용하세요.",
  image_too_large:
    "현재 안전한 입력 한도를 초과합니다. 원본을 임의로 축소하지 않습니다. 지원되는 원본을 사용하거나 대형 이미지 지원을 기다려 주세요.",
  invalid_image:
    "유효한 지원 PNG가 아닙니다. 그림 편집기에서 정지 PNG로 내보내고 다시 시도하세요.",
  source_required: "작업을 시작하기 전에 일러스트를 선택하세요.",
  python_unavailable:
    "Python을 실행할 수 없습니다. VTUBER_PYTHON에 Python 3.12 실행 파일을 지정하고 앱을 다시 시작하세요.",
  job_busy: "다른 작업이 실행 중입니다. 완료를 기다리거나 취소하세요.",
  job_missing: "현재 실행 중인 작업이 아닙니다. 화면을 새로 확인하세요.",
  worker_failed:
    "작업 프로세스가 종료되었습니다. 원본은 보존되어 있습니다. 다시 시도하세요.",
  worker_timeout: "시간이 초과되어 작업을 중단했습니다. 다시 시도하세요.",
  worker_protocol:
    "잘못된 작업 응답을 받아 중단했습니다. Worker와 앱의 버전이 같은지 확인하세요.",
};
export const catalogs = { en, ja, ko };
export function message(locale: Locale, key: string): string {
  return catalogs[locale][key as MessageKey] ?? catalogs[locale].error;
}
