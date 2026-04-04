/// SQLite-backed storage for conversations and messages.
use tokio_rusqlite::Connection;

use crate::agent::types::{ChatMessage, Conversation, MessageRole};
use crate::error::AppError;

/// Fetch all conversations ordered by most recently updated.
pub async fn get_conversations(conn: &Connection) -> Result<Vec<Conversation>, AppError> {
    conn.call(|conn| {
        let mut stmt = conn.prepare(
            "SELECT id, title, created_at, updated_at FROM conversations ORDER BY updated_at DESC",
        )?;
        let rows = stmt
            .query_map([], |row| {
                Ok(Conversation {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    created_at: row.get(2)?,
                    updated_at: row.get(3)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(rows)
    })
    .await
    .map_err(crate::memory::db::map_db_err("Failed to get conversations"))
}

/// Create a new conversation.
pub async fn create_conversation(conn: &Connection, conv: &Conversation) -> Result<(), AppError> {
    let conv = conv.clone();
    conn.call(move |conn| {
        conn.execute(
            "INSERT INTO conversations (id, title, created_at, updated_at) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![conv.id, conv.title, conv.created_at, conv.updated_at],
        )?;
        Ok(())
    })
    .await
    .map_err(crate::memory::db::map_db_err(
        "Failed to create conversation",
    ))
}

/// Delete a conversation and its messages (cascade).
pub async fn delete_conversation(conn: &Connection, id: &str) -> Result<bool, AppError> {
    let id = id.to_string();
    conn.call(move |conn| {
        let affected = conn.execute("DELETE FROM conversations WHERE id = ?1", [&id])?;
        Ok(affected > 0)
    })
    .await
    .map_err(crate::memory::db::map_db_err(
        "Failed to delete conversation",
    ))
}

/// Update a conversation's title and updated_at timestamp.
pub async fn update_conversation(
    conn: &Connection,
    id: &str,
    title: &str,
    updated_at: &str,
) -> Result<(), AppError> {
    let id = id.to_string();
    let title = title.to_string();
    let updated_at = updated_at.to_string();
    conn.call(move |conn| {
        conn.execute(
            "UPDATE conversations SET title = ?1, updated_at = ?2 WHERE id = ?3",
            rusqlite::params![title, updated_at, id],
        )?;
        Ok(())
    })
    .await
    .map_err(crate::memory::db::map_db_err(
        "Failed to update conversation",
    ))
}

/// Fetch all messages for a conversation in chronological order.
pub async fn get_messages(
    conn: &Connection,
    conversation_id: &str,
) -> Result<Vec<ChatMessage>, AppError> {
    let conversation_id = conversation_id.to_string();
    conn.call(move |conn| {
        let mut stmt = conn.prepare(
            "SELECT id, conversation_id, role, content, created_at \
             FROM messages WHERE conversation_id = ?1 ORDER BY created_at ASC",
        )?;
        let rows = stmt
            .query_map([&conversation_id], |row| {
                let role_str: String = row.get(2)?;
                let role = if role_str == "user" {
                    MessageRole::User
                } else {
                    MessageRole::Assistant
                };
                Ok(ChatMessage {
                    id: row.get(0)?,
                    conversation_id: row.get(1)?,
                    role,
                    content: row.get(3)?,
                    created_at: row.get(4)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(rows)
    })
    .await
    .map_err(crate::memory::db::map_db_err("Failed to get messages"))
}

/// Insert a single message.
pub async fn insert_message(conn: &Connection, msg: &ChatMessage) -> Result<(), AppError> {
    let msg = msg.clone();
    let role_str = match msg.role {
        MessageRole::User => "user",
        MessageRole::Assistant => "assistant",
    };
    let role_str = role_str.to_string();
    conn.call(move |conn| {
        conn.execute(
            "INSERT INTO messages (id, conversation_id, role, content, created_at) \
             VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![
                msg.id,
                msg.conversation_id,
                role_str,
                msg.content,
                msg.created_at
            ],
        )?;
        Ok(())
    })
    .await
    .map_err(crate::memory::db::map_db_err("Failed to insert message"))
}

/// Check whether a conversation exists.
pub async fn conversation_exists(conn: &Connection, id: &str) -> Result<bool, AppError> {
    let id = id.to_string();
    conn.call(move |conn| {
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM conversations WHERE id = ?1",
            [&id],
            |row| row.get(0),
        )?;
        Ok(count > 0)
    })
    .await
    .map_err(crate::memory::db::map_db_err(
        "Failed to check conversation",
    ))
}
