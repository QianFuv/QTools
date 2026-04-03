use tokio::sync::Mutex;

use crate::agent::types::{AgentState, ChatMessage, Conversation};
use crate::error::AppError;

/// Return all conversations, newest first.
///
/// # Errors
///
/// Returns `AppError::Internal` if the state lock is poisoned.
#[tauri::command]
pub async fn get_conversations(
    state: tauri::State<'_, Mutex<AgentState>>,
) -> Result<Vec<Conversation>, AppError> {
    let state = state.lock().await;
    Ok(state.conversations.clone())
}

/// Create a new empty conversation and return it.
///
/// # Returns
///
/// The newly created `Conversation`.
///
/// # Errors
///
/// Returns `AppError::Internal` if the state lock is poisoned.
#[tauri::command]
pub async fn create_conversation(
    state: tauri::State<'_, Mutex<AgentState>>,
) -> Result<Conversation, AppError> {
    let now = chrono::Utc::now().to_rfc3339();
    let conv = Conversation {
        id: uuid::Uuid::new_v4().to_string(),
        title: "New Chat".to_string(),
        created_at: now.clone(),
        updated_at: now,
    };
    let mut state = state.lock().await;
    state.conversations.insert(0, conv.clone());
    Ok(conv)
}

/// Delete a conversation and its messages.
///
/// # Arguments
///
/// * `conversation_id` - The UUID of the conversation to delete.
///
/// # Errors
///
/// Returns `AppError::ConversationNotFound` if the ID does not exist.
#[tauri::command]
pub async fn delete_conversation(
    state: tauri::State<'_, Mutex<AgentState>>,
    conversation_id: String,
) -> Result<(), AppError> {
    let mut state = state.lock().await;
    let before = state.conversations.len();
    state.conversations.retain(|c| c.id != conversation_id);
    if state.conversations.len() == before {
        return Err(AppError::ConversationNotFound(conversation_id));
    }
    state.messages.remove(&conversation_id);
    Ok(())
}

/// Return all messages for a given conversation.
///
/// # Arguments
///
/// * `conversation_id` - The UUID of the conversation.
///
/// # Returns
///
/// A chronologically ordered list of `ChatMessage`.
///
/// # Errors
///
/// Returns `AppError::ConversationNotFound` if the conversation does not exist.
#[tauri::command]
pub async fn get_messages(
    state: tauri::State<'_, Mutex<AgentState>>,
    conversation_id: String,
) -> Result<Vec<ChatMessage>, AppError> {
    let state = state.lock().await;
    if !state.conversations.iter().any(|c| c.id == conversation_id) {
        return Err(AppError::ConversationNotFound(conversation_id));
    }
    Ok(state
        .messages
        .get(&conversation_id)
        .cloned()
        .unwrap_or_default())
}
