use figment::{
    Figment,
    providers::{Env, Serialized},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub default_model: String,
    pub poll_interval_secs: u64,
    pub poll_timeout_secs: u64,
    pub output_dir: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            default_model: "chirp-fenix".into(),
            poll_interval_secs: 5,
            poll_timeout_secs: 600,
            output_dir: ".".into(),
        }
    }
}

impl AppConfig {
    pub fn load() -> Self {
        Figment::new()
            .merge(Serialized::defaults(AppConfig::default()))
            .merge(Env::prefixed("SUNO_").split("_"))
            .extract()
            .unwrap_or_default()
    }
}
