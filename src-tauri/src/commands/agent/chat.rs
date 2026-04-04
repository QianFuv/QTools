use rig::message::Message;
use tauri::ipc::Channel;
use tokio::sync::Mutex;

use crate::agent::provider::build_agent;
use crate::agent::types::{AgentState, ChatMessage, MessageRole, StreamEvent};
use crate::error::AppError;
use crate::memory::chat_store;
use crate::memory::tools::new_tool_call_log;

/// Send a user message and stream the assistant response back.
///
/// # Arguments
///
/// * `conversation_id` - The conversation to append to.
/// * `content` - The user's message text.
/// * `on_event` - A Tauri channel for streaming `StreamEvent` to the frontend.
#[tauri::command]
pub async fn send_message(
    state: tauri::State<'_, Mutex<AgentState>>,
    conversation_id: String,
    content: String,
    on_event: Channel<StreamEvent>,
) -> Result<(), AppError> {
    let (settings, db, history) = {
        let state = state.lock().await;
        if !chat_store::conversation_exists(&state.db, &conversation_id).await? {
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
        chat_store::insert_message(&state.db, &user_msg).await?;

        let msgs = chat_store::get_messages(&state.db, &conversation_id).await?;
        let history: Vec<Message> = msgs
            .iter()
            .map(|m| Message::from(m.content.as_str()))
            .collect();

        let preview: String = content.chars().take(30).collect();
        let title = if content.chars().count() > 30 {
            format!("{preview}...")
        } else {
            preview
        };
        chat_store::update_conversation(
            &state.db,
            &conversation_id,
            &title,
            &chrono::Utc::now().to_rfc3339(),
        )
        .await
        .ok();

        (state.settings.clone(), state.db.clone(), history)
    };

    let tool_log = new_tool_call_log();
    let agent = build_agent(&settings, &db, &tool_log)?;

    let response = match agent.chat(&content, history).await {
        Ok(text) => text,
        Err(e) => {
            let _ = on_event.send(StreamEvent::Error {
                message: e.to_string(),
            });
            return Err(e);
        }
    };

    if let Ok(calls) = tool_log.lock() {
        for call in calls.iter() {
            let _ = on_event.send(StreamEvent::ToolCall {
                name: call.name.clone(),
                args: call.args.clone(),
                result: call.result.clone(),
            });
        }
    }

    let now = chrono::Utc::now().to_rfc3339();
    let assistant_msg = ChatMessage {
        id: uuid::Uuid::new_v4().to_string(),
        conversation_id: conversation_id.clone(),
        role: MessageRole::Assistant,
        content: response,
        created_at: now,
    };

    {
        let state = state.lock().await;
        chat_store::insert_message(&state.db, &assistant_msg).await?;
    }

    let _ = on_event.send(StreamEvent::Done {
        message: assistant_msg,
    });

    Ok(())
}
