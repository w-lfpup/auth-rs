use serde::{Deserialize, Serialize};
use serde_json;
use std::env;
use std::path;
use std::path::{Path, PathBuf};
use tokio::fs;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Config {
    pub host_and_port: String,
    pub sqlite_auth_db_filepath: PathBuf,
}

impl Config {
    pub async fn try_from(source_path: &PathBuf) -> Result<Config, String> {
        // see if config exists
        let config_json = match fs::read_to_string(source_path).await {
            Ok(r) => r,
            Err(e) => return Err(e.to_string()),
        };

        let mut config: Config = match serde_json::from_str(&config_json) {
            Ok(j) => j,
            Err(e) => return Err(e.to_string()),
        };

        // get target directory
        let config_path = match path::absolute(&source_path) {
            Ok(pb) => pb,
            Err(e) => return Err(e.to_string()),
        };

        let parent_dir = match config_path.parent() {
            Some(p) => p,
            _ => {
                return Err("parent directory of config not found".to_string());
            }
        };

        // get target directory relative to config path
        let target_filepath = parent_dir.join(config.sqlite_auth_db_filepath);
        let target_filepath_abs = match path::absolute(target_filepath) {
            Ok(pb) => pb,
            Err(e) => return Err(e.to_string()),
        };

        config.sqlite_auth_db_filepath = target_filepath_abs;

        Ok(config)
    }
}
