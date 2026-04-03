mod agent;
mod commands;
mod error;

use tauri::Manager;
use tokio::sync::Mutex;

use crate::agent::types::AgentState;
use crate::commands::agent::settings::load_settings_from_disk;

/// Build and launch the Tauri application.
///
/// Registers all plugins, command handlers, and managed state,
/// then starts the event loop.
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let settings = load_settings_from_disk(app.handle());
            let state = AgentState {
                settings,
                ..AgentState::default()
            };
            app.manage(Mutex::new(state));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::greeter::greet,
            commands::agent::settings::get_agent_settings,
            commands::agent::settings::save_agent_settings,
            commands::agent::conversations::get_conversations,
            commands::agent::conversations::create_conversation,
            commands::agent::conversations::delete_conversation,
            commands::agent::conversations::get_messages,
            commands::agent::chat::send_message,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
