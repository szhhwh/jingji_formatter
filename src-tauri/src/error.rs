use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("{0}")]
    Err(String),
    #[error("stdIOError: ")]
    IO(#[from] std::io::Error),
    #[error("JsonError: ")]
    JsonIO(#[from] serde_json::error::Error),
    #[error("unknown error")]
    Unknown,
}

impl From<String> for AppError {
    fn from(s: String) -> Self {
        AppError::Err(s)
    }
}