use sqlx::PgPool;
mod commands;

mod config;
pub use config::ProcessConfig;

mod error;
pub use error::ProcessError;

mod entity;
pub use entity::{NewProcess, Process};

use crate::primitives::*;

#[derive(Clone)]
pub struct Processes {
    config: ProcessConfig,
    pool: PgPool,
}

impl Processes {
    pub fn new(pool: &PgPool, config: ProcessConfig) -> Self {
        Self {
            pool: pool.clone(),
            config,
        }
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

        crate::job::spawn_download_job(&self.pool, id)
            .await
            .map_err(|_| ProcessError::JobSpawnFailed)?;

        Ok(Process {
            id,
            youtube_url: query.youtube_url.parse().expect("Invalid URL"),
            status: query.process_status,
            created_at: query.created_at,
        })
    }

    pub async fn get(&self, process_id: ProcessId) -> Result<Process, ProcessError> {
        let query = sqlx::query!(
            r#"
            SELECT id, youtube_url, status AS "process_status:ProcessStatus", created_at
            FROM processes
            WHERE id = $1
        "#,
            uuid::Uuid::from(process_id),
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(Process {
            id: process_id,
            youtube_url: query.youtube_url.parse().expect("Invalid URL"),
            status: query.process_status,
            created_at: query.created_at,
        })
    }

    pub async fn update_status(
        &self,
        process_id: ProcessId,
        status: ProcessStatus,
    ) -> Result<(), ProcessError> {
        sqlx::query!(
            r#"
            UPDATE processes
            SET status = $2, updated_at = NOW()
            WHERE id = $1
        "#,
            uuid::Uuid::from(process_id),
            status as ProcessStatus,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_all_pending(&self) -> Result<Vec<Process>, ProcessError> {
        let query = sqlx::query!(
            r#"
            SELECT id, youtube_url, status AS "process_status:ProcessStatus", created_at
            FROM processes
            WHERE status = 'PENDING'
        "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(query
            .into_iter()
            .map(|row| Process {
                id: ProcessId::from(row.id),
                youtube_url: row.youtube_url.parse().expect("Invalid URL"),
                status: row.process_status,
                created_at: row.created_at,
            })
            .collect())
    }

    pub async fn spawn_all_pending_jobs(&self) -> Result<(), ProcessError> {
        let pending_processes = self.get_all_pending().await?;

        for process in pending_processes {
            crate::job::spawn_download_job(&self.pool, process.id)
                .await
                .map_err(|_| ProcessError::JobSpawnFailed)?;
        }

        Ok(())
    }
}
