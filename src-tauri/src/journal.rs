use crate::{
    durable::{lock, plain, publish},
    worker::Job,
    AppError, Result,
};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::Read,
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Record {
    version: u32,
    sequence: u64,
    updated_ms: u64,
    job: Job,
}
fn sequences(dir: &Path) -> Result<Vec<u64>> {
    plain(dir)?;
    let mut seq = vec![];
    for entry in fs::read_dir(dir)? {
        let name = entry?.file_name().to_string_lossy().into_owned();
        if name.len() == 13
            && name.ends_with(".json")
            && name.as_bytes()[..8].iter().all(u8::is_ascii_digit)
        {
            seq.push(
                name[..8]
                    .parse()
                    .map_err(|_| AppError::new("journal_invalid"))?,
            );
        }
        if seq.len() > 10000 {
            return Err(AppError::new("inspection_limit"));
        }
    }
    seq.sort_unstable();
    Ok(seq)
}
pub fn append(root: &Path, job: &Job) -> Result<()> {
    #[cfg(test)]
    if root.join(".test-write-failure").exists() {
        return Err(std::io::Error::from(std::io::ErrorKind::StorageFull).into());
    }
    if uuid::Uuid::parse_str(&job.id)
        .map(|x| x.to_string())
        .ok()
        .as_deref()
        != Some(&job.id)
    {
        return Err(AppError::new("journal_invalid"));
    }
    plain(root)?;
    let dir = root.join(&job.id);
    if !dir.try_exists()? {
        fs::create_dir(&dir)?;
    }
    plain(&dir)?;
    let seq = sequences(&dir)?.last().copied().unwrap_or(0) + 1;
    if seq > 99999999 {
        return Err(AppError::new("inspection_limit"));
    }
    let record = Record {
        version: 1,
        sequence: seq,
        updated_ms: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64,
        job: job.clone(),
    };
    publish(
        &dir.join(format!("{seq:08}.json")),
        &serde_json::to_vec(&record)?,
    )
}
pub fn history(root: &Path) -> Result<Vec<Job>> {
    plain(root)?;
    let mut records = vec![];
    for entry in fs::read_dir(root)? {
        let entry = entry?;
        let id = entry.file_name().to_string_lossy().into_owned();
        if uuid::Uuid::parse_str(&id).is_err() {
            continue;
        }
        let result = (|| -> Result<Record> {
            let seq = sequences(&entry.path())?
                .last()
                .copied()
                .ok_or_else(|| AppError::new("journal_invalid"))?;
            let path = entry.path().join(format!("{seq:08}.json"));
            plain(&path)?;
            let mut data = vec![];
            fs::File::open(path)?.take(65537).read_to_end(&mut data)?;
            if data.len() > 65536 {
                return Err(AppError::new("journal_invalid"));
            }
            let r: Record = serde_json::from_slice(&data)?;
            if r.version != 1
                || r.sequence == 0
                || r.sequence != seq
                || r.job.id != id
                || r.job.progress > 100
                || ![
                    "dummy",
                    "onnx-synthetic-v1",
                    "runtime-install-v1",
                    "runtime-probe-v1",
                ]
                .contains(&r.job.provider.as_str())
                || uuid::Uuid::parse_str(&r.job.project_id).is_err()
                || ![
                    "queued",
                    "running",
                    "completed",
                    "cancelled",
                    "failed",
                    "interrupted",
                    "recovery_required",
                ]
                .contains(&r.job.status.as_str())
                || (r.job.status == "completed" && r.job.progress != 100)
            {
                return Err(AppError::new("journal_invalid"));
            }
            Ok(r)
        })();
        match result {
            Ok(r) => records.push((r.updated_ms, r.job)),
            Err(_) => records.push((
                0,
                Job {
                    id,
                    project_id: String::new(),
                    status: "recovery_required".into(),
                    progress: 0,
                    provider: "dummy".into(),
                    error: Some("journal_invalid".into()),
                },
            )),
        }
        if records.len() > 10000 {
            return Err(AppError::new("inspection_limit"));
        }
    }
    records.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.id.cmp(&b.1.id)));
    Ok(records.into_iter().map(|x| x.1).collect())
}
pub fn initialize(root: &Path) -> Result<Vec<Job>> {
    if root.try_exists()? {
        plain(root)?;
    }
    fs::create_dir_all(root)?;
    let guard = match lock(&root.join(".runtime.lock")) {
        Ok(g) => Some(g),
        Err(e) if e.code == "project_busy" => None,
        Err(e) => return Err(e),
    };
    let mut jobs = history(root)?;
    if guard.is_some() {
        for j in &mut jobs {
            if j.status == "running" || j.status == "queued" {
                j.status = "interrupted".into();
                j.error = Some("job_interrupted".into());
                if append(root, j).is_err() {
                    j.status = "recovery_required".into();
                    j.error = Some("journal_write_failed".into());
                }
            }
        }
    }
    Ok(jobs)
}
