use crate::db::Database;
use axum::{
    extract::{Path, Query, State},
    http::{header, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

/// The quiz in `index.html` always has eight questions.
const QUIZ_TOTAL: i64 = 8;
const MAX_NAME_CHARS: usize = 20;
const MAX_PROGRESS_STEPS: usize = 20;

#[derive(Clone)]
pub struct AppState {
    pub db: Database,
}

#[derive(Deserialize)]
pub struct LeaderboardQuery {
    limit: Option<u32>,
}

#[derive(Deserialize)]
pub struct QuizSubmission {
    name: String,
    score: i64,
    total: i64,
}

#[derive(Deserialize)]
pub struct ProgressPayload {
    completed: Vec<bool>,
}

#[derive(Serialize)]
struct Health {
    status: &'static str,
    database: &'static str,
    version: &'static str,
}

/// `GET /api/health`
pub async fn health(State(state): State<AppState>) -> Response {
    let db = state.db.clone();

    match spawn_db("health", move || db.health()).await {
        Ok(()) => json_response(
            StatusCode::OK,
            Health {
                status: "ok",
                database: "sqlite",
                version: env!("CARGO_PKG_VERSION"),
            },
        ),
        Err(response) => response,
    }
}

/// `GET /api/leaderboard?limit=10`
pub async fn leaderboard(
    State(state): State<AppState>,
    Query(query): Query<LeaderboardQuery>,
) -> Response {
    let limit = query.limit.unwrap_or(10).clamp(1, 50) as usize;
    let db = state.db.clone();

    match spawn_db("leaderboard", move || db.leaderboard(limit)).await {
        Ok(entries) => json_response(StatusCode::OK, json!({ "entries": entries })),
        Err(response) => response,
    }
}

/// `POST /api/quiz-attempts`
pub async fn submit_quiz(
    State(state): State<AppState>,
    Json(payload): Json<QuizSubmission>,
) -> Response {
    if payload.total != QUIZ_TOTAL {
        return error_response(
            StatusCode::UNPROCESSABLE_ENTITY,
            "total must match the 8-question quiz",
        );
    }

    if payload.score < 0 || payload.score > payload.total {
        return error_response(StatusCode::UNPROCESSABLE_ENTITY, "score is out of range");
    }

    let name = payload.name.trim();
    let name_len = name.chars().count();
    if !(2..=MAX_NAME_CHARS).contains(&name_len) {
        return error_response(
            StatusCode::UNPROCESSABLE_ENTITY,
            "name must be between 2 and 20 characters",
        );
    }
    if name.chars().any(char::is_control) {
        return error_response(
            StatusCode::UNPROCESSABLE_ENTITY,
            "name contains invalid characters",
        );
    }

    let name = name.to_owned();
    let score = payload.score;
    let total = payload.total;
    let db = state.db.clone();

    match spawn_db("submit_quiz", move || {
        let attempt = db.insert_attempt(&name, score, total)?;
        let (rank, best) = db.player_standing(&name, total)?;
        Ok((attempt, rank, best))
    })
    .await
    {
        Ok((attempt, rank, best)) => json_response(
            StatusCode::CREATED,
            json!({ "ok": true, "rank": rank, "best": best, "attempt": attempt }),
        ),
        Err(response) => response,
    }
}

/// `GET /api/progress/:client_id`
pub async fn get_progress(
    State(state): State<AppState>,
    Path(client_id): Path<String>,
) -> Response {
    if let Err(response) = validate_client_id(&client_id) {
        return response;
    }

    let db = state.db.clone();
    match spawn_db("get_progress", move || db.get_progress(&client_id)).await {
        Ok(progress) => json_response(StatusCode::OK, progress),
        Err(response) => response,
    }
}

/// `PUT /api/progress/:client_id`
pub async fn put_progress(
    State(state): State<AppState>,
    Path(client_id): Path<String>,
    Json(payload): Json<ProgressPayload>,
) -> Response {
    if let Err(response) = validate_client_id(&client_id) {
        return response;
    }
    if payload.completed.len() > MAX_PROGRESS_STEPS {
        return error_response(
            StatusCode::UNPROCESSABLE_ENTITY,
            "too many progress steps",
        );
    }

    let db = state.db.clone();
    match spawn_db("put_progress", move || {
        db.put_progress(&client_id, &payload.completed)
    })
    .await
    {
        Ok(progress) => json_response(StatusCode::OK, progress),
        Err(response) => response,
    }
}

/// `DELETE /api/progress/:client_id`
pub async fn delete_progress(
    State(state): State<AppState>,
    Path(client_id): Path<String>,
) -> Response {
    if let Err(response) = validate_client_id(&client_id) {
        return response;
    }

    let db = state.db.clone();
    match spawn_db("delete_progress", move || db.delete_progress(&client_id)).await {
        Ok(()) => {
            let mut response = StatusCode::NO_CONTENT.into_response();
            response
                .headers_mut()
                .insert(header::CACHE_CONTROL, HeaderValue::from_static("no-store"));
            response
        }
        Err(response) => response,
    }
}

fn validate_client_id(client_id: &str) -> Result<(), Response> {
    let valid_length = (8..=64).contains(&client_id.len());
    let valid_characters = client_id
        .bytes()
        .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-' || byte == b'_');

    if valid_length && valid_characters {
        Ok(())
    } else {
        Err(error_response(
            StatusCode::BAD_REQUEST,
            "invalid client id",
        ))
    }
}

/// Run a blocking SQLite operation without stalling the Tokio runtime.
async fn spawn_db<T, F>(label: &'static str, operation: F) -> Result<T, Response>
where
    T: Send + 'static,
    F: FnOnce() -> rusqlite::Result<T> + Send + 'static,
{
    match tokio::task::spawn_blocking(operation).await {
        Ok(Ok(value)) => Ok(value),
        Ok(Err(error)) => {
            eprintln!("database error in {label}: {error}");
            Err(error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                "database error",
            ))
        }
        Err(error) => {
            eprintln!("database task failed in {label}: {error}");
            Err(error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                "database task failed",
            ))
        }
    }
}

fn json_response<T: Serialize>(status: StatusCode, body: T) -> Response {
    let mut response = Json(body).into_response();
    *response.status_mut() = status;
    response
        .headers_mut()
        .insert(header::CACHE_CONTROL, HeaderValue::from_static("no-store"));
    response
}

fn error_response(status: StatusCode, message: &str) -> Response {
    json_response(status, json!({ "error": message }))
}
