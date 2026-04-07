/// Application-wide error type for Tauri command handlers.
///
/// Implements `serde::Serialize` so that Tauri can send structured
/// error messages back to the frontend over IPC.
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    /// A catch-all variant for unexpected internal failures.
    #[error("{0}")]
    Internal(String),

    /// An error originating from the LLM provider or rig library.
    #[error("Provider error: {0}")]
    Provider(String),

    /// An error related to agent settings persistence.
    #[error("Settings error: {0}")]
    Settings(String),

    /// The requested conversation was not found.
    #[error("Conversation not found: {0}")]
    ConversationNotFound(String),

    /// An error originating from the Canvas LMS integration.
    #[error("Canvas error: {0}")]
    Canvas(String),
}

impl serde::Serialize for AppError {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

impl From<tokio_rusqlite::Error<rusqlite::Error>> for AppError {
    fn from(e: tokio_rusqlite::Error<rusqlite::Error>) -> Self {
        AppError::Internal(e.to_string())
    }
}
