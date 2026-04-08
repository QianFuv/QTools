use std::fs;
use std::path::Path;

use tokio::sync::Mutex;

use crate::data_dir::DataDir;
use crate::error::AppError;

use super::types::{CanvasSettings, CanvasState};

const SETTINGS_FILE: &str = "canvas-settings.json";

/// Return the current Canvas settings.
///
/// # Errors
///
/// Returns `AppError::Canvas` if the state lock is poisoned.
#[tauri::command]
pub async fn get_canvas_settings(
    state: tauri::State<'_, Mutex<CanvasState>>,
) -> Result<CanvasSettings, AppError> {
    let state = state.lock().await;
    Ok(state.settings.clone())
}

/// Persist updated Canvas settings to disk, update in-memory state,
/// and invalidate the cache.
///
/// # Arguments
///
/// * `settings` - The new settings to save.
///
/// # Errors
///
/// Returns `AppError::Canvas` if the file cannot be written.
#[tauri::command]
pub async fn save_canvas_settings(
    data_dir: tauri::State<'_, DataDir>,
    state: tauri::State<'_, Mutex<CanvasState>>,
    settings: CanvasSettings,
) -> Result<(), AppError> {
    let dir = &data_dir.0;
    fs::create_dir_all(dir).map_err(|e| AppError::Canvas(e.to_string()))?;

    let path = dir.join(SETTINGS_FILE);
    let json =
        serde_json::to_string_pretty(&settings).map_err(|e| AppError::Canvas(e.to_string()))?;
    fs::write(&path, json).map_err(|e| AppError::Canvas(e.to_string()))?;

    let mut state = state.lock().await;
    state.settings = settings;
    state.cache = None;
    Ok(())
}

/// Load Canvas settings from disk, falling back to defaults if the
/// file is missing or corrupt.
///
/// # Arguments
///
/// * `dir` - The data directory containing `canvas-settings.json`.
///
/// # Returns
///
/// The loaded or default `CanvasSettings`.
pub fn load_canvas_settings(dir: &Path) -> CanvasSettings {
    let path = dir.join(SETTINGS_FILE);
    let Ok(data) = fs::read_to_string(&path) else {
        return CanvasSettings::default();
    };
    serde_json::from_str(&data).unwrap_or_default()
}
