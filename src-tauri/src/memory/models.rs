use serde::{Deserialize, Serialize};

/// Sentinel UUID for the root node that parents all top-level edges.
pub const ROOT_NODE_UUID: &str = "00000000-0000-0000-0000-000000000000";

/// A conceptual anchor point in the memory graph.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct Node {
    pub uuid: String,
    pub created_at: String,
}

/// A versioned content snapshot belonging to a node.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Memory {
    pub id: i64,
    pub node_uuid: Option<String>,
    pub content: String,
    pub deprecated: bool,
    pub migrated_to: Option<i64>,
    pub created_at: String,
}

/// A directed relationship between two nodes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Edge {
    pub id: i64,
    pub parent_uuid: String,
    pub child_uuid: String,
    pub name: String,
    pub priority: i32,
    pub disclosure: Option<String>,
    pub created_at: String,
}

/// A URI route mapping `(domain, path)` to an edge.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct Path {
    pub domain: String,
    pub path: String,
    pub edge_id: Option<i64>,
    pub created_at: String,
}

/// A keyword bound to a node for cross-node linking.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct GlossaryKeyword {
    pub id: i64,
    pub keyword: String,
    pub node_uuid: String,
    pub created_at: String,
}
