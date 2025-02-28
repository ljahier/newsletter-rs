use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    success: bool,
    data: Option<T>,
    error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(msg: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(msg),
        }
    }
}

pub fn response_success<T: Serialize>(status: StatusCode, data: T) -> impl IntoResponse {
    (status, Json(ApiResponse::success(data)))
}

pub fn response_err(status: StatusCode, msg: String) -> impl IntoResponse {
    (status, Json(ApiResponse::<()>::error(msg)))
}
