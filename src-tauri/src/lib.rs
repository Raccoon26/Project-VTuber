pub mod durable;
pub mod gpu;
pub mod index;
pub mod journal;
pub mod model;
pub mod recovery;
#[cfg(test)]
mod resilience_tests;
pub mod storage;
pub mod worker;

use serde::Serialize;
#[derive(Debug, Clone, Serialize)]
pub struct AppError {
    pub code: String,
}
impl AppError {
    pub fn new(code: &str) -> Self {
        Self { code: code.into() }
    }
}
pub type Result<T> = std::result::Result<T, AppError>;
impl From<std::io::Error> for AppError {
    fn from(error: std::io::Error) -> Self {
        Self::new(match error.kind() {
            std::io::ErrorKind::PermissionDenied => "access_denied",
            std::io::ErrorKind::StorageFull => "disk_full",
            _ => "storage_error",
        })
    }
}
impl From<serde_json::Error> for AppError {
    fn from(_: serde_json::Error) -> Self {
        Self::new("invalid_project")
    }
}
