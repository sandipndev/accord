use serde::{Deserialize, Serialize};

use crate::tracks::TracksConfig;

#[derive(Clone, Default, Debug, Deserialize, Serialize)]
pub struct AppConfig {
    pub tracks: TracksConfig,
}
