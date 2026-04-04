mod agent;
mod commands;
mod error;
mod memory;

use tauri::Manager;
use tokio::sync::Mutex;

use crate::agent::types::AgentState;
use crate::commands::agent::settings::load_settings_from_disk;
use crate::memory::db::init_db;

/// Build and launch the Tauri application.
///
/// Registers all plugins, command handlers, and managed state,
/// then starts the event loop.
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let handle = app.handle().clone();
            let settings = load_settings_from_disk(app.handle());

            tauri::async_runtime::block_on(async move {
                let data_dir = handle
                    .path()
                    .app_data_dir()
                    .expect("failed to resolve app data dir");
                std::fs::create_dir_all(&data_dir).ok();
                let db_path = data_dir.join("qtools.db");

                let db = init_db(&db_path)
                    .await
                    .expect("failed to initialize database");

                let state = AgentState { db, settings };
                handle.manage(Mutex::new(state));
            });

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
            commands::memory::tools::read_memory,
            commands::memory::tools::create_memory,
            commands::memory::tools::update_memory,
            commands::memory::tools::delete_memory,
            commands::memory::tools::add_alias,
            commands::memory::tools::manage_triggers,
            commands::memory::tools::search_memory,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
