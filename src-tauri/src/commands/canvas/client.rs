use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use reqwest::Client;
use serde::Deserialize;

use crate::error::AppError;

use super::types::{CanvasAssignment, CanvasCourse};

/// Raw course object returned by the Canvas API.
#[derive(Debug, Deserialize)]
struct ApiCourse {
    id: i64,
    name: Option<String>,
    course_code: Option<String>,
}

/// Raw assignment object returned by the Canvas API.
#[derive(Debug, Deserialize)]
struct ApiAssignment {
    id: i64,
    course_id: i64,
    name: Option<String>,
    due_at: Option<String>,
    points_possible: Option<f64>,
    html_url: Option<String>,
    #[serde(default)]
    submission_types: Vec<String>,
    #[serde(default)]
    has_submitted_submissions: bool,
    #[serde(default)]
    is_quiz_assignment: bool,
}

/// Canvas API HTTP client with pagination support.
pub struct CanvasClient {
    client: Client,
    base_url: String,
}

impl CanvasClient {
    /// Create a new Canvas API client.
    ///
    /// # Arguments
    ///
    /// * `base_url` - The Canvas instance base URL (e.g. `https://canvas.uts.edu.au`).
    /// * `token` - A personal access token for authentication.
    ///
    /// # Errors
    ///
    /// Returns `AppError::Canvas` if the HTTP client cannot be built.
    pub fn new(base_url: &str, token: &str) -> Result<Self, AppError> {
        let mut headers = HeaderMap::new();
        let auth_value = format!("Bearer {token}");
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&auth_value)
                .map_err(|e| AppError::Canvas(format!("Invalid token: {e}")))?,
        );

        let client = Client::builder()
            .default_headers(headers)
            .build()
            .map_err(|e| AppError::Canvas(format!("Failed to build HTTP client: {e}")))?;

        let base_url = base_url.trim_end_matches('/').to_string();
        Ok(Self { client, base_url })
    }

    /// Fetch all active courses for the authenticated user.
    ///
    /// # Errors
    ///
    /// Returns `AppError::Canvas` on network or API errors.
    pub async fn fetch_courses(&self) -> Result<Vec<CanvasCourse>, AppError> {
        let url = format!(
            "{}/api/v1/courses?enrollment_state=active&per_page=100",
            self.base_url
        );
        let items: Vec<ApiCourse> = self.fetch_paginated(&url).await?;

        Ok(items
            .into_iter()
            .map(|c| CanvasCourse {
                id: c.id,
                name: c.name.unwrap_or_default(),
                course_code: c.course_code.unwrap_or_default(),
            })
            .collect())
    }

    /// Fetch all assignments for a given course.
    ///
    /// # Arguments
    ///
    /// * `course_id` - The Canvas course ID.
    /// * `course_name` - The display name to attach to each assignment.
    /// * `completed_ids` - Set of assignment IDs manually marked as completed.
    ///
    /// # Errors
    ///
    /// Returns `AppError::Canvas` on network or API errors.
    pub async fn fetch_assignments(
        &self,
        course_id: i64,
        course_name: &str,
        completed_ids: &std::collections::HashSet<i64>,
    ) -> Result<Vec<CanvasAssignment>, AppError> {
        let url = format!(
            "{}/api/v1/courses/{}/assignments?include[]=submission&per_page=100&order_by=due_at",
            self.base_url, course_id
        );
        let items: Vec<ApiAssignment> = self.fetch_paginated(&url).await?;

        Ok(items
            .into_iter()
            .map(|a| CanvasAssignment {
                id: a.id,
                course_id: a.course_id,
                course_name: course_name.to_string(),
                name: a.name.unwrap_or_default(),
                due_at: a.due_at,
                points_possible: a.points_possible,
                html_url: a.html_url.unwrap_or_default(),
                submission_types: a.submission_types,
                has_submitted_submissions: a.has_submitted_submissions,
                is_quiz: a.is_quiz_assignment,
                manually_completed: completed_ids.contains(&a.id),
            })
            .collect())
    }

    /// Fetch all pages for a paginated Canvas API endpoint.
    ///
    /// # Arguments
    ///
    /// * `url` - The initial request URL.
    ///
    /// # Returns
    ///
    /// A combined vector of all items across all pages.
    ///
    /// # Errors
    ///
    /// Returns `AppError::Canvas` on network, deserialization, or HTTP errors.
    async fn fetch_paginated<T: serde::de::DeserializeOwned>(
        &self,
        url: &str,
    ) -> Result<Vec<T>, AppError> {
        let mut all_items = Vec::new();
        let mut next_url = Some(url.to_string());

        while let Some(current_url) = next_url.take() {
            let resp = self
                .client
                .get(&current_url)
                .send()
                .await
                .map_err(|e| AppError::Canvas(format!("Request failed: {e}")))?;

            if !resp.status().is_success() {
                let status = resp.status();
                let body = resp.text().await.unwrap_or_default();
                return Err(AppError::Canvas(format!("API returned {status}: {body}")));
            }

            next_url = parse_next_link(resp.headers());

            let items: Vec<T> = resp
                .json()
                .await
                .map_err(|e| AppError::Canvas(format!("Failed to parse response: {e}")))?;

            all_items.extend(items);
        }

        Ok(all_items)
    }
}

/// Extract the `rel="next"` URL from the Link response header.
fn parse_next_link(headers: &HeaderMap) -> Option<String> {
    let link_header = headers.get("link")?.to_str().ok()?;
    for part in link_header.split(',') {
        let part = part.trim();
        if part.ends_with("rel=\"next\"") {
            let url = part.split('>').next()?.trim().trim_start_matches('<');
            return Some(url.to_string());
        }
    }
    None
}
