use axum::{http::StatusCode, response::Html, routing::get, Router};
use std::net::SocketAddr;

/// The website is embedded into the binary at compile time.
/// Edit `index.html`, then rebuild with `cargo run`.
const INDEX_HTML: &str = include_str!("../index.html");

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(home))
        .route("/index.html", get(home))
        .fallback(not_found);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running at http://{addr}");

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("failed to bind TCP listener");

    axum::serve(listener, app)
        .await
        .expect("server error");
}

async fn home() -> Html<&'static str> {
    Html(INDEX_HTML)
}

async fn not_found() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "404 Not Found")
}
