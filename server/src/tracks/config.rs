use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TracksConfig {
    pub home_absolute_path: String,
    #[serde(default = "shift_min")]
    pub shift_min: i32,
    #[serde(default = "shift_max")]
    pub shift_max: i32,
}

impl Default for TracksConfig {
    fn default() -> Self {
        Self {
            home_absolute_path: "".to_string(),
            shift_min: shift_min(),
            shift_max: shift_max(),
        }
    }
}

fn shift_min() -> i32 {
    -6
}
fn shift_max() -> i32 {
    4
}
