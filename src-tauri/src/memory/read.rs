/// Memory read operations with formatted output.
use std::future::Future;
use std::pin::Pin;

use tokio_rusqlite::Connection;

use crate::error::AppError;
use crate::memory::boot;
use crate::memory::graph;
use crate::memory::uri::parse_uri;

/// Read a memory by URI, including system:// special URIs.
///
/// # Returns
///
/// A formatted string with memory content, metadata, and children.
pub fn read_memory<'a>(
    conn: &'a Connection,
    uri: &'a str,
) -> Pin<Box<dyn Future<Output = Result<String, AppError>> + Send + 'a>> {
    Box::pin(async move {
        let parsed = parse_uri(uri)?;

        if parsed.domain == "system" {
            return boot::handle_system_uri(conn, &parsed.path).await;
        }

        let edge = graph::resolve_edge(conn, &parsed.domain, &parsed.path).await?;
        let mem = graph::get_active_memory(conn, &edge.child_uuid)
            .await?
            .ok_or_else(|| AppError::Internal(format!("No active memory for {uri}")))?;

        let aliases = graph::count_aliases(conn, &edge.child_uuid).await?;
        let children = graph::get_children(conn, &edge.child_uuid).await?;

        let mut output = String::new();
        output.push_str(&format!("MEMORY: {uri}\n"));
        output.push_str(&format!("Memory ID: {} | Aliases: {aliases}\n", mem.id));
        if edge.priority != 0 {
            output.push_str(&format!("Priority: {}\n", edge.priority));
        }
        if let Some(ref d) = edge.disclosure {
            output.push_str(&format!("When to recall: {d}\n"));
        }
        output.push_str(&format!("\n{}\n", mem.content));

        if !children.is_empty() {
            output.push_str("\n--- CHILDREN ---\n");
            for (child_edge, child_mem) in &children {
                let prio = if child_edge.priority != 0 {
                    format!(" [★{}]", child_edge.priority)
                } else {
                    String::new()
                };
                let snippet = child_mem
                    .as_ref()
                    .map(|m| {
                        let s: String = m.content.chars().take(80).collect();
                        if m.content.chars().count() > 80 {
                            format!("{s}...")
                        } else {
                            s
                        }
                    })
                    .unwrap_or_else(|| "(empty)".to_string());
                output.push_str(&format!("- {}{}: {}\n", child_edge.name, prio, snippet));
                if let Some(ref d) = child_edge.disclosure {
                    output.push_str(&format!("  disclosure: {d}\n"));
                }
            }
        }

        Ok(output)
    })
}
