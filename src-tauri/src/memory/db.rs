use tokio_rusqlite::Connection;

use crate::error::AppError;
use crate::memory::models::ROOT_NODE_UUID;

/// Convert a `tokio_rusqlite::Error` to `AppError` with a context message.
pub fn map_db_err(
    msg: &str,
) -> impl FnOnce(tokio_rusqlite::Error<rusqlite::Error>) -> AppError + '_ {
    move |e| AppError::Internal(format!("{msg}: {e}"))
}

const SCHEMA: &str = r#"
CREATE TABLE IF NOT EXISTS nodes (
    uuid TEXT PRIMARY KEY,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS memories (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    node_uuid TEXT REFERENCES nodes(uuid),
    content TEXT NOT NULL,
    deprecated INTEGER NOT NULL DEFAULT 0,
    migrated_to INTEGER,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS edges (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    parent_uuid TEXT NOT NULL REFERENCES nodes(uuid),
    child_uuid TEXT NOT NULL REFERENCES nodes(uuid),
    name TEXT NOT NULL,
    priority INTEGER NOT NULL DEFAULT 0,
    disclosure TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    UNIQUE(parent_uuid, child_uuid)
);

CREATE TABLE IF NOT EXISTS paths (
    domain TEXT NOT NULL DEFAULT 'core',
    path TEXT NOT NULL,
    edge_id INTEGER REFERENCES edges(id),
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    PRIMARY KEY (domain, path)
);

CREATE TABLE IF NOT EXISTS glossary_keywords (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    keyword TEXT NOT NULL,
    node_uuid TEXT NOT NULL REFERENCES nodes(uuid) ON DELETE CASCADE,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    UNIQUE(keyword, node_uuid)
);

CREATE TABLE IF NOT EXISTS search_documents (
    domain TEXT NOT NULL,
    path TEXT NOT NULL,
    node_uuid TEXT NOT NULL,
    memory_id INTEGER NOT NULL,
    uri TEXT NOT NULL,
    content TEXT NOT NULL,
    disclosure TEXT,
    search_terms TEXT NOT NULL DEFAULT '',
    priority INTEGER NOT NULL DEFAULT 0,
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    PRIMARY KEY (domain, path)
);

CREATE TABLE IF NOT EXISTS conversations (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS messages (
    id TEXT PRIMARY KEY,
    conversation_id TEXT NOT NULL REFERENCES conversations(id) ON DELETE CASCADE,
    role TEXT NOT NULL CHECK(role IN ('user', 'assistant')),
    content TEXT NOT NULL,
    created_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_messages_conversation ON messages(conversation_id);
CREATE INDEX IF NOT EXISTS idx_memories_node ON memories(node_uuid);
CREATE INDEX IF NOT EXISTS idx_edges_parent ON edges(parent_uuid);
CREATE INDEX IF NOT EXISTS idx_edges_child ON edges(child_uuid);
"#;

/// Open the SQLite database and initialize all tables.
///
/// # Arguments
///
/// * `db_path` - Absolute path to the SQLite database file.
///
/// # Returns
///
/// An open `tokio_rusqlite::Connection` with schema created.
///
/// # Errors
///
/// Returns `AppError::Internal` if the database cannot be opened or schema fails.
pub async fn init_db(db_path: &std::path::Path) -> Result<Connection, AppError> {
    let conn = Connection::open(db_path)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to open database: {e}")))?;

    conn.call(|conn| {
        conn.execute_batch("PRAGMA journal_mode=WAL;")?;
        conn.execute_batch("PRAGMA foreign_keys=ON;")?;
        conn.execute_batch(SCHEMA)?;
        Ok(())
    })
    .await
    .map_err(crate::memory::db::map_db_err("Failed to create schema"))?;

    ensure_root_node(&conn).await?;
    init_fts(&conn).await?;

    Ok(conn)
}

/// Ensure the root sentinel node exists.
async fn ensure_root_node(conn: &Connection) -> Result<(), AppError> {
    conn.call(|conn| {
        conn.execute(
            "INSERT OR IGNORE INTO nodes (uuid) VALUES (?1)",
            [ROOT_NODE_UUID],
        )?;
        Ok(())
    })
    .await
    .map_err(crate::memory::db::map_db_err("Failed to create root node"))
}

/// Initialize the FTS5 virtual table.
async fn init_fts(conn: &Connection) -> Result<(), AppError> {
    conn.call(|conn| {
        conn.execute_batch(
            "CREATE VIRTUAL TABLE IF NOT EXISTS search_fts USING fts5(\
                uri, content, disclosure, search_terms, \
                tokenize = 'unicode61'\
            );",
        )?;
        Ok(())
    })
    .await
    .map_err(crate::memory::db::map_db_err("Failed to create FTS table"))
}
