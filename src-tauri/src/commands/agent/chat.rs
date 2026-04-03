use rig::message::Message;
use tauri::ipc::Channel;
use tokio::sync::Mutex;

use crate::agent::provider::build_agent;
use crate::agent::types::{AgentState, ChatMessage, MessageRole, StreamEvent};
use crate::error::AppError;

/// Send a user message and stream the assistant response back.
///
/// The user message is stored immediately. The assistant response is
/// obtained via the rig library and delivered through a Tauri `Channel`.
///
/// # Arguments
///
/// * `conversation_id` - The conversation to append to.
/// * `content` - The user's message text.
/// * `on_event` - A Tauri channel for streaming `StreamEvent` to the frontend.
///
/// # Errors
///
/// Returns `AppError::ConversationNotFound` if the conversation does not exist.
/// Returns `AppError::Provider` if the LLM request fails.
#[tauri::command]
pub async fn send_message(
    state: tauri::State<'_, Mutex<AgentState>>,
    conversation_id: String,
    content: String,
    on_event: Channel<StreamEvent>,
) -> Result<(), AppError> {
    let (settings, history) = {
        let mut state = state.lock().await;
        if !state.conversations.iter().any(|c| c.id == conversation_id) {
            return Err(AppError::ConversationNotFound(conversation_id));
        }

        let now = chrono::Utc::now().to_rfc3339();
        let user_msg = ChatMessage {
            id: uuid::Uuid::new_v4().to_string(),
            conversation_id: conversation_id.clone(),
            role: MessageRole::User,
            content: content.clone(),
            created_at: now,
        };
        state
            .messages
            .entry(conversation_id.clone())
            .or_default()
            .push(user_msg);

        let msgs = state
            .messages
            .get(&conversation_id)
            .cloned()
            .unwrap_or_default();
        let history: Vec<Message> = msgs
            .iter()
            .map(|m| Message::from(m.content.as_str()))
            .collect();

        if let Some(conv) = state
            .conversations
            .iter_mut()
            .find(|c| c.id == conversation_id)
        {
            conv.updated_at = chrono::Utc::now().to_rfc3339();
            if conv.title == "New Chat" && !content.is_empty() {
                let preview: String = content.chars().take(30).collect();
                conv.title = if content.chars().count() > 30 {
                    format!("{preview}...")
                } else {
                    preview
                };
            }
        }

        (state.settings.clone(), history)
    };

    let agent = build_agent(&settings)?;

    let response = match agent.chat(&content, history).await {
        Ok(text) => text,
        Err(e) => {
            let _ = on_event.send(StreamEvent::Error {
                message: e.to_string(),
            });
            return Err(e);
        }
    };

    let now = chrono::Utc::now().to_rfc3339();
    let assistant_msg = ChatMessage {
        id: uuid::Uuid::new_v4().to_string(),
        conversation_id: conversation_id.clone(),
        role: MessageRole::Assistant,
        content: response.clone(),
        created_at: now,
    };

    {
        let mut state = state.lock().await;
        state
            .messages
            .entry(conversation_id)
            .or_default()
            .push(assistant_msg.clone());
    }

    let _ = on_event.send(StreamEvent::Done {
        message: assistant_msg,
    });

    Ok(())
}
