/// Full-text search using SQLite FTS5 with the simple tokenizer.
use tokio_rusqlite::Connection;

use crate::error::AppError;

/// Search result from the FTS index.
#[derive(Debug)]
pub struct SearchResult {
    pub uri: String,
    pub priority: i32,
    pub disclosure: Option<String>,
    pub snippet: String,
}

/// Search memories by query text.
///
/// # Arguments
///
/// * `conn` - Database connection.
/// * `query` - Search keywords.
/// * `domain` - Optional domain filter.
/// * `limit` - Max results (default 10, max 100).
///
/// # Returns
///
/// A formatted string with search results.
pub async fn search_memory(
    conn: &Connection,
    query: &str,
    domain: Option<&str>,
    limit: Option<i64>,
) -> Result<String, AppError> {
    let query = query.to_string();
    let domain = domain.map(|s| s.to_string());
    let limit = limit.unwrap_or(10).min(100);
    let query_display = query.clone();

    let results: Vec<SearchResult> = conn
        .call(move |conn| {
            let sql = if domain.is_some() {
                "SELECT sf.uri, sd.priority, sd.disclosure, \
                        snippet(search_fts, 1, '>>>', '<<<', '...', 32) as snip \
                 FROM search_fts sf \
                 JOIN search_documents sd ON sf.uri = sd.uri \
                 WHERE search_fts MATCH ?1 AND sd.domain = ?2 \
                 ORDER BY sd.priority ASC, rank \
                 LIMIT ?3"
            } else {
                "SELECT sf.uri, sd.priority, sd.disclosure, \
                        snippet(search_fts, 1, '>>>', '<<<', '...', 32) as snip \
                 FROM search_fts sf \
                 JOIN search_documents sd ON sf.uri = sd.uri \
                 WHERE search_fts MATCH ?1 \
                 ORDER BY sd.priority ASC, rank \
                 LIMIT ?2"
            };

            let mut stmt = conn.prepare(sql)?;
            let rows = if let Some(ref d) = domain {
                stmt.query_map(rusqlite::params![query, d, limit], |row| {
                    Ok(SearchResult {
                        uri: row.get(0)?,
                        priority: row.get(1)?,
                        disclosure: row.get(2)?,
                        snippet: row.get(3)?,
                    })
                })?
                .collect::<Result<Vec<_>, _>>()?
            } else {
                stmt.query_map(rusqlite::params![query, limit], |row| {
                    Ok(SearchResult {
                        uri: row.get(0)?,
                        priority: row.get(1)?,
                        disclosure: row.get(2)?,
                        snippet: row.get(3)?,
                    })
                })?
                .collect::<Result<Vec<_>, _>>()?
            };
            Ok(rows)
        })
        .await
        .map_err(crate::memory::db::map_db_err("Search failed"))?;

    let mut output = format!("# Search results for \"{query_display}\"\n\n");
    if results.is_empty() {
        output.push_str("No results found.\n");
    } else {
        for (i, r) in results.iter().enumerate() {
            let prio = if r.priority != 0 {
                format!(" [★{}]", r.priority)
            } else {
                String::new()
            };
            output.push_str(&format!("{}. {}{}\n", i + 1, r.uri, prio));
            if let Some(ref d) = r.disclosure {
                output.push_str(&format!("   disclosure: {d}\n"));
            }
            output.push_str(&format!("   {}\n", r.snippet));
        }
    }

    Ok(output)
}

/// Upsert a search document and its FTS entry.
#[allow(dead_code, clippy::too_many_arguments)]
pub async fn upsert_search_document(
    conn: &Connection,
    domain: &str,
    path: &str,
    node_uuid: &str,
    memory_id: i64,
    content: &str,
    disclosure: Option<&str>,
    priority: i32,
) -> Result<(), AppError> {
    let uri = format!("{domain}://{path}");
    let domain = domain.to_string();
    let path = path.to_string();
    let node_uuid = node_uuid.to_string();
    let content = content.to_string();
    let disclosure = disclosure.map(|s| s.to_string());
    let uri_clone = uri.clone();

    conn.call(move |conn| {
        conn.execute(
            "DELETE FROM search_fts WHERE uri = ?1",
            rusqlite::params![uri_clone],
        )?;
        conn.execute(
            "INSERT OR REPLACE INTO search_documents \
             (domain, path, node_uuid, memory_id, uri, content, disclosure, priority) \
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            rusqlite::params![
                domain, path, node_uuid, memory_id, uri, content, disclosure, priority
            ],
        )?;
        conn.execute(
            "INSERT INTO search_fts (uri, content, disclosure, search_terms) \
             VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![uri, content, disclosure.unwrap_or_default(), ""],
        )?;
        Ok(())
    })
    .await
    .map_err(crate::memory::db::map_db_err(
        "Failed to upsert search document",
    ))
}
