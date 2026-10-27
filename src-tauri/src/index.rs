use crate::{
    durable::{lock, plain},
    recovery::ProjectList,
    AppError, Result,
};
use rusqlite::{params, Connection};
use std::{
    fs,
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

pub fn search(root: &Path, query: &str) -> Result<Vec<String>> {
    let path = root.join("index.sqlite");
    plain(&path)?;
    let db = Connection::open_with_flags(path, rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY)
        .map_err(|_| AppError::new("index_unavailable"))?;
    let mut stmt = db.prepare("SELECT id FROM projects WHERE status='healthy' AND instr(lower(name),lower(?1))>0 ORDER BY name").map_err(|_|AppError::new("index_unavailable"))?;
    let rows = stmt
        .query_map([query], |r| r.get::<_, String>(0))
        .map_err(|_| AppError::new("index_unavailable"))?;
    rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(|_| AppError::new("index_unavailable"))
}

// Always derived from a fresh file scan. Never use an old DB to certify a project.
pub fn rebuild(root: &Path, list: &ProjectList) -> Result<()> {
    let _guard = lock(&root.join(".index.lock"))?;
    let dest = root.join("index.sqlite");
    if dest.try_exists()? {
        plain(&dest)?;
    }
    let temp = root.join(format!(".index-{}.pending", uuid::Uuid::new_v4()));
    let result = (|| {
        let mut db = Connection::open(&temp).map_err(|_| AppError::new("index_unavailable"))?;
        db.execute_batch("PRAGMA temp_store=MEMORY; PRAGMA journal_mode=DELETE; PRAGMA synchronous=FULL; PRAGMA user_version=1; CREATE TABLE projects(id TEXT PRIMARY KEY, name TEXT, dimension TEXT, target TEXT, revision INTEGER, status TEXT NOT NULL, checked_at INTEGER NOT NULL);").map_err(|_|AppError::new("index_unavailable"))?;
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let tx = db
            .transaction()
            .map_err(|_| AppError::new("index_unavailable"))?;
        for p in &list.healthy {
            tx.execute(
                "INSERT INTO projects VALUES(?1,?2,?3,?4,?5,'healthy',?6)",
                params![
                    p.id,
                    p.name,
                    p.dimension,
                    p.target_profile,
                    p.revision as i64,
                    now as i64
                ],
            )
            .map_err(|_| AppError::new("index_unavailable"))?;
        }
        for p in &list.issues {
            tx.execute(
                "INSERT INTO projects VALUES(?1,NULL,NULL,NULL,?2,?3,?4)",
                params![
                    p.id,
                    p.recoverable.last().map(|v| *v as i64),
                    p.status,
                    now as i64
                ],
            )
            .map_err(|_| AppError::new("index_unavailable"))?;
        }
        tx.commit()
            .map_err(|_| AppError::new("index_unavailable"))?;
        db.close().map_err(|_| AppError::new("index_unavailable"))?;
        fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open(&temp)?
            .sync_all()?;
        fs::rename(&temp, &dest)?;
        Ok(())
    })();
    let _ = fs::remove_file(temp);
    result
}
