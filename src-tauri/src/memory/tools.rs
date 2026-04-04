/// Rig `Tool` trait implementations for memory operations.
///
/// Each tool holds a cloned `Connection` and a shared `ToolCallLog`
/// so tool invocations can be reported to the frontend.
use std::sync::{Arc, Mutex};

use rig::completion::ToolDefinition;
use rig::tool::Tool;
use serde::Deserialize;
use serde_json::json;
use tokio_rusqlite::Connection;

#[derive(Debug, thiserror::Error)]
#[error("{0}")]
pub struct MemoryToolError(String);

impl From<crate::error::AppError> for MemoryToolError {
    fn from(e: crate::error::AppError) -> Self {
        MemoryToolError(e.to_string())
    }
}

/// A recorded tool invocation.
#[derive(Debug, Clone)]
pub struct ToolCallRecord {
    pub name: String,
    pub args: String,
    pub result: String,
}

/// Shared log of tool calls made during a single agent turn.
pub type ToolCallLog = Arc<Mutex<Vec<ToolCallRecord>>>;

/// Create a new empty tool call log.
pub fn new_tool_call_log() -> ToolCallLog {
    Arc::new(Mutex::new(Vec::new()))
}

fn log_call(log: &ToolCallLog, name: &str, args: &str, result: &str) {
    if let Ok(mut v) = log.lock() {
        v.push(ToolCallRecord {
            name: name.to_string(),
            args: args.to_string(),
            result: result.to_string(),
        });
    }
}

// --- ReadMemory ---

#[derive(Deserialize)]
pub struct ReadMemoryArgs {
    uri: String,
}

pub struct ReadMemoryTool {
    conn: Connection,
    log: ToolCallLog,
}

impl Tool for ReadMemoryTool {
    const NAME: &'static str = "read_memory";
    type Error = MemoryToolError;
    type Args = ReadMemoryArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "read_memory".to_string(),
            description: "Read a memory by URI. Special URIs: system://boot (load core identity), \
                          system://index (full index), system://recent (recent memories), \
                          system://glossary (all keywords)."
                .to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "uri": { "type": "string", "description": "Memory URI (e.g. 'core://agent', 'system://boot')" }
                },
                "required": ["uri"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let result = crate::memory::read::read_memory(&self.conn, &args.uri).await?;
        log_call(&self.log, "read_memory", &args.uri, &result);
        Ok(result)
    }
}

// --- CreateMemory ---

#[derive(Deserialize)]
pub struct CreateMemoryArgs {
    parent_uri: String,
    content: String,
    #[serde(default)]
    priority: i32,
    title: Option<String>,
    disclosure: Option<String>,
}

pub struct CreateMemoryTool {
    conn: Connection,
    log: ToolCallLog,
}

impl Tool for CreateMemoryTool {
    const NAME: &'static str = "create_memory";
    type Error = MemoryToolError;
    type Args = CreateMemoryArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "create_memory".to_string(),
            description: "Create a new memory under a parent URI. Use 'core://' for root level."
                .to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "parent_uri": { "type": "string", "description": "Parent URI" },
                    "content": { "type": "string", "description": "Memory content (Markdown)" },
                    "priority": { "type": "integer", "description": "Priority (lower = higher, default 0)" },
                    "title": { "type": "string", "description": "Path segment name" },
                    "disclosure": { "type": "string", "description": "When to recall this memory" }
                },
                "required": ["parent_uri", "content"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let args_str = format!("parent={}, title={:?}", args.parent_uri, args.title);
        let result = crate::memory::write::create_memory(
            &self.conn,
            &args.parent_uri,
            &args.content,
            args.priority,
            args.title.as_deref(),
            args.disclosure.as_deref(),
        )
        .await?;
        log_call(&self.log, "create_memory", &args_str, &result);
        Ok(result)
    }
}

// --- UpdateMemory ---

#[derive(Deserialize)]
pub struct UpdateMemoryArgs {
    uri: String,
    old_string: Option<String>,
    new_string: Option<String>,
    append: Option<String>,
    priority: Option<i32>,
    disclosure: Option<String>,
}

pub struct UpdateMemoryTool {
    conn: Connection,
    log: ToolCallLog,
}

impl Tool for UpdateMemoryTool {
    const NAME: &'static str = "update_memory";
    type Error = MemoryToolError;
    type Args = UpdateMemoryArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "update_memory".to_string(),
            description:
                "Update a memory's content (patch or append) or metadata (priority, disclosure)."
                    .to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "uri": { "type": "string", "description": "Memory URI to update" },
                    "old_string": { "type": "string", "description": "Text to find (patch mode)" },
                    "new_string": { "type": "string", "description": "Replacement text (patch mode)" },
                    "append": { "type": "string", "description": "Text to append" },
                    "priority": { "type": "integer", "description": "New priority" },
                    "disclosure": { "type": "string", "description": "New trigger condition" }
                },
                "required": ["uri"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let result = crate::memory::write::update_memory(
            &self.conn,
            &args.uri,
            args.old_string.as_deref(),
            args.new_string.as_deref(),
            args.append.as_deref(),
            args.priority,
            args.disclosure.as_deref(),
        )
        .await?;
        log_call(&self.log, "update_memory", &args.uri, &result);
        Ok(result)
    }
}

// --- DeleteMemory ---

#[derive(Deserialize)]
pub struct DeleteMemoryArgs {
    uri: String,
}

pub struct DeleteMemoryTool {
    conn: Connection,
    log: ToolCallLog,
}

impl Tool for DeleteMemoryTool {
    const NAME: &'static str = "delete_memory";
    type Error = MemoryToolError;
    type Args = DeleteMemoryArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "delete_memory".to_string(),
            description: "Delete a memory path. Refuses if children would become orphaned."
                .to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "uri": { "type": "string", "description": "Memory URI to delete" }
                },
                "required": ["uri"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let result = crate::memory::write::delete_memory(&self.conn, &args.uri).await?;
        log_call(&self.log, "delete_memory", &args.uri, &result);
        Ok(result)
    }
}

// --- AddAlias ---

#[derive(Deserialize)]
pub struct AddAliasArgs {
    new_uri: String,
    target_uri: String,
    #[serde(default)]
    priority: i32,
    disclosure: Option<String>,
}

pub struct AddAliasTool {
    conn: Connection,
    log: ToolCallLog,
}

impl Tool for AddAliasTool {
    const NAME: &'static str = "add_alias";
    type Error = MemoryToolError;
    type Args = AddAliasArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "add_alias".to_string(),
            description:
                "Create an alias URI pointing to an existing memory. Shares the same content."
                    .to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "new_uri": { "type": "string", "description": "New alias URI" },
                    "target_uri": { "type": "string", "description": "Existing URI to alias" },
                    "priority": { "type": "integer", "description": "Priority (default 0)" },
                    "disclosure": { "type": "string", "description": "Trigger condition" }
                },
                "required": ["new_uri", "target_uri"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let args_str = format!("{} -> {}", args.new_uri, args.target_uri);
        let result = crate::memory::alias::add_alias(
            &self.conn,
            &args.new_uri,
            &args.target_uri,
            args.priority,
            args.disclosure.as_deref(),
        )
        .await?;
        log_call(&self.log, "add_alias", &args_str, &result);
        Ok(result)
    }
}

// --- ManageTriggers ---

#[derive(Deserialize)]
pub struct ManageTriggersArgs {
    uri: String,
    #[serde(default)]
    add: Vec<String>,
    #[serde(default)]
    remove: Vec<String>,
}

pub struct ManageTriggersTool {
    conn: Connection,
    log: ToolCallLog,
}

impl Tool for ManageTriggersTool {
    const NAME: &'static str = "manage_triggers";
    type Error = MemoryToolError;
    type Args = ManageTriggersArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "manage_triggers".to_string(),
            description: "Add or remove glossary keywords bound to a memory node.".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "uri": { "type": "string", "description": "Target memory URI" },
                    "add": { "type": "array", "items": { "type": "string" }, "description": "Keywords to bind" },
                    "remove": { "type": "array", "items": { "type": "string" }, "description": "Keywords to unbind" }
                },
                "required": ["uri"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let result =
            crate::memory::glossary::manage_triggers(&self.conn, &args.uri, args.add, args.remove)
                .await?;
        log_call(&self.log, "manage_triggers", &args.uri, &result);
        Ok(result)
    }
}

// --- SearchMemory ---

#[derive(Deserialize)]
pub struct SearchMemoryArgs {
    query: String,
    domain: Option<String>,
    limit: Option<i64>,
}

pub struct SearchMemoryTool {
    conn: Connection,
    log: ToolCallLog,
}

impl Tool for SearchMemoryTool {
    const NAME: &'static str = "search_memory";
    type Error = MemoryToolError;
    type Args = SearchMemoryArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "search_memory".to_string(),
            description: "Full-text search across all memories.".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "query": { "type": "string", "description": "Search keywords" },
                    "domain": { "type": "string", "description": "Domain filter (e.g. 'core')" },
                    "limit": { "type": "integer", "description": "Max results (default 10, max 100)" }
                },
                "required": ["query"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let result = crate::memory::search::search_memory(
            &self.conn,
            &args.query,
            args.domain.as_deref(),
            args.limit,
        )
        .await?;
        log_call(&self.log, "search_memory", &args.query, &result);
        Ok(result)
    }
}

/// Create all 7 memory tools with a shared call log.
pub fn create_memory_tools(
    conn: &Connection,
    log: &ToolCallLog,
) -> Vec<Box<dyn rig::tool::ToolDyn>> {
    vec![
        Box::new(ReadMemoryTool {
            conn: conn.clone(),
            log: log.clone(),
        }),
        Box::new(CreateMemoryTool {
            conn: conn.clone(),
            log: log.clone(),
        }),
        Box::new(UpdateMemoryTool {
            conn: conn.clone(),
            log: log.clone(),
        }),
        Box::new(DeleteMemoryTool {
            conn: conn.clone(),
            log: log.clone(),
        }),
        Box::new(AddAliasTool {
            conn: conn.clone(),
            log: log.clone(),
        }),
        Box::new(ManageTriggersTool {
            conn: conn.clone(),
            log: log.clone(),
        }),
        Box::new(SearchMemoryTool {
            conn: conn.clone(),
            log: log.clone(),
        }),
    ]
}
