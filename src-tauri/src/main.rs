#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::{
    path::PathBuf,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
};
use tauri::{Manager, State};
use vtuber_core::{
    gpu::{self, GpuReport},
    model::CharacterProject,
    recovery::{ProjectList, RecoveryPreview, RevisionReport},
    storage::ProjectStore,
    worker::{Job, JobManager},
    AppError, Result,
};

struct AppState {
    store: Arc<Mutex<ProjectStore>>,
    jobs: JobManager,
    script: PathBuf,
    python: PathBuf,
    import_cancel: Arc<AtomicBool>,
    importing: Arc<AtomicBool>,
    preview_cache: Arc<Mutex<Option<(String, serde_json::Value)>>>,
    dropped: Mutex<Option<PathBuf>>,
}
#[tauri::command]
fn create_project(
    state: State<AppState>,
    name: String,
    dimension: String,
    target: String,
) -> Result<CharacterProject> {
    state.store.lock().unwrap().create(name, dimension, target)
}
#[tauri::command]
fn list_projects(state: State<AppState>) -> Result<Vec<CharacterProject>> {
    state.store.lock().unwrap().list()
}
#[tauri::command]
async fn list_projects_v2(state: State<'_, AppState>) -> Result<ProjectList> {
    let store = state.store.clone();
    tauri::async_runtime::spawn_blocking(move || store.lock().unwrap().list_v2())
        .await
        .map_err(|_| AppError::new("storage_error"))?
}
#[tauri::command]
async fn search_projects(state: State<'_, AppState>, query: String) -> Result<ProjectList> {
    let store = state.store.clone();
    tauri::async_runtime::spawn_blocking(move || store.lock().unwrap().search_projects(&query))
        .await
        .map_err(|_| AppError::new("storage_error"))?
}
#[tauri::command]
async fn project_history(state: State<'_, AppState>, id: String) -> Result<Vec<RevisionReport>> {
    let store = state.store.clone();
    tauri::async_runtime::spawn_blocking(move || store.lock().unwrap().inspect(&id))
        .await
        .map_err(|_| AppError::new("storage_error"))?
}
#[tauri::command]
async fn preview_recovery(
    state: State<'_, AppState>,
    id: String,
    revision: u64,
) -> Result<RecoveryPreview> {
    let store = state.store.clone();
    tauri::async_runtime::spawn_blocking(move || {
        store.lock().unwrap().preview_recovery(&id, revision)
    })
    .await
    .map_err(|_| AppError::new("storage_error"))?
}
#[tauri::command]
async fn recover_project(
    state: State<'_, AppState>,
    id: String,
    revision: u64,
    token: String,
) -> Result<CharacterProject> {
    let store = state.store.clone();
    tauri::async_runtime::spawn_blocking(move || {
        store.lock().unwrap().recover(&id, revision, &token)
    })
    .await
    .map_err(|_| AppError::new("storage_error"))?
}
#[tauri::command]
fn job_history(state: State<AppState>) -> Result<Vec<Job>> {
    state.jobs.history()
}
#[tauri::command]
fn open_project(state: State<AppState>, id: String) -> Result<CharacterProject> {
    state.store.lock().unwrap().open(&id)
}
#[tauri::command]
fn save_project(
    state: State<AppState>,
    id: String,
    revision: u64,
    name: String,
) -> Result<CharacterProject> {
    state.store.lock().unwrap().rename(&id, revision, name)
}
#[tauri::command]
async fn import_source(
    state: State<'_, AppState>,
    id: String,
    revision: u64,
) -> Result<Option<CharacterProject>> {
    let selected = rfd::AsyncFileDialog::new()
        .add_filter("PNG / PSD", &["png", "psd"])
        .pick_file()
        .await;
    let Some(file) = selected else {
        return Ok(None);
    };
    run_import(&state, id, revision, file.path().to_owned())
        .await
        .map(Some)
}
#[tauri::command]
async fn import_dropped(
    state: State<'_, AppState>,
    id: String,
    revision: u64,
) -> Result<CharacterProject> {
    let path = state
        .dropped
        .lock()
        .unwrap()
        .take()
        .ok_or_else(|| AppError::new("unsupported_image"))?;
    run_import(&state, id, revision, path).await
}
async fn run_import(
    state: &AppState,
    id: String,
    revision: u64,
    path: PathBuf,
) -> Result<CharacterProject> {
    if state.importing.swap(true, Ordering::SeqCst) {
        return Err(AppError::new("project_busy"));
    }
    state.import_cancel.store(false, Ordering::SeqCst);
    let cancel = state.import_cancel.clone();
    let importing = state.importing.clone();
    let store = state.store.clone();
    tauri::async_runtime::spawn_blocking(move || {
        let result = store
            .lock()
            .unwrap()
            .import_image(&id, revision, &path, &cancel);
        importing.store(false, Ordering::SeqCst);
        result
    })
    .await
    .map_err(|_| AppError::new("storage_error"))?
}
#[tauri::command]
fn cancel_import(state: State<AppState>) {
    state.import_cancel.store(true, Ordering::SeqCst);
}
#[tauri::command]
async fn source_preview(state: State<'_, AppState>, id: String) -> Result<serde_json::Value> {
    use base64::Engine;
    let store = state.store.clone();
    let cache = state.preview_cache.clone();
    tauri::async_runtime::spawn_blocking(move || {
        let source_hash = store
            .lock()
            .unwrap()
            .open(&id)?
            .sources
            .first()
            .ok_or_else(|| AppError::new("source_required"))?
            .sha256
            .clone();
        if let Some((key, value)) = cache.lock().unwrap().as_ref() {
            if key == &source_hash {
                return Ok(value.clone());
            }
        }
        let mut inspection = store
            .lock()
            .unwrap()
            .inspect_source(&id, &AtomicBool::new(false))?;
        let data = base64::engine::general_purpose::STANDARD.encode(&inspection.preview_png);
        inspection.preview_png.clear();
        let mut value = serde_json::to_value(inspection)?;
        value["preview_url"] = format!("data:image/png;base64,{data}").into();
        if serde_json::to_vec(&value)?.len() < 8 * 1024 * 1024 {
            *cache.lock().unwrap() = Some((source_hash, value.clone()));
        }
        Ok(value)
    })
    .await
    .map_err(|_| AppError::new("invalid_image"))?
}
#[tauri::command]
fn start_job(state: State<AppState>, id: String) -> Result<Job> {
    let project = state.store.lock().unwrap().open(&id)?;
    if project.sources.is_empty() {
        return Err(AppError::new("source_required"));
    }
    state
        .jobs
        .start(id, state.python.clone(), state.script.clone())
}
#[tauri::command]
fn job_status(state: State<AppState>) -> Option<Job> {
    state.jobs.snapshot()
}
#[tauri::command]
fn cancel_job(state: State<AppState>, id: String) -> Result<()> {
    state.jobs.cancel(&id)
}
#[tauri::command]
async fn gpu_report() -> std::result::Result<GpuReport, String> {
    tauri::async_runtime::spawn_blocking(gpu::detect)
        .await
        .map_err(|_| "gpu_unknown".into())
}
fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // Portable by default: no project or WebView cache in the user's system drive.
            let data = std::env::var_os("VTUBER_DATA_DIR")
                .map(PathBuf::from)
                .unwrap_or(
                    std::env::current_exe()?
                        .parent()
                        .ok_or("Missing executable folder")?
                        .join("vtuber-data"),
                );
            if !data.is_absolute() {
                return Err("VTUBER_DATA_DIR must be absolute".into());
            }
            let root = data.join("projects");
            let script = if cfg!(debug_assertions) {
                PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../workers/dummy.py")
            } else {
                app.path().resource_dir()?.join("workers/dummy.py")
            };
            let python = std::env::var_os("VTUBER_PYTHON")
                .map(PathBuf::from)
                .unwrap_or_else(|| "python".into());
            app.manage(AppState {
                store: Arc::new(Mutex::new(ProjectStore::new(root).map_err(|e| e.code)?)),
                jobs: JobManager::persistent(data.join("jobs")).map_err(|e| e.code)?,
                script,
                python,
                import_cancel: Arc::new(AtomicBool::new(false)),
                importing: Arc::new(AtomicBool::new(false)),
                preview_cache: Arc::new(Mutex::new(None)),
                dropped: Mutex::new(None),
            });
            tauri::WebviewWindowBuilder::new(app, "main", tauri::WebviewUrl::default())
                .title("Project VTuber AI · DEV-02")
                .inner_size(1180.0, 860.0)
                .min_inner_size(800.0, 650.0)
                .data_directory(data.join("webview"))
                .build()?;
            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::DragDrop(tauri::DragDropEvent::Drop { paths, .. }) = event {
                *window.state::<AppState>().dropped.lock().unwrap() = if paths.len() == 1 {
                    Some(paths[0].clone())
                } else {
                    None
                };
            }
        })
        .invoke_handler(tauri::generate_handler![
            create_project,
            list_projects,
            list_projects_v2,
            search_projects,
            project_history,
            preview_recovery,
            recover_project,
            job_history,
            open_project,
            save_project,
            import_source,
            import_dropped,
            cancel_import,
            source_preview,
            start_job,
            job_status,
            cancel_job,
            gpu_report
        ])
        .build(tauri::generate_context!())
        .expect("Desktop initialization failed")
        .run(|app, event| {
            if let tauri::RunEvent::ExitRequested { .. } = event {
                app.state::<AppState>().jobs.shutdown();
            }
        });
}
