/// Core graph operations on nodes, edges, and paths.
///
/// Provides CRUD primitives and cycle detection for the
/// memory graph structure.
use rusqlite::OptionalExtension;
use tokio_rusqlite::Connection;

use crate::error::AppError;
use crate::memory::models::{Edge, ROOT_NODE_UUID};

/// Resolve a `(domain, path)` to the child node UUID.
///
/// # Errors
///
/// Returns `AppError::Internal` if the path does not exist.
pub async fn resolve_path(
    conn: &Connection,
    domain: &str,
    path: &str,
) -> Result<(i64, String), AppError> {
    let domain = domain.to_string();
    let path = path.to_string();
    conn.call(move |conn| {
        let mut stmt = conn.prepare(
            "SELECT e.id, e.child_uuid FROM paths p \
             JOIN edges e ON p.edge_id = e.id \
             WHERE p.domain = ?1 AND p.path = ?2",
        )?;
        let result = stmt.query_row(rusqlite::params![domain, path], |row| {
            Ok((row.get::<_, i64>(0)?, row.get::<_, String>(1)?))
        })?;
        Ok(result)
    })
    .await
    .map_err(crate::memory::db::map_db_err("Path not found"))
}

/// Resolve a `(domain, path)` to the full edge record.
pub async fn resolve_edge(conn: &Connection, domain: &str, path: &str) -> Result<Edge, AppError> {
    let domain = domain.to_string();
    let path = path.to_string();
    conn.call(move |conn| {
        let mut stmt = conn.prepare(
            "SELECT e.id, e.parent_uuid, e.child_uuid, e.name, e.priority, e.disclosure, e.created_at \
             FROM paths p JOIN edges e ON p.edge_id = e.id \
             WHERE p.domain = ?1 AND p.path = ?2",
        )?;
        let edge = stmt.query_row(rusqlite::params![domain, path], |row| {
            Ok(Edge {
                id: row.get(0)?,
                parent_uuid: row.get(1)?,
                child_uuid: row.get(2)?,
                name: row.get(3)?,
                priority: row.get(4)?,
                disclosure: row.get(5)?,
                created_at: row.get(6)?,
            })
        })?;
        Ok(edge)
    })
    .await
    .map_err(crate::memory::db::map_db_err("Edge not found"))
}

/// Get the active (non-deprecated) memory for a node.
pub async fn get_active_memory(
    conn: &Connection,
    node_uuid: &str,
) -> Result<Option<crate::memory::models::Memory>, AppError> {
    let node_uuid = node_uuid.to_string();
    conn.call(move |conn| {
        let mut stmt = conn.prepare(
            "SELECT id, node_uuid, content, deprecated, migrated_to, created_at \
             FROM memories WHERE node_uuid = ?1 AND deprecated = 0 LIMIT 1",
        )?;
        let result = stmt
            .query_row(rusqlite::params![node_uuid], |row| {
                Ok(crate::memory::models::Memory {
                    id: row.get(0)?,
                    node_uuid: row.get(1)?,
                    content: row.get(2)?,
                    deprecated: row.get::<_, i32>(3)? != 0,
                    migrated_to: row.get(4)?,
                    created_at: row.get(5)?,
                })
            })
            .optional()?;
        Ok(result)
    })
    .await
    .map_err(crate::memory::db::map_db_err("Failed to get memory"))
}

/// Create a new node and return its UUID.
pub async fn create_node(conn: &Connection) -> Result<String, AppError> {
    let uuid = uuid::Uuid::new_v4().to_string();
    let uuid_clone = uuid.clone();
    conn.call(move |conn| {
        conn.execute("INSERT INTO nodes (uuid) VALUES (?1)", [&uuid_clone])?;
        Ok(())
    })
    .await
    .map_err(crate::memory::db::map_db_err("Failed to create node"))?;
    Ok(uuid)
}

/// Insert a memory record for a node.
pub async fn insert_memory(
    conn: &Connection,
    node_uuid: &str,
    content: &str,
) -> Result<i64, AppError> {
    let node_uuid = node_uuid.to_string();
    let content = content.to_string();
    conn.call(move |conn| {
        conn.execute(
            "INSERT INTO memories (node_uuid, content) VALUES (?1, ?2)",
            rusqlite::params![node_uuid, content],
        )?;
        Ok(conn.last_insert_rowid())
    })
    .await
    .map_err(crate::memory::db::map_db_err("Failed to insert memory"))
}

/// Create an edge between two nodes.
pub async fn create_edge(
    conn: &Connection,
    parent_uuid: &str,
    child_uuid: &str,
    name: &str,
    priority: i32,
    disclosure: Option<&str>,
) -> Result<i64, AppError> {
    let parent_uuid = parent_uuid.to_string();
    let child_uuid = child_uuid.to_string();
    let name = name.to_string();
    let disclosure = disclosure.map(|s| s.to_string());
    conn.call(move |conn| {
        conn.execute(
            "INSERT INTO edges (parent_uuid, child_uuid, name, priority, disclosure) \
             VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![parent_uuid, child_uuid, name, priority, disclosure],
        )?;
        Ok(conn.last_insert_rowid())
    })
    .await
    .map_err(crate::memory::db::map_db_err("Failed to create edge"))
}

/// Create a path entry linking a URI to an edge.
pub async fn create_path(
    conn: &Connection,
    domain: &str,
    path: &str,
    edge_id: i64,
) -> Result<(), AppError> {
    let domain = domain.to_string();
    let path = path.to_string();
    conn.call(move |conn| {
        conn.execute(
            "INSERT INTO paths (domain, path, edge_id) VALUES (?1, ?2, ?3)",
            rusqlite::params![domain, path, edge_id],
        )?;
        Ok(())
    })
    .await
    .map_err(crate::memory::db::map_db_err("Failed to create path"))
}

/// Get children of a node, ordered by priority.
pub async fn get_children(
    conn: &Connection,
    node_uuid: &str,
) -> Result<Vec<(Edge, Option<crate::memory::models::Memory>)>, AppError> {
    let node_uuid = node_uuid.to_string();
    conn.call(move |conn| {
        let mut stmt = conn.prepare(
            "SELECT e.id, e.parent_uuid, e.child_uuid, e.name, e.priority, e.disclosure, e.created_at, \
                    m.id, m.node_uuid, m.content, m.deprecated, m.migrated_to, m.created_at \
             FROM edges e \
             LEFT JOIN memories m ON m.node_uuid = e.child_uuid AND m.deprecated = 0 \
             WHERE e.parent_uuid = ?1 \
             ORDER BY e.priority ASC, e.name ASC",
        )?;
        let rows = stmt
            .query_map([&node_uuid], |row| {
                let edge = Edge {
                    id: row.get(0)?,
                    parent_uuid: row.get(1)?,
                    child_uuid: row.get(2)?,
                    name: row.get(3)?,
                    priority: row.get(4)?,
                    disclosure: row.get(5)?,
                    created_at: row.get(6)?,
                };
                let mem_id: Option<i64> = row.get(7)?;
                let memory = mem_id.map(|id| crate::memory::models::Memory {
                    id,
                    node_uuid: row.get(8).ok(),
                    content: row.get(9).unwrap_or_default(),
                    deprecated: row.get::<_, i32>(10).unwrap_or(0) != 0,
                    migrated_to: row.get(11).ok().flatten(),
                    created_at: row.get(12).unwrap_or_default(),
                });
                Ok((edge, memory))
            })?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(rows)
    })
    .await
    .map_err(crate::memory::db::map_db_err("Failed to get children"))
}

/// Check whether `target_uuid` is an ancestor of `node_uuid`.
///
/// Used to prevent cycles when adding aliases.
pub async fn is_ancestor(
    conn: &Connection,
    node_uuid: &str,
    target_uuid: &str,
) -> Result<bool, AppError> {
    if node_uuid == target_uuid {
        return Ok(true);
    }
    if node_uuid == ROOT_NODE_UUID {
        return Ok(false);
    }
    let node_uuid = node_uuid.to_string();
    let target_uuid = target_uuid.to_string();
    conn.call(move |conn| {
        let mut current = node_uuid;
        for _ in 0..100 {
            let parent: Option<String> = conn
                .prepare("SELECT parent_uuid FROM edges WHERE child_uuid = ?1 LIMIT 1")?
                .query_row([&current], |row| row.get(0))
                .optional()?;
            match parent {
                Some(p) if p == target_uuid => return Ok(true),
                Some(p) if p == ROOT_NODE_UUID => return Ok(false),
                Some(p) => current = p,
                None => return Ok(false),
            }
        }
        Ok(false)
    })
    .await
    .map_err(crate::memory::db::map_db_err("Cycle check failed"))
}

/// Count how many paths point to a given node.
pub async fn count_aliases(conn: &Connection, node_uuid: &str) -> Result<i64, AppError> {
    let node_uuid = node_uuid.to_string();
    conn.call(move |conn| {
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM paths p JOIN edges e ON p.edge_id = e.id WHERE e.child_uuid = ?1",
            [&node_uuid],
            |row| row.get(0),
        )?;
        Ok(count)
    })
    .await
    .map_err(crate::memory::db::map_db_err("Failed to count aliases"))
}
