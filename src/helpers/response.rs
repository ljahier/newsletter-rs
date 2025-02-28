use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
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

pub fn response_success<T: Serialize>(status: StatusCode, data: T) -> Response {
    (status, Json(ApiResponse::success(data))).into_response()
}

pub fn response_err(status: StatusCode, msg: String) -> Response {
    (status, Json(ApiResponse::<()>::error(msg))).into_response()
}
