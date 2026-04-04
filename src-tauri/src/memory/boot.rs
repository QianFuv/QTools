/// System URI handlers: boot, index, recent, glossary.
use tokio_rusqlite::Connection;

use crate::error::AppError;

/// Default core memory URIs loaded on boot.
const CORE_MEMORY_URIS: &[&str] = &["core://agent", "core://user"];

/// Dispatch system:// URIs to their handlers.
pub async fn handle_system_uri(conn: &Connection, path: &str) -> Result<String, AppError> {
    match path {
        "boot" => generate_boot(conn).await,
        "index" => generate_index(conn, None).await,
        "glossary" => generate_glossary(conn).await,
        p if p.starts_with("index/") => {
            let domain = &p[6..];
            generate_index(conn, Some(domain)).await
        }
        p if p.starts_with("recent") => {
            let n = p
                .strip_prefix("recent/")
                .and_then(|s| s.parse::<usize>().ok())
                .unwrap_or(10)
                .min(100);
            generate_recent(conn, n).await
        }
        _ => Err(AppError::Internal(format!(
            "Unknown system URI: system://{path}"
        ))),
    }
}

/// Load default core memory URIs on boot.
async fn generate_boot(conn: &Connection) -> Result<String, AppError> {
    let mut output = String::from("# Core Memories\n");
    let mut loaded = 0;
    let mut failed = Vec::new();

    for uri in CORE_MEMORY_URIS {
        match crate::memory::read::read_memory(conn, uri).await {
            Ok(content) => {
                output
                    .push_str("\n============================================================\n\n");
                output.push_str(&content);
                loaded += 1;
            }
            Err(e) => {
                failed.push(format!("- {uri}: {e}"));
            }
        }
    }

    let total = CORE_MEMORY_URIS.len();
    let header = format!("# Loaded: {loaded}/{total} memories\n");
    if !failed.is_empty() {
        output = format!(
            "{header}\n## Failed to load:\n{}\n\n{output}",
            failed.join("\n")
        );
    } else {
        output = format!("{header}\n{output}");
    }

    let recent = generate_recent(conn, 5).await.unwrap_or_default();
    if !recent.is_empty() {
        output.push_str("\n---\n\n");
        output.push_str(&recent);
    }

    Ok(output)
}

/// Generate a full memory index grouped by domain.
async fn generate_index(
    conn: &Connection,
    domain_filter: Option<&str>,
) -> Result<String, AppError> {
    let filter = domain_filter.map(|s| s.to_string());
    conn.call(move |conn| {
        let mut output = String::from("# Memory Index\n\n");
        let query = if let Some(ref d) = filter {
            format!(
                "SELECT p.domain, p.path, e.name, e.priority, e.disclosure \
                 FROM paths p JOIN edges e ON p.edge_id = e.id \
                 WHERE p.domain = '{}' \
                 ORDER BY p.domain, e.priority ASC, p.path",
                d
            )
        } else {
            "SELECT p.domain, p.path, e.name, e.priority, e.disclosure \
             FROM paths p JOIN edges e ON p.edge_id = e.id \
             ORDER BY p.domain, e.priority ASC, p.path"
                .to_string()
        };
        let mut stmt = conn.prepare(&query)?;
        let rows = stmt
            .query_map([], |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, i32>(3)?,
                    row.get::<_, Option<String>>(4)?,
                ))
            })?
            .collect::<Result<Vec<_>, _>>()?;

        let mut current_domain = String::new();
        for (domain, path, _name, priority, disclosure) in &rows {
            if *domain != current_domain {
                output.push_str(&format!("\n## {domain}://\n"));
                current_domain.clone_from(domain);
            }
            let prio = if *priority != 0 {
                format!(" [★{priority}]")
            } else {
                String::new()
            };
            output.push_str(&format!("- {domain}://{path}{prio}"));
            if let Some(d) = disclosure {
                output.push_str(&format!("  ({d})"));
            }
            output.push('\n');
        }

        if rows.is_empty() {
            output.push_str("(no memories yet)\n");
        }

        Ok(output)
    })
    .await
    .map_err(crate::memory::db::map_db_err("Failed to generate index"))
}

/// Generate a list of recently modified memories.
async fn generate_recent(conn: &Connection, n: usize) -> Result<String, AppError> {
    let n = n as i64;
    conn.call(move |conn| {
        let mut output = format!("# Recently Modified Memories (top {n})\n\n");
        let mut stmt = conn.prepare(
            "SELECT p.domain, p.path, e.priority, e.disclosure, m.created_at \
             FROM memories m \
             JOIN nodes nd ON m.node_uuid = nd.uuid \
             JOIN edges e ON e.child_uuid = nd.uuid \
             JOIN paths p ON p.edge_id = e.id \
             WHERE m.deprecated = 0 \
             ORDER BY m.created_at DESC LIMIT ?1",
        )?;
        let rows = stmt
            .query_map([n], |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, i32>(2)?,
                    row.get::<_, Option<String>>(3)?,
                    row.get::<_, String>(4)?,
                ))
            })?
            .collect::<Result<Vec<_>, _>>()?;

        for (i, (domain, path, priority, disclosure, created)) in rows.iter().enumerate() {
            let prio = if *priority != 0 {
                format!(" [★{priority}]")
            } else {
                String::new()
            };
            output.push_str(&format!(
                "{}. {domain}://{path}{prio}  modified: {created}\n",
                i + 1
            ));
            if let Some(d) = disclosure {
                output.push_str(&format!("   disclosure: {d}\n"));
            }
        }

        if rows.is_empty() {
            output.push_str("(no memories yet)\n");
        }

        Ok(output)
    })
    .await
    .map_err(crate::memory::db::map_db_err("Failed to generate recent"))
}

/// Generate a glossary index of all keywords.
async fn generate_glossary(conn: &Connection) -> Result<String, AppError> {
    conn.call(|conn| {
        let mut output = String::from("# Glossary Keywords\n\n");
        let mut stmt = conn.prepare(
            "SELECT gk.keyword, p.domain, p.path \
             FROM glossary_keywords gk \
             JOIN edges e ON e.child_uuid = gk.node_uuid \
             JOIN paths p ON p.edge_id = e.id \
             ORDER BY gk.keyword",
        )?;
        let rows = stmt
            .query_map([], |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                ))
            })?
            .collect::<Result<Vec<_>, _>>()?;

        for (keyword, domain, path) in &rows {
            output.push_str(&format!("- @{keyword} -> {domain}://{path}\n"));
        }

        if rows.is_empty() {
            output.push_str("(no keywords defined)\n");
        }

        Ok(output)
    })
    .await
    .map_err(crate::memory::db::map_db_err("Failed to generate glossary"))
}
