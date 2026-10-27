use crate::{
    durable::{lock, plain},
    journal,
    model::CharacterProject,
    storage::{hash, ProjectStore},
    worker::{Job, JobManager},
};
use std::{
    fs,
    path::{Path, PathBuf},
    process::{Child, Command, Stdio},
    thread,
    time::{Duration, Instant},
};

fn setup() -> (tempfile::TempDir, ProjectStore, CharacterProject) {
    let tmp = tempfile::tempdir().unwrap();
    let store = ProjectStore::new(tmp.path().join("日本語 한글 projects")).unwrap();
    let p = store
        .create("日本語 한글".into(), "2d".into(), "vbridger-vts".into())
        .unwrap();
    (tmp, store, p)
}
fn source(s: &ProjectStore, p: &CharacterProject) -> CharacterProject {
    let path = s.root.join("fixture.png");
    fs::write(&path, include_bytes!("../../samples/diagnostic.png")).unwrap();
    s.import_png(&p.id, p.revision, &path).unwrap()
}
fn bytes(s: &ProjectStore, p: &CharacterProject, rev: u64) -> Vec<u8> {
    fs::read(s.dir(&p.id).unwrap().join(format!("{rev:08}.json"))).unwrap()
}
fn child(
    s: &ProjectStore,
    p: &CharacterProject,
    mode: &str,
    marker: &Path,
    fault: Option<&str>,
) -> Child {
    let mut c = Command::new(std::env::current_exe().unwrap());
    c.args([
        "--ignored",
        "--exact",
        "resilience_tests::process_helper",
        "--nocapture",
    ])
    .env("VTUBER_TEST_ROOT", &s.root)
    .env("VTUBER_TEST_ID", &p.id)
    .env("VTUBER_TEST_MODE", mode)
    .env("VTUBER_TEST_MARKER", marker)
    .stdout(Stdio::null())
    .stderr(Stdio::null());
    if let Some(f) = fault {
        c.env("VTUBER_TEST_FAULT", f);
    }
    c.spawn().unwrap()
}
fn ready(marker: &Path, c: &mut Child) {
    let start = Instant::now();
    while !marker.exists() {
        assert!(
            c.try_wait().unwrap().is_none(),
            "child exited before checkpoint"
        );
        assert!(start.elapsed() < Duration::from_secs(15));
        thread::sleep(Duration::from_millis(20));
    }
}
#[test]
#[ignore = "subprocess fixture; invoked explicitly by resilience tests"]
fn process_helper() {
    let Some(root) = std::env::var_os("VTUBER_TEST_ROOT") else {
        return;
    };
    let s = ProjectStore::new(root.into()).unwrap();
    let id = std::env::var("VTUBER_TEST_ID").unwrap();
    let mode = std::env::var("VTUBER_TEST_MODE").unwrap();
    let marker = PathBuf::from(std::env::var_os("VTUBER_TEST_MARKER").unwrap());
    if mode == "lock" {
        let _guard = lock(&s.dir(&id).unwrap().join(".write.lock")).unwrap();
        fs::write(marker, b"locked").unwrap();
        loop {
            thread::sleep(Duration::from_millis(50));
        }
    }
    if mode == "job" {
        let root = s.root.join("jobs");
        let _m = JobManager::persistent(root.clone()).unwrap();
        let _owner = lock(&root.join(".runtime.lock")).unwrap();
        journal::append(&root, &sample_job(&id, "running")).unwrap();
        fs::write(marker, b"running").unwrap();
        loop {
            thread::sleep(Duration::from_millis(50));
        }
    }
    if mode == "race" {
        fs::write(&marker, b"ready").unwrap();
        while !s.root.join("go").exists() {
            thread::sleep(Duration::from_millis(10));
        }
    }
    if mode == "disk_full" {
        unsafe {
            std::env::remove_var("VTUBER_TEST_MARKER");
        }
    }
    let result = if mode == "recover" {
        let preview = s.preview_recovery(&id, 1).unwrap();
        s.recover(&id, 1, &preview.token)
    } else {
        s.rename(&id, 1, "changed".into())
    };
    fs::write(
        marker,
        match result {
            Ok(_) => "ok".into(),
            Err(e) => e.code,
        },
    )
    .unwrap();
}

#[test]
fn isolate_recover_validate_preserve_and_stale_preview() {
    let (_tmp, s, p) = setup();
    let p = source(&s, &p);
    let normal = s
        .create("normal".into(), "3d".into(), "vbridger-vmc".into())
        .unwrap();
    let original = bytes(&s, &p, 2);
    let corrupt = s.dir(&p.id).unwrap().join("00000003.json");
    fs::write(&corrupt, b"{partial").unwrap();
    let list = s.list_v2().unwrap();
    assert_eq!(list.healthy.len(), 1);
    assert_eq!(list.healthy[0].id, normal.id);
    assert_eq!(list.issues[0].status, "recoverable");
    let preview = s.preview_recovery(&p.id, 2).unwrap();
    assert!(preview.latest.is_none());
    let restored = s.recover(&p.id, 2, &preview.token).unwrap();
    assert_eq!(restored.revision, 4);
    s.verify(&restored).unwrap();
    assert_eq!(original, bytes(&s, &p, 2));
    assert_eq!(fs::read(&corrupt).unwrap(), b"{partial");
    assert_eq!(s.open(&p.id).unwrap().revision, 4);
    assert_eq!(
        s.recover(&p.id, 2, &preview.token).unwrap_err().code,
        "revision_conflict"
    );
    assert_eq!(s.inspect(&p.id).unwrap().len(), 4);
    assert!(restored
        .gates
        .iter()
        .all(|g| g.status == crate::model::GateStatus::Unverified));
}
#[test]
fn damaged_blob_is_never_a_valid_recovery_candidate() {
    let (_tmp, s, p) = setup();
    let p = source(&s, &p);
    s.rename(&p.id, 2, "later".into()).unwrap();
    let blob = s
        .dir(&p.id)
        .unwrap()
        .join(format!("sources/{}.png", p.sources[0].sha256));
    fs::write(blob, b"damage").unwrap();
    let history = s.inspect(&p.id).unwrap();
    assert!(history[1].project.is_none());
    assert!(history[2].project.is_none());
    assert!(s.preview_recovery(&p.id, 2).is_err());
    assert_eq!(s.list_v2().unwrap().issues[0].recoverable, vec![1]);
}
#[test]
fn old_damage_future_schema_unknown_fields_and_identity() {
    let (_tmp, s, p) = setup();
    s.rename(&p.id, 1, "second".into()).unwrap();
    fs::write(s.dir(&p.id).unwrap().join("00000001.json"), b"{}").unwrap();
    assert_eq!(s.list_v2().unwrap().healthy.len(), 1);
    assert!(s.inspect(&p.id).unwrap()[0].project.is_none());
    let path = s.dir(&p.id).unwrap().join("00000003.json");
    let mut value = serde_json::to_value(&p).unwrap();
    value["schema_version"] = 99.into();
    fs::write(&path, serde_json::to_vec(&value).unwrap()).unwrap();
    assert_eq!(s.list_v2().unwrap().issues[0].status, "schema_unsupported");
    assert_eq!(
        s.preview_recovery(&p.id, 2).unwrap_err().code,
        "schema_unsupported"
    );
    value["schema_version"] = 1.into();
    value["revision"] = 3.into();
    value["unknown"] = true.into();
    fs::write(&path, serde_json::to_vec(&value).unwrap()).unwrap();
    assert!(s.open(&p.id).is_err());
    value.as_object_mut().unwrap().remove("unknown");
    value["id"] = uuid::Uuid::new_v4().to_string().into();
    fs::write(&path, serde_json::to_vec(&value).unwrap()).unwrap();
    assert!(s.open(&p.id).is_err());
}
#[test]
fn sqlite_missing_corrupt_old_stale_manual_added_removed_rebuilt() {
    let (_tmp, s, p) = setup();
    let original = bytes(&s, &p, 1);
    let db = s.root.join("index.sqlite");
    for state in ["missing", "corrupt", "old", "stale"] {
        if db.exists() {
            fs::remove_file(&db).unwrap();
        }
        if state == "corrupt" {
            fs::write(&db, b"not sqlite").unwrap();
        }
        if state == "old" || state == "stale" {
            let c = rusqlite::Connection::open(&db).unwrap();
            c.execute_batch("PRAGMA user_version=999; CREATE TABLE wrong(x);")
                .unwrap();
        }
        let list = s.list_v2().unwrap();
        assert!(
            list.index_warning.is_none(),
            "index error: {:?}",
            list.index_warning
        );
        let c = rusqlite::Connection::open(&db).unwrap();
        assert_eq!(
            c.query_row("SELECT name FROM projects WHERE id=?1", [&p.id], |r| r
                .get::<_, String>(
                0
            ))
            .unwrap(),
            p.name
        );
        assert_eq!(
            c.query_row("PRAGMA integrity_check", [], |r| r.get::<_, String>(0))
                .unwrap(),
            "ok"
        );
    }
    let added = s
        .create("manual".into(), "2d".into(), "vtube-studio".into())
        .unwrap();
    assert_eq!(s.list_v2().unwrap().healthy.len(), 2);
    fs::rename(s.dir(&added.id).unwrap(), s.root.join("moved-fixture")).unwrap();
    assert_eq!(s.list_v2().unwrap().healthy.len(), 1);
    let c = rusqlite::Connection::open(&db).unwrap();
    assert_eq!(
        c.query_row("SELECT count(*) FROM projects", [], |r| r.get::<_, u32>(0))
            .unwrap(),
        1
    );
    assert_eq!(bytes(&s, &p, 1), original);
}
#[test]
fn index_failure_does_not_undo_committed_files() {
    let (_tmp, s, p) = setup();
    let guard = lock(&s.root.join(".index.lock")).unwrap();
    let p = s.rename(&p.id, 1, "saved".into()).unwrap();
    let list = s.list_v2().unwrap();
    assert!(list.index_warning.is_some());
    assert_eq!(list.healthy[0].revision, 2);
    drop(guard);
    assert!(s.list_v2().unwrap().index_warning.is_none());
    assert_eq!(s.open(&p.id).unwrap().name, "saved");
}
#[test]
fn actual_process_kill_at_each_publication_boundary() {
    for stage in ["empty", "partial", "before_publish", "after_publish"] {
        let (tmp, s, p) = setup();
        let before = bytes(&s, &p, 1);
        let marker = tmp.path().join("checkpoint");
        let mut c = child(&s, &p, "rename", &marker, Some(stage));
        ready(&marker, &mut c);
        c.kill().unwrap();
        c.wait().unwrap();
        let reopened = ProjectStore::new(s.root.clone()).unwrap();
        let expected = if stage == "after_publish" { 2 } else { 1 };
        assert_eq!(reopened.open(&p.id).unwrap().revision, expected);
        assert_eq!(bytes(&s, &p, 1), before);
        assert!(fs::read_dir(s.dir(&p.id).unwrap()).unwrap().any(|x| x
            .unwrap()
            .file_name()
            .to_string_lossy()
            .ends_with(".pending")));
        reopened
            .rename(&p.id, expected, "after restart".into())
            .unwrap();
    }
}
#[test]
fn disk_full_fault_never_replaces_previous_revision() {
    let (tmp, s, p) = setup();
    let before = bytes(&s, &p, 1);
    let marker = tmp.path().join("result");
    let mut c = child(&s, &p, "disk_full", &marker, Some("before_publish"));
    assert!(c.wait().unwrap().success());
    assert_eq!(fs::read_to_string(marker).unwrap(), "disk_full");
    assert_eq!(s.open(&p.id).unwrap().revision, 1);
    assert_eq!(bytes(&s, &p, 1), before);
}
#[test]
fn process_lock_release_after_termination_and_contention() {
    let (tmp, s, p) = setup();
    let marker = tmp.path().join("lock");
    let mut c = child(&s, &p, "lock", &marker, None);
    ready(&marker, &mut c);
    assert_eq!(
        s.rename(&p.id, 1, "blocked".into()).unwrap_err().code,
        "project_busy"
    );
    assert_eq!(s.list_v2().unwrap().issues[0].status, "project_busy");
    c.kill().unwrap();
    c.wait().unwrap();
    assert!(s.dir(&p.id).unwrap().join(".write.lock").exists());
    assert!(s.rename(&p.id, 1, "unlocked".into()).is_ok());
}
#[test]
fn two_processes_cannot_publish_same_revision() {
    let (tmp, s, p) = setup();
    let a = tmp.path().join("a");
    let b = tmp.path().join("b");
    let mut ca = child(&s, &p, "race", &a, None);
    let mut cb = child(&s, &p, "race", &b, None);
    ready(&a, &mut ca);
    ready(&b, &mut cb);
    fs::write(s.root.join("go"), b"go").unwrap();
    assert!(ca.wait().unwrap().success());
    assert!(cb.wait().unwrap().success());
    let results = [
        fs::read_to_string(a).unwrap(),
        fs::read_to_string(b).unwrap(),
    ];
    assert_eq!(results.iter().filter(|r| r.as_str() == "ok").count(), 1);
    assert!(results
        .iter()
        .any(|r| r == "project_busy" || r == "revision_conflict"));
    assert_eq!(s.open(&p.id).unwrap().revision, 2);
}
fn sample_job(project_id: &str, status: &str) -> Job {
    Job {
        id: uuid::Uuid::new_v4().to_string(),
        project_id: project_id.into(),
        status: status.into(),
        progress: 23,
        provider: "dummy".into(),
        error: None,
    }
}
#[test]
fn job_restarts_interrupted_not_completed_and_preserves_project() {
    let (tmp, s, p) = setup();
    let before = bytes(&s, &p, 1);
    let marker = tmp.path().join("job");
    let mut c = child(&s, &p, "job", &marker, None);
    ready(&marker, &mut c);
    let root = s.root.join("jobs");
    assert_eq!(journal::initialize(&root).unwrap()[0].status, "running");
    c.kill().unwrap();
    c.wait().unwrap();
    let manager = JobManager::persistent(root.clone()).unwrap();
    assert_eq!(manager.snapshot().unwrap().status, "interrupted");
    assert_eq!(manager.history().unwrap()[0].progress, 23);
    assert_eq!(bytes(&s, &p, 1), before);
    let j = manager.snapshot().unwrap();
    fs::write(root.join(j.id).join("00000003.json"), b"broken").unwrap();
    assert_eq!(
        journal::initialize(&root).unwrap()[0].status,
        "recovery_required"
    );
}
#[test]
fn persisted_real_worker_complete_cancel_and_reload() {
    let (_tmp, s, p) = setup();
    let p = source(&s, &p);
    let root = s.root.join("jobs");
    let m = JobManager::persistent(root.clone()).unwrap();
    let py = std::env::var_os("VTUBER_PYTHON").unwrap().into();
    let script = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../workers/dummy.py");
    m.start(p.id.clone(), py, script.clone()).unwrap();
    wait_job(&m);
    assert_eq!(m.snapshot().unwrap().status, "completed");
    let py = std::env::var_os("VTUBER_PYTHON").unwrap().into();
    let j = m.start(p.id.clone(), py, script).unwrap();
    m.cancel(&j.id).unwrap();
    wait_job(&m);
    assert_eq!(m.snapshot().unwrap().status, "cancelled");
    let history = JobManager::persistent(root).unwrap().history().unwrap();
    assert_eq!(history.len(), 2);
    assert!(history.iter().any(|j| j.status == "completed"));
    assert_eq!(s.open(&p.id).unwrap().revision, 2);
}
fn wait_job(m: &JobManager) {
    let start = Instant::now();
    while m
        .snapshot()
        .is_some_and(|j| ["running", "queued"].contains(&j.status.as_str()))
    {
        assert!(start.elapsed() < Duration::from_secs(25));
        thread::sleep(Duration::from_millis(20));
    }
}
#[test]
fn recovery_conflict_when_same_revision_bytes_change() {
    let (_tmp, s, p) = setup();
    s.rename(&p.id, 1, "second".into()).unwrap();
    let preview = s.preview_recovery(&p.id, 1).unwrap();
    let path = s.dir(&p.id).unwrap().join("00000002.json");
    fs::write(path, b"damaged after preview").unwrap();
    assert_eq!(
        s.recover(&p.id, 1, &preview.token).unwrap_err().code,
        "revision_conflict"
    );
    assert_eq!(s.inspect(&p.id).unwrap().len(), 2);
}

#[test]
fn recovery_process_failure_preserves_corrupt_and_valid_history() {
    for stage in ["before_publish", "after_publish"] {
        let (tmp, s, p) = setup();
        let before = bytes(&s, &p, 1);
        let broken = s.dir(&p.id).unwrap().join("00000002.json");
        fs::write(&broken, b"partial").unwrap();
        let marker = tmp.path().join("recovery");
        let mut c = child(&s, &p, "recover", &marker, Some(stage));
        ready(&marker, &mut c);
        c.kill().unwrap();
        c.wait().unwrap();
        assert_eq!(bytes(&s, &p, 1), before);
        assert_eq!(fs::read(broken).unwrap(), b"partial");
        if stage == "after_publish" {
            assert_eq!(s.open(&p.id).unwrap().revision, 3);
        } else {
            assert!(s.open(&p.id).is_err());
            let preview = s.preview_recovery(&p.id, 1).unwrap();
            s.recover(&p.id, 1, &preview.token).unwrap();
        }
    }
}
#[test]
fn journal_write_failure_stops_worker_without_claiming_completion() {
    let (_tmp, s, p) = setup();
    let root = s.root.join("jobs");
    let m = JobManager::persistent(root.clone()).unwrap();
    let py = std::env::var_os("VTUBER_PYTHON").unwrap().into();
    let script = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../workers/dummy.py");
    m.start(p.id.clone(), py, script).unwrap();
    let start = Instant::now();
    while m.snapshot().unwrap().progress == 0 {
        assert!(start.elapsed() < Duration::from_secs(5));
        thread::sleep(Duration::from_millis(10));
    }
    fs::write(root.join(".test-write-failure"), b"inject").unwrap();
    wait_job(&m);
    assert_eq!(m.snapshot().unwrap().status, "recovery_required");
    fs::remove_file(root.join(".test-write-failure")).unwrap();
    assert_eq!(
        JobManager::persistent(root)
            .unwrap()
            .snapshot()
            .unwrap()
            .status,
        "interrupted"
    );
    assert_eq!(s.open(&p.id).unwrap().revision, 1);
}
#[test]
fn search_revalidates_files_and_global_failure_is_not_an_empty_list() {
    let (tmp, s, p) = setup();
    assert_eq!(s.search_projects("한글").unwrap().healthy[0].id, p.id);
    assert!(s.search_projects("absent").unwrap().healthy.is_empty());
    let moved = tmp.path().join("offline");
    fs::rename(&s.root, moved).unwrap();
    assert!(s.list_v2().is_err());
}
#[cfg(windows)]
#[test]
fn windows_denied_handle_and_junction_are_isolated() {
    use std::os::windows::fs::OpenOptionsExt;
    let (tmp, s, p) = setup();
    let path = s.dir(&p.id).unwrap().join("00000001.json");
    let before = hash(&fs::read(&path).unwrap());
    let denied = fs::OpenOptions::new()
        .read(true)
        .share_mode(0)
        .open(&path)
        .unwrap();
    assert!(s.open(&p.id).is_err());
    assert_eq!(s.list_v2().unwrap().issues.len(), 1);
    drop(denied);
    assert_eq!(hash(&fs::read(&path).unwrap()), before);
    let link = s.root.join(uuid::Uuid::new_v4().to_string());
    let target = tmp.path().join("outside");
    fs::create_dir(&target).unwrap();
    let status = Command::new("cmd")
        .args(["/c", "mklink", "/J"])
        .arg(&link)
        .arg(&target)
        .stdout(Stdio::null())
        .status()
        .unwrap();
    assert!(status.success());
    assert_eq!(plain(&link).unwrap_err().code, "unsafe_path");
    assert!(s
        .list_v2()
        .unwrap()
        .issues
        .iter()
        .any(|i| i.error == "unsafe_path"));
    assert_eq!(fs::read_dir(target).unwrap().count(), 0);
}
