mod api;
mod db;

use api::AppState;
use axum::{
    body::Body,
    http::{header, HeaderMap, HeaderValue, StatusCode},
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Router,
};
use db::Database;
use std::net::SocketAddr;
use std::path::Path;

/// The website and its media are embedded into the binary at compile time.
/// Edit `index.html` or `assets/`, then rebuild with `cargo run`.
const INDEX_HTML: &str = include_str!("../index.html");
const CREATOR_IMAGE: &[u8] = include_bytes!("../assets/creator.jpg");
const CREATOR_IMAGE_SM: &[u8] = include_bytes!("../assets/creator-sm.jpg");

#[tokio::main]
async fn main() {
    let db_path =
        std::env::var("PLH_DB_PATH").unwrap_or_else(|_| "python-learning-hub.db".to_owned());
    let database = Database::open(Path::new(&db_path)).unwrap_or_else(|error| {
        eprintln!("failed to open database at {db_path}: {error}");
        std::process::exit(1);
    });
    println!("SQLite database ready at {db_path}");

    let app = Router::new()
        .route("/", get(home))
        .route("/index.html", get(home))
        .route("/assets/creator.jpg", get(creator_image))
        .route("/assets/creator-sm.jpg", get(creator_image_sm))
        .route("/api/health", get(api::health))
        .route("/api/leaderboard", get(api::leaderboard))
        .route("/api/quiz-attempts", post(api::submit_quiz))
        .route(
            "/api/progress/:client_id",
            get(api::get_progress)
                .put(api::put_progress)
                .delete(api::delete_progress),
        )
        .with_state(AppState { db: database })
        .fallback(not_found);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running at http://{addr}");

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("failed to bind TCP listener");

    axum::serve(listener, app).await.expect("server error");
}

async fn home() -> Response {
    let mut response = Html(INDEX_HTML).into_response();
    // Lets the frontend detect the SQLite backend without probing a 404 route.
    response
        .headers_mut()
        .insert("x-plh-backend", HeaderValue::from_static("sqlite"));
    response
}

async fn creator_image(headers: HeaderMap) -> Response {
    serve_asset(&headers, CREATOR_IMAGE, "image/jpeg")
}

async fn creator_image_sm(headers: HeaderMap) -> Response {
    serve_asset(&headers, CREATOR_IMAGE_SM, "image/jpeg")
}

/// Serve embedded bytes with basic HTTP Range support.
fn serve_asset(headers: &HeaderMap, data: &'static [u8], content_type: &'static str) -> Response {
    let total = data.len();
    let cache = HeaderValue::from_static("public, max-age=31536000, immutable");

    if let Some(range_header) = headers.get(header::RANGE).and_then(|v| v.to_str().ok()) {
        if let Some(spec) = range_header.strip_prefix("bytes=") {
            if !spec.contains(',') {
                let mut parts = spec.split('-');
                let start_raw = parts.next().unwrap_or("").trim();
                let end_raw = parts.next().unwrap_or("").trim();
                let start = start_raw.parse::<usize>().ok();
                let end = end_raw.parse::<usize>().ok();

                if let Some(start) = start {
                    if start >= total {
                        let mut response = Response::new(Body::empty());
                        *response.status_mut() = StatusCode::RANGE_NOT_SATISFIABLE;
                        response.headers_mut().insert(
                            header::CONTENT_RANGE,
                            HeaderValue::from_str(&format!("bytes */{total}")).unwrap(),
                        );
                        return response;
                    }

                    let end = end.unwrap_or(total - 1).min(total - 1).max(start);
                    let slice = &data[start..=end];

                    let mut response = Response::new(Body::from(slice));
                    *response.status_mut() = StatusCode::PARTIAL_CONTENT;
                    let h = response.headers_mut();
                    h.insert(header::CONTENT_TYPE, HeaderValue::from_static(content_type));
                    h.insert(
                        header::CONTENT_RANGE,
                        HeaderValue::from_str(&format!("bytes {start}-{end}/{total}")).unwrap(),
                    );
                    h.insert(
                        header::CONTENT_LENGTH,
                        HeaderValue::from_str(&(end - start + 1).to_string()).unwrap(),
                    );
                    h.insert(header::ACCEPT_RANGES, HeaderValue::from_static("bytes"));
                    h.insert(header::CACHE_CONTROL, cache);
                    return response;
                }
            }
        }
    }

    let mut response = Response::new(Body::from(data));
    let h = response.headers_mut();
    h.insert(header::CONTENT_TYPE, HeaderValue::from_static(content_type));
    h.insert(
        header::CONTENT_LENGTH,
        HeaderValue::from_str(&total.to_string()).unwrap(),
    );
    h.insert(header::ACCEPT_RANGES, HeaderValue::from_static("bytes"));
    h.insert(header::CACHE_CONTROL, cache);
    response
}

async fn not_found() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "404 Not Found")
}
