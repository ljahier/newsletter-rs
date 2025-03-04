use crate::AppState;
use crate::helpers::response::{response_err, response_success};
use crate::models::contact::{ContactListWithMembers, NewContactRequest};
use crate::models::contact_lists::{ContactList, NewContactListRequest};
use axum::Json;
use axum::extract::Path;
use axum::response::Response;
use axum::{extract::State, http::StatusCode};
use chrono::Utc;
use sqlx::Row;
use tracing::error;
use uuid::Uuid;

#[tracing::instrument(skip(state))]
pub async fn list_contact_lists(State(state): State<AppState>) -> Response {
    let lists = match sqlx::query_as::<_, ContactList>(
        "select id, name, type, created_at, updated_at from contact_lists",
    )
    .fetch_all(&state.db_pool)
    .await
    {
        Ok(list) => list,
        Err(e) => {
            error!("Erreur de récupération des listes de contacts: {:?}", e);
            return response_err(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Erreur de base de données".to_string(),
            );
        }
    };

    response_success(StatusCode::OK, lists)
}
#[tracing::instrument(skip(state))]
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
            error!(
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

#[tracing::instrument(skip(state))]
pub async fn get_contact_list_by_id(
    State(state): State<AppState>,
    Path(list_id): Path<String>,
) -> Response {
    let contact_list = sqlx::query_as::<_, ContactList>(
        "SELECT id, name, type, created_at, updated_at FROM contact_lists WHERE id = ?",
    )
    .bind(&list_id)
    .fetch_optional(&state.db_pool)
    .await;

    if let Err(e) = contact_list {
        error!(
            "Erreur lors de la récupération de la liste de contacts: {:?}",
            e
        );
        return response_err(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Erreur de base de données".to_string(),
        );
    }

    let contact_list = contact_list.unwrap();
    if contact_list.is_none() {
        return response_err(
            StatusCode::NOT_FOUND,
            "Liste de contacts non trouvée".to_string(),
        );
    }
    let list = contact_list.unwrap();

    let members_query =
        sqlx::query("SELECT contact_id FROM contact_list_members WHERE list_id = ?")
            .bind(&list_id)
            .fetch_all(&state.db_pool)
            .await;

    if let Err(e) = members_query {
        error!("Erreur lors de la récupération des membres: {:?}", e);
        return response_err(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Erreur lors de la récupération des membres".to_string(),
        );
    }

    let member_ids = members_query
        .unwrap()
        .into_iter()
        .map(|row| row.get::<String, _>("contact_id"))
        .collect::<Vec<String>>();

    let result = ContactListWithMembers {
        id: list.id,
        name: list.name,
        list_type: list.list_type,
        created_at: list.created_at,
        updated_at: list.updated_at,
        members: member_ids,
    };

    response_success(StatusCode::OK, result)
}

#[tracing::instrument(skip(state))]
pub async fn create_contact(
    State(state): State<AppState>,
    Path(list_id): Path<String>,
    Json(payload): Json<NewContactRequest>,
) -> Response {
    let contact_id = Uuid::new_v4().to_string();

    if let Err(err) = sqlx::query(
        "insert into contacts (id, first_name, last_name, address, postal_code, city, email, custom_fields, created_at, updated_at)
         values (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&contact_id)
    .bind(&payload.first_name)
    .bind(&payload.last_name)
    .bind(&payload.address)
    .bind(&payload.postal_code)
    .bind(&payload.city)
    .bind(&payload.email)
    .bind(&payload.custom_fields)
    .bind(Utc::now())
    .bind(Utc::now())
    .execute(&state.db_pool)
    .await {
        error!("Erreur lors de la création du contact: {:?}", err);
        return response_err(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Erreur lors de la création du contact".to_string(),
        );
    }

    if let Err(err) = sqlx::query(
        "insert into contact_list_members (contact_id, list_id)
         values (?, ?)",
    )
    .bind(&contact_id)
    .bind(&list_id)
    .execute(&state.db_pool)
    .await
    {
        error!(
            "Erreur lors de l'association du contact à la liste: {:?}",
            err
        );
        return response_err(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Erreur lors de l'association du contact à la liste".to_string(),
        );
    };

    response_success(StatusCode::CREATED, "Contact créé et ajouté à la liste")
}
