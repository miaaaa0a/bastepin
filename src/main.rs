use askama::Template;
use axum::{
    Json, Router,
    extract::{self, DefaultBodyLimit, Path},
    response::Html,
    routing::{get, post},
};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::error::Error;
use tower_http::services::ServeDir;

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();
    dotenv()?;

    let static_web = ServeDir::new("./web");
    let app = Router::new()
        .route("/api/upload", post(upload))
        .route("/{hash}", get(get_by_hash))
        .fallback_service(static_web)
        .layer(DefaultBodyLimit::disable());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:2763").await?;
    axum::serve(listener, app).await?;
    Ok(())
}

#[axum::debug_handler]
async fn get_by_hash(Path(hash): Path<String>) -> Html<String> {
    let hm = storage::read_into_hashmap().unwrap();
    let result = hm.get(&hash);
    /*match result {
        Some(content) => Json(Response { code: 200, content: content.to_string() }),
        None => Json(Response { code: 404, content: "".to_string() })
    }*/
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
async fn upload(extract::Json(payload): extract::Json<Response>) -> Json<Response> {
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
    let result = storage::write(&encoded);
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
