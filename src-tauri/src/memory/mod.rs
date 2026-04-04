/// URI-graph memory system inspired by Nocturne Memory.
///
/// Provides persistent, structured memory storage using SQLite
/// with URI-based routing (`domain://path`), version-controlled
/// content, and FTS5 full-text search with Chinese support.
pub mod alias;
pub mod boot;
pub mod chat_store;
pub mod db;
pub mod glossary;
pub mod graph;
pub mod models;
pub mod read;
pub mod search;
pub mod tools;
pub mod uri;
pub mod write;
