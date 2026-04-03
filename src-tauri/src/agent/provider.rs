use rig::client::CompletionClient;
use rig::completion::Chat;
use rig::message::Message;
use rig::providers::{anthropic, openai};

use crate::agent::types::{AgentSettings, ApiFormat};
use crate::error::AppError;

/// Wraps provider-specific rig agent types behind a single enum.
///
/// This enum dispatch pattern is necessary because rig's `Agent<M>`
/// is generic over the model type, making it non-object-safe.
///
/// - `OpenAiChat` uses `CompletionsClient` (Chat Completions API),
///   compatible with third-party OpenAI-compatible services.
/// - `OpenAiResponses` uses the default `Client` (Responses API).
/// - `Anthropic` uses the Anthropic provider.
pub enum AgentKind {
    OpenAiChat(rig::agent::Agent<openai::completion::CompletionModel>),
    OpenAiResponses(rig::agent::Agent<openai::responses_api::ResponsesCompletionModel>),
    Anthropic(rig::agent::Agent<anthropic::completion::CompletionModel>),
}

impl AgentKind {
    /// Send a chat message with conversation history and return the response.
    ///
    /// # Arguments
    ///
    /// * `content` - The new user message text.
    /// * `history` - Previous messages in the conversation.
    ///
    /// # Returns
    ///
    /// The assistant's response text.
    ///
    /// # Errors
    ///
    /// Returns `AppError::Provider` if the LLM request fails.
    pub async fn chat(&self, content: &str, history: Vec<Message>) -> Result<String, AppError> {
        let msg: Message = content.into();
        match self {
            AgentKind::OpenAiChat(agent) => Chat::chat(agent, msg, history)
                .await
                .map_err(|e| AppError::Provider(e.to_string())),
            AgentKind::OpenAiResponses(agent) => Chat::chat(agent, msg, history)
                .await
                .map_err(|e| AppError::Provider(e.to_string())),
            AgentKind::Anthropic(agent) => Chat::chat(agent, msg, history)
                .await
                .map_err(|e| AppError::Provider(e.to_string())),
        }
    }
}

/// Build a rig agent from the current settings.
///
/// # Arguments
///
/// * `settings` - The agent configuration including provider, model, and credentials.
///
/// # Returns
///
/// An `AgentKind` wrapping the provider-specific agent.
///
/// # Errors
///
/// Returns `AppError::Provider` if client construction fails.
pub fn build_agent(settings: &AgentSettings) -> Result<AgentKind, AppError> {
    match settings.api_format {
        ApiFormat::OpenAiChat => {
            let client = openai::CompletionsClient::builder()
                .api_key(&settings.api_key)
                .base_url(&settings.base_url)
                .build()
                .map_err(|e| AppError::Provider(e.to_string()))?;
            let agent = client
                .agent(&settings.model)
                .preamble(&settings.system_prompt)
                .build();
            Ok(AgentKind::OpenAiChat(agent))
        }
        ApiFormat::OpenAiResponses => {
            let client = openai::Client::builder()
                .api_key(&settings.api_key)
                .base_url(&settings.base_url)
                .build()
                .map_err(|e| AppError::Provider(e.to_string()))?;
            let agent = client
                .agent(&settings.model)
                .preamble(&settings.system_prompt)
                .build();
            Ok(AgentKind::OpenAiResponses(agent))
        }
        ApiFormat::Anthropic => {
            let client = anthropic::Client::builder()
                .api_key(&settings.api_key)
                .base_url(&settings.base_url)
                .build()
                .map_err(|e| AppError::Provider(e.to_string()))?;
            let agent = client
                .agent(&settings.model)
                .preamble(&settings.system_prompt)
                .build();
            Ok(AgentKind::Anthropic(agent))
        }
    }
}
