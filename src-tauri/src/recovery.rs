use crate::{
    durable::{lock, plain, publish},
    model::CharacterProject,
    storage::{hash, ProjectStore},
    AppError, Result,
};
use serde::Serialize;
use std::{fs, io::Read};

#[derive(Debug, Clone, Serialize)]
pub struct RevisionReport {
    pub revision: u64,
    pub project: Option<CharacterProject>,
    pub error: Option<String>,
}
#[derive(Debug, Clone, Serialize)]
pub struct ProjectIssue {
    pub id: String,
    pub status: String,
    pub error: String,
    pub recoverable: Vec<u64>,
}
#[derive(Debug, Clone, Serialize)]
pub struct ProjectList {
    pub version: u32,
    pub healthy: Vec<CharacterProject>,
    pub issues: Vec<ProjectIssue>,
    pub index_warning: Option<String>,
}
#[derive(Debug, Clone, Serialize)]
pub struct RecoveryPreview {
    pub selected: CharacterProject,
    pub latest: Option<CharacterProject>,
    pub latest_revision: u64,
    pub token: String,
    pub changed_fields: Vec<String>,
}

impl ProjectStore {
    pub fn search_projects(&self, query: &str) -> Result<ProjectList> {
        if query.chars().count() > 120 {
            return Err(AppError::new("invalid_name"));
        }
        let mut list = self.list_v2()?;
        let ids = if list.index_warning.is_none() {
            crate::index::search(&self.root, query).ok()
        } else {
            None
        };
        if let Some(ids) = ids {
            list.healthy.retain(|p| ids.contains(&p.id));
        } else {
            list.index_warning = Some("index_unavailable".into());
            list.healthy
                .retain(|p| p.name.to_lowercase().contains(&query.to_lowercase()));
        }
        Ok(list)
    }
    pub fn list_v2(&self) -> Result<ProjectList> {
        plain(&self.root)?;
        let mut result = ProjectList {
            version: 2,
            healthy: vec![],
            issues: vec![],
            index_warning: None,
        };
        // Enumeration failures are global; individual unreadable entries are isolated.
        for item in fs::read_dir(&self.root)? {
            let id = item?.file_name().to_string_lossy().into_owned();
            if uuid::Uuid::parse_str(&id).is_err() {
                continue;
            }
            match self.inspect(&id) {
                Ok(history) => {
                    if let Some(p) = history.last().and_then(|r| r.project.clone()) {
                        result.healthy.push(p);
                    } else {
                        let error = history
                            .last()
                            .and_then(|r| r.error.clone())
                            .unwrap_or("invalid_project".into());
                        let recoverable = history
                            .iter()
                            .filter(|r| r.project.is_some())
                            .map(|r| r.revision)
                            .collect::<Vec<_>>();
                        let status = issue_status(&error, !recoverable.is_empty());
                        result.issues.push(ProjectIssue {
                            id,
                            status: status.into(),
                            error,
                            recoverable,
                        });
                    }
                }
                Err(e) => result.issues.push(ProjectIssue {
                    id,
                    status: issue_status(&e.code, false).into(),
                    error: e.code,
                    recoverable: vec![],
                }),
            }
        }
        result.healthy.sort_by(|a, b| a.name.cmp(&b.name));
        result.issues.sort_by(|a, b| a.id.cmp(&b.id));
        if let Err(e) = crate::index::rebuild(&self.root, &result) {
            result.index_warning = Some(e.code);
        }
        Ok(result)
    }
    pub(crate) fn revisions(&self, id: &str) -> Result<Vec<u64>> {
        let mut revisions = vec![];
        for entry in fs::read_dir(self.dir(id)?)? {
            let name = entry?.file_name().to_string_lossy().into_owned();
            if name.len() == 13
                && name.ends_with(".json")
                && name.as_bytes()[..8].iter().all(u8::is_ascii_digit)
            {
                revisions.push(
                    name[..8]
                        .parse()
                        .map_err(|_| AppError::new("invalid_project"))?,
                );
            }
            if revisions.len() > 10_000 {
                return Err(AppError::new("inspection_limit"));
            }
        }
        revisions.sort_unstable();
        Ok(revisions)
    }
    fn inspect_unlocked(&self, id: &str) -> Result<Vec<RevisionReport>> {
        Ok(self
            .revisions(id)?
            .into_iter()
            .map(|revision| match self.read_revision(id, revision) {
                Ok(p) => RevisionReport {
                    revision,
                    project: Some(p),
                    error: None,
                },
                Err(e) => RevisionReport {
                    revision,
                    project: None,
                    error: Some(e.code),
                },
            })
            .collect())
    }
    pub fn inspect(&self, id: &str) -> Result<Vec<RevisionReport>> {
        let _guard = lock(&self.dir(id)?.join(".write.lock"))?;
        self.inspect_unlocked(id)
    }
    fn fingerprint(&self, id: &str) -> Result<String> {
        let mut signatures = Vec::new();
        for rev in self.revisions(id)? {
            let path = self.dir(id)?.join(format!("{rev:08}.json"));
            plain(&path)?;
            let mut data = Vec::new();
            fs::File::open(&path)?
                .take(1024 * 1024 + 1)
                .read_to_end(&mut data)?;
            if data.len() > 1024 * 1024 {
                return Err(AppError::new("inspection_limit"));
            }
            signatures.push((rev, hash(&data)));
        }
        Ok(hash(&serde_json::to_vec(&signatures)?))
    }
    fn preview_unlocked(&self, id: &str, revision: u64) -> Result<RecoveryPreview> {
        let selected = self.read_revision(id, revision)?;
        let latest_revision = self.latest(&self.dir(id)?)?;
        let latest = match self.read_revision(id, latest_revision) {
            Ok(p) => Some(p),
            Err(e) if e.code == "schema_unsupported" => return Err(e),
            Err(_) => None,
        };
        let mut changed_fields = vec![];
        if let Some(current) = &latest {
            let a = serde_json::to_value(current)?;
            let b = serde_json::to_value(&selected)?;
            for key in [
                "name",
                "dimension",
                "target_profile",
                "sources",
                "gates",
                "master_status",
            ] {
                if a[key] != b[key] {
                    changed_fields.push(key.into());
                }
            }
        } else {
            changed_fields.push("latest_unreadable".into());
        }
        Ok(RecoveryPreview {
            selected,
            latest,
            latest_revision,
            token: self.fingerprint(id)?,
            changed_fields,
        })
    }
    pub fn preview_recovery(&self, id: &str, revision: u64) -> Result<RecoveryPreview> {
        let _guard = lock(&self.dir(id)?.join(".write.lock"))?;
        self.preview_unlocked(id, revision)
    }
    pub fn recover(&self, id: &str, revision: u64, token: &str) -> Result<CharacterProject> {
        let dir = self.dir(id)?;
        let _guard = lock(&dir.join(".write.lock"))?;
        let preview = self.preview_unlocked(id, revision)?;
        if preview.token != token {
            return Err(AppError::new("revision_conflict"));
        }
        let mut restored = preview.selected;
        restored.revision = preview
            .latest_revision
            .checked_add(1)
            .ok_or_else(|| AppError::new("invalid_project"))?;
        restored.validate()?;
        self.verify(&restored)?;
        publish(
            &dir.join(format!("{:08}.json", restored.revision)),
            &serde_json::to_vec_pretty(&restored)?,
        )?;
        // Reopen and rehash before acknowledging recovery. Old files are never changed.
        self.open_unlocked(id)
    }
}
fn issue_status(error: &str, recoverable: bool) -> &'static str {
    match error {
        "access_denied" => "access_denied",
        "project_busy" => "project_busy",
        "schema_unsupported" => "schema_unsupported",
        "unsafe_path" | "inspection_limit" | "storage_error" => "uninspectable",
        _ if recoverable => "recoverable",
        _ => "unrecoverable",
    }
}
