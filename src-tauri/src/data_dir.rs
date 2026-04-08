use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

const CONFIG_FILE: &str = "data-path.json";
const DEFAULT_DATA_SUBDIR: &str = "data";

/// Persistent configuration that records the user-chosen data directory.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataDirConfig {
    pub data_dir: String,
}

/// Wrapper around the resolved data directory path, stored as Tauri managed state.
pub struct DataDir(pub PathBuf);

/// Resolve the data directory for the application.
///
/// Reads `data-path.json` next to the running executable.  If the file is
/// missing or cannot be parsed, the default `{exe_dir}/data` is used and
/// the config file is created for future runs.
///
/// The directory is created if it does not already exist.
///
/// # Returns
///
/// The absolute path to the data directory.
///
/// # Panics
///
/// Panics if the executable path cannot be determined.
pub fn resolve_data_dir() -> PathBuf {
    let exe_dir = std::env::current_exe()
        .expect("failed to determine executable path")
        .parent()
        .expect("executable has no parent directory")
        .to_path_buf();

    let config_path = exe_dir.join(CONFIG_FILE);

    let data_dir = if config_path.exists() {
        std::fs::read_to_string(&config_path)
            .ok()
            .and_then(|s| serde_json::from_str::<DataDirConfig>(&s).ok())
            .map(|cfg| resolve_relative(&exe_dir, &cfg.data_dir))
            .unwrap_or_else(|| exe_dir.join(DEFAULT_DATA_SUBDIR))
    } else {
        let default = exe_dir.join(DEFAULT_DATA_SUBDIR);
        let cfg = DataDirConfig {
            data_dir: format!("./{DEFAULT_DATA_SUBDIR}"),
        };
        if let Ok(json) = serde_json::to_string_pretty(&cfg) {
            let _ = std::fs::write(&config_path, json);
        }
        default
    };

    std::fs::create_dir_all(&data_dir).ok();
    data_dir
}

/// Write a new data directory path to `data-path.json`.
///
/// # Arguments
///
/// * `new_path` - The new data directory path (absolute or relative to exe dir).
///
/// # Errors
///
/// Returns an error message if the config file cannot be written.
pub fn write_data_dir_config(new_path: &str) -> Result<(), String> {
    let exe_dir = std::env::current_exe()
        .map_err(|e| format!("failed to determine executable path: {e}"))?
        .parent()
        .ok_or("executable has no parent directory")?
        .to_path_buf();

    let config_path = exe_dir.join(CONFIG_FILE);
    let cfg = DataDirConfig {
        data_dir: new_path.to_string(),
    };
    let json = serde_json::to_string_pretty(&cfg)
        .map_err(|e| format!("failed to serialize config: {e}"))?;
    std::fs::write(&config_path, json).map_err(|e| format!("failed to write config: {e}"))
}

/// Resolve a potentially relative path against a base directory.
fn resolve_relative(base: &Path, path_str: &str) -> PathBuf {
    let p = PathBuf::from(path_str);
    if p.is_absolute() {
        p
    } else {
        base.join(p)
    }
}

/// List the known data file names that should be migrated.
pub const DATA_FILES: &[&str] = &["qtools.db", "agent-settings.json", "canvas-settings.json"];
