pub mod gpu;
pub mod model;
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
    fn from(_: std::io::Error) -> Self {
        Self::new("storage_error")
    }
}
impl From<serde_json::Error> for AppError {
    fn from(_: serde_json::Error) -> Self {
        Self::new("invalid_project")
    }
}
