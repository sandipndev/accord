use super::{NewTrack, Track, TrackError};
use crate::primitives::{TrackId, YoutubeUrl};

use sqlx::PgPool;

#[derive(Clone)]
pub struct TracksRepo {
    pool: PgPool,
}

impl TracksRepo {
    pub fn new(pool: &PgPool) -> Self {
        Self { pool: pool.clone() }
    }

    pub async fn create(&self, new_track: NewTrack) -> Result<Track, TrackError> {
        let id = TrackId::new();
        let query = sqlx::query!(
            r#"
            INSERT INTO tracks(id, name, youtube_url)
            VALUES ($1, $2, $3)
            RETURNING id, name, youtube_url, created_at
            "#,
            uuid::Uuid::from(id),
            String::from(new_track.name().await?),
            String::from(new_track.youtube_url),
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(Track {
            id: TrackId::from(query.id),
            name: query.name,
            youtube_url: YoutubeUrl::from(query.youtube_url),
            created_at: query.created_at,
        })
    }

    pub async fn get_by_id(&self, id: TrackId) -> Result<Track, TrackError> {
        let query = sqlx::query!(
            r#"
            SELECT id, name, youtube_url, created_at
            FROM tracks
            WHERE id = $1
        "#,
            uuid::Uuid::from(id)
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(Track {
            id: TrackId::from(query.id),
            name: query.name,
            youtube_url: YoutubeUrl::from(query.youtube_url),
            created_at: query.created_at,
        })
    }

    pub async fn get_all(&self) -> Result<Vec<Track>, TrackError> {
        let query = sqlx::query!(
            r#"
            SELECT id, name, youtube_url, created_at
            FROM tracks
            ORDER BY created_at DESC
        "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(query
            .into_iter()
            .map(|data| Track {
                id: TrackId::from(data.id),
                name: data.name,
                youtube_url: YoutubeUrl::from(data.youtube_url),
                created_at: data.created_at,
            })
            .collect())
    }
}
