use serde::{Deserialize, Serialize};
use serde_json;
use std::env;
use std::path;
use std::path::{Path, PathBuf};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SqliteSettings {
    db_path: PathBuf,
    max_connections: u64,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AuthSettingsSettings {
    origin_timestamp_ms: u64,
    logical_volume_base: u64,
    logical_volume_length: u64,
    signup_session_length_ms: u64,
    session_length_ms: u64,
    public_session_length_ms: u64,
    public_session_stale_length_ms: u64,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Config {
    sqlite: SqliteSettings,
    auth: AuthSettingsSettings,
}

impl Config {
    pub async fn try_from(config_path: &Path, config_str: &str) -> Result<Config, String> {
        let mut config: Config = match serde_json::from_str(&config_str) {
            Ok(j) => j,
            Err(e) => return Err(e.to_string()),
        };

        // get target directory
        let config_abs_path = match path::absolute(&config_path) {
            Ok(pb) => pb,
            Err(e) => return Err(e.to_string()),
        };

        let parent_dir = match config_abs_path.parent() {
            Some(p) => p,
            _ => {
                return Err("parent directory of config not found".to_string());
            }
        };

        // get target directory relative to config path
        let target_filepath = parent_dir.join(config.sqlite.db_path);
        let target_filepath_abs = match path::absolute(target_filepath) {
            Ok(pb) => pb,
            Err(e) => return Err(e.to_string()),
        };

        config.sqlite.db_path = target_filepath_abs;

        Ok(config)
    }
}
