use sqlx::PgPool;

mod error;
use error::ProcessError;

mod entity;
pub use entity::{NewProcess, Process};

use crate::primitives::*;

#[derive(Clone)]
pub struct Processes {
    pool: PgPool,
}

impl Processes {
    pub fn new(pool: &PgPool) -> Self {
        Self { pool: pool.clone() }
    }

    pub async fn create(&self, new_process: NewProcess) -> Result<Process, ProcessError> {
        let id = ProcessId::new();
        let query = sqlx::query!(
            r#"
            INSERT INTO processes(id, youtube_url)
            VALUES ($1, $2)
            RETURNING id, youtube_url, status AS "process_status:ProcessStatus", created_at
        "#,
            uuid::Uuid::from(id),
            String::from(new_process.youtube_url),
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(Process {
            id,
            youtube_url: query.youtube_url.parse().expect("Invalid URL"),
            status: query.process_status,
            created_at: query.created_at,
        })
    }
}
