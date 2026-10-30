//! Bounded v2 supervisor. Only the registered runtime script reaches this API from IPC.
use crate::{AppError, Result};
use serde::Deserialize;
use serde_json::Value;
use std::{
    io::{BufRead, BufReader, Read, Write},
    path::Path,
    process::{Command, Stdio},
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc,
    },
    thread,
    time::{Duration, Instant},
};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Event {
    protocol: u32,
    job_id: String,
    provider: String,
    kind: String,
    progress: u8,
    data: Option<Value>,
}

#[cfg(windows)]
struct ProcessGroup(windows_sys::Win32::Foundation::HANDLE);
#[cfg(windows)]
impl ProcessGroup {
    fn attach(child: &std::process::Child) -> Result<Self> {
        use std::{mem::size_of, os::windows::io::AsRawHandle};
        use windows_sys::Win32::System::JobObjects::*;
        unsafe {
            let handle = CreateJobObjectW(std::ptr::null(), std::ptr::null());
            if handle.is_null() {
                return Err(AppError::new("runtime_isolation"));
            }
            let group = Self(handle);
            let mut limits: JOBOBJECT_EXTENDED_LIMIT_INFORMATION = std::mem::zeroed();
            limits.BasicLimitInformation.LimitFlags = JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE
                | JOB_OBJECT_LIMIT_JOB_MEMORY
                | JOB_OBJECT_LIMIT_ACTIVE_PROCESS;
            // Windows venv redirectors remain alive alongside their real Python child.
            limits.BasicLimitInformation.ActiveProcessLimit = 8;
            limits.JobMemoryLimit = 512 * 1024 * 1024;
            if SetInformationJobObject(
                handle,
                JobObjectExtendedLimitInformation,
                &limits as *const _ as *const _,
                size_of::<JOBOBJECT_EXTENDED_LIMIT_INFORMATION>() as u32,
            ) == 0
                || AssignProcessToJobObject(handle, child.as_raw_handle() as _) == 0
            {
                return Err(AppError::new("runtime_isolation"));
            }
            Ok(group)
        }
    }
}
#[cfg(windows)]
impl Drop for ProcessGroup {
    fn drop(&mut self) {
        unsafe {
            windows_sys::Win32::Foundation::CloseHandle(self.0);
        }
    }
}

pub fn execute(
    python: &Path,
    script: &Path,
    root: &Path,
    request: &Value,
    cancel: &AtomicBool,
    timeout: Duration,
    progress: &mut dyn FnMut(u8),
) -> Result<Value> {
    let mut command = Command::new(python);
    command
        .args(["-I", "-B", "-u"])
        .arg(script)
        .current_dir(root)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null());
    // Do not forward API keys, user Python hooks or proxy credentials into a provider.
    command.env_clear();
    for key in [
        "SYSTEMROOT",
        "WINDIR",
        "PATH",
        "COMSPEC",
        "PATHEXT",
        "SYSTEMDRIVE",
    ] {
        if let Some(value) = std::env::var_os(key) {
            command.env(key, value);
        }
    }
    let temp = root.join("tmp");
    std::fs::create_dir_all(&temp)?;
    crate::durable::plain(&temp)?;
    command
        .env("TEMP", &temp)
        .env("TMP", &temp)
        .env("PYTHONDONTWRITEBYTECODE", "1")
        .env("PIP_DISABLE_PIP_VERSION_CHECK", "1")
        .env("PIP_CACHE_DIR", &temp)
        .env("OMP_NUM_THREADS", "1")
        .env("OPENBLAS_NUM_THREADS", "1");
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        command.creation_flags(0x08000000);
    }
    let mut child = command
        .spawn()
        .map_err(|_| AppError::new("python_unavailable"))?;
    #[cfg(windows)]
    let group = match ProcessGroup::attach(&child) {
        Ok(group) => group,
        Err(error) => {
            let _ = child.kill();
            let _ = child.wait();
            return Err(error);
        }
    };
    let result = (|| {
        let bytes = serde_json::to_vec(request)?;
        if bytes.len() > 65536 {
            return Err(AppError::new("worker_protocol"));
        }
        let mut stdin = child
            .stdin
            .take()
            .ok_or_else(|| AppError::new("worker_failed"))?;
        stdin.write_all(&bytes)?;
        stdin.write_all(b"\n")?;
        drop(stdin);
        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| AppError::new("worker_failed"))?;
        let (tx, rx) = mpsc::sync_channel(16);
        thread::spawn(move || {
            let mut reader = BufReader::new(stdout);
            loop {
                let mut line = String::new();
                match reader.by_ref().take(65537).read_line(&mut line) {
                    Ok(0) => break,
                    Ok(n) if n <= 65536 && line.ends_with('\n') => {
                        if tx.send(Ok(line)).is_err() {
                            break;
                        }
                    }
                    _ => {
                        let _ = tx.send(Err(()));
                        break;
                    }
                }
            }
        });
        let began = Instant::now();
        let mut previous = 0;
        let mut count = 0;
        let mut completed = false;
        let mut output = None;
        loop {
            if cancel.load(Ordering::SeqCst) {
                return Err(AppError::new("cancelled"));
            }
            if began.elapsed() > timeout {
                return Err(AppError::new("worker_timeout"));
            }
            match rx.recv_timeout(Duration::from_millis(25)) {
                Ok(Ok(line)) => {
                    let event: Event = serde_json::from_str(&line)
                        .map_err(|_| AppError::new("worker_protocol"))?;
                    count += 1;
                    if count > 1000
                        || completed
                        || event.protocol != 2
                        || event.job_id != request["job_id"]
                        || event.provider != "onnx-synthetic-v1"
                    {
                        return Err(AppError::new("worker_protocol"));
                    }
                    if event.kind == "error" {
                        let code = event
                            .data
                            .as_ref()
                            .and_then(|v| v["code"].as_str())
                            .unwrap_or("runtime_failed");
                        let allowed = [
                            "download_consent",
                            "download_origin",
                            "download_failed",
                            "model_size",
                            "model_hash",
                            "runtime_manifest",
                            "worker_timeout",
                            "disk_full",
                            "ram_insufficient",
                            "runtime_io",
                            "runtime_failed",
                            "runtime_platform",
                            "runtime_not_installed",
                            "runtime_integrity",
                            "runtime_setup_failed",
                            "inference_mismatch",
                            "resource_unknown",
                            "backend_unverified",
                            "network_denied",
                            "unsafe_path",
                        ];
                        return Err(AppError::new(if allowed.contains(&code) {
                            code
                        } else {
                            "runtime_failed"
                        }));
                    }
                    if event.progress < previous || event.progress > 100 {
                        return Err(AppError::new("worker_protocol"));
                    }
                    previous = event.progress;
                    match event.kind.as_str() {
                        "progress" if output.is_none() && event.data.is_none() => {
                            progress(previous)
                        }
                        "result" if output.is_none() && previous == 100 && event.data.is_some() => {
                            output = event.data
                        }
                        "completed"
                            if output.is_some() && previous == 100 && event.data.is_none() =>
                        {
                            completed = true
                        }
                        _ => return Err(AppError::new("worker_protocol")),
                    }
                }
                Ok(Err(())) => return Err(AppError::new("worker_protocol")),
                Err(mpsc::RecvTimeoutError::Timeout) => {}
                Err(mpsc::RecvTimeoutError::Disconnected) => {
                    if let Some(status) = child.try_wait()? {
                        return if completed && status.success() {
                            output.ok_or_else(|| AppError::new("worker_protocol"))
                        } else {
                            Err(AppError::new("worker_failed"))
                        };
                    }
                    thread::sleep(Duration::from_millis(10));
                }
            }
        }
    })();
    let _ = child.kill();
    #[cfg(windows)]
    drop(group); // kill remaining descendants before reporting terminal state
    let _ = child.wait();
    result
}
