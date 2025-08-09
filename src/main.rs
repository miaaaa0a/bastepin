use std::error::Error;
use askama::Template;
use serde::{Deserialize, Serialize};
use tower_http::services::ServeDir;
use axum::{
    extract::{self, Path}, response::Html, routing::{get, post}, Json, Router
};

#[derive(Debug, Deserialize, Serialize)]
struct Response {
    code: u16,
    content: String
}

#[derive(Template)]
#[template(path = "upload.html")]
struct UploadTemplate {
    content: String
}

macro_rules! html {
    ($p: expr) => {
        std::fs::read_to_string($p).unwrap()
    };
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
async fn get_by_hash(Path(hash): Path<String>) -> Html<String> {
    let hm = storage::read_into_hashmap().unwrap();
    let result = hm.get(&hash);
    /*match result {
        Some(content) => Json(Response { code: 200, content: content.to_string() }),
        None => Json(Response { code: 404, content: "".to_string() })
    }*/
    if let Some(x) = result {
        let html = UploadTemplate { content: x.to_string() };
        Html(html.render().unwrap())
    } else {
        Html(html!("./templates/error.html"))
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