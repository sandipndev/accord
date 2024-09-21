use serde::{Deserialize, Serialize};

use crate::process::ProcessConfig;

#[derive(Clone, Default, Debug, Deserialize, Serialize)]
pub struct AppConfig {
    pub process: ProcessConfig,
}
