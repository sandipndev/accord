use super::{NewSemitone, Semitone};
use crate::tracks::error::TrackError;

use crate::primitives::{SemitoneId, SemitoneStatus, TrackId};

use sqlx::PgPool;

#[derive(Clone)]
pub struct SemitonesRepo {
    pool: PgPool,
}

impl SemitonesRepo {
    pub fn new(pool: &PgPool) -> Self {
        Self { pool: pool.clone() }
    }

    pub async fn create(&self, new_semitone: NewSemitone) -> Result<Semitone, TrackError> {
        let id = SemitoneId::new();
        let query = sqlx::query!(
            r#"
            INSERT INTO semitones(id, track_id, shift)
            VALUES ($1, $2, $3)
            RETURNING id, track_id, shift, status AS "status!: SemitoneStatus", created_at
            "#,
            uuid::Uuid::from(id),
            uuid::Uuid::from(new_semitone.track_id),
            new_semitone.shift,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(Semitone {
            id: SemitoneId::from(query.id),
            track_id: TrackId::from(query.track_id),
            shift: query.shift,
            status: query.status,
            created_at: query.created_at,
        })
    }

    pub async fn get_by_track_id(&self, track_id: TrackId) -> Result<Vec<Semitone>, TrackError> {
        let query = sqlx::query!(
            r#"
            SELECT id, track_id, shift, status AS "status!: SemitoneStatus", created_at
            FROM semitones
            WHERE track_id = $1
"#,
            uuid::Uuid::from(track_id)
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(query
            .into_iter()
            .map(|data| Semitone {
                id: SemitoneId::from(data.id),
                track_id: TrackId::from(data.track_id),
                shift: data.shift,
                status: data.status,
                created_at: data.created_at,
            })
            .collect())
    }

    pub async fn get_all_pending_semitone_ids(&self) -> Result<Vec<SemitoneId>, TrackError> {
        let query = sqlx::query!(r#"SELECT id FROM semitones WHERE status = 'PENDING'"#)
            .fetch_all(&self.pool)
            .await?;

        Ok(query
            .into_iter()
            .map(|data| SemitoneId::from(data.id))
            .collect())
    }
}
