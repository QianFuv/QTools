use std::fs;
use std::path::Path;

use tokio::sync::Mutex;

use crate::agent::types::{AgentSettings, AgentState};
use crate::data_dir::DataDir;
use crate::error::AppError;

/// Return the current agent settings.
///
/// # Errors
///
/// Returns `AppError::Internal` if the state lock is poisoned.
#[tauri::command]
pub async fn get_agent_settings(
    state: tauri::State<'_, Mutex<AgentState>>,
) -> Result<AgentSettings, AppError> {
    let state = state.lock().await;
    Ok(state.settings.clone())
}

/// Persist updated agent settings to disk and update in-memory state.
///
/// # Arguments
///
/// * `settings` - The new settings to save.
///
/// # Errors
///
/// Returns `AppError::Settings` if the file cannot be written.
#[tauri::command]
pub async fn save_agent_settings(
    data_dir: tauri::State<'_, DataDir>,
    state: tauri::State<'_, Mutex<AgentState>>,
    settings: AgentSettings,
) -> Result<(), AppError> {
    let dir = &data_dir.0;
    fs::create_dir_all(dir).map_err(|e| AppError::Settings(e.to_string()))?;

    let path = dir.join("agent-settings.json");
    let json =
        serde_json::to_string_pretty(&settings).map_err(|e| AppError::Settings(e.to_string()))?;
    fs::write(&path, json).map_err(|e| AppError::Settings(e.to_string()))?;

    let mut state = state.lock().await;
    state.settings = settings;
    Ok(())
}

/// Load settings from disk, falling back to defaults if the file is
/// missing or corrupt.
///
/// # Arguments
///
/// * `dir` - The data directory containing `agent-settings.json`.
///
/// # Returns
///
/// The loaded or default `AgentSettings`.
pub fn load_settings_from_disk(dir: &Path) -> AgentSettings {
    let path = dir.join("agent-settings.json");
    let Ok(data) = fs::read_to_string(&path) else {
        return AgentSettings::default();
    };
    serde_json::from_str(&data).unwrap_or_default()
}
