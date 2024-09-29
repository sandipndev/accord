use thiserror::Error;

#[derive(Error, Debug)]
pub enum CommandError {
    #[error("CommandError - CommandFailed")]
    CommandFailed,
}
