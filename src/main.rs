use askama::Template;
use axum::{
    Json, Router,
    extract::{self, DefaultBodyLimit, Path, State},
    response::Html,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use std::error::Error;
use tower_http::services::ServeDir;

use crate::storage::Storage;

#[derive(Debug, Deserialize, Serialize)]
struct Response {
    code: u16,
    content: String,
}

#[derive(Template)]
#[template(path = "upload.html")]
struct UploadTemplate {
    content: String,
}

macro_rules! html {
    ($p: expr) => {
        std::fs::read_to_string($p).unwrap()
    };
}

pub mod encoding;
pub mod storage;
pub mod tests;

const UPLOAD_LIMIT: usize = 1_048_576;
const DB_PATH: &str = "./storage";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();

    let state = Storage::new(DB_PATH);

    let static_web = ServeDir::new("./web");
    let app = Router::new()
        .route("/api/upload", post(upload))
        .route("/{hash}", get(get_by_hash))
        .with_state(state)
        .fallback_service(static_web)
        .layer(DefaultBodyLimit::disable());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:2763").await?;
    axum::serve(listener, app).await?;
    Ok(())
}

#[axum::debug_handler]
async fn get_by_hash(State(state): State<Storage>, Path(hash): Path<String>) -> Html<String> {
    let result = state.get(hash);

    if let Ok(Some(x)) = result {
        let content = str::from_utf8(&x).unwrap();
        let html = UploadTemplate {
            content: content.to_string(),
        };
        Html(html.render().unwrap())
    } else {
        Html(html!("./templates/error.html"))
    }
}

#[axum::debug_handler]
async fn upload(
    State(state): State<Storage>,
    extract::Json(payload): extract::Json<Response>,
) -> Json<Response> {
    if payload.content.len() > UPLOAD_LIMIT {
        return Json(Response {
            code: 413,
            content: format!(
                "upload too big! {} >>> {} bytes",
                payload.content.len(),
                UPLOAD_LIMIT
            ),
        });
    }

    let encoded = encoding::encode(&payload.content).unwrap();
    let result = state.write(&encoded);
    match result {
        Ok(h) => Json(Response {
            code: 200,
            content: h,
        }),
        Err(e) => Json(Response {
            code: 502,
            content: format!("error while writing to db: {:?}", e),
        }),
    }
}
