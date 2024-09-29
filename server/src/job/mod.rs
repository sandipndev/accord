mod error;
mod executor;

pub use error::JobError;
use executor::{JobExecutionError, JobExecutor};

use sqlxmq::{job, CurrentJob, JobRegistry, JobRunnerHandle};

use crate::{app::AccordeApp, primitives::SemitoneId};

pub async fn start_job_runner(
    pool: &sqlx::PgPool,
    app: AccordeApp,
) -> Result<JobRunnerHandle, JobError> {
    let mut registry = JobRegistry::new(&[semitone_conversion_job]);

    registry.set_context(app);

    Ok(registry
        .runner(pool)
        .set_keep_alive(false)
        .set_concurrency(1, 4)
        .run()
        .await?)
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct JobData {
    pub semitone_id: SemitoneId,
}

#[job(name = "semitone_conversion")]
async fn semitone_conversion_job(
    mut current_job: CurrentJob,
    app: AccordeApp,
) -> Result<(), JobError> {
    let process_data: JobData = current_job.json()?.expect("couldn't parse json");
    let semitone_id = process_data.semitone_id;

    JobExecutor::builder(&mut current_job)
        .build()
        .expect("couldn't build JobExecutor")
        .execute(|_| async move {
            match app.tracks().semitones().convert(semitone_id).await {
                Ok(_) => Ok(()),
                Err(e) => Err(JobError::from(e)),
            }
        })
        .await?;
    Ok(())
}

pub async fn spawn_semitone_conversion_job(
    pool: &sqlx::PgPool,
    semitone_id: SemitoneId,
) -> Result<(), JobError> {
    let json = JobData { semitone_id };
    match semitone_conversion_job
        .builder()
        .set_json(&json)?
        .spawn(pool)
        .await
    {
        Err(sqlx::Error::Database(err)) if err.message().contains("duplicate key") => Ok(()),
        Err(e) => Err(e.into()),
        Ok(_) => Ok(()),
    }
}
