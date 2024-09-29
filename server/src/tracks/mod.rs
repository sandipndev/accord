pub mod semitones;
use semitones::{NewSemitone, Semitones};

mod entity;
pub use entity::{NewTrack, Track};

mod repo;
use repo::TracksRepo;

mod config;
pub use config::TracksConfig;

mod error;
pub use error::TrackError;

use crate::primitives::{TrackId, YoutubeUrl};

use sqlx::PgPool;

#[derive(Clone)]
pub struct Tracks {
    config: TracksConfig,
    repo: TracksRepo,
    semitones: Semitones,
}

impl Tracks {
    pub fn new(pool: &PgPool, config: TracksConfig) -> Self {
        Self {
            config,
            repo: TracksRepo::new(pool),
            semitones: Semitones::new(pool),
        }
    }

    pub fn semitones(&self) -> &Semitones {
        &self.semitones
    }

    pub async fn spawn_all_pending_semitone_conversion_jobs(&self) -> Result<(), TrackError> {
        self.semitones.spawn_all_pending_conversion_jobs().await
    }

    pub async fn create(&self, new_track: NewTrack) -> Result<Track, TrackError> {
        new_track.youtube_url.validate().await?;
        let track = self.repo.create(new_track).await?;

        for shift in self.config.shift_min..self.config.shift_max {
            self.semitones
                .create(NewSemitone {
                    track_id: track.id,
                    shift,
                })
                .await?;
        }

        Ok(track)
    }

    pub async fn get_by_id(&self, track_id: TrackId) -> Result<Track, TrackError> {
        self.repo.get_by_id(track_id).await
    }

    pub async fn get_all(&self) -> Result<Vec<Track>, TrackError> {
        self.repo.get_all().await
    }
}

impl YoutubeUrl {
    pub async fn validate(&self) -> Result<(), TrackError> {
        let parsed_youtube_url = url::Url::parse(&String::from(self.clone()))?;
        match parsed_youtube_url.host_str() {
            Some("www.youtube.com") | Some("youtube.com") | Some("youtu.be") => {}
            _ => return Err(TrackError::InvalidYoutubeURL),
        }

        if parsed_youtube_url
            .query_pairs()
            .any(|(key, _)| key == "list")
        {
            return Err(TrackError::InvalidYoutubeURL);
        }

        let metadata = crate::commands::metadata::get_metadata(self.clone()).await?;
        if metadata.duration_s > 1200 {
            return Err(TrackError::ExceedsTimeLimit);
        };

        Ok(())
    }
}
