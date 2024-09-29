use async_graphql::{scalar, Enum};
use serde::{Deserialize, Serialize};

use crate::entity_id;

entity_id!(TrackId);
entity_id!(SemitoneId);

scalar!(TrackId);
scalar!(SemitoneId);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, Enum)]
#[sqlx(type_name = "semitone_status", rename_all = "UPPERCASE")]
pub enum SemitoneStatus {
    Pending,
    Processing,
    Completed,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct YoutubeUrl(String);
scalar!(YoutubeUrl);

impl From<YoutubeUrl> for String {
    fn from(youtube_url: YoutubeUrl) -> Self {
        youtube_url.0
    }
}

impl From<String> for YoutubeUrl {
    fn from(youtube_url: String) -> Self {
        Self(youtube_url)
    }
}
