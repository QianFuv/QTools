use std::time::Instant;

use serde::{Deserialize, Serialize};

/// Persistent configuration for the Canvas LMS integration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanvasSettings {
    pub api_url: String,
    pub api_token: String,
    pub cache_ttl_minutes: u64,
    pub ignored_course_ids: Vec<i64>,
}

impl Default for CanvasSettings {
    fn default() -> Self {
        Self {
            api_url: "https://canvas.uts.edu.au".to_string(),
            api_token: String::new(),
            cache_ttl_minutes: 30,
            ignored_course_ids: Vec::new(),
        }
    }
}

/// A Canvas LMS course.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanvasCourse {
    pub id: i64,
    pub name: String,
    pub course_code: String,
}

/// A Canvas LMS assignment or quiz with local completion tracking.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanvasAssignment {
    pub id: i64,
    pub course_id: i64,
    pub course_name: String,
    pub name: String,
    pub due_at: Option<String>,
    pub points_possible: Option<f64>,
    pub html_url: String,
    pub submission_types: Vec<String>,
    pub has_submitted_submissions: bool,
    pub is_quiz: bool,
    pub manually_completed: bool,
}

/// Cached snapshot of Canvas data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanvasData {
    pub courses: Vec<CanvasCourse>,
    pub assignments: Vec<CanvasAssignment>,
    pub fetched_at: String,
}

/// Application state for the Canvas subsystem.
pub struct CanvasState {
    pub settings: CanvasSettings,
    pub cache: Option<CachedData>,
}

/// In-memory cache entry with wall-clock expiry.
pub struct CachedData {
    pub data: CanvasData,
    pub cached_at: Instant,
}
