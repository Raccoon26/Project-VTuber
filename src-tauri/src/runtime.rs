//! Registry and orchestrator. File revisions remain authoritative and untouched.
use crate::{
    durable::{plain, publish},
    storage::hash,
    AppError, Result,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{
    fs,
    io::Read,
    path::{Path, PathBuf},
    sync::atomic::{AtomicBool, Ordering},
    time::Duration,
};

pub const PROVIDER: &str = "onnx-synthetic-v1";
const MANIFEST: &str = include_str!("../../models/synthetic-affine-v1.json");
const LOCK: &str = include_str!("../../workers/runtime-lock.json");
const SCRIPT: &[u8] = include_bytes!("../../workers/runtime.py");
const MODEL: &[u8] = include_bytes!("../../models/synthetic-affine-v1.onnx");

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ModelManifest {
    pub schema_version: u32,
    pub id: String,
    pub version: String,
    pub revision: String,
    pub provider: String,
    pub kind: String,
    pub developer: String,
    pub official_source: String,
    pub download_url: Option<String>,
    pub format: String,
    pub bytes: u64,
    pub sha256: String,
    pub code_license: String,
    pub weights_license: String,
    pub dependency_licenses: Vec<String>,
    pub license_review: String,
    pub commercial_use: bool,
    pub redistribution: bool,
    pub allowed_regions: Vec<String>,
    pub output_restrictions: String,
    pub runtime: String,
    pub backends: Vec<String>,
    pub ram_mib: u64,
    pub vram_mib: u64,
    pub disk_mib: u64,
    pub input: String,
    pub output: String,
    pub preprocessing: String,
    pub verified_hardware: Vec<String>,
    pub evidence: Vec<String>,
    pub network_during_inference: bool,
}
impl ModelManifest {
    pub fn validate(&self, region: &str) -> Result<()> {
        if self.schema_version != 1
            || self.version != "1.0.0"
            || self.id != "synthetic-affine"
            || self.provider != PROVIDER
            || self.runtime != "onnx-cpu-win-py312-v1"
            || self.format != "onnx"
            || self.bytes != MODEL.len() as u64
            || self.sha256 != hash(MODEL)
            || self.revision != format!("sha256:{}", self.sha256)
            || self.network_during_inference
            || self.download_url.is_some()
            || self.official_source != "https://github.com/Raccoon26/Project-VTuber"
            || self.backends != ["onnx_cpu"]
            || self.ram_mib != 512
            || self.disk_mib != 512
            || self.vram_mib != 0
        {
            return Err(AppError::new("runtime_manifest"));
        }
        if self.license_review != "approved"
            || !self.commercial_use
            || !self.redistribution
            || self.weights_license != "CC0-1.0"
            || self.code_license != "project-owned"
            || !self.allowed_regions.iter().any(|x| x == "*" || x == region)
        {
            return Err(AppError::new("model_license"));
        }
        Ok(())
    }
}
pub fn manifest() -> Result<ModelManifest> {
    let model: ModelManifest =
        serde_json::from_str(MANIFEST).map_err(|_| AppError::new("runtime_manifest"))?;
    model.validate("*")?;
    Ok(model)
}
fn bounded(path: &Path, limit: u64) -> Result<Vec<u8>> {
    for ancestor in path.ancestors() {
        plain(ancestor)?;
    }
    let mut options = fs::OpenOptions::new();
    options.read(true);
    #[cfg(windows)]
    {
        use std::os::windows::fs::OpenOptionsExt;
        options.share_mode(1);
    }
    let file = options.open(path)?;
    if !file.metadata()?.is_file() || file.metadata()?.len() > limit {
        return Err(AppError::new("runtime_integrity"));
    }
    let mut data = Vec::new();
    file.take(limit + 1).read_to_end(&mut data)?;
    if data.len() as u64 > limit {
        return Err(AppError::new("runtime_integrity"));
    }
    Ok(data)
}
fn uuid_id(id: &str) -> Result<()> {
    if uuid::Uuid::parse_str(id)
        .map(|v| v.to_string())
        .ok()
        .as_deref()
        != Some(id)
    {
        return Err(AppError::new("unsafe_path"));
    }
    Ok(())
}

#[derive(Clone)]
pub struct RuntimeManager {
    root: PathBuf,
    python: PathBuf,
    script: PathBuf,
}
impl RuntimeManager {
    pub fn new(root: PathBuf, python: PathBuf, script: PathBuf) -> Result<Self> {
        if !root.is_absolute() {
            return Err(AppError::new("unsafe_path"));
        }
        for ancestor in root.ancestors().filter(|p| p.exists()) {
            plain(ancestor)?;
        }
        fs::create_dir_all(&root)?;
        for name in ["staging", "artifacts", "reports", "tmp"] {
            fs::create_dir_all(root.join(name))?;
            plain(&root.join(name))?;
        }
        manifest()?;
        Ok(Self {
            root,
            python,
            script,
        })
    }
    pub fn consent_token(&self) -> String {
        hash(format!("{MANIFEST}{LOCK}").as_bytes())
    }
    pub fn view(&self) -> Result<Value> {
        let lock: Value = serde_json::from_str(LOCK)?;
        let bytes: u64 = lock["files"]
            .as_array()
            .unwrap()
            .iter()
            .map(|x| x["bytes"].as_u64().unwrap())
            .sum();
        let state = match self.active_environment() {
            Ok(_) => "installed_unverified",
            Err(e) if e.code == "runtime_not_installed" => "approval_required",
            Err(_) => "blocked",
        };
        let installed = state == "installed_unverified";
        Ok(
            json!({"version":1,"manifest":manifest()?,"consent_token":self.consent_token(),
            "download_bytes":bytes,"disk_mib":512,"install_state":state,
            "backends":[{"id":"onnx_cpu","status":if installed {"installed_unverified"} else {"not_installed"}},
            {"id":"pytorch_cpu","status":"not_installed"},{"id":"pytorch_cuda","status":"not_installed"},
            {"id":"onnx_directml","status":"not_installed"},{"id":"cuda","status":"unverified"}],
            "scope":"managed_provider_environment"}),
        )
    }
    fn active_environment(&self) -> Result<PathBuf> {
        let dir = self.root.join("active");
        if !dir.try_exists()? {
            return Err(AppError::new("runtime_not_installed"));
        }
        plain(&dir)?;
        let mut records = Vec::new();
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let name = entry.file_name().to_string_lossy().into_owned();
            if name.len() == 13
                && name.ends_with(".json")
                && name.as_bytes()[..8].iter().all(u8::is_ascii_digit)
            {
                records.push(entry.path());
            }
            if records.len() > 128 {
                return Err(AppError::new("runtime_integrity"));
            }
        }
        records.sort();
        let record: Value = serde_json::from_slice(&bounded(
            records
                .last()
                .ok_or_else(|| AppError::new("runtime_not_installed"))?,
            65536,
        )?)?;
        let generation = record["generation"]
            .as_str()
            .ok_or_else(|| AppError::new("runtime_integrity"))?;
        if generation.len() != 32
            || !generation
                .bytes()
                .all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase())
            || record["lock_hash"] != hash(LOCK.as_bytes())
        {
            return Err(AppError::new("runtime_integrity"));
        }
        let root = self.root.join("environments").join(generation);
        let receipt = bounded(&root.join("receipt.json"), 4 * 1024 * 1024)?;
        if record["receipt_hash"] != hash(&receipt) {
            return Err(AppError::new("runtime_integrity"));
        }
        let receipt: Value = serde_json::from_slice(&receipt)?;
        let python = root.join("venv/Scripts/python.exe");
        if receipt["files"]["Scripts/python.exe"] != hash(&bounded(&python, 32 * 1024 * 1024)?) {
            return Err(AppError::new("runtime_integrity"));
        }
        Ok(python)
    }
    fn request(&self, id: &str, action: &str) -> Value {
        json!({"protocol":2,"provider":PROVIDER,"job_id":id,"action":action,"root":self.root,
               "lock_hash":hash(LOCK.as_bytes())})
    }
    fn call(
        &self,
        python: &Path,
        request: &Value,
        cancel: &AtomicBool,
        progress: &mut dyn FnMut(u8),
    ) -> Result<Value> {
        if !self.python.is_absolute()
            || !python.is_absolute()
            || hash(&bounded(&self.script, 1024 * 1024)?) != hash(SCRIPT)
        {
            return Err(AppError::new("runtime_integrity"));
        }
        crate::runtime_process::execute(
            python,
            &self.script,
            &self.root,
            request,
            cancel,
            if request["action"] == "install" {
                Duration::from_secs(600)
            } else {
                Duration::from_secs(60)
            },
            progress,
        )
    }
    pub fn install(
        &self,
        id: &str,
        approved: bool,
        token: &str,
        cancel: &AtomicBool,
        progress: &mut dyn FnMut(u8),
    ) -> Result<()> {
        if !approved || token != self.consent_token() {
            return Err(AppError::new("download_consent"));
        }
        let mut request = self.request(id, "install");
        request["consent"] = true.into();
        request["lock"] = serde_json::from_str(LOCK)?;
        request["model_hex"] = MODEL
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect::<String>()
            .into();
        let result = self.call(&self.python, &request, cancel, progress)?;
        if result["state"] != "runnable" || result["probe"]["output"] != json!([2.25, 5.5]) {
            return Err(AppError::new("inference_mismatch"));
        }
        self.report(id, &result)
    }
    pub fn probe(&self, id: &str, cancel: &AtomicBool, progress: &mut dyn FnMut(u8)) -> Result<()> {
        let result = self.call(&self.python, &self.request(id, "probe"), cancel, progress)?;
        if result["output"] != json!([2.25, 5.5]) || result["backend"] != "onnx_cpu" {
            return Err(AppError::new("inference_mismatch"));
        }
        self.report(id, &result)
    }
    fn report(&self, id: &str, value: &Value) -> Result<()> {
        uuid_id(id)?;
        let digest = hash(&serde_json::to_vec(value)?);
        publish(
            &self.root.join("reports").join(format!("{id}.json")),
            &serde_json::to_vec(&json!({"version":1,"job_id":id,"sha256":digest,"data":value}))?,
        )
    }
    pub fn result(&self, id: &str) -> Result<Value> {
        uuid_id(id)?;
        let report: Value = serde_json::from_slice(&bounded(
            &self.root.join("reports").join(format!("{id}.json")),
            65536,
        )?)?;
        let value = &report["data"];
        if report["version"] != 1
            || report["job_id"] != id
            || report["sha256"] != hash(&serde_json::to_vec(value)?)
        {
            return Err(AppError::new("artifact_invalid"));
        }
        if value["kind"] == "runtime_diagnostic" {
            let artifact: DiagnosticArtifact = serde_json::from_value(value.clone())
                .map_err(|_| AppError::new("artifact_invalid"))?;
            artifact.validate(id, artifact.source_hash.as_deref())?;
            let bytes = bounded(
                &self
                    .root
                    .join("artifacts")
                    .join(format!("{}.json", report["sha256"].as_str().unwrap())),
                65536,
            )?;
            if hash(&bytes) != report["sha256"] {
                return Err(AppError::new("artifact_invalid"));
            }
        } else {
            let probe = if value["state"] == "runnable" {
                &value["probe"]
            } else {
                value
            };
            if probe["output"] != json!([2.25, 5.5]) || probe["backend"] != "onnx_cpu" {
                return Err(AppError::new("artifact_invalid"));
            }
        }
        Ok(value.clone())
    }
    pub fn infer(
        &self,
        id: &str,
        source_hash: Option<String>,
        cancel: &AtomicBool,
        progress: &mut dyn FnMut(u8),
    ) -> Result<Value> {
        uuid_id(id)?;
        self.active_environment()?;
        let stage = self.root.join("staging").join(id);
        fs::create_dir(&stage)?;
        let mut request = self.request(id, "infer");
        request["stage"] = json!(stage);
        request["source_hash"] = json!(source_hash);
        request["input"] = json!([1.0, 2.0, 3.0]);
        let result = self.call(&self.python, &request, cancel, progress)?;
        if cancel.load(Ordering::SeqCst) {
            return Err(AppError::new("cancelled"));
        }
        if result["artifact"] != "result.json" {
            return Err(AppError::new("artifact_invalid"));
        }
        let bytes = bounded(&stage.join("result.json"), 65536)?;
        if result["sha256"] != hash(&bytes) {
            return Err(AppError::new("artifact_invalid"));
        }
        let artifact: DiagnosticArtifact =
            serde_json::from_slice(&bytes).map_err(|_| AppError::new("artifact_invalid"))?;
        artifact.validate(id, source_hash.as_deref())?;
        Ok(serde_json::to_value(artifact)?)
    }
    // Called only after the core reopens and verifies the unchanged original source.
    pub fn accept(&self, id: &str, artifact: &Value) -> Result<()> {
        uuid_id(id)?;
        let checked: DiagnosticArtifact = serde_json::from_value(artifact.clone())
            .map_err(|_| AppError::new("artifact_invalid"))?;
        checked.validate(id, checked.source_hash.as_deref())?;
        let bytes = serde_json::to_vec(artifact)?;
        let target = self
            .root
            .join("artifacts")
            .join(format!("{}.json", hash(&bytes)));
        publish(&target, &bytes)?;
        self.report(id, artifact)
    }
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DiagnosticArtifact {
    version: u32,
    kind: String,
    job_id: String,
    provider: String,
    model_version: String,
    model_hash: String,
    source_hash: Option<String>,
    input: Vec<f64>,
    preprocessing: String,
    seed: Option<u64>,
    uncertainty: String,
    output: Vec<f64>,
    backend: String,
    device: String,
    load_ms: f64,
    inference_ms: f64,
    cpu_seconds: f64,
    peak_ram_bytes: u64,
}
impl DiagnosticArtifact {
    fn validate(&self, id: &str, source: Option<&str>) -> Result<()> {
        if self.version != 1
            || self.kind != "runtime_diagnostic"
            || self.job_id != id
            || self.provider != PROVIDER
            || self.model_version != "1.0.0"
            || self.model_hash != hash(MODEL)
            || self.source_hash.as_deref() != source
            || self.source_hash.as_ref().is_some_and(|h| {
                h.len() != 64
                    || !h
                        .bytes()
                        .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
            })
            || self.input != [1.0, 2.0, 3.0]
            || self.output != [2.25, 5.5]
            || self.backend != "onnx_cpu"
            || self.device != "CPU"
            || self.preprocessing != "none"
            || self.seed.is_some()
            || self.uncertainty != "unknown"
            || ![self.load_ms, self.inference_ms, self.cpu_seconds]
                .iter()
                .all(|x| x.is_finite() && *x >= 0.0 && *x < 60000.0)
            || self.peak_ram_bytes > 512 * 1024 * 1024
        {
            return Err(AppError::new("artifact_invalid"));
        }
        Ok(())
    }
}
