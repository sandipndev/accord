use async_graphql::*;

use crate::app::AccordeApp;
use crate::primitives::*;

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Track {
    pub id: TrackId,
    pub name: String,
    pub youtube_url: YoutubeUrl,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl From<crate::tracks::Track> for Track {
    fn from(track: crate::tracks::Track) -> Self {
        Self {
            id: track.id,
            name: track.name,
            youtube_url: track.youtube_url,
            created_at: track.created_at,
        }
    }
}

#[derive(SimpleObject)]
pub struct Semitone {
    pub id: SemitoneId,
    pub track_id: TrackId,
    pub shift: i32,
    pub status: SemitoneStatus,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl From<crate::tracks::semitones::Semitone> for Semitone {
    fn from(semitone: crate::tracks::semitones::Semitone) -> Self {
        Self {
            id: semitone.id,
            track_id: semitone.track_id,
            shift: semitone.shift,
            status: semitone.status,
            created_at: semitone.created_at,
        }
    }
}

#[ComplexObject]
impl Track {
    async fn semitones(&self, ctx: &Context<'_>) -> Result<Vec<Semitone>> {
        let app = ctx.data_unchecked::<AccordeApp>();
        let semitones = app.tracks().semitones().get_by_track_id(self.id).await?;
        Ok(semitones
            .into_iter()
            .map(|semitone| semitone.into())
            .collect())
    }
}
