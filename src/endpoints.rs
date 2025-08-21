use askama::Template;
use axum::{
    Json,
    extract::{self, Path, State},
    response::Html,
};
use serde::{Deserialize, Serialize};
use crate::{
    storage::Storage,
    encoding
};

#[derive(Debug, Deserialize, Serialize)]
pub struct Response {
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

const UPLOAD_LIMIT: usize = 1_048_576;
pub const DB_PATH: &str = "./storage";

#[axum::debug_handler]
pub async fn get_by_hash(State(state): State<Storage>, Path(hash): Path<String>) -> Html<String> {
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
pub async fn upload(
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
