// src/handlers/contact_lists.rs

use crate::AppState;
use crate::helpers::response::{response_err, response_success};
use crate::models::contact_lists::{ContactList, NewContactListRequest};
use axum::Json;
use axum::response::Response;
use axum::{extract::State, http::StatusCode};
use uuid::Uuid;

pub async fn list_contact_lists(State(state): State<AppState>) -> Response {
    let lists = match sqlx::query_as::<_, ContactList>(
        "select id, name, type, created_at, updated_at from contact_lists",
    )
    .fetch_all(&state.db_pool)
    .await
    {
        Ok(list) => list,
        Err(e) => {
            eprintln!("Erreur de récupération des listes de contacts: {:?}", e);
            return response_err(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Erreur de base de données".to_string(),
            );
        }
    };

    response_success(StatusCode::OK, lists)
}

pub async fn create_contact_list(
    State(state): State<AppState>,
    Json(payload): Json<NewContactListRequest>,
) -> Response {
    let list_type = payload.list_type.to_lowercase();
    if list_type != "automatic" && list_type != "manual" {
        return response_err(
            StatusCode::BAD_REQUEST,
            "Type de liste invalide. Doit être 'automatic' ou 'manual'.".to_string(),
        );
    }

    let id = Uuid::new_v4().to_string();
    let result = sqlx::query("insert into contact_lists (id, name, type) values (?, ?, ?)")
        .bind(&id)
        .bind(&payload.name)
        .bind(&list_type)
        .execute(&state.db_pool)
        .await;

    match result {
        Ok(_) => response_success(StatusCode::CREATED, "Liste de contacts créée".to_string()),
        Err(e) => {
            eprintln!(
                "Erreur lors de la création de la liste de contacts: {:?}",
                e
            );
            response_err(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Erreur lors de la création de la liste".to_string(),
            )
        }
    }
}
