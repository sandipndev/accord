use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProcessError {
    #[error("ProcessError - Sqlx: {0}")]
    Sqlx(#[from] sqlx::Error),
    #[error("ProcessError - JobSpawnFailed")]
    JobSpawnFailed,
    #[error("ProcessError - CommandFailed")]
    CommandFailed,
    #[error("ProcessError - FileNotFound")]
    FileNotFound,
}
