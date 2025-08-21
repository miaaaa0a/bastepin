use crate::{endpoints::DB_PATH, storage::Storage};
use axum::{
    Router,
    extract::DefaultBodyLimit,
    routing::{get, post},
};
use std::error::Error;
use tower_http::services::ServeDir;
pub mod encoding;
pub mod endpoints;
pub mod storage;
pub mod tests;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();

    let state = Storage::new(DB_PATH);

    let static_web = ServeDir::new("./web");
    let app = Router::new()
        .route("/api/upload", post(endpoints::upload))
        .route("/{hash}", get(endpoints::get_by_hash))
        .with_state(state)
        .fallback_service(static_web)
        .layer(DefaultBodyLimit::disable());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:2763").await?;
    axum::serve(listener, app).await?;
    Ok(())
}
