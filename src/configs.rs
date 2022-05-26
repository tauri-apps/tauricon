use std::{fs::File, path};

use serde::{Deserialize, Serialize};
use thiserror::Error as AsError;

#[derive(Debug, AsError)]
pub enum ConfigError {
    #[error("Failed to read config file")]
    Io(#[from] std::io::Error),
    #[error("Failed to parse config file")]
    Json(#[from] serde_json::Error),
    #[error("Config file not found")]
    FileNotFound(path::PathBuf),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TauriBundle {
    pub icon: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TauriConfig {
    pub bundle: TauriBundle,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TauriJson {
    pub tauri: TauriConfig,
}

pub fn get_config() -> Result<TauriJson, ConfigError> {
    let cwd = std::env::current_dir()?;
    let config_path = cwd.join("src-tauri").join("tauri.conf.json");

    if !config_path.exists() {
        return Err(ConfigError::FileNotFound(config_path));
    }

    let file = File::open(config_path)?;

    serde_json::from_reader(file).map_err(ConfigError::from)
}
