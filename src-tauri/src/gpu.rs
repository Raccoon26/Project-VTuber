use serde::Serialize;
use std::{
    process::{Command, Stdio},
    thread,
    time::{Duration, Instant},
};
#[derive(Debug, Serialize)]
pub struct GpuReport {
    pub detection: String,
    pub devices: Vec<Device>,
    pub inference: String,
    pub dummy_cpu_available: bool,
}
#[derive(Debug, Serialize)]
pub struct Device {
    pub name: String,
    pub vram_mib: u64,
    pub driver: String,
}
pub fn detect() -> GpuReport {
    let mut report = GpuReport {
        detection: "unknown".into(),
        devices: vec![],
        inference: "unverified".into(),
        dummy_cpu_available: true,
    };
    let mut cmd = Command::new("nvidia-smi");
    cmd.args([
        "--query-gpu=name,memory.total,driver_version",
        "--format=csv,noheader,nounits",
    ])
    .stdout(Stdio::piped())
    .stderr(Stdio::null());
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        cmd.creation_flags(0x08000000);
    }
    if let Ok(mut child) = cmd.spawn() {
        let start = Instant::now();
        loop {
            match child.try_wait() {
                Ok(Some(status)) => {
                    if status.success() {
                        if let Ok(out) = child.wait_with_output() {
                            if out.stdout.len() < 65536 {
                                for line in String::from_utf8_lossy(&out.stdout).lines() {
                                    let fields: Vec<_> = line.split(',').map(str::trim).collect();
                                    if fields.len() == 3 {
                                        if let Ok(vram) = fields[1].parse() {
                                            report.devices.push(Device {
                                                name: fields[0].into(),
                                                vram_mib: vram,
                                                driver: fields[2].into(),
                                            });
                                        }
                                    }
                                }
                                if !report.devices.is_empty() {
                                    report.detection = "detected".into();
                                }
                            }
                        }
                    }
                    break;
                }
                Ok(None) if start.elapsed() < Duration::from_secs(3) => {
                    thread::sleep(Duration::from_millis(20))
                }
                _ => {
                    let _ = child.kill();
                    let _ = child.wait();
                    break;
                }
            }
        }
    }
    report
}
