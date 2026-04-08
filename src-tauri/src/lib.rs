mod agent;
mod commands;
mod data_dir;
mod error;
mod memory;

use tauri::Manager;
use tokio::sync::Mutex;

use crate::agent::types::AgentState;
use crate::commands::agent::settings::load_settings_from_disk;
use crate::commands::canvas::settings::load_canvas_settings;
use crate::data_dir::{resolve_data_dir, DataDir};
use crate::memory::db::init_db;

/// Build and launch the Tauri application.
///
/// Registers all plugins, command handlers, and managed state,
/// then starts the event loop.
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let handle = app.handle().clone();
            let data_dir = resolve_data_dir();
            let settings = load_settings_from_disk(&data_dir);
            let canvas_settings = load_canvas_settings(&data_dir);

            handle.manage(DataDir(data_dir.clone()));

            tauri::async_runtime::block_on(async move {
                let db_path = data_dir.join("qtools.db");

                let db = init_db(&db_path)
                    .await
                    .expect("failed to initialize database");

                let state = AgentState { db, settings };
                handle.manage(Mutex::new(state));

                let canvas_state = crate::commands::canvas::types::CanvasState {
                    settings: canvas_settings,
                    cache: None,
                };
                handle.manage(Mutex::new(canvas_state));
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
            commands::canvas::settings::get_canvas_settings,
            commands::canvas::settings::save_canvas_settings,
            commands::canvas::commands::fetch_canvas_data,
            commands::canvas::commands::refresh_canvas_data,
            commands::canvas::commands::set_task_completion,
            commands::canvas::commands::get_completed_tasks,
            commands::data_dir::get_data_dir,
            commands::data_dir::set_data_dir,
            commands::data_dir::migrate_data,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
