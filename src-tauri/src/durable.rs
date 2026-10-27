use crate::{AppError, Result};
use std::{
    fs::{self, File, OpenOptions, TryLockError},
    io::Write,
    path::Path,
};

pub fn plain(path: &Path) -> Result<()> {
    let meta = fs::symlink_metadata(path)?;
    #[cfg(windows)]
    {
        use std::os::windows::fs::MetadataExt;
        if meta.file_attributes() & 0x400 != 0 {
            return Err(AppError::new("unsafe_path"));
        }
    }
    if meta.file_type().is_symlink() {
        return Err(AppError::new("unsafe_path"));
    }
    Ok(())
}

// The persistent file is never unlinked. The OS owns lock lifetime, not a PID.
pub fn lock(path: &Path) -> Result<File> {
    if path.try_exists()? {
        plain(path)?;
    }
    let mut options = OpenOptions::new();
    options.read(true).write(true).create(true).truncate(false);
    #[cfg(windows)]
    {
        use std::os::windows::fs::OpenOptionsExt;
        options.share_mode(3); // deny delete/rename while handles are alive
    }
    let file = options.open(path)?;
    file.try_lock().map_err(|e| match e {
        TryLockError::WouldBlock => AppError::new("project_busy"),
        TryLockError::Error(e) => e.into(),
    })?;
    Ok(file)
}

pub fn publish(path: &Path, bytes: &[u8]) -> Result<()> {
    let temp = path.with_extension(format!("{}.pending", uuid::Uuid::new_v4()));
    let result = (|| {
        let mut f = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temp)?;
        checkpoint("empty")?;
        let middle = bytes.len() / 2;
        f.write_all(&bytes[..middle])?;
        checkpoint("partial")?;
        f.write_all(&bytes[middle..])?;
        f.sync_all()?;
        drop(f);
        checkpoint("before_publish")?;
        fs::hard_link(&temp, path)?;
        checkpoint("after_publish")?;
        Ok(())
    })();
    let _ = fs::remove_file(temp);
    result
}

fn checkpoint(_stage: &str) -> Result<()> {
    #[cfg(test)]
    if std::env::var("VTUBER_TEST_FAULT").ok().as_deref() == Some(_stage) {
        // Only test binaries have this hook. Each child gets its own environment.
        if let Ok(marker) = std::env::var("VTUBER_TEST_MARKER") {
            fs::write(marker, _stage)?;
            loop {
                std::thread::sleep(std::time::Duration::from_millis(50));
            }
        }
        return Err(std::io::Error::from(std::io::ErrorKind::StorageFull).into());
    }
    Ok(())
}
