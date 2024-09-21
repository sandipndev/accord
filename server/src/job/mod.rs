mod error;
mod executor;

pub use error::JobError;
use executor::{JobExecutionError, JobExecutor};

use sqlxmq::{job, CurrentJob, JobRegistry, JobRunnerHandle};

use crate::{app::AccordeApp, primitives::ProcessId};

pub async fn start_job_runner(
    pool: &sqlx::PgPool,
    app: AccordeApp,
) -> Result<JobRunnerHandle, JobError> {
    let mut registry = JobRegistry::new(&[download_job, convert_job]);

    registry.set_context(app);

    Ok(registry.runner(pool).set_keep_alive(false).run().await?)
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct JobData {
    pub process_id: ProcessId,
}

#[job(name = "download")]
async fn download_job(mut current_job: CurrentJob, app: AccordeApp) -> Result<(), JobError> {
    let process_data: JobData = current_job.json()?.expect("couldn't parse json");
    let process_id = process_data.process_id;

    JobExecutor::builder(&mut current_job)
        .build()
        .expect("couldn't build JobExecutor")
        .execute(|_| async move {
            match app.processes().download(process_id).await {
                Ok(_) => Ok(()),
                Err(e) => Err(JobError::from(e)),
            }
        })
        .await?;
    Ok(())
}

pub async fn spawn_download_job(
    pool: &sqlx::PgPool,
    process_id: ProcessId,
) -> Result<(), JobError> {
    let json = JobData { process_id };
    match download_job.builder().set_json(&json)?.spawn(pool).await {
        Err(sqlx::Error::Database(err)) if err.message().contains("duplicate key") => Ok(()),
        Err(e) => Err(e.into()),
        Ok(_) => Ok(()),
    }
}

#[job(name = "convert")]
async fn convert_job(mut current_job: CurrentJob, app: AccordeApp) -> Result<(), JobError> {
    let process_data: JobData = current_job.json()?.expect("couldn't parse json");
    let process_id = process_data.process_id;

    JobExecutor::builder(&mut current_job)
        .build()
        .expect("couldn't build JobExecutor")
        .execute(|_| async move {
            match app.processes().convert(process_id).await {
                Ok(_) => Ok(()),
                Err(e) => Err(JobError::from(e)),
            }
        })
        .await?;
    Ok(())
}

pub async fn spawn_convert_job(pool: &sqlx::PgPool, process_id: ProcessId) -> Result<(), JobError> {
    let json = JobData { process_id };
    match convert_job.builder().set_json(&json)?.spawn(pool).await {
        Err(sqlx::Error::Database(err)) if err.message().contains("duplicate key") => Ok(()),
        Err(e) => Err(e.into()),
        Ok(_) => Ok(()),
    }
}
