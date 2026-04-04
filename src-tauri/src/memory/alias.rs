/// Alias operations: create alternative URI routes to existing nodes.
use tokio_rusqlite::Connection;

use crate::error::AppError;
use crate::memory::graph;
use crate::memory::models::ROOT_NODE_UUID;
use crate::memory::uri::{is_valid_domain, parse_uri, split_parent_path};

/// Create an alias URI pointing to an existing memory node.
///
/// # Arguments
///
/// * `conn` - Database connection.
/// * `new_uri` - The new alias URI to create.
/// * `target_uri` - The existing URI to alias.
/// * `priority` - Priority for the new alias edge.
/// * `disclosure` - Trigger condition for the new alias.
///
/// # Returns
///
/// A success message.
///
/// # Errors
///
/// Returns error if target doesn't exist, new_uri already exists,
/// or the alias would create a cycle.
pub async fn add_alias(
    conn: &Connection,
    new_uri: &str,
    target_uri: &str,
    priority: i32,
    disclosure: Option<&str>,
) -> Result<String, AppError> {
    let new_parsed = parse_uri(new_uri)?;
    let target_parsed = parse_uri(target_uri)?;

    if !is_valid_domain(&new_parsed.domain) {
        return Err(AppError::Internal(format!(
            "Invalid domain: {}",
            new_parsed.domain
        )));
    }

    let (_, target_uuid) =
        graph::resolve_path(conn, &target_parsed.domain, &target_parsed.path).await?;

    let parent_uuid = if new_parsed.path.is_empty() || !new_parsed.path.contains('/') {
        ROOT_NODE_UUID.to_string()
    } else {
        let (parent_path, _) = split_parent_path(&new_parsed.path);
        let (_, uuid) = graph::resolve_path(conn, &new_parsed.domain, parent_path).await?;
        uuid
    };

    if graph::is_ancestor(conn, &parent_uuid, &target_uuid).await? {
        return Err(AppError::Internal(
            "Cannot create alias: would create a cycle".to_string(),
        ));
    }

    let (_, leaf) = split_parent_path(&new_parsed.path);
    let edge_id =
        graph::create_edge(conn, &parent_uuid, &target_uuid, leaf, priority, disclosure).await?;
    graph::create_path(conn, &new_parsed.domain, &new_parsed.path, edge_id).await?;

    Ok(format!("Alias created: {} -> {}", new_uri, target_uri))
}
