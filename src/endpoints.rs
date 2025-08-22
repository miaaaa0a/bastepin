use crate::storage::Storage;
use askama::Template;
use axum::{
    Json,
    extract::{self, Path, State},
    http::StatusCode,
    response::{Html, IntoResponse},
};
use serde::{Deserialize, Serialize};

macro_rules! html {
    ($p: expr) => {
        std::fs::read_to_string($p).expect("error while reading html file")
    };
}

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

#[derive(Debug)]
pub enum AppError {
    Read,
    Write,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            AppError::Read => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Html(html!("./templates/error.html")),
            )
                .into_response(),
            AppError::Write => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(Response {
                    code: 500,
                    content: "error while writing".to_string(),
                }),
            )
                .into_response(),
        }
    }
}

const UPLOAD_LIMIT: usize = 1_048_576;
pub const DB_PATH: &str = "./storage";

#[axum::debug_handler]
pub async fn get_by_hash(
    State(state): State<Storage>,
    Path(hash): Path<String>,
) -> Result<Html<String>, AppError> {
    let result = state.get(hash).map_err(|_e| AppError::Read)?;

    if let Some(x) = result {
        let content = str::from_utf8(&x).map_err(|_e| AppError::Read)?;
        let html = UploadTemplate {
            content: content.to_string(),
        };
        let rendered = html.render().map_err(|_e| AppError::Read)?;
        Ok(Html(rendered))
    } else {
        Ok(Html(html!("./templates/error.html")))
    }
}

#[axum::debug_handler]
pub async fn upload(
    State(state): State<Storage>,
    extract::Json(payload): extract::Json<Response>,
) -> Result<Json<Response>, AppError> {
    if payload.content.len() > UPLOAD_LIMIT {
        return Ok(Json(Response {
            code: 413,
            content: format!(
                "upload too big! {} >>> {} bytes",
                payload.content.len(),
                UPLOAD_LIMIT
            ),
        }));
    }
    
    let result = state.write(&payload.content).map_err(|_e| AppError::Write)?;
    Ok(Json(Response {
        code: 200,
        content: result,
    }))
}
