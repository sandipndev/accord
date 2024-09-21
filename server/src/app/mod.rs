mod config;
mod error;

use sqlx::PgPool;

pub use config::*;
pub use error::*;

use crate::process::Processes;

#[derive(Clone)]
pub struct AccordeApp {
    processes: Processes,
}

impl AccordeApp {
    pub(crate) async fn run(pool: PgPool, config: AppConfig) -> Result<Self, ApplicationError> {
        let processes = Processes::new(&pool, config.process);
        Ok(Self { processes })
    }

    pub fn processes(&self) -> &Processes {
        &self.processes
    }
}
