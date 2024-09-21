mod config;
mod error;

use sqlx::PgPool;

pub use config::*;
pub use error::*;

#[derive(Clone)]
pub struct AccordeApp {
    pool: PgPool,
}

impl AccordeApp {
    pub(crate) async fn run(pool: PgPool, _config: AppConfig) -> Result<Self, ApplicationError> {
        Ok(Self { pool })
    }
}
