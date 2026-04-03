/// Application-wide error type for Tauri command handlers.
///
/// Implements `serde::Serialize` so that Tauri can send structured
/// error messages back to the frontend over IPC.
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    /// A catch-all variant for unexpected internal failures.
    #[error("{0}")]
    Internal(String),
}

impl serde::Serialize for AppError {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}
