export const en = {
  runtimeJobCompleted:
    "Local AI task completed. No character model was generated.",
  runtimeTitle: "Local AI readiness",
  runtimeVerified: "CPU calculation verified in this run.",
  runtimeInstalled: "Installed. Check this computer before use.",
  runtimeNeedsSetup: "Local AI preparation is needed.",
  runtimeBlocked:
    "The installed AI environment could not be verified. Prepare a new environment to retry; existing files will be kept.",
  runtimeDiagnosticOnly:
    "This small calculation checks the AI engine. It does not analyze or modify your character, create a model, or rig it.",
  runtimeSetupDetails: "Preparation and download details",
  runtimeSetupPurpose:
    "Install a dedicated CPU environment and a 216-byte synthetic test model. It can run offline after setup.",
  runtimeDisk: "Required free space",
  runtimeLicenseNotice:
    "Full runtime and third-party license notices are retained with the installation. Synthetic weights: CC0. No payment or account is required.",
  runtimeDownloadOrigin:
    "Source: pinned official PyPI files (files.pythonhosted.org). Your images are not sent.",
  runtimeConsent: "I approve this download and installation.",
  runtimePrepare: "Prepare local AI",
  runtimeProbe: "Check CPU calculation",
  runtimeRun: "Run local test calculation",
  runtimeResult: "Verified synthetic result",
  runtimeExpert: "Backend and model diagnostics",
  download_consent:
    "Review the download details and approve preparation first.",
  download_origin:
    "The download source was not approved. No file was activated.",
  download_failed:
    "Download stopped. Check the connection and retry preparation.",
  model_size: "The received file size was unexpected. It was not installed.",
  model_hash: "File integrity could not be confirmed. It will not be executed.",
  runtime_manifest:
    "This model registration is not supported. Keep the current project and check the application version.",
  model_license:
    "Model use is blocked until license and regional restrictions are verified.",
  ram_insufficient:
    "There is not enough available memory. Close other tasks and retry. The original is preserved.",
  backend_unverified:
    "This execution method has not passed its calculation test. Use the verified CPU path.",
  runtime_not_installed: "Prepare the local AI environment first.",
  runtime_integrity:
    "The AI environment has changed or is incomplete. Prepare a new verified environment; previous files are retained.",
  runtime_setup_failed:
    "AI preparation failed. The incomplete environment was not activated. Retry preparation after checking the diagnostic.",
  runtime_platform:
    "This runtime package requires Windows x64 and Python 3.12.",
  inference_mismatch:
    "The calculation result did not match its check. The result was not accepted.",
  provider_blocked: "This AI provider is not approved for execution.",
  artifact_invalid:
    "The output failed validation. Existing project files are preserved.",
  artifact_unverified: "This job has no confirmed completed result.",
  runtime_isolation:
    "The operating system could not apply the execution limits. The worker was stopped.",
  network_denied:
    "The provider attempted an operation outside its local calculation permissions. It was stopped.",
  resource_unknown:
    "Available computer resources could not be checked. Retry the diagnostic.",
  runtime_io:
    "AI files could not be accessed. Check disk space and permissions, then retry.",
  runtime_failed:
    "The local AI test stopped. Your original and saved revisions are preserved. Check preparation and retry.",

  legacy_name_encoding:
    "Some legacy layer names may use a different regional encoding. Original name bytes are preserved; Unicode names are preferred.",
  previewLoading: "Preparing a display copy…",
  originalPreview: "Source image display copy",
  previewCopy: "Display copy: sRGB / 8-bit; original precision preserved",
  originalSize: "Original",
  sourceDetails: "Source and layer information",
  layerInspectionOnly:
    "Layer records are preserved for inspection. This view does not recomposite edited layers.",
  layerVisible: "Visible",
  layerHidden: "Hidden",
  importWorking:
    "Reading and checking the image. The original remains unchanged.",
  cancelInput: "Cancel image input",
  import_cancelled:
    "Image input was cancelled. Existing saved versions are preserved.",
  invalid_icc:
    "The embedded color profile could not be read safely. Open a separate copy in an image editor and check its profile.",
  unsupported_color:
    "This color mode or precision is not supported yet. Keep the original and try an RGB 8-bit or 16-bit copy.",
  unsupported_psb:
    "Large-document PSB is not supported yet. Keep the original and use a supported PSD copy.",
  unsupported_compression:
    "This PSD compression method is not supported yet. Keep the original; try a PSD copy with a compatible saved composite.",
  unsupported_image:
    "This image type is not supported. Use a static PNG or a supported PSD.",
  psd_cached_composite_only:
    "This preview uses the composite saved inside the PSD. Layer edits, effects and masks are not re-rendered here.",
  psd_global_data_uninterpreted:
    "Some Photoshop document data is preserved in the original but is not interpreted.",
  psd_layer_render_unverified:
    "Some layer features are preserved as records only. Photoshop-equivalent layer rendering is unverified.",
  psd_16bit_layer_blocks_uninterpreted:
    "16-bit layer extension blocks are preserved but are not yet interpreted.",
  color_assumed:
    "No usable color profile is declared. The display copy assumes sRGB; color accuracy is unverified.",
  gamma_chromaticities_preview_unverified:
    "Gamma/chromaticity information is preserved. This preview does not yet apply that custom color space.",
  srgb_declared: "sRGB declared",
  srgb_assumed: "sRGB assumed; unverified",
  icc_to_srgb: "ICC converted to display sRGB",

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
    "To retry, check the original project and AI readiness, then start a new task. No automatic resume is performed.",
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
  foundation: "DEV-03 · Local AI execution",
  notice:
    "This build preserves source images and tests local CPU inference. It does not generate or rig a character.",
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
  import: "Choose PNG / PSD illustration",
  sourceHint:
    "Select or drop a PNG/PSD. Original bytes stay intact. Input up to64MiB; preview has a separate memory limit.",
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
    "GPU inference is unverified. The local AI panel checks the supported CPU calculation separately.",
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
    "This image is damaged or outside the supported PNG/PSD structure. Keep the original and try a separate supported copy.",
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
  runtimeJobCompleted:
    "ローカルAIの処理が完了しました。キャラクターモデルは生成していません。",
  runtimeTitle: "ローカルAIの準備",
  runtimeVerified: "今回のCPU演算を確認しました。",
  runtimeInstalled: "インストール済みです。このPCで動作を確認できます。",
  runtimeNeedsSetup: "ローカルAIの準備が必要です。",
  runtimeBlocked:
    "AI環境を確認できませんでした。既存ファイルを残して、新しい環境を準備してください。",
  runtimeDiagnosticOnly:
    "小さな演算でAIエンジンを確認します。キャラクターの解析・変更、モデル生成、リギングは行いません。",
  runtimeSetupDetails: "準備とダウンロードの詳細",
  runtimeSetupPurpose:
    "専用のCPU実行環境と216バイトの合成テストモデルを用意します。準備後はオフラインで実行できます。",
  runtimeDisk: "必要な空き容量",
  runtimeLicenseNotice:
    "実行環境と依存部品のライセンス表記をインストール先に保持します。合成重みはCC0です。支払い・アカウントは不要です。",
  runtimeDownloadOrigin:
    "取得先：固定済みの公式PyPIファイル（files.pythonhosted.org）。画像は送信しません。",
  runtimeConsent: "このダウンロードとインストールに同意します。",
  runtimePrepare: "ローカルAIを準備",
  runtimeProbe: "CPU演算を確認",
  runtimeRun: "ローカルのテスト演算を実行",
  runtimeResult: "検証済みの合成結果",
  runtimeExpert: "バックエンドとモデルの診断",
  download_consent: "ダウンロード内容を確認して準備に同意してください。",
  download_origin:
    "許可されていない取得先です。ファイルは有効化されていません。",
  download_failed:
    "取得が停止しました。接続を確認して準備を再試行してください。",
  model_size:
    "受信したファイルのサイズが異なります。インストールしていません。",
  model_hash: "ファイルの整合性を確認できません。実行しません。",
  runtime_manifest:
    "このモデル登録は未対応です。プロジェクトを保持してアプリのバージョンを確認してください。",
  model_license: "ライセンスと地域制限を確認するまでモデルを使用できません。",
  ram_insufficient:
    "使用可能なメモリが不足しています。他の処理を閉じて再試行してください。原本は保持されています。",
  backend_unverified:
    "この実行方式は演算検証が済んでいません。検証済みのCPU方式を使用してください。",
  runtime_not_installed: "まずローカルAI環境を準備してください。",
  runtime_integrity:
    "AI環境が変更されたか不完全です。検証済みの環境を再準備してください。以前のファイルは保持されます。",
  runtime_setup_failed:
    "AIの準備に失敗しました。不完全な環境は有効化されていません。診断を確認して再試行してください。",
  runtime_platform: "この実行環境はWindows x64とPython 3.12が必要です。",
  inference_mismatch:
    "演算結果が検証値と一致しません。結果を採用していません。",
  provider_blocked: "このAIプロバイダーは実行を許可されていません。",
  artifact_invalid:
    "出力の検証に失敗しました。既存のプロジェクトは保持されています。",
  artifact_unverified: "このジョブには確認済みの完了結果がありません。",
  runtime_isolation: "OSが実行制限を適用できませんでした。処理を停止しました。",
  network_denied: "ローカル演算の許可範囲外の操作があったため停止しました。",
  resource_unknown:
    "使用可能なPC資源を確認できませんでした。診断を再試行してください。",
  runtime_io:
    "AIファイルにアクセスできません。空き容量と権限を確認して再試行してください。",
  runtime_failed:
    "ローカルAIの試験が停止しました。原本と保存履歴は保持されています。準備状態を確認して再試行してください。",

  legacy_name_encoding:
    "古いレイヤー名は地域別の文字コードにより異なって表示される場合があります。原本の名前データは保持しています。",
  previewLoading: "表示用コピーを準備しています…",
  originalPreview: "原画の表示用コピー",
  previewCopy: "表示用コピー: sRGB / 8-bit・原本の精度は保持",
  originalSize: "原本",
  sourceDetails: "原本とレイヤー情報",
  layerInspectionOnly:
    "レイヤーの記録を保持して表示します。この画面は編集したレイヤーを再合成しません。",
  layerVisible: "表示",
  layerHidden: "非表示",
  importWorking: "画像を読み込み検査しています。原本は変更しません。",
  cancelInput: "画像入力をキャンセル",
  import_cancelled:
    "画像入力をキャンセルしました。既存の保存履歴は保持されています。",
  invalid_icc:
    "埋め込みプロファイルを安全に読めませんでした。画像編集ソフトで別コピーのプロファイルを確認してください。",
  unsupported_color:
    "未対応の色モードまたは精度です。原本を保管し、RGB 8-bit または16-bitのコピーをお試しください。",
  unsupported_psb:
    "大判文書PSBは未対応です。原本を保管し、対応するPSDコピーを使用してください。",
  unsupported_compression:
    "未対応のPSD圧縮方式です。原本を保管し、合成画像付きの互換PSDコピーをお試しください。",
  unsupported_image:
    "未対応の画像形式です。静止PNGまたは対応PSDを使用してください。",
  psd_cached_composite_only:
    "PSDに保存された合成画像を表示しています。レイヤー・効果・マスクは再描画しません。",
  psd_global_data_uninterpreted:
    "一部のPhotoshop文書情報は原本に保持されますが、まだ解釈しません。",
  psd_layer_render_unverified:
    "一部のレイヤー機能は記録のみ保持します。Photoshopと同等の描画は未検証です。",
  psd_16bit_layer_blocks_uninterpreted:
    "16-bitレイヤー拡張は原本に保持しますが、まだ解釈しません。",
  color_assumed:
    "使用可能な色プロファイルがありません。表示コピーはsRGBと仮定し、色の正確性は未検証です。",
  gamma_chromaticities_preview_unverified:
    "ガンマ・色度情報は保持します。この表示は独自色空間をまだ適用しません。",
  srgb_declared: "sRGB指定",
  srgb_assumed: "sRGB仮定・未検証",
  icc_to_srgb: "ICCから表示用sRGBへ変換",

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
    "再試行する場合は元のプロジェクトとAI準備状態を確認し、新しい処理を開始してください。自動再開は行いません。",
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
  foundation: "DEV-03 · ローカルAI実行",
  notice:
    "このビルドでは原画の保持とローカルCPU推論を検証します。キャラクター生成・リギングは未実装です。",
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
  import: "PNG / PSDイラストを選択",
  sourceHint:
    "PNG/PSDを選択またはドロップしてください。原本を保持します。入力64MiBまで・表示用メモリ制限は別です。",
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
  inference:
    "GPU推論は未検証です。対応するCPU演算はローカルAI欄で別途確認します。",
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
    "画像が破損しているか未対応のPNG/PSD構造です。原本を保管し、対応する別コピーをお試しください。",
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
  runtimeJobCompleted:
    "로컬 AI 작업을 완료했습니다. 캐릭터 모델은 생성하지 않았습니다.",
  runtimeTitle: "로컬 AI 준비",
  runtimeVerified: "이번 실행에서 CPU 연산을 확인했습니다.",
  runtimeInstalled:
    "설치되어 있습니다. 이 컴퓨터에서 동작을 확인할 수 있습니다.",
  runtimeNeedsSetup: "로컬 AI 준비가 필요합니다.",
  runtimeBlocked:
    "설치된 AI 환경을 확인할 수 없습니다. 기존 파일을 보존하고 새 환경을 준비해 다시 시도하세요.",
  runtimeDiagnosticOnly:
    "작은 계산으로 AI 엔진을 검사합니다. 캐릭터 분석·변경, 모델 생성 또는 리깅 기능은 아닙니다.",
  runtimeSetupDetails: "준비 및 다운로드 안내",
  runtimeSetupPurpose:
    "전용 CPU 실행 환경과 216바이트 합성 시험 모델을 준비합니다. 설치 후에는 오프라인으로 실행할 수 있습니다.",
  runtimeDisk: "필요한 여유 공간",
  runtimeLicenseNotice:
    "실행 환경과 종속 구성요소의 전체 라이선스 고지문을 설치 위치에 보존합니다. 합성 가중치는 CC0이며 결제나 계정은 필요 없습니다.",
  runtimeDownloadOrigin:
    "출처: 버전과 해시가 고정된 공식 PyPI 파일(files.pythonhosted.org). 사용자 이미지는 전송하지 않습니다.",
  runtimeConsent: "이 다운로드와 설치에 동의합니다.",
  runtimePrepare: "로컬 AI 준비하기",
  runtimeProbe: "CPU 연산 검사",
  runtimeRun: "로컬 시험 연산 실행",
  runtimeResult: "검증된 합성 계산 결과",
  runtimeExpert: "백엔드 및 모델 진단",
  download_consent: "다운로드 내용을 확인하고 먼저 설치에 동의해 주세요.",
  download_origin:
    "허용되지 않은 다운로드 출처입니다. 파일을 활성화하지 않았습니다.",
  download_failed:
    "다운로드가 중단됐습니다. 연결을 확인하고 준비를 다시 시도해 주세요.",
  model_size: "받은 파일 크기가 예상과 다릅니다. 설치하지 않았습니다.",
  model_hash: "파일 무결성을 확인할 수 없습니다. 이 파일은 실행하지 않습니다.",
  runtime_manifest:
    "지원하지 않는 모델 등록 정보입니다. 현재 프로젝트를 보존하고 앱 버전을 확인해 주세요.",
  model_license:
    "라이선스와 지역 제한 확인 전에는 이 모델을 사용할 수 없습니다.",
  ram_insufficient:
    "사용 가능한 메모리가 부족합니다. 다른 작업을 닫고 다시 시도해 주세요. 원본은 보존됩니다.",
  backend_unverified:
    "이 실행 방식은 연산 검증을 통과하지 않았습니다. 검증된 CPU 방식을 사용해 주세요.",
  runtime_not_installed: "먼저 로컬 AI 실행 환경을 준비해 주세요.",
  runtime_integrity:
    "AI 환경이 변경됐거나 불완전합니다. 검증된 환경을 다시 준비해 주세요. 기존 파일은 보존합니다.",
  runtime_setup_failed:
    "AI 준비에 실패했습니다. 불완전한 환경은 활성화하지 않았습니다. 진단을 확인한 뒤 다시 준비해 주세요.",
  runtime_platform: "이 실행 환경은 Windows x64와 Python 3.12가 필요합니다.",
  inference_mismatch:
    "계산 결과가 검증값과 일치하지 않습니다. 결과를 등록하지 않았습니다.",
  provider_blocked: "실행이 승인되지 않은 AI Provider입니다.",
  artifact_invalid:
    "출력 검증에 실패했습니다. 기존 프로젝트 파일은 보존됩니다.",
  artifact_unverified: "이 작업에는 완료가 확인된 결과가 없습니다.",
  runtime_isolation:
    "운영체제가 실행 제한을 적용하지 못했습니다. Worker를 중단했습니다.",
  network_denied: "로컬 연산 허용 범위를 벗어난 작업이 감지되어 중단했습니다.",
  resource_unknown:
    "사용 가능한 컴퓨터 자원을 확인하지 못했습니다. 진단을 다시 실행해 주세요.",
  runtime_io:
    "AI 파일에 접근하지 못했습니다. 여유 공간과 권한을 확인한 뒤 다시 시도해 주세요.",
  runtime_failed:
    "로컬 AI 시험이 중단됐습니다. 원본과 저장 이력은 보존됩니다. 준비 상태를 확인한 뒤 다시 시도해 주세요.",

  legacy_name_encoding:
    "일부 오래된 레이어 이름은 지역별 문자 인코딩에 따라 다르게 보일 수 있습니다. 원본 이름 데이터는 보존됩니다.",
  previewLoading: "화면에 표시할 사본을 준비하고 있습니다…",
  originalPreview: "원본 이미지의 표시용 사본",
  previewCopy: "표시용 사본: sRGB / 8-bit · 원본 정밀도 보존",
  originalSize: "원본",
  sourceDetails: "원본 및 레이어 정보",
  layerInspectionOnly:
    "레이어 기록을 보존하여 보여 줍니다. 이 화면은 수정한 레이어를 다시 합성하지 않습니다.",
  layerVisible: "표시",
  layerHidden: "숨김",
  importWorking: "이미지를 읽고 검사하고 있습니다. 원본은 변경되지 않습니다.",
  cancelInput: "이미지 입력 취소",
  import_cancelled: "이미지 입력을 취소했습니다. 기존 저장 이력은 보존됩니다.",
  invalid_icc:
    "색상 프로필을 안전하게 읽지 못했습니다. 이미지 편집기에서 별도 사본의 프로필을 확인하세요.",
  unsupported_color:
    "아직 지원하지 않는 색상 모드 또는 정밀도입니다. 원본을 보관하고 RGB 8-bit 또는 16-bit 사본을 사용하세요.",
  unsupported_psb:
    "대형 문서 PSB는 아직 지원하지 않습니다. 원본을 보관하고 지원되는 PSD 사본을 사용하세요.",
  unsupported_compression:
    "아직 지원하지 않는 PSD 압축 방식입니다. 원본을 보관하고 합성 이미지가 저장된 호환 PSD 사본을 사용하세요.",
  unsupported_image:
    "지원하지 않는 이미지 유형입니다. 정지 PNG 또는 지원되는 PSD를 사용하세요.",
  psd_cached_composite_only:
    "PSD 안에 저장된 합성 이미지입니다. 이 화면에서 레이어·효과·마스크를 다시 렌더링하지 않습니다.",
  psd_global_data_uninterpreted:
    "일부 Photoshop 문서 정보는 원본에 보존되지만 아직 해석하지 않습니다.",
  psd_layer_render_unverified:
    "일부 레이어 기능은 기록으로만 보존합니다. Photoshop과 동일한 레이어 렌더링은 미검증입니다.",
  psd_16bit_layer_blocks_uninterpreted:
    "16-bit 레이어 확장 블록은 원본에 보존하지만 아직 해석하지 않습니다.",
  color_assumed:
    "사용할 수 있는 색상 프로필이 없습니다. 표시용 사본은 sRGB로 가정하며 색상 정확성은 미검증입니다.",
  gamma_chromaticities_preview_unverified:
    "감마·색도 정보는 보존합니다. 이 미리보기는 해당 사용자 색 공간을 아직 적용하지 않습니다.",
  srgb_declared: "sRGB 명시",
  srgb_assumed: "sRGB 가정 · 미검증",
  icc_to_srgb: "ICC에서 표시용 sRGB로 변환",

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
    "재시도하려면 원래 프로젝트와 AI 준비 상태를 확인한 뒤 새 작업을 시작하세요. 자동 재개는 하지 않습니다.",
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
  foundation: "DEV-03 · 로컬 AI 실행",
  notice:
    "이 버전은 원본 보존과 로컬 CPU 추론을 시험합니다. 캐릭터 생성과 리깅은 아직 구현되지 않았습니다.",
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
  import: "PNG / PSD 일러스트 선택",
  sourceHint:
    "PNG/PSD를 선택하거나 끌어 놓으세요. 원본을 그대로 보존합니다. 입력 최대64MiB · 미리보기 메모리 한도는 별도입니다.",
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
    "GPU 추론은 미검증입니다. 지원하는 CPU 연산은 로컬 AI 준비 화면에서 별도로 검사합니다.",
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
    "이미지가 손상되었거나 지원 범위를 벗어난 PNG/PSD 구조입니다. 원본을 보관하고 지원되는 별도 사본을 사용하세요.",
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
