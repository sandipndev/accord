mod entity;
pub use entity::{NewSemitone, Semitone};

mod repo;
use repo::SemitonesRepo;

use crate::job::spawn_semitone_conversion_job;

use crate::primitives::{SemitoneId, TrackId};
use crate::tracks::error::TrackError;

use sqlx::PgPool;

#[derive(Clone)]
pub struct Semitones {
    pool: PgPool,
    repo: SemitonesRepo,
}

impl Semitones {
    pub fn new(pool: &PgPool) -> Self {
        Self {
            pool: pool.clone(),
            repo: SemitonesRepo::new(pool),
        }
    }

    pub async fn create(&self, new_semitone: NewSemitone) -> Result<Semitone, TrackError> {
        let semitone = self.repo.create(new_semitone).await?;
        spawn_semitone_conversion_job(&self.pool, semitone.id)
            .await
            .map_err(|_| TrackError::CouldNotSpawnConversionJob)?;
        Ok(semitone)
    }

    pub async fn get_by_track_id(&self, track_id: TrackId) -> Result<Vec<Semitone>, TrackError> {
        self.repo.get_by_track_id(track_id).await
    }

    pub async fn spawn_all_pending_conversion_jobs(&self) -> Result<(), TrackError> {
        let semitone_ids = self.repo.get_all_pending_semitone_ids().await?;
        for semitone_id in semitone_ids.into_iter() {
            spawn_semitone_conversion_job(&self.pool, semitone_id)
                .await
                .map_err(|_| TrackError::CouldNotSpawnConversionJob)?;
        }
        Ok(())
    }

    pub async fn convert(&self, semitone_id: SemitoneId) -> Result<(), TrackError> {
        println!("TODO: Convert Semitone ID: {:?}", semitone_id);
        Ok(())
    }
}
