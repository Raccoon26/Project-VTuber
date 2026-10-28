use crate::{AppError, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum GateStatus {
    Pass,
    Fail,
    Unverified,
    NotApplicable,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Gate {
    pub id: String,
    pub status: GateStatus,
    pub evidence: Vec<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceAsset {
    pub sha256: String,
    pub bytes: u64,
    pub width: u32,
    pub height: u32,
    pub format: String,
    pub provenance: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CharacterProject {
    pub schema_version: u32,
    pub id: String,
    pub revision: u64,
    pub name: String,
    pub dimension: String,
    pub target_profile: String,
    pub sources: Vec<SourceAsset>,
    pub gates: Vec<Gate>,
    pub master_status: String,
}
pub const PROFILES: &[(&str, &str)] = &[
    ("vtube-studio", "2d"),
    ("vbridger-vts", "2d"),
    ("inochi-session", "2d"),
    ("prprlive", "2d"),
    ("vseeface", "3d"),
    ("warudo", "3d"),
    ("vnyan", "3d"),
    ("vrchat", "3d"),
    ("vbridger-vmc", "3d"),
    ("glb", "3d"),
];
pub fn validate_settings(name: &str, dimension: &str, target: &str) -> Result<()> {
    if name.trim().is_empty() || name.chars().count() > 120 || name.chars().any(char::is_control) {
        return Err(AppError::new("invalid_name"));
    }
    if !PROFILES.contains(&(target, dimension)) {
        return Err(AppError::new("invalid_profile"));
    }
    Ok(())
}
impl CharacterProject {
    pub fn new(name: String, dimension: String, target: String) -> Result<Self> {
        validate_settings(&name, &dimension, &target)?;
        Ok(Self {
            schema_version: 1,
            id: uuid::Uuid::new_v4().to_string(),
            revision: 1,
            name,
            dimension,
            target_profile: target,
            sources: vec![],
            gates: ["A", "B", "C", "D", "E", "F"]
                .iter()
                .map(|id| Gate {
                    id: id.to_string(),
                    status: GateStatus::Unverified,
                    evidence: vec![],
                })
                .collect(),
            master_status: "not_generated".into(),
        })
    }
    pub fn validate(&self) -> Result<()> {
        if ![1, 2].contains(&self.schema_version) {
            return Err(AppError::new("schema_unsupported"));
        }
        if uuid::Uuid::parse_str(&self.id)
            .map(|x| x.to_string())
            .ok()
            .as_deref()
            != Some(&self.id)
            || self.revision == 0
            || self.revision > 99999999
        {
            return Err(AppError::new("invalid_project"));
        }
        validate_settings(&self.name, &self.dimension, &self.target_profile)?;
        if self.master_status != "not_generated" || self.sources.len() > 1 || self.gates.len() != 6
        {
            return Err(AppError::new("invalid_project"));
        }
        for (g, expected) in self.gates.iter().zip(["A", "B", "C", "D", "E", "F"]) {
            if g.id != expected || !g.evidence.is_empty() || g.status != GateStatus::Unverified {
                return Err(AppError::new("invalid_project"));
            }
        }
        for s in &self.sources {
            if s.sha256.len() != 64
                || !s
                    .sha256
                    .bytes()
                    .all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase())
                || s.provenance != "original"
                || !((self.schema_version == 1 && s.format == "png")
                    || (self.schema_version == 2 && s.format == "psd"))
                || s.bytes > 64 * 1024 * 1024
                || s.width == 0
                || s.height == 0
                || u64::from(s.width) * u64::from(s.height) > 40_000_000
            {
                return Err(AppError::new("invalid_project"));
            }
        }
        Ok(())
    }
}
