/// Tauri command wrappers for the 7 memory tools.
use tokio::sync::Mutex;

use crate::agent::types::AgentState;
use crate::error::AppError;

/// Read a memory by URI (including system:// special URIs).
#[tauri::command]
pub async fn read_memory(
    state: tauri::State<'_, Mutex<AgentState>>,
    uri: String,
) -> Result<String, AppError> {
    let state = state.lock().await;
    crate::memory::read::read_memory(&state.db, &uri).await
}

/// Create a new memory under a parent URI.
#[tauri::command]
pub async fn create_memory(
    state: tauri::State<'_, Mutex<AgentState>>,
    parent_uri: String,
    content: String,
    priority: i32,
    title: Option<String>,
    disclosure: Option<String>,
) -> Result<String, AppError> {
    let state = state.lock().await;
    crate::memory::write::create_memory(
        &state.db,
        &parent_uri,
        &content,
        priority,
        title.as_deref(),
        disclosure.as_deref(),
    )
    .await
}

/// Update an existing memory's content or metadata.
#[tauri::command]
pub async fn update_memory(
    state: tauri::State<'_, Mutex<AgentState>>,
    uri: String,
    old_string: Option<String>,
    new_string: Option<String>,
    append: Option<String>,
    priority: Option<i32>,
    disclosure: Option<String>,
) -> Result<String, AppError> {
    let state = state.lock().await;
    crate::memory::write::update_memory(
        &state.db,
        &uri,
        old_string.as_deref(),
        new_string.as_deref(),
        append.as_deref(),
        priority,
        disclosure.as_deref(),
    )
    .await
}

/// Delete a memory path.
#[tauri::command]
pub async fn delete_memory(
    state: tauri::State<'_, Mutex<AgentState>>,
    uri: String,
) -> Result<String, AppError> {
    let state = state.lock().await;
    crate::memory::write::delete_memory(&state.db, &uri).await
}

/// Create an alias URI pointing to an existing memory.
#[tauri::command]
pub async fn add_alias(
    state: tauri::State<'_, Mutex<AgentState>>,
    new_uri: String,
    target_uri: String,
    priority: Option<i32>,
    disclosure: Option<String>,
) -> Result<String, AppError> {
    let state = state.lock().await;
    crate::memory::alias::add_alias(
        &state.db,
        &new_uri,
        &target_uri,
        priority.unwrap_or(0),
        disclosure.as_deref(),
    )
    .await
}

/// Add or remove glossary keywords for a memory.
#[tauri::command]
pub async fn manage_triggers(
    state: tauri::State<'_, Mutex<AgentState>>,
    uri: String,
    add: Option<Vec<String>>,
    remove: Option<Vec<String>>,
) -> Result<String, AppError> {
    let state = state.lock().await;
    crate::memory::glossary::manage_triggers(
        &state.db,
        &uri,
        add.unwrap_or_default(),
        remove.unwrap_or_default(),
    )
    .await
}

/// Search memories by query text.
#[tauri::command]
pub async fn search_memory(
    state: tauri::State<'_, Mutex<AgentState>>,
    query: String,
    domain: Option<String>,
    limit: Option<i64>,
) -> Result<String, AppError> {
    let state = state.lock().await;
    crate::memory::search::search_memory(&state.db, &query, domain.as_deref(), limit).await
}
