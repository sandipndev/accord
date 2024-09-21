#[derive(Clone, Default, Debug, serde::Deserialize, serde::Serialize)]
pub struct ProcessConfig {
    pub home_absolute_path: String,
}
