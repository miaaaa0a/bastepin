use std::error::Error;
use serde::{Deserialize, Serialize};
use tower_http::services::ServeDir;
use axum::{
    extract::{self, Path}, routing::{get, post}, Json, Router
};
use tracing::info;

#[derive(Debug, Deserialize, Serialize)]
struct Response {
    code: u16,
    content: String
}

pub mod storage;
pub mod encoding;
pub mod tests;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();
    
    let static_web = ServeDir::new("./web");
    let app = Router::new()
        .route("/api/upload", post(upload))
        .route("/{hash}", get(get_by_hash))
        .fallback_service(static_web);
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:2763").await?;
    axum::serve(listener, app).await?;
    Ok(())
}

#[axum::debug_handler]
async fn get_by_hash(Path(hash): Path<String>) -> Json<Response> {
    let hm = storage::read_into_hashmap().unwrap();
    let result = hm.get(&hash);
    info!("{}", hash);
    info!("{:?}", result);
    match result {
        Some(content) => Json(Response { code: 200, content: content.to_string() }),
        None => Json(Response { code: 404, content: "".to_string() })
    }
}

#[axum::debug_handler]
async fn upload(extract::Json(payload): extract::Json<Response>) -> Json<Response> {
    let encoded = encoding::encode(&payload.content).unwrap();
    let result = storage::write(&encoded);
    match result {
        Ok(h) => Json(Response{ code: 200, content: h }),
        Err(e) => Json(Response{ code: 502, content: format!("error while writing to db: {:?}", e) })
    }
}