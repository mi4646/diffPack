use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Git error: {0}")]
    Git(#[from] git2::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Pack error: {0}")]
    Pack(String),

    #[error("Invalid path: {0}")]
    InvalidPath(String),

    #[error("Repository not found: {0}")]
    RepoNotFound(String),

    #[error("Commit not found: {0}")]
    CommitNotFound(String),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("{0}")]
    Other(String),
}

impl From<AppError> for String {
    fn from(error: AppError) -> Self {
        error.to_string()
    }
}

pub type Result<T> = std::result::Result<T, AppError>;
