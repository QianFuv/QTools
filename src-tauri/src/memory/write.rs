/// Memory write operations: create, update, delete.
use tokio_rusqlite::Connection;

use crate::error::AppError;
use crate::memory::graph;
use crate::memory::models::ROOT_NODE_UUID;
use crate::memory::uri::{is_valid_domain, parse_uri};

/// Create a new memory node under a parent URI.
///
/// # Arguments
///
/// * `conn` - Database connection.
/// * `parent_uri` - Parent URI (e.g. `"core://"` for root).
/// * `content` - Memory text.
/// * `priority` - Retrieval priority (lower = higher).
/// * `title` - Path segment name (optional, auto-assigns numeric ID).
/// * `disclosure` - Trigger condition (optional).
///
/// # Returns
///
/// The created URI string.
pub async fn create_memory(
    conn: &Connection,
    parent_uri: &str,
    content: &str,
    priority: i32,
    title: Option<&str>,
    disclosure: Option<&str>,
) -> Result<String, AppError> {
    let parsed = parse_uri(parent_uri)?;
    if !is_valid_domain(&parsed.domain) {
        return Err(AppError::Internal(format!(
            "Invalid domain: {}",
            parsed.domain
        )));
    }

    let parent_uuid = if parsed.path.is_empty() {
        ROOT_NODE_UUID.to_string()
    } else {
        let (_, uuid) = graph::resolve_path(conn, &parsed.domain, &parsed.path).await?;
        uuid
    };

    let node_uuid = graph::create_node(conn).await?;
    graph::insert_memory(conn, &node_uuid, content).await?;

    let name = title.unwrap_or(&node_uuid);
    let edge_id =
        graph::create_edge(conn, &parent_uuid, &node_uuid, name, priority, disclosure).await?;

    let child_path = if parsed.path.is_empty() {
        name.to_string()
    } else {
        format!("{}/{}", parsed.path, name)
    };
    graph::create_path(conn, &parsed.domain, &child_path, edge_id).await?;

    let uri = format!("{}://{}", parsed.domain, child_path);
    Ok(uri)
}

/// Update an existing memory's content or metadata.
///
/// Content editing supports patch mode (`old_string`/`new_string`)
/// and append mode (`append`). Metadata (priority, disclosure) can
/// be updated independently.
pub async fn update_memory(
    conn: &Connection,
    uri: &str,
    old_string: Option<&str>,
    new_string: Option<&str>,
    append: Option<&str>,
    priority: Option<i32>,
    disclosure: Option<&str>,
) -> Result<String, AppError> {
    let parsed = parse_uri(uri)?;
    let edge = graph::resolve_edge(conn, &parsed.domain, &parsed.path).await?;

    if let Some(prio) = priority {
        let edge_id = edge.id;
        let disclosure = disclosure.map(|s| s.to_string());
        conn.call(move |conn| {
            if let Some(d) = disclosure {
                conn.execute(
                    "UPDATE edges SET priority = ?1, disclosure = ?2 WHERE id = ?3",
                    rusqlite::params![prio, d, edge_id],
                )?;
            } else {
                conn.execute(
                    "UPDATE edges SET priority = ?1 WHERE id = ?2",
                    rusqlite::params![prio, edge_id],
                )?;
            }
            Ok(())
        })
        .await
        .map_err(crate::memory::db::map_db_err("Failed to update edge"))?;
    } else if let Some(d) = disclosure {
        let edge_id = edge.id;
        let d = d.to_string();
        conn.call(move |conn| {
            conn.execute(
                "UPDATE edges SET disclosure = ?1 WHERE id = ?2",
                rusqlite::params![d, edge_id],
            )?;
            Ok(())
        })
        .await
        .map_err(crate::memory::db::map_db_err("Failed to update disclosure"))?;
    }

    let has_content_change = old_string.is_some() && new_string.is_some() || append.is_some();
    if !has_content_change {
        return Ok(format!("Updated metadata for {uri}"));
    }

    let mem = graph::get_active_memory(conn, &edge.child_uuid)
        .await?
        .ok_or_else(|| AppError::Internal(format!("No active memory for {uri}")))?;

    let new_content = if let (Some(old), Some(new)) = (old_string, new_string) {
        let count = mem.content.matches(old).count();
        if count == 0 {
            return Err(AppError::Internal(
                "old_string not found in memory content".to_string(),
            ));
        }
        if count > 1 {
            return Err(AppError::Internal(format!(
                "old_string found {count} times, must be unique"
            )));
        }
        mem.content.replacen(old, new, 1)
    } else if let Some(text) = append {
        format!("{}\n{}", mem.content, text)
    } else {
        return Ok(format!("No content change for {uri}"));
    };

    let old_id = mem.id;
    let node_uuid = edge.child_uuid.clone();
    let new_id = graph::insert_memory(conn, &node_uuid, &new_content).await?;

    conn.call(move |conn| {
        conn.execute(
            "UPDATE memories SET deprecated = 1, migrated_to = ?1 WHERE id = ?2",
            rusqlite::params![new_id, old_id],
        )?;
        Ok(())
    })
    .await
    .map_err(crate::memory::db::map_db_err(
        "Failed to deprecate old memory",
    ))?;

    Ok(format!("Updated {uri}"))
}

/// Delete a memory path and garbage-collect orphaned nodes.
pub async fn delete_memory(conn: &Connection, uri: &str) -> Result<String, AppError> {
    let parsed = parse_uri(uri)?;
    let edge = graph::resolve_edge(conn, &parsed.domain, &parsed.path).await?;

    let children = graph::get_children(conn, &edge.child_uuid).await?;
    if !children.is_empty() {
        let aliases = graph::count_aliases(conn, &edge.child_uuid).await?;
        if aliases <= 1 {
            return Err(AppError::Internal(format!(
                "Cannot delete {uri}: has {} children that would become orphaned",
                children.len()
            )));
        }
    }

    let domain = parsed.domain.clone();
    let path = parsed.path.clone();
    let edge_id = edge.id;
    let child_uuid = edge.child_uuid.clone();

    conn.call(move |conn| {
        conn.execute(
            "DELETE FROM paths WHERE domain = ?1 AND (path = ?2 OR path LIKE ?3)",
            rusqlite::params![domain, path, format!("{path}/%")],
        )?;

        let path_count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM paths WHERE edge_id = ?1",
            [edge_id],
            |row| row.get(0),
        )?;
        if path_count == 0 {
            conn.execute("DELETE FROM edges WHERE id = ?1", [edge_id])?;
        }

        let remaining_edges: i64 = conn.query_row(
            "SELECT COUNT(*) FROM edges WHERE child_uuid = ?1",
            [&child_uuid],
            |row| row.get(0),
        )?;
        if remaining_edges == 0 {
            conn.execute(
                "UPDATE memories SET deprecated = 1 WHERE node_uuid = ?1 AND deprecated = 0",
                [&child_uuid],
            )?;
        }

        Ok(())
    })
    .await
    .map_err(crate::memory::db::map_db_err("Failed to delete memory"))?;

    Ok(format!("Deleted {uri}"))
}
