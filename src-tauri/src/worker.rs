use crate::{AppError, Result};
use serde::{Deserialize, Serialize};
use std::{
    io::{BufRead, BufReader, Read, Write},
    path::PathBuf,
    process::{Command, Stdio},
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc, Arc, Mutex,
    },
    thread,
    time::{Duration, Instant},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Job {
    pub id: String,
    pub project_id: String,
    pub status: String,
    pub progress: u8,
    pub provider: String,
    pub error: Option<String>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Event {
    protocol: u32,
    job_id: String,
    kind: String,
    progress: u8,
}
pub struct JobManager {
    journal: Option<PathBuf>,
    job: Arc<Mutex<Option<Job>>>,
    cancel: Arc<AtomicBool>,
}
impl Default for JobManager {
    fn default() -> Self {
        Self {
            journal: None,
            job: Arc::new(Mutex::new(None)),
            cancel: Arc::new(AtomicBool::new(false)),
        }
    }
}
fn hidden(command: &mut Command) {
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        command.creation_flags(0x08000000);
    }
    #[cfg(not(windows))]
    {
        let _ = command;
    }
}
impl JobManager {
    pub fn persistent(root: PathBuf) -> Result<Self> {
        let jobs = crate::journal::initialize(&root)?;
        Ok(Self {
            journal: Some(root),
            job: Arc::new(Mutex::new(
                jobs.last()
                    .filter(|j| j.status != "running" && j.status != "queued")
                    .cloned(),
            )),
            cancel: Arc::new(AtomicBool::new(false)),
        })
    }
    pub fn history(&self) -> Result<Vec<Job>> {
        match &self.journal {
            Some(root) => crate::journal::history(root),
            None => Ok(self.snapshot().into_iter().collect()),
        }
    }
    pub fn snapshot(&self) -> Option<Job> {
        self.job.lock().unwrap().clone()
    }
    pub fn cancel(&self, id: &str) -> Result<()> {
        let guard = self.job.lock().unwrap();
        if guard.as_ref().map(|j| j.id.as_str()) != Some(id) {
            return Err(AppError::new("job_missing"));
        }
        self.cancel.store(true, Ordering::SeqCst);
        Ok(())
    }
    pub fn shutdown(&self) {
        self.cancel.store(true, Ordering::SeqCst);
    }
    pub fn start(&self, project_id: String, python: PathBuf, script: PathBuf) -> Result<Job> {
        let mut guard = self.job.lock().unwrap();
        if guard
            .as_ref()
            .is_some_and(|j| j.status == "running" || j.status == "queued")
        {
            return Err(AppError::new("job_busy"));
        }
        let journal_guard = match &self.journal {
            Some(root) => Some(crate::durable::lock(&root.join(".runtime.lock"))?),
            None => None,
        };
        let mut job = Job {
            id: uuid::Uuid::new_v4().to_string(),
            project_id,
            status: "running".into(),
            progress: 0,
            provider: "dummy".into(),
            error: None,
        };
        if let Some(root) = &self.journal {
            job.status = "queued".into();
            crate::journal::append(root, &job)?;
        }
        *guard = Some(job.clone());
        self.cancel.store(false, Ordering::SeqCst);
        let state = self.job.clone();
        let cancel = self.cancel.clone();
        let id = job.id.clone();
        let journal = self.journal.clone();
        thread::spawn(move || {
            let _owner = journal_guard;
            let mut journal_failed = false;
            {
                let mut guard = state.lock().unwrap();
                let j = guard.as_mut().unwrap();
                j.status = "running".into();
                if let Some(root) = &journal {
                    if crate::journal::append(root, j).is_err() {
                        j.status = "recovery_required".into();
                        j.error = Some("journal_write_failed".into());
                        return;
                    }
                }
            }
            let result = execute(
                python,
                script,
                &id,
                &cancel,
                Duration::from_secs(20),
                |progress| {
                    if let Some(j) = state.lock().unwrap().as_mut() {
                        j.progress = progress;
                        if let Some(root) = &journal {
                            if crate::journal::append(root, j).is_err() {
                                journal_failed = true;
                                cancel.store(true, Ordering::SeqCst);
                            }
                        }
                    }
                },
            );
            let mut guard = state.lock().unwrap();
            let j = guard.as_mut().unwrap();
            match result {
                Ok(()) => {
                    j.status = "completed".into();
                    j.progress = 100;
                }
                Err(e) if e.code == "cancelled" => j.status = "cancelled".into(),
                Err(e) => {
                    j.status = "failed".into();
                    j.error = Some(e.code);
                }
            }
            if journal_failed {
                j.status = "recovery_required".into();
                j.error = Some("journal_write_failed".into());
            }
            if let Some(root) = &journal {
                if crate::journal::append(root, j).is_err() {
                    j.status = "recovery_required".into();
                    j.error = Some("journal_write_failed".into());
                }
            }
            drop(_owner); // release OS ownership before exposing terminal state
        });
        Ok(job)
    }
}
fn execute(
    python: PathBuf,
    script: PathBuf,
    id: &str,
    cancel: &AtomicBool,
    timeout: Duration,
    mut progress: impl FnMut(u8),
) -> Result<()> {
    let mut command = Command::new(python);
    command
        .arg("-I")
        .arg("-B")
        .arg("-u")
        .arg(script)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null());
    hidden(&mut command);
    let mut child = command
        .spawn()
        .map_err(|_| AppError::new("python_unavailable"))?;
    let request = serde_json::json!({"protocol":1,"job_id":id,"provider":"dummy"});
    let result = (|| {
        let mut input = child
            .stdin
            .take()
            .ok_or_else(|| AppError::new("worker_failed"))?;
        writeln!(input, "{request}").map_err(|_| AppError::new("worker_failed"))?;
        drop(input);
        let output = child
            .stdout
            .take()
            .ok_or_else(|| AppError::new("worker_failed"))?;
        let (sender, receiver) = mpsc::sync_channel(16);
        thread::spawn(move || {
            let mut reader = BufReader::new(output);
            loop {
                let mut line = String::new();
                let read = reader.by_ref().take(65537).read_line(&mut line);
                match read {
                    Ok(0) => break,
                    Ok(n) if n <= 65536 && line.ends_with('\n') => {
                        if sender.send(Ok(line)).is_err() {
                            break;
                        }
                    }
                    _ => {
                        let _ = sender.send(Err(()));
                        break;
                    }
                }
            }
        });
        let start = Instant::now();
        let mut previous = 0;
        let mut completed = false;
        let mut count = 0;
        loop {
            if cancel.load(Ordering::SeqCst) {
                return Err(AppError::new("cancelled"));
            }
            if start.elapsed() > timeout {
                return Err(AppError::new("worker_timeout"));
            }
            match receiver.recv_timeout(Duration::from_millis(40)) {
                Ok(Ok(line)) => {
                    count += 1;
                    let event: Event = serde_json::from_str(&line)
                        .map_err(|_| AppError::new("worker_protocol"))?;
                    if completed
                        || count > 1000
                        || event.protocol != 1
                        || event.job_id != id
                        || event.progress > 100
                        || event.progress < previous
                    {
                        return Err(AppError::new("worker_protocol"));
                    }
                    previous = event.progress;
                    match event.kind.as_str() {
                        "progress" => progress(event.progress),
                        "completed" if event.progress == 100 => completed = true,
                        _ => return Err(AppError::new("worker_protocol")),
                    }
                }
                Ok(Err(())) => return Err(AppError::new("worker_protocol")),
                Err(mpsc::RecvTimeoutError::Timeout) => {}
                Err(mpsc::RecvTimeoutError::Disconnected) => {
                    if let Some(status) = child
                        .try_wait()
                        .map_err(|_| AppError::new("worker_failed"))?
                    {
                        return if completed && status.success() {
                            Ok(())
                        } else {
                            Err(AppError::new("worker_failed"))
                        };
                    }
                    thread::sleep(Duration::from_millis(10));
                }
            }
        }
    })();
    // Reap on success, protocol error, crash, timeout and cancellation.
    let _ = child.kill();
    let _ = child.wait();
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn crashed_oversized_and_hung_workers_are_reaped() {
        let tmp = tempfile::tempdir().unwrap();
        for (index, code, expected, timeout) in [
            (
                0,
                "import sys; sys.exit(7)",
                "worker_failed",
                Duration::from_secs(5),
            ),
            (
                1,
                "print('x'*65537)",
                "worker_protocol",
                Duration::from_secs(5),
            ),
            (
                2,
                "import time; time.sleep(60)",
                "worker_timeout",
                Duration::from_millis(500),
            ),
        ] {
            let path = tmp.path().join(format!("fixture{index}.py"));
            std::fs::write(&path, code).unwrap();
            let result = execute(
                python(),
                path,
                "fixture",
                &AtomicBool::new(false),
                timeout,
                |_| {},
            );
            assert_eq!(result.unwrap_err().code, expected);
        }
    }
    fn python() -> PathBuf {
        std::env::var_os("VTUBER_PYTHON")
            .unwrap_or_else(|| "python".into())
            .into()
    }
    fn script() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../workers/dummy.py")
    }
    fn wait(manager: &JobManager) -> Job {
        let start = Instant::now();
        loop {
            let j = manager.snapshot().unwrap();
            if j.status != "running" {
                return j;
            }
            assert!(start.elapsed() < Duration::from_secs(25));
            thread::sleep(Duration::from_millis(20));
        }
    }
    #[test]
    fn dummy_completes_in_real_python() {
        let m = JobManager::default();
        m.start("fixture".into(), python(), script()).unwrap();
        let j = wait(&m);
        assert_eq!(j.status, "completed", "{:?}", j.error);
        assert_eq!(j.progress, 100);
    }
    #[test]
    fn cancellation_and_single_job() {
        let m = JobManager::default();
        let j = m.start("fixture".into(), python(), script()).unwrap();
        assert!(m.start("fixture".into(), python(), script()).is_err());
        m.cancel(&j.id).unwrap();
        assert_eq!(wait(&m).status, "cancelled");
    }
    #[test]
    fn missing_worker_fails_without_app_crash() {
        let m = JobManager::default();
        m.start(
            "fixture".into(),
            PathBuf::from("does-not-exist-python"),
            script(),
        )
        .unwrap();
        assert_eq!(wait(&m).error.as_deref(), Some("python_unavailable"));
    }
    #[test]
    fn malformed_worker_output_rejected() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("bad.py");
        std::fs::write(&path, "print('{}')").unwrap();
        let m = JobManager::default();
        m.start("fixture".into(), python(), path).unwrap();
        assert_eq!(wait(&m).error.as_deref(), Some("worker_protocol"));
    }
}
