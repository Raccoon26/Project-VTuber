use crate::{
    durable::{lock, plain, publish},
    model::{CharacterProject, SourceAsset},
    AppError, Result,
};
use sha2::{Digest, Sha256};
use std::{
    fs::{self, File},
    io::Read,
    path::{Path, PathBuf},
};

const MAX_MANIFEST: u64 = 1024 * 1024;
pub struct ProjectStore {
    pub(crate) root: PathBuf,
}

pub fn hash(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

impl ProjectStore {
    pub fn new(root: PathBuf) -> Result<Self> {
        if root.try_exists()? {
            plain(&root)?;
        }
        fs::create_dir_all(&root)?;
        plain(&root)?;
        Ok(Self {
            root: fs::canonicalize(root)?,
        })
    }
    pub(crate) fn dir(&self, id: &str) -> Result<PathBuf> {
        if uuid::Uuid::parse_str(id)
            .map(|v| v.to_string())
            .ok()
            .as_deref()
            != Some(id)
        {
            return Err(AppError::new("unsafe_path"));
        }
        plain(&self.root)?;
        let dir = self.root.join(id);
        if dir.try_exists()? {
            plain(&dir)?;
        }
        Ok(dir)
    }
    pub(crate) fn latest(&self, dir: &Path) -> Result<u64> {
        let mut latest = 0;
        for file in fs::read_dir(dir)? {
            let file = file?;
            let name = file.file_name().to_string_lossy().into_owned();
            if name.len() == 13
                && name.ends_with(".json")
                && name.as_bytes()[..8].iter().all(|x| x.is_ascii_digit())
            {
                latest = latest.max(name[..8].parse().unwrap_or(0));
            }
        }
        if latest == 0 {
            return Err(AppError::new("invalid_project"));
        }
        Ok(latest)
    }
    pub fn create(
        &self,
        name: String,
        dimension: String,
        target: String,
    ) -> Result<CharacterProject> {
        let project = CharacterProject::new(name, dimension, target)?;
        let dir = self.dir(&project.id)?;
        fs::create_dir(&dir)?;
        let _lock = lock(&dir.join(".write.lock"))?;
        fs::create_dir(dir.join("sources"))?;
        publish(
            &dir.join("00000001.json"),
            &serde_json::to_vec_pretty(&project)?,
        )?;
        Ok(project)
    }
    pub fn open(&self, id: &str) -> Result<CharacterProject> {
        let dir = self.dir(id)?;
        let _lock = lock(&dir.join(".write.lock"))?;
        self.open_unlocked(id)
    }
    pub(crate) fn open_unlocked(&self, id: &str) -> Result<CharacterProject> {
        let revision = self.latest(&self.dir(id)?)?;
        self.read_revision(id, revision)
    }
    pub(crate) fn read_revision(&self, id: &str, revision: u64) -> Result<CharacterProject> {
        let dir = self.dir(id)?;
        let path = dir.join(format!("{revision:08}.json"));
        plain(&path)?;
        let mut bytes = Vec::new();
        File::open(path)?
            .take(MAX_MANIFEST + 1)
            .read_to_end(&mut bytes)?;
        if bytes.len() as u64 > MAX_MANIFEST {
            return Err(AppError::new("invalid_project"));
        }
        let value: serde_json::Value = serde_json::from_slice(&bytes)?;
        if value
            .get("schema_version")
            .and_then(|x| x.as_u64())
            .is_some_and(|v| ![1, 2].contains(&v))
        {
            return Err(AppError::new("schema_unsupported"));
        }
        let project: CharacterProject = serde_json::from_value(value)?;
        project.validate()?;
        if project.id != id || project.revision != revision {
            return Err(AppError::new("invalid_project"));
        }
        self.verify(&project)?;
        Ok(project)
    }
    pub fn list(&self) -> Result<Vec<CharacterProject>> {
        Ok(self.list_v2()?.healthy)
    }
    pub(crate) fn commit(
        &self,
        mut project: CharacterProject,
        expected: u64,
    ) -> Result<CharacterProject> {
        let dir = self.dir(&project.id)?;
        if self.latest(&dir)? != expected {
            return Err(AppError::new("revision_conflict"));
        }
        project.revision = expected
            .checked_add(1)
            .ok_or_else(|| AppError::new("invalid_project"))?;
        project.validate()?;
        publish(
            &dir.join(format!("{:08}.json", project.revision)),
            &serde_json::to_vec_pretty(&project)?,
        )?;
        Ok(project)
    }
    pub fn rename(&self, id: &str, revision: u64, name: String) -> Result<CharacterProject> {
        let _lock = lock(&self.dir(id)?.join(".write.lock"))?;
        let mut p = self.open_unlocked(id)?;
        p.name = name;
        self.commit(p, revision)
    }
    pub fn import_png(&self, id: &str, revision: u64, path: &Path) -> Result<CharacterProject> {
        let _lock = lock(&self.dir(id)?.join(".write.lock"))?;
        let mut p = self.open_unlocked(id)?;
        if p.revision != revision {
            return Err(AppError::new("revision_conflict"));
        }
        if !p.sources.is_empty() {
            return Err(AppError::new("source_exists"));
        }
        plain(path)?;
        let mut bytes = Vec::new();
        File::open(path)?
            .take(64 * 1024 * 1024 + 1)
            .read_to_end(&mut bytes)?;
        if bytes.len() > 64 * 1024 * 1024 {
            return Err(AppError::new("image_too_large"));
        }
        let mut decoder = png::Decoder::new(bytes.as_slice());
        decoder.set_limits(png::Limits {
            bytes: 192 * 1024 * 1024,
        });
        let mut reader = decoder
            .read_info()
            .map_err(|_| AppError::new("invalid_image"))?;
        let info = reader.info();
        let (width, height) = (info.width, info.height);
        if width == 0
            || height == 0
            || u64::from(width) * u64::from(height) > 40_000_000
            || reader.output_buffer_size() > 160 * 1024 * 1024
        {
            return Err(AppError::new("image_too_large"));
        }
        if info.animation_control.is_some() {
            return Err(AppError::new("invalid_image"));
        }
        reader
            .next_frame(&mut vec![0; reader.output_buffer_size()])
            .map_err(|_| AppError::new("invalid_image"))?;
        reader
            .finish()
            .map_err(|_| AppError::new("invalid_image"))?;
        let sha256 = hash(&bytes);
        let sources = self.dir(id)?.join("sources");
        plain(&sources)?;
        let dest = sources.join(format!("{sha256}.png"));
        if dest.exists() {
            plain(&dest)?;
            if fs::read(&dest)? != bytes {
                return Err(AppError::new("source_changed"));
            }
        } else {
            publish(&dest, &bytes)?;
        }
        p.sources.push(SourceAsset {
            sha256,
            bytes: bytes.len() as u64,
            width,
            height,
            format: "png".into(),
            provenance: "original".into(),
        });
        self.commit(p, revision)
    }
    pub fn verify(&self, p: &CharacterProject) -> Result<()> {
        let sources = self.dir(&p.id)?.join("sources");
        plain(&sources)?;
        for source in &p.sources {
            let path = sources.join(format!("{}.{}", source.sha256, source.format));
            plain(&path)?;
            let mut file = File::open(path)?;
            if file.metadata()?.len() != source.bytes {
                return Err(AppError::new("source_changed"));
            }
            let mut hasher = Sha256::new();
            let mut buf = [0; 65536];
            loop {
                let n = file.read(&mut buf)?;
                if n == 0 {
                    break;
                }
                hasher.update(&buf[..n]);
            }
            if format!("{:x}", hasher.finalize()) != source.sha256 {
                return Err(AppError::new("source_changed"));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn published_sample_validates_but_future_or_forged_gates_do_not() {
        let mut p: CharacterProject =
            serde_json::from_str(include_str!("../../samples/character-project.json")).unwrap();
        p.validate().unwrap();
        p.schema_version = 999;
        assert_eq!(p.validate().unwrap_err().code, "schema_unsupported");
        p.schema_version = 1;
        p.gates[0].status = crate::model::GateStatus::Pass;
        assert_eq!(p.validate().unwrap_err().code, "invalid_project");
    }
    #[test]
    fn unicode_filenames_are_ignored_without_panicking() {
        let (_tmp, s, p) = setup();
        fs::write(s.dir(&p.id).unwrap().join("日本ab.json"), b"{}").unwrap();
        assert_eq!(s.open(&p.id).unwrap().revision, 1);
    }
    #[test]
    fn publication_never_overwrites_a_revision() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("revision.json");
        publish(&path, b"previous").unwrap();
        assert!(publish(&path, b"replacement").is_err());
        assert_eq!(fs::read(&path).unwrap(), b"previous");
    }
    fn setup() -> (tempfile::TempDir, ProjectStore, CharacterProject) {
        let tmp = tempfile::tempdir().unwrap();
        let store = ProjectStore::new(tmp.path().join("projects")).unwrap();
        let p = store
            .create("テスト 한글".into(), "2d".into(), "vbridger-vts".into())
            .unwrap();
        (tmp, store, p)
    }
    fn png_file(path: &Path) {
        let mut enc = png::Encoder::new(File::create(path).unwrap(), 2, 2);
        enc.set_color(png::ColorType::Rgba);
        enc.set_depth(png::BitDepth::Eight);
        enc.write_header()
            .unwrap()
            .write_image_data(&[127; 16])
            .unwrap();
    }
    #[test]
    fn save_reopen_conflict_and_interrupted_write() {
        let (_tmp, s, p) = setup();
        let p2 = s.rename(&p.id, 1, "New".into()).unwrap();
        assert_eq!(p2.revision, 2);
        assert_eq!(s.open(&p.id).unwrap().name, "New");
        assert_eq!(
            s.rename(&p.id, 1, "stale".into()).unwrap_err().code,
            "revision_conflict"
        );
        fs::write(
            s.dir(&p.id).unwrap().join("00000003.dead.pending"),
            b"incomplete",
        )
        .unwrap();
        assert_eq!(s.open(&p.id).unwrap().revision, 2);
    }
    #[test]
    fn source_bytes_preserved_and_tamper_detected() {
        let (tmp, s, p) = setup();
        let source = tmp.path().join("input.png");
        png_file(&source);
        let original = fs::read(&source).unwrap();
        let p = s.import_png(&p.id, 1, &source).unwrap();
        let stored = s
            .dir(&p.id)
            .unwrap()
            .join("sources")
            .join(format!("{}.png", p.sources[0].sha256));
        assert_eq!(original, fs::read(&stored).unwrap());
        assert_eq!(original, fs::read(source).unwrap());
        s.verify(&p).unwrap();
        fs::write(stored, b"tampered").unwrap();
        assert_eq!(s.open(&p.id).unwrap_err().code, "source_changed");
    }
    #[test]
    fn bad_inputs_do_not_create_revision() {
        let (tmp, s, p) = setup();
        let source = tmp.path().join("bad.png");
        fs::write(&source, b"not a PNG").unwrap();
        assert!(s.import_png(&p.id, 1, &source).is_err());
        assert_eq!(s.open(&p.id).unwrap().revision, 1);
        assert_eq!(s.open("../escape").unwrap_err().code, "unsafe_path");
        assert!(s
            .create("".into(), "2d".into(), "vtube-studio".into())
            .is_err());
        assert!(s
            .create("test".into(), "3d".into(), "vbridger-vts".into())
            .is_err());
    }
    #[test]
    fn corrupt_latest_is_not_silently_rolled_back() {
        let (_tmp, s, p) = setup();
        fs::write(s.dir(&p.id).unwrap().join("00000002.json"), b"{}").unwrap();
        assert!(s.open(&p.id).is_err());
    }
}
