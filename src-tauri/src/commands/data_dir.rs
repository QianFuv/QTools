use std::path::PathBuf;

use crate::data_dir::{DataDir, DATA_FILES};
use crate::error::AppError;

/// Return the current data directory as an absolute path string.
///
/// # Errors
///
/// Returns `AppError::Internal` if the state cannot be read.
#[tauri::command]
pub async fn get_data_dir(data_dir: tauri::State<'_, DataDir>) -> Result<String, AppError> {
    Ok(data_dir.0.to_string_lossy().to_string())
}

/// Persist a new data directory path to `data-path.json`.
///
/// The change takes effect after the application is restarted.
///
/// # Arguments
///
/// * `path` - The new data directory (absolute or relative to the executable).
///
/// # Errors
///
/// Returns `AppError::Settings` if the path is empty or the config cannot be
/// written.
#[tauri::command]
pub async fn set_data_dir(path: String) -> Result<(), AppError> {
    if path.trim().is_empty() {
        return Err(AppError::Settings(
            "data directory path cannot be empty".to_string(),
        ));
    }
    crate::data_dir::write_data_dir_config(&path).map_err(AppError::Settings)
}

/// Copy data files from one directory to another.
///
/// Copies `qtools.db`, `agent-settings.json`, and `canvas-settings.json`
/// when they exist in the source directory.  If `delete_source` is `true`,
/// the copied files are removed from the source after a successful copy.
///
/// # Arguments
///
/// * `source` - Absolute path to the source data directory.
/// * `dest` - Absolute path to the destination data directory.
/// * `delete_source` - Whether to delete source files after copying.
///
/// # Errors
///
/// Returns `AppError::Internal` if any file operation fails.
#[tauri::command]
pub async fn migrate_data(
    source: String,
    dest: String,
    delete_source: bool,
) -> Result<(), AppError> {
    let src = PathBuf::from(&source);
    let dst = PathBuf::from(&dest);

    if !src.is_dir() {
        return Err(AppError::Internal(format!(
            "source directory does not exist: {source}"
        )));
    }

    std::fs::create_dir_all(&dst)
        .map_err(|e| AppError::Internal(format!("failed to create destination directory: {e}")))?;

    for name in DATA_FILES {
        let from = src.join(name);
        if from.exists() {
            let to = dst.join(name);
            std::fs::copy(&from, &to)
                .map_err(|e| AppError::Internal(format!("failed to copy {name}: {e}")))?;
        }
    }

    if delete_source {
        for name in DATA_FILES {
            let from = src.join(name);
            if from.exists() {
                std::fs::remove_file(&from)
                    .map_err(|e| AppError::Internal(format!("failed to delete {name}: {e}")))?;
            }
        }
    }

    Ok(())
}
