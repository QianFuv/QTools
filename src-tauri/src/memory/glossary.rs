/// Glossary keyword management and content scanning.
use tokio_rusqlite::Connection;

use crate::error::AppError;
use crate::memory::graph;
use crate::memory::uri::parse_uri;

/// Add or remove keywords bound to a memory node.
///
/// # Arguments
///
/// * `conn` - Database connection.
/// * `uri` - Target memory URI.
/// * `add` - Keywords to bind.
/// * `remove` - Keywords to unbind.
///
/// # Returns
///
/// A summary of changes and current keywords.
pub async fn manage_triggers(
    conn: &Connection,
    uri: &str,
    add: Vec<String>,
    remove: Vec<String>,
) -> Result<String, AppError> {
    let parsed = parse_uri(uri)?;
    let (_, node_uuid) = graph::resolve_path(conn, &parsed.domain, &parsed.path).await?;

    let mut added = Vec::new();
    let mut skipped = Vec::new();
    let mut removed = Vec::new();

    for keyword in &add {
        let kw = keyword.clone();
        let uuid = node_uuid.clone();
        let result = conn
            .call(move |conn| {
                conn.execute(
                    "INSERT OR IGNORE INTO glossary_keywords (keyword, node_uuid) VALUES (?1, ?2)",
                    rusqlite::params![kw, uuid],
                )?;
                Ok(conn.changes())
            })
            .await
            .map_err(crate::memory::db::map_db_err("Failed to add keyword"))?;
        if result > 0 {
            added.push(keyword.clone());
        } else {
            skipped.push(keyword.clone());
        }
    }

    for keyword in &remove {
        let kw = keyword.clone();
        let uuid = node_uuid.clone();
        let result = conn
            .call(move |conn| {
                let affected = conn.execute(
                    "DELETE FROM glossary_keywords WHERE keyword = ?1 AND node_uuid = ?2",
                    rusqlite::params![kw, uuid],
                )?;
                Ok(affected)
            })
            .await
            .map_err(crate::memory::db::map_db_err("Failed to remove keyword"))?;
        if result > 0 {
            removed.push(keyword.clone());
        }
    }

    let uuid = node_uuid.clone();
    let current: Vec<String> = conn
        .call(move |conn| {
            let mut stmt = conn.prepare(
                "SELECT keyword FROM glossary_keywords WHERE node_uuid = ?1 ORDER BY keyword",
            )?;
            let rows = stmt
                .query_map([&uuid], |row| row.get(0))?
                .collect::<Result<Vec<String>, _>>()?;
            Ok(rows)
        })
        .await
        .map_err(crate::memory::db::map_db_err("Failed to list keywords"))?;

    let mut output = format!("Triggers for {uri}:\n");
    if !added.is_empty() {
        output.push_str(&format!("  Added: {}\n", added.join(", ")));
    }
    if !skipped.is_empty() {
        output.push_str(&format!("  Already existed: {}\n", skipped.join(", ")));
    }
    if !removed.is_empty() {
        output.push_str(&format!("  Removed: {}\n", removed.join(", ")));
    }
    output.push_str(&format!("  Current: {}\n", current.join(", ")));

    Ok(output)
}
