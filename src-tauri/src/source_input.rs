use crate::{
    durable::{lock, plain, publish},
    image_input::{self, Inspection, MAX_FILE},
    model::{CharacterProject, SourceAsset},
    storage::{hash, ProjectStore},
    AppError, Result,
};
use std::{
    fs::{self, OpenOptions},
    io::Read,
    path::Path,
    sync::atomic::AtomicBool,
};

fn source_bytes(path: &Path, cancel: &AtomicBool) -> Result<Vec<u8>> {
    // Inspect all ancestors as well as the leaf. Cooperative local-filesystem scope.
    for ancestor in path.ancestors() {
        plain(ancestor)?;
    }
    let mut options = OpenOptions::new();
    options.read(true);
    #[cfg(windows)]
    {
        use std::os::windows::fs::OpenOptionsExt;
        options.share_mode(1);
    }
    let mut file = options.open(path)?;
    let before = file.metadata()?;
    if !before.is_file() || before.len() > MAX_FILE as u64 {
        return Err(AppError::new("image_too_large"));
    }
    let mut bytes = Vec::with_capacity(before.len() as usize);
    let mut block = [0; 65536];
    loop {
        image_input::check_cancel(cancel)?;
        let n = file.read(&mut block)?;
        if n == 0 {
            break;
        }
        if bytes.len() + n > MAX_FILE {
            return Err(AppError::new("image_too_large"));
        }
        bytes.extend_from_slice(&block[..n]);
    }
    let after = file.metadata()?;
    if before.len() != after.len()
        || bytes.len() as u64 != before.len()
        || before.modified()? != after.modified()?
    {
        return Err(AppError::new("source_changed"));
    }
    Ok(bytes)
}
impl ProjectStore {
    pub fn import_image(
        &self,
        id: &str,
        revision: u64,
        path: &Path,
        cancel: &AtomicBool,
    ) -> Result<CharacterProject> {
        let bytes = source_bytes(path, cancel)?;
        let inspection = image_input::inspect(&bytes, cancel)?;
        image_input::check_cancel(cancel)?;
        let dir = self.dir(id)?;
        let _lock = lock(&dir.join(".write.lock"))?;
        let mut project = self.open_unlocked(id)?;
        if project.revision != revision {
            return Err(AppError::new("revision_conflict"));
        }
        if !project.sources.is_empty() {
            return Err(AppError::new("source_exists"));
        }
        let sources = dir.join("sources");
        plain(&sources)?;
        let dest = sources.join(format!("{}.{}", inspection.source_hash, inspection.format));
        if dest.try_exists()? {
            plain(&dest)?;
            if fs::metadata(&dest)?.len() != bytes.len() as u64 || fs::read(&dest)? != bytes {
                return Err(AppError::new("source_changed"));
            }
        } else {
            publish(&dest, &bytes)?;
        }
        image_input::check_cancel(cancel)?;
        project.schema_version = if inspection.format == "psd" { 2 } else { 1 };
        project.sources.push(SourceAsset {
            sha256: inspection.source_hash,
            bytes: bytes.len() as u64,
            width: inspection.width,
            height: inspection.height,
            format: inspection.format,
            provenance: "original".into(),
        });
        let saved = self.commit(project, revision)?;
        self.verify(&saved)?;
        Ok(saved)
    }
    pub fn inspect_source(&self, id: &str, cancel: &AtomicBool) -> Result<Inspection> {
        let _lock = lock(&self.dir(id)?.join(".write.lock"))?;
        let project = self.open_unlocked(id)?;
        let source = project
            .sources
            .first()
            .ok_or_else(|| AppError::new("source_required"))?;
        let path = self
            .dir(id)?
            .join("sources")
            .join(format!("{}.{}", source.sha256, source.format));
        let bytes = source_bytes(&path, cancel)?;
        if hash(&bytes) != source.sha256 {
            return Err(AppError::new("source_changed"));
        }
        image_input::inspect(&bytes, cancel)
    }
}
