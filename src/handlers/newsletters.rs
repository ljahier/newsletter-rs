use std::collections::{HashMap, HashSet};

use crate::AppState;
use crate::helpers::response::{response_err, response_success};
use crate::models::newsletters::{NewsletterRaw, NewsletterRequest, NewsletterWithLists};
use axum::Json;
use axum::{extract::State, http::StatusCode, response::Response};
use chrono::{NaiveDateTime, TimeZone, Utc};
use uuid::Uuid;

pub async fn get_newsletters(State(state): State<AppState>) -> Response {
    let query = r#"
        select 
            s.id,
            s.name,
            s.send_date,
            s.status,
            s.content_html,
            s.content_plain,
            s.sent_at,
            s.sent_by,
            s.created_at,
            s.updated_at,
            group_concat(cl.name, ',') as contact_lists
        from sendings s
        left join sending_contact_lists scl on scl.sending_id = s.id
        left join contact_lists cl on cl.id = scl.contact_list_id
        where s.type = 'newsletter'
        group by s.id
        order by s.created_at desc
    "#;

    let raw_newsletters = match sqlx::query_as::<_, NewsletterRaw>(query)
        .fetch_all(&state.db_pool)
        .await
    {
        Ok(list) => list,
        Err(e) => {
            eprintln!("Erreur de récupération des newsletters: {:?}", e);
            return response_err(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Erreur de base de données".to_string(),
            );
        }
    };

    let newsletters: Vec<NewsletterWithLists> = raw_newsletters
        .into_iter()
        .map(|raw| {
            let lists = raw
                .contact_lists
                .map(|s| s.split(',').map(|s| s.to_string()).collect())
                .unwrap_or_else(Vec::new);
            NewsletterWithLists {
                id: raw.id,
                name: raw.name,
                send_date: raw.send_date,
                status: raw.status,
                content_html: raw.content_html,
                content_plain: raw.content_plain,
                sent_at: raw.sent_at,
                sent_by: raw.sent_by,
                created_at: raw.created_at,
                updated_at: raw.updated_at,
                contact_lists: lists,
            }
        })
        .collect();

    response_success(StatusCode::OK, newsletters)
}

pub async fn create_newsletter(
    State(state): State<AppState>,
    Json(payload): Json<NewsletterRequest>,
) -> Response {
    let (status, send_date, sent_at) = if payload.action == "send" {
        if let Some(ref send_date_str) = payload.send_date {
            match NaiveDateTime::parse_from_str(send_date_str, "%Y-%m-%dT%H:%M") {
                Ok(naive_dt) => {
                    let dt = Utc.from_utc_datetime(&naive_dt);
                    ("pending", Some(dt), None)
                }
                Err(e) => {
                    eprintln!("Erreur de parsing de send_date: {:?}", e);
                    return response_err(
                        StatusCode::BAD_REQUEST,
                        "Format de date invalide".to_string(),
                    );
                }
            }
        } else {
            ("sent", None, Some(Utc::now()))
        }
    } else if payload.action == "save" {
        ("draft", None, None)
    } else {
        return response_err(StatusCode::BAD_REQUEST, "Action invalide".to_string());
    };

    let (content_plain, content_html) = if payload.content_type.to_lowercase() == "text" {
        (Some(payload.content), None)
    } else if payload.content_type.to_lowercase() == "html" {
        (None, Some(payload.content))
    } else {
        return response_err(
            StatusCode::BAD_REQUEST,
            "Type de contenu invalide".to_string(),
        );
    };

    let id = Uuid::new_v4().to_string();

    let result = sqlx::query(
        "insert into sendings (id, type, name, send_date, status, content_html, content_plain, sent_at, sent_by)
         values (?, 'newsletter', ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&id)
    .bind(&payload.name)
    .bind(send_date)
    .bind(status)
    .bind(content_html)
    .bind(content_plain)
    .bind(sent_at)
    .bind(None::<String>)
    .execute(&state.db_pool)
    .await;

    if let Err(e) = result {
        eprintln!("Erreur lors de la création de la newsletter: {:?}", e);
        return response_err(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Erreur lors de la création".to_string(),
        );
    }

    if let Some(list_ids) = payload.contact_list_ids {
        let unique_ids: HashSet<String> = list_ids.into_iter().collect();
        for list_id in unique_ids {
            let assoc_result: Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> = sqlx::query(
                "INSERT INTO sending_contact_lists (sending_id, contact_list_id) VALUES (?, ?)",
            )
            .bind(&id)
            .bind(list_id)
            .execute(&state.db_pool)
            .await;
            if let Err(e) = assoc_result {
                eprintln!(
                    "Erreur lors de l'association de la newsletter avec une liste: {:?}",
                    e
                );
                return response_err(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Erreur lors de l'association de la newsletter aux listes".to_string(),
                );
            }
        }
    }

    response_success(StatusCode::CREATED, "Newsletter créée".to_string())
}
