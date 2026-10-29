use crate::{
    runtime::{self, RuntimeManager},
    runtime_process,
    worker::JobManager,
};
use serde_json::{json, Value};
use std::{
    fs,
    path::PathBuf,
    sync::atomic::{AtomicBool, Ordering},
    thread,
    time::{Duration, Instant},
};

fn python() -> PathBuf {
    std::env::var_os("VTUBER_PYTHON")
        .expect("test.ps1 sets actual Python")
        .into()
}
fn manager() -> RuntimeManager {
    RuntimeManager::new(
        std::env::var_os("VTUBER_RUNTIME_DIR")
            .expect("Run scripts/prepare-runtime.ps1 before tests")
            .into(),
        python(),
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../workers/runtime.py"),
    )
    .unwrap()
}
fn wait(manager: &JobManager) -> crate::worker::Job {
    let now = Instant::now();
    loop {
        let j = manager.snapshot().unwrap();
        if !["queued", "running"].contains(&j.status.as_str()) {
            return j;
        }
        assert!(now.elapsed() < Duration::from_secs(60));
        thread::sleep(Duration::from_millis(20));
    }
}
#[test]
fn registry_rejects_unreviewed_wrong_hash_region_schema_format_and_future_versions() {
    let original = serde_json::to_value(runtime::manifest().unwrap()).unwrap();
    for (field, value) in [
        ("schema_version", json!(2)),
        ("sha256", json!("a".repeat(64))),
        ("bytes", json!(215)),
        ("license_review", json!("unreviewed")),
        ("weights_license", json!("research-only")),
        ("commercial_use", json!(false)),
        ("redistribution", json!(false)),
        ("format", json!("pickle")),
        ("version", json!("2.0.0")),
        ("download_url", json!("https://evil.example/model")),
        ("network_during_inference", json!(true)),
        ("allowed_regions", json!(["XX"])),
    ] {
        let mut value0 = original.clone();
        value0[field] = value;
        let model: runtime::ModelManifest = serde_json::from_value(value0).unwrap();
        assert!(model.validate("JP").is_err(), "{field}");
    }
    let mut missing = original.clone();
    missing.as_object_mut().unwrap().remove("sha256");
    assert!(serde_json::from_value::<runtime::ModelManifest>(missing).is_err());
    let mut unknown = original;
    unknown["remote_code"] = json!(true);
    assert!(serde_json::from_value::<runtime::ModelManifest>(unknown).is_err());
    assert_eq!(
        runtime::manifest().unwrap().sha256,
        crate::storage::hash(include_bytes!("../../models/synthetic-affine-v1.onnx"))
    );
}
#[test]
fn consent_blocks_before_process_and_arbitrary_runtime_paths_are_not_accepted() {
    let tmp = tempfile::tempdir().unwrap();
    let m =
        RuntimeManager::new(tmp.path().join("rt"), python(), PathBuf::from("missing.py")).unwrap();
    let id = uuid::Uuid::new_v4().to_string();
    for (approve, token) in [(false, m.consent_token()), (true, "stale".into())] {
        assert_eq!(
            m.install(&id, approve, &token, &AtomicBool::new(false), &mut |_| {})
                .unwrap_err()
                .code,
            "download_consent"
        );
    }
    assert_eq!(m.result("../escape").unwrap_err().code, "unsafe_path");
    assert_eq!(
        m.probe(&id, &AtomicBool::new(false), &mut |_| {})
            .unwrap_err()
            .code,
        "storage_error"
    );
}
#[test]
fn report_and_artifact_tampering_and_corrupt_installation_are_not_certified() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path().join("runtime");
    let m = RuntimeManager::new(root.clone(), python(), PathBuf::from("unused.py")).unwrap();
    let id = uuid::Uuid::new_v4().to_string();
    let artifact = json!({"version":1,"kind":"runtime_diagnostic","job_id":id,
      "provider":runtime::PROVIDER,"model_version":"1.0.0","model_hash":runtime::manifest().unwrap().sha256,
      "source_hash":null,"input":[1.0,2.0,3.0],"preprocessing":"none","seed":null,
      "uncertainty":"unknown","output":[2.25,5.5],"backend":"onnx_cpu","device":"CPU",
      "load_ms":1.0,"inference_ms":0.1,"cpu_seconds":0.1,"peak_ram_bytes":1000});
    let mut bad = artifact.clone();
    bad["output"] = json!([0, 0]);
    assert_eq!(m.accept(&id, &bad).unwrap_err().code, "artifact_invalid");
    assert!(!root.join("reports").join(format!("{id}.json")).exists());
    m.accept(&id, &artifact).unwrap();
    assert_eq!(m.result(&id).unwrap(), artifact);
    let report = root.join("reports").join(format!("{id}.json"));
    let original = fs::read(&report).unwrap();
    let mut changed: Value = serde_json::from_slice(&original).unwrap();
    changed["data"]["output"] = json!([0, 0]);
    fs::write(&report, serde_json::to_vec(&changed).unwrap()).unwrap();
    assert_eq!(m.result(&id).unwrap_err().code, "artifact_invalid");
    fs::write(&report, original).unwrap();
    let artifact_path = fs::read_dir(root.join("artifacts"))
        .unwrap()
        .next()
        .unwrap()
        .unwrap()
        .path();
    fs::write(artifact_path, b"corrupt").unwrap();
    assert_eq!(m.result(&id).unwrap_err().code, "artifact_invalid");
    assert_eq!(m.view().unwrap()["install_state"], "approval_required");
    fs::create_dir(root.join("active")).unwrap();
    fs::write(root.join("active/00000001.json"), b"corrupt").unwrap();
    assert_eq!(m.view().unwrap()["install_state"], "blocked");
}
#[test]
fn actual_onnx_cpu_result_journal_and_original_png_psd_history_are_preserved() {
    let m = manager();
    let tmp = tempfile::tempdir().unwrap();
    let store = crate::storage::ProjectStore::new(tmp.path().join("projects")).unwrap();
    for (name, bytes) in [
        (
            "png",
            include_bytes!("../../tests/fixtures/source/groups-merged.png").as_slice(),
        ),
        (
            "psd",
            include_bytes!("../../tests/fixtures/source/groups.psd").as_slice(),
        ),
    ] {
        let input = tmp.path().join(format!("원본_日本語.{name}"));
        fs::write(&input, bytes).unwrap();
        let p = store
            .create("合成 시험".into(), "2d".into(), "vbridger-vts".into())
            .unwrap();
        let p = store
            .import_image(&p.id, p.revision, &input, &AtomicBool::new(false))
            .unwrap();
        let manifest = fs::read(
            tmp.path()
                .join("projects")
                .join(&p.id)
                .join("00000002.json"),
        )
        .unwrap();
        let jobs = JobManager::persistent(tmp.path().join(format!("jobs-{name}"))).unwrap();
        let runtime = m.clone();
        let source = p.sources[0].sha256.clone();
        let job = jobs
            .start_task(
                p.id.clone(),
                runtime::PROVIDER.into(),
                move |id, cancel, progress| {
                    let artifact = runtime.infer(id, Some(source), cancel, progress)?;
                    runtime.accept(id, &artifact)
                },
            )
            .unwrap();
        let done = wait(&jobs);
        assert_eq!(done.status, "completed", "{:?}", done.error);
        let output = m.result(&job.id).unwrap();
        assert_eq!(output["output"], json!([2.25, 5.5]));
        assert_eq!(output["source_hash"], p.sources[0].sha256);
        assert_eq!(store.open(&p.id).unwrap().revision, 2);
        assert_eq!(fs::read(&input).unwrap(), bytes);
        assert_eq!(
            manifest,
            fs::read(
                tmp.path()
                    .join("projects")
                    .join(&p.id)
                    .join("00000002.json")
            )
            .unwrap()
        );
        assert_eq!(
            store.list_v2().unwrap().healthy.len(),
            if name == "png" { 1 } else { 2 }
        );
        assert_eq!(
            crate::journal::initialize(&tmp.path().join(format!("jobs-{name}"))).unwrap()[0].status,
            "completed"
        );
    }
}
#[test]
fn actual_runtime_cancel_then_retry_without_completed_artifact() {
    let m = manager();
    let tmp = tempfile::tempdir().unwrap();
    let jobs = JobManager::persistent(tmp.path().join("jobs")).unwrap();
    let runtime = m.clone();
    let j = jobs
        .start_task(
            uuid::Uuid::nil().to_string(),
            runtime::PROVIDER.into(),
            move |id, cancel, progress| runtime.infer(id, None, cancel, progress).map(|_| ()),
        )
        .unwrap();
    jobs.cancel(&j.id).unwrap();
    assert_eq!(wait(&jobs).status, "cancelled");
    assert!(m.result(&j.id).is_err());
    let runtime = m.clone();
    jobs.start_task(
        uuid::Uuid::nil().to_string(),
        "runtime-probe-v1".into(),
        move |id, cancel, progress| runtime.probe(id, cancel, progress),
    )
    .unwrap();
    assert_eq!(wait(&jobs).status, "completed");
}
#[test]
fn journal_failure_prevents_real_runtime_and_restart_marks_interrupted() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path().join("jobs");
    let jobs = JobManager::persistent(root.clone()).unwrap();
    fs::write(root.join(".test-write-failure"), b"fault").unwrap();
    assert!(jobs
        .start_task(
            uuid::Uuid::nil().to_string(),
            runtime::PROVIDER.into(),
            |_, _, _| panic!("must not execute")
        )
        .is_err());
    fs::remove_file(root.join(".test-write-failure")).unwrap();
    let job = crate::worker::Job {
        id: uuid::Uuid::new_v4().to_string(),
        project_id: uuid::Uuid::nil().to_string(),
        status: "running".into(),
        progress: 25,
        provider: runtime::PROVIDER.into(),
        error: None,
    };
    crate::journal::append(&root, &job).unwrap();
    assert_eq!(
        crate::journal::initialize(&root).unwrap()[0].status,
        "interrupted"
    );
}

fn fixture(code: &str, timeout: Duration, cancel: &AtomicBool) -> crate::Result<Value> {
    let tmp = tempfile::tempdir().unwrap();
    let script = tmp.path().join("fixture.py");
    fs::write(&script, code).unwrap();
    runtime_process::execute(
        &python(),
        &script,
        tmp.path(),
        &json!({"protocol":2,"job_id":"fixture"}),
        cancel,
        timeout,
        &mut |_| {},
    )
}
fn stream(events: &str) -> String {
    format!("import json,sys\nr=json.loads(sys.stdin.readline())\ndef emit(kind,p,data=None,job='fixture',protocol=2):\n print(json.dumps(dict(protocol=protocol,job_id=job,provider='onnx-synthetic-v1',kind=kind,progress=p,data=data)),flush=True)\n{events}")
}
#[test]
fn v2_rejects_wrong_id_version_progress_duplicate_terminal_and_missing_result() {
    for events in [
        "emit('progress',0,job='wrong')",
        "emit('progress',0,protocol=99)",
        "emit('progress',50)\nemit('progress',10)",
        "emit('completed',100)",
        "emit('result',100,{})\nemit('completed',100)\nemit('completed',100)",
        "emit('result',100,{})\nemit('result',100,{})",
        "emit('progress',101)",
    ] {
        assert_eq!(
            fixture(
                &stream(events),
                Duration::from_secs(5),
                &AtomicBool::new(false)
            )
            .unwrap_err()
            .code,
            "worker_protocol",
            "{events}"
        );
    }
}
#[test]
fn v2_crash_timeout_large_output_cancel_and_unknown_errors_are_bounded() {
    for (code, expected, timeout) in [
        ("import sys;sys.exit(7)".into(), "worker_failed", 5),
        ("print('x'*65537)".into(), "worker_protocol", 5),
        ("import time;time.sleep(60)".into(), "worker_timeout", 1),
        (
            stream("emit('error',0,{'code':'C:/private/secret'})"),
            "runtime_failed",
            5,
        ),
    ] {
        assert_eq!(
            fixture(&code, Duration::from_secs(timeout), &AtomicBool::new(false))
                .unwrap_err()
                .code,
            expected
        );
    }
    assert_eq!(
        fixture(
            "import time;time.sleep(60)",
            Duration::from_secs(5),
            &AtomicBool::new(true)
        )
        .unwrap_err()
        .code,
        "cancelled"
    );
}
#[test]
fn v2_os_memory_limit_prevents_oversized_allocation() {
    let code=stream("try:\n x=bytearray(800*1024*1024)\n emit('result',100,{'unexpected':True})\n emit('completed',100)\nexcept MemoryError:\n emit('error',0,{'code':'ram_insufficient'})");
    assert_eq!(
        fixture(&code, Duration::from_secs(10), &AtomicBool::new(false))
            .unwrap_err()
            .code,
        "ram_insufficient"
    );
}
#[test]
fn v2_cancellation_during_process_is_observed() {
    let flag = std::sync::Arc::new(AtomicBool::new(false));
    let worker = flag.clone();
    let handle = thread::spawn(move || {
        fixture(
            "import time;time.sleep(60)",
            Duration::from_secs(5),
            &worker,
        )
    });
    thread::sleep(Duration::from_millis(200));
    flag.store(true, Ordering::SeqCst);
    assert_eq!(handle.join().unwrap().unwrap_err().code, "cancelled");
}

#[cfg(windows)]
#[test]
fn v2_timeout_reaps_a_real_descendant_in_the_job_object() {
    use windows_sys::Win32::{
        Foundation::{CloseHandle, WAIT_OBJECT_0},
        System::Threading::{OpenProcess, WaitForSingleObject, PROCESS_SYNCHRONIZE},
    };
    let tmp = tempfile::tempdir().unwrap();
    let marker = tmp.path().join("pid.json");
    let script = tmp.path().join("child.py");
    let marker_literal = serde_json::to_string(&marker).unwrap();
    fs::write(&script,format!("import sys,subprocess,time,json\nsys.stdin.readline()\np=subprocess.Popen([sys.executable,'-c','import time; time.sleep(60)'],stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL)\nopen({marker_literal},'w').write(str(p.pid))\ntime.sleep(60)\n")).unwrap();
    let root = tmp.path().to_owned();
    let handle = thread::spawn(move || {
        runtime_process::execute(
            &python(),
            &script,
            &root,
            &json!({"protocol":2,"job_id":"fixture"}),
            &AtomicBool::new(false),
            Duration::from_secs(3),
            &mut |_| {},
        )
    });
    let start = Instant::now();
    while !marker.exists() {
        assert!(start.elapsed() < Duration::from_secs(3));
        thread::sleep(Duration::from_millis(20));
    }
    let pid: u32 = fs::read_to_string(marker).unwrap().parse().unwrap();
    let child = unsafe { OpenProcess(PROCESS_SYNCHRONIZE, 0, pid) };
    assert!(!child.is_null());
    assert_eq!(handle.join().unwrap().unwrap_err().code, "worker_timeout");
    assert_eq!(unsafe { WaitForSingleObject(child, 5000) }, WAIT_OBJECT_0);
    unsafe {
        CloseHandle(child);
    }
}
