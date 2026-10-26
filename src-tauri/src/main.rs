#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};
use tauri::{Manager, State};
use vtuber_core::{
    gpu::{self, GpuReport},
    model::CharacterProject,
    storage::ProjectStore,
    worker::{Job, JobManager},
    AppError, Result,
};

struct AppState {
    store: Arc<Mutex<ProjectStore>>,
    jobs: JobManager,
    script: PathBuf,
    python: PathBuf,
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
        .add_filter("PNG", &["png"])
        .pick_file()
        .await;
    let Some(file) = selected else {
        return Ok(None);
    };
    let path = file.path().to_owned();
    let store = state.store.clone();
    tauri::async_runtime::spawn_blocking(move || {
        store
            .lock()
            .unwrap()
            .import_png(&id, revision, &path)
            .map(Some)
    })
    .await
    .map_err(|_| AppError::new("storage_error"))?
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
                jobs: JobManager::default(),
                script,
                python,
            });
            tauri::WebviewWindowBuilder::new(app, "main", tauri::WebviewUrl::default())
                .title("Project VTuber AI · DEV-00")
                .inner_size(1180.0, 860.0)
                .min_inner_size(800.0, 650.0)
                .data_directory(data.join("webview"))
                .build()?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            create_project,
            list_projects,
            open_project,
            save_project,
            import_source,
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
