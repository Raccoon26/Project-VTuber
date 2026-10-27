export const en = {
  recovery: "Storage recovery",
  projectId: "Project ID",
  diagnose: "Inspect saved versions",
  history: "Saved versions",
  historyHint:
    "Only versions with intact referenced originals can be selected. This is a data preview, not a rendered character.",
  previewRecovery: "Preview recovery",
  changedFields: "Changed fields",
  noChanges: "No content changes",
  sourceCount: "Referenced originals",
  recoveryPreserves:
    "All existing saved versions and original files remain untouched. Recovery creates a new version.",
  recoveryExcludes:
    "Changes after the selected version are not included in the restored state. Missing or damaged originals cannot be reconstructed. Job history is separate.",
  confirmRecovery:
    "I reviewed the changes and want to create a restored version.",
  restore: "Create restored version",
  cancelRecovery: "Cancel recovery",
  latest_unreadable: "Latest saved data cannot be read",
  recoverable:
    "This project has damaged saved data. Earlier versions may be available.",
  unrecoverable:
    "No intact saved version was found. Preserve the files and check a backup.",
  uninspectable:
    "This project could not be inspected. Check access and diagnostics.",
  project_busy:
    "Another process is using this project. Wait and refresh; do not delete lock files.",
  access_denied:
    "File access was denied. Check folder permissions and other applications.",
  disk_full:
    "There is not enough storage space. Existing versions remain preserved. Free space and inspect before retrying.",
  inspection_limit:
    "This history exceeds the inspection limit. Preserve it and request expert diagnostics.",
  index_unavailable:
    "The search index could not be updated. The list uses verified files; refresh to rebuild.",
  queued: "Waiting to start",
  interrupted:
    "The previous job stopped unexpectedly; completion was not confirmed.",
  recovery_required:
    "Job records need inspection before their outcome can be trusted.",
  job_interrupted:
    "The application stopped before a terminal result was safely recorded.",
  journal_invalid:
    "The job record is damaged or unsupported. It has been preserved.",
  journal_write_failed:
    "Job state could not be saved. Do not treat this run as completed.",
  jobHistory: "Job history",
  retryHint:
    "To retry, open and verify the original project, then start a new dummy job. No automatic resume is performed.",
  search: "Search projects",
  refresh: "Refresh and rebuild index",
  field_name: "Name",
  field_dimension: "Model type",
  field_target_profile: "Broadcast profile",
  field_sources: "Referenced originals",
  field_gates: "Quality results",
  field_master_status: "Model state",
  field_latest_unreadable:
    "Latest state is unreadable; exact differences are unknown",
  title: "Character workshop",
  subtitle: "Preserve the original. Build with evidence.",
  foundation: "DEV-01 · Storage recovery",
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
  recovery: "保存データの復旧",
  projectId: "プロジェクト ID",
  diagnose: "保存履歴を確認",
  history: "保存履歴",
  historyHint:
    "参照元画像が正常な履歴のみ選択できます。これはデータの確認であり、キャラクター表示ではありません。",
  previewRecovery: "復旧内容を確認",
  changedFields: "変更される項目",
  noChanges: "内容の変更なし",
  sourceCount: "参照元画像数",
  recoveryPreserves:
    "既存の履歴と元画像を残したまま、新しい保存版を作成します。",
  recoveryExcludes:
    "選択版以降の変更は復旧状態に含まれません。欠損・破損した元画像は再生成できません。作業履歴は別に保持されます。",
  confirmRecovery: "変更内容を確認し、復旧版を作成します。",
  restore: "復旧版を作成",
  cancelRecovery: "復旧をキャンセル",
  latest_unreadable: "最新の保存データを読み取れません",
  recoverable: "保存データに問題があります。以前の版を確認できます。",
  unrecoverable:
    "正常な保存版が見つかりません。ファイルを保管しバックアップを確認してください。",
  uninspectable: "検査できませんでした。アクセス権と診断を確認してください。",
  project_busy:
    "別のプロセスが使用中です。待って再読み込みしてください。ロックファイルを削除しないでください。",
  access_denied:
    "ファイルへのアクセスが拒否されました。権限と他のアプリを確認してください。",
  disk_full:
    "保存容量が不足しています。既存の履歴は保持されています。空き容量と履歴を確認してください。",
  inspection_limit:
    "履歴が検査上限を超えています。保管して専門診断を行ってください。",
  index_unavailable:
    "検索索引を更新できません。検証済みファイルから表示しています。再読み込みで再構築します。",
  queued: "開始待ち",
  interrupted: "前回の作業が中断されました。完了は確認されていません。",
  recovery_required: "結果を確認するには作業記録の検査が必要です。",
  job_interrupted: "終了結果を安全に保存する前にアプリが停止しました。",
  journal_invalid:
    "作業記録が破損または未対応です。元の記録は保持されています。",
  journal_write_failed:
    "作業状態を保存できませんでした。完了とは扱わないでください。",
  jobHistory: "作業履歴",
  retryHint:
    "再試行する場合は元のプロジェクトを開いて確認し、新しいダミー作業を開始してください。自動再開は行いません。",
  search: "プロジェクト検索",
  refresh: "再読み込み・索引再構築",
  field_name: "名前",
  field_dimension: "モデル形式",
  field_target_profile: "配信プロファイル",
  field_sources: "参照元画像",
  field_gates: "品質結果",
  field_master_status: "モデル状態",
  field_latest_unreadable: "最新状態を読めないため差分は不明",
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
  recovery: "저장 데이터 복구",
  projectId: "프로젝트 ID",
  diagnose: "진단 및 저장 이력 확인",
  history: "저장 이력",
  historyHint:
    "참조 원본까지 정상인 버전만 선택할 수 있습니다. 캐릭터 렌더링이 아닌 저장 데이터 미리보기입니다.",
  previewRecovery: "복구 미리보기",
  changedFields: "변경 항목",
  noChanges: "내용 변경 없음",
  sourceCount: "참조 원본 수",
  recoveryPreserves:
    "기존 저장 이력과 원본 파일은 모두 보존하고 새 버전을 만듭니다.",
  recoveryExcludes:
    "선택한 버전 이후 변경은 복구 상태에 포함되지 않습니다. 없거나 손상된 원본은 재생성하지 못합니다. 작업 기록은 별도로 유지됩니다.",
  confirmRecovery: "변경 내용을 확인했으며 새 복구 버전을 만들겠습니다.",
  restore: "새 복구 버전 만들기",
  cancelRecovery: "복구 취소",
  latest_unreadable: "최신 저장 데이터를 읽을 수 없습니다",
  recoverable:
    "이 프로젝트의 저장 데이터에 문제가 있습니다. 이전 버전을 확인할 수 있습니다.",
  unrecoverable:
    "정상 저장 버전을 찾지 못했습니다. 파일을 보존하고 백업을 확인하세요.",
  uninspectable:
    "프로젝트를 검사하지 못했습니다. 접근 권한과 진단을 확인하세요.",
  project_busy:
    "다른 프로세스가 사용 중입니다. 잠시 후 새로고침하세요. 잠금 파일을 삭제하지 마세요.",
  access_denied:
    "파일 접근이 거부되었습니다. 폴더 권한과 다른 프로그램의 사용 여부를 확인하세요.",
  disk_full:
    "저장 공간이 부족합니다. 기존 이력은 보존됩니다. 공간을 확보하고 이력을 확인한 뒤 다시 시도하세요.",
  inspection_limit:
    "이력이 검사 한도를 초과합니다. 데이터를 보존하고 전문가 진단을 진행하세요.",
  index_unavailable:
    "검색 인덱스를 갱신하지 못했습니다. 검증한 파일로 목록을 표시합니다. 새로고침하면 재구축합니다.",
  queued: "실행 대기 중",
  interrupted:
    "이전 작업이 비정상적으로 중단되었습니다. 완료가 확인되지 않았습니다.",
  recovery_required: "작업 결과를 확인하려면 기록 검사가 필요합니다.",
  job_interrupted: "종료 결과를 안전하게 기록하기 전에 앱이 종료되었습니다.",
  journal_invalid:
    "작업 기록이 손상되었거나 지원하지 않는 형식입니다. 기존 기록은 보존했습니다.",
  journal_write_failed:
    "작업 상태를 저장하지 못했습니다. 완료된 작업으로 취급하지 마세요.",
  jobHistory: "작업 기록",
  retryHint:
    "재시도하려면 원래 프로젝트를 열어 확인한 뒤 새 더미 작업을 시작하세요. 자동 재개는 하지 않습니다.",
  search: "프로젝트 검색",
  refresh: "새로고침 및 인덱스 재구축",
  field_name: "이름",
  field_dimension: "모델 종류",
  field_target_profile: "방송 프로필",
  field_sources: "참조 원본",
  field_gates: "품질 결과",
  field_master_status: "모델 상태",
  field_latest_unreadable: "최신 상태를 읽지 못해 정확한 차이를 알 수 없음",
  title: "캐릭터 제작실",
  subtitle: "원본을 지키고, 검증을 거쳐 완성합니다.",
  foundation: "DEV-01 · 저장 및 복구",
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
