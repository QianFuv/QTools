use std::collections::HashSet;
use std::time::{Duration, Instant};

use tokio::sync::Mutex;

use crate::agent::types::AgentState;
use crate::error::AppError;

use super::client::CanvasClient;
use super::types::{CachedData, CanvasData, CanvasState};

/// Fetch Canvas courses and assignments, returning cached data when fresh.
///
/// # Errors
///
/// Returns `AppError::Canvas` if the API token is not configured or the
/// request fails.
#[tauri::command]
pub async fn fetch_canvas_data(
    canvas_state: tauri::State<'_, Mutex<CanvasState>>,
    agent_state: tauri::State<'_, Mutex<AgentState>>,
) -> Result<CanvasData, AppError> {
    let state = canvas_state.lock().await;

    if let Some(ref cached) = state.cache {
        let ttl = Duration::from_secs(state.settings.cache_ttl_minutes * 60);
        if cached.cached_at.elapsed() < ttl {
            return Ok(cached.data.clone());
        }
    }

    let settings = state.settings.clone();
    drop(state);

    fetch_fresh(canvas_state, agent_state, &settings).await
}

/// Force-refresh Canvas data, bypassing the cache.
///
/// # Errors
///
/// Returns `AppError::Canvas` if the API token is not configured or the
/// request fails.
#[tauri::command]
pub async fn refresh_canvas_data(
    canvas_state: tauri::State<'_, Mutex<CanvasState>>,
    agent_state: tauri::State<'_, Mutex<AgentState>>,
) -> Result<CanvasData, AppError> {
    let settings = {
        let state = canvas_state.lock().await;
        state.settings.clone()
    };

    fetch_fresh(canvas_state, agent_state, &settings).await
}

/// Mark or unmark an assignment as manually completed.
///
/// # Arguments
///
/// * `assignment_id` - The Canvas assignment ID.
/// * `completed` - `true` to mark as completed, `false` to remove.
///
/// # Errors
///
/// Returns `AppError::Canvas` on database errors.
#[tauri::command]
pub async fn set_task_completion(
    canvas_state: tauri::State<'_, Mutex<CanvasState>>,
    agent_state: tauri::State<'_, Mutex<AgentState>>,
    assignment_id: i64,
    completed: bool,
) -> Result<(), AppError> {
    let agent = agent_state.lock().await;
    let aid = assignment_id;

    agent
        .db
        .call(move |conn| {
            if completed {
                conn.execute(
                    "INSERT OR IGNORE INTO canvas_completed_tasks (assignment_id) VALUES (?1)",
                    [aid],
                )?;
            } else {
                conn.execute(
                    "DELETE FROM canvas_completed_tasks WHERE assignment_id = ?1",
                    [aid],
                )?;
            }
            Ok(())
        })
        .await
        .map_err(|e: tokio_rusqlite::Error<rusqlite::Error>| {
            AppError::Canvas(format!("Failed to update completion: {e}"))
        })?;

    let mut state = canvas_state.lock().await;
    if let Some(ref mut cached) = state.cache {
        for a in &mut cached.data.assignments {
            if a.id == assignment_id {
                a.manually_completed = completed;
            }
        }
    }

    Ok(())
}

/// Return all assignment IDs that have been manually marked as completed.
///
/// # Errors
///
/// Returns `AppError::Canvas` on database errors.
#[tauri::command]
pub async fn get_completed_tasks(
    agent_state: tauri::State<'_, Mutex<AgentState>>,
) -> Result<Vec<i64>, AppError> {
    let agent = agent_state.lock().await;
    agent
        .db
        .call(|conn| {
            let mut stmt = conn.prepare("SELECT assignment_id FROM canvas_completed_tasks")?;
            let ids: Vec<i64> = stmt
                .query_map([], |row| row.get(0))?
                .filter_map(|r| r.ok())
                .collect();
            Ok(ids)
        })
        .await
        .map_err(|e: tokio_rusqlite::Error<rusqlite::Error>| {
            AppError::Canvas(format!("Failed to query completions: {e}"))
        })
}

/// Fetch fresh data from the Canvas API and update the cache.
async fn fetch_fresh(
    canvas_state: tauri::State<'_, Mutex<CanvasState>>,
    agent_state: tauri::State<'_, Mutex<AgentState>>,
    settings: &super::types::CanvasSettings,
) -> Result<CanvasData, AppError> {
    if settings.api_token.is_empty() {
        return Err(AppError::Canvas(
            "Canvas API token is not configured".to_string(),
        ));
    }

    let client = CanvasClient::new(&settings.api_url, &settings.api_token)?;
    let completed_ids = get_completed_set(&agent_state).await?;

    let all_courses = client.fetch_courses().await?;
    let ignored: HashSet<i64> = settings.ignored_course_ids.iter().copied().collect();
    let active_courses: Vec<_> = all_courses
        .iter()
        .filter(|c| !ignored.contains(&c.id))
        .collect();

    let mut assignments = Vec::new();
    for course in &active_courses {
        match client
            .fetch_assignments(course.id, &course.name, &completed_ids)
            .await
        {
            Ok(mut course_assignments) => assignments.append(&mut course_assignments),
            Err(e) => {
                eprintln!(
                    "Warning: failed to fetch assignments for {}: {e}",
                    course.name
                );
            }
        }
    }

    let now = chrono::Utc::now().to_rfc3339();
    let data = CanvasData {
        courses: all_courses,
        assignments,
        fetched_at: now,
    };

    let mut state = canvas_state.lock().await;
    state.cache = Some(CachedData {
        data: data.clone(),
        cached_at: Instant::now(),
    });

    Ok(data)
}

/// Load the set of manually completed assignment IDs from the database.
async fn get_completed_set(
    agent_state: &tauri::State<'_, Mutex<AgentState>>,
) -> Result<HashSet<i64>, AppError> {
    let agent = agent_state.lock().await;
    agent
        .db
        .call(|conn| {
            let mut stmt = conn.prepare("SELECT assignment_id FROM canvas_completed_tasks")?;
            let ids: HashSet<i64> = stmt
                .query_map([], |row| row.get(0))?
                .filter_map(|r| r.ok())
                .collect();
            Ok(ids)
        })
        .await
        .map_err(|e: tokio_rusqlite::Error<rusqlite::Error>| {
            AppError::Canvas(format!("Failed to query completions: {e}"))
        })
}
