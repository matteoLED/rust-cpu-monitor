use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub refresh_interval: u64,
    pub alert_thresholds: AlertThresholds,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AlertThresholds {
    pub cpu_usage: f32,
    pub memory_usage: u64,
}

impl Config {
    pub fn load_from_file(file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let config_data = fs::read_to_string(file_path)?;
        let config: Config = toml::from_str(&config_data)?;
        Ok(config)
    }
}
