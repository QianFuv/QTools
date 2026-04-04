use tokio::sync::Mutex;

use crate::agent::types::{AgentState, ChatMessage, Conversation};
use crate::error::AppError;
use crate::memory::chat_store;

/// Return all conversations, newest first.
#[tauri::command]
pub async fn get_conversations(
    state: tauri::State<'_, Mutex<AgentState>>,
) -> Result<Vec<Conversation>, AppError> {
    let state = state.lock().await;
    chat_store::get_conversations(&state.db).await
}

/// Create a new empty conversation and return it.
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
    let state = state.lock().await;
    chat_store::create_conversation(&state.db, &conv).await?;
    Ok(conv)
}

/// Delete a conversation and its messages.
///
/// # Arguments
///
/// * `conversation_id` - The UUID of the conversation to delete.
#[tauri::command]
pub async fn delete_conversation(
    state: tauri::State<'_, Mutex<AgentState>>,
    conversation_id: String,
) -> Result<(), AppError> {
    let state = state.lock().await;
    let deleted = chat_store::delete_conversation(&state.db, &conversation_id).await?;
    if !deleted {
        return Err(AppError::ConversationNotFound(conversation_id));
    }
    Ok(())
}

/// Return all messages for a given conversation.
///
/// # Arguments
///
/// * `conversation_id` - The UUID of the conversation.
#[tauri::command]
pub async fn get_messages(
    state: tauri::State<'_, Mutex<AgentState>>,
    conversation_id: String,
) -> Result<Vec<ChatMessage>, AppError> {
    let state = state.lock().await;
    if !chat_store::conversation_exists(&state.db, &conversation_id).await? {
        return Err(AppError::ConversationNotFound(conversation_id));
    }
    chat_store::get_messages(&state.db, &conversation_id).await
}
