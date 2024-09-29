mod config;
mod error;

use sqlx::PgPool;

pub use config::*;
pub use error::*;

use crate::tracks::Tracks;

#[derive(Clone)]
pub struct AccordeApp {
    tracks: Tracks,
}

impl AccordeApp {
    pub(crate) async fn run(pool: PgPool, config: AppConfig) -> Result<Self, ApplicationError> {
        let tracks = Tracks::new(&pool, config.tracks);
        Ok(Self { tracks })
    }

    pub fn tracks(&self) -> &Tracks {
        &self.tracks
    }
}
