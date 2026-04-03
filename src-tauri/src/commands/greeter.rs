use crate::error::AppError;

/// Generate a greeting message for the given name.
///
/// # Arguments
///
/// * `name` - The name to greet.
///
/// # Returns
///
/// A formatted greeting string.
///
/// # Errors
///
/// Returns `AppError::Internal` if the name is empty.
#[tauri::command]
pub fn greet(name: &str) -> Result<String, AppError> {
    if name.is_empty() {
        return Err(AppError::Internal("Name cannot be empty".to_string()));
    }
    Ok(format!("Hello, {}! You've been greeted from Rust!", name))
}
