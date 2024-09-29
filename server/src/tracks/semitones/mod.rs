mod entity;
pub use entity::{NewSemitone, Semitone};

mod repo;
use repo::SemitonesRepo;

use crate::job::spawn_semitone_conversion_job;

use crate::primitives::{SemitoneId, SemitoneStatus, TrackId};
use crate::tracks::{config::TracksConfig, error::TrackError};

use sqlx::PgPool;

#[derive(Clone)]
pub struct Semitones {
    pub pool: PgPool,
    repo: SemitonesRepo,
    config: TracksConfig,
}

impl Semitones {
    pub fn new(pool: &PgPool, config: TracksConfig) -> Self {
        Self {
            pool: pool.clone(),
            repo: SemitonesRepo::new(pool),
            config,
        }
    }

    pub async fn create(&self, new_semitone: NewSemitone) -> Result<Semitone, TrackError> {
        let semitone = self.repo.create(new_semitone).await?;
        Ok(semitone)
    }

    pub async fn get_by_track_id(&self, track_id: TrackId) -> Result<Vec<Semitone>, TrackError> {
        self.repo.get_by_track_id(track_id).await
    }

    pub async fn spawn_all_pending_conversion_jobs(&self) -> Result<(), TrackError> {
        let semitones = self.repo.get_all_pending_semitones().await?;
        for semitone in semitones.into_iter() {
            spawn_semitone_conversion_job(&self.pool, semitone.id)
                .await
                .map_err(|_| TrackError::CouldNotSpawnJob)?;
        }
        Ok(())
    }

    pub async fn convert(&self, semitone_id: SemitoneId) -> Result<(), TrackError> {
        self.repo
            .update_status(semitone_id, SemitoneStatus::Processing)
            .await?;

        let semitone = self.repo.get_by_id(semitone_id).await?;

        crate::commands::convert::shift_pitch_of_track_by(
            semitone.track_id,
            semitone.shift,
            &self.config.home_absolute_path,
        )
        .await?;

        self.repo
            .update_status(semitone_id, SemitoneStatus::Completed)
            .await?;

        Ok(())
    }
}
