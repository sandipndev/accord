mod config;
mod error;

use sqlx::PgPool;

pub use config::*;
pub use error::*;

use crate::primitives::ProcessID;

#[derive(Clone)]
pub struct AccordeApp {
    pool: PgPool,
}

impl AccordeApp {
    pub(crate) async fn run(pool: PgPool, _config: AppConfig) -> Result<Self, ApplicationError> {
        Ok(Self { pool })
    }

    pub async fn accode(&self, youtube_url: String) -> Result<ProcessID, ApplicationError> {
        Ok(ProcessID::new())
    }
}
