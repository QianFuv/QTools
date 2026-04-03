use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Supported LLM API formats.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ApiFormat {
    #[serde(rename = "openai_chat")]
    OpenAiChat,
    #[serde(rename = "openai_responses")]
    OpenAiResponses,
    #[serde(rename = "anthropic")]
    Anthropic,
}

/// Persistent configuration for the agent LLM provider.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentSettings {
    pub base_url: String,
    pub api_key: String,
    pub model: String,
    pub api_format: ApiFormat,
    pub system_prompt: String,
}

impl Default for AgentSettings {
    fn default() -> Self {
        Self {
            base_url: "https://api.openai.com/v1".to_string(),
            api_key: String::new(),
            model: "gpt-4o".to_string(),
            api_format: ApiFormat::OpenAiChat,
            system_prompt: "You are a helpful assistant.".to_string(),
        }
    }
}

/// A single conversation session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    pub id: String,
    pub title: String,
    pub created_at: String,
    pub updated_at: String,
}

/// The role of a chat message sender.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MessageRole {
    User,
    Assistant,
}

/// A single message within a conversation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub id: String,
    pub conversation_id: String,
    pub role: MessageRole,
    pub content: String,
    pub created_at: String,
}

/// Events streamed from the backend to the frontend during chat.
///
/// `Delta` is reserved for future streaming support.
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase", tag = "event", content = "data")]
pub enum StreamEvent {
    #[allow(dead_code)]
    Delta {
        content: String,
    },
    Done {
        message: ChatMessage,
    },
    Error {
        message: String,
    },
}

/// In-memory state for the agent subsystem.
///
/// Designed so that `conversations` and `messages` can later be
/// replaced by a sqlite-backed store without changing command signatures.
#[derive(Default)]
pub struct AgentState {
    pub conversations: Vec<Conversation>,
    pub messages: HashMap<String, Vec<ChatMessage>>,
    pub settings: AgentSettings,
}
