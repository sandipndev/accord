use super::TrackError;
use crate::primitives::{TrackId, YoutubeUrl};

use chrono::{DateTime, Utc};

pub struct Track {
    pub id: TrackId,
    pub name: String,
    pub youtube_url: YoutubeUrl,
    pub created_at: DateTime<Utc>,
}

pub struct NewTrack {
    pub youtube_url: YoutubeUrl,
}
impl NewTrack {
    pub async fn name(&self) -> Result<String, TrackError> {
        let metadata = crate::commands::metadata::get_metadata(self.youtube_url.clone()).await?;
        Ok(metadata.title)
    }
}
