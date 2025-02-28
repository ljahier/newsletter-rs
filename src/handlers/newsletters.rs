use axum::{extract::State, http::StatusCode, response::IntoResponse};

use crate::{AppState, helpers::response::response_success};

#[derive(Debug, sqlx::FromRow)]
pub struct Newsletter {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub created_at: String,
}

/// GET /newsletters
pub async fn list_newsletters(State(state): State<AppState>) -> impl IntoResponse {
    // Récupérer la liste des newsletters depuis la base de données
    response_success(StatusCode::OK, ())
}
