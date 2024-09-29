use thiserror::Error;

#[derive(Error, Debug)]
pub enum TrackError {
    #[error("TrackError - Sqlx: {0}")]
    Sqlx(#[from] sqlx::Error),
    #[error("TrackError - CommandError: {0}")]
    CommandError(#[from] crate::commands::error::CommandError),
    #[error("CouldNotSpawnConversionJob")]
    CouldNotSpawnConversionJob,
    #[error("ExceedsTimeLimit")]
    ExceedsTimeLimit,
    #[error("InvalidYoutubeURL")]
    InvalidYoutubeURL,
    #[error("UrlParseError")]
    UrlParseError(#[from] url::ParseError),
}
