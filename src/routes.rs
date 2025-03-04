use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{Router, middleware};
use serde_json::json;

use crate::AppState;
use crate::handlers::auth::login;
use crate::handlers::contact_lists::{create_contact_list, list_contact_lists};
use crate::handlers::newsletters::{create_newsletter, get_newsletters, send_newsletter};
use crate::helpers::{auth::auth_middleware, response::response_success};
use crate::telemetry::request_id_middleware;

pub fn create_routes(state: &AppState) -> Router {
    let public_api_routes = Router::new()
        .route("/login", post(login))
        .with_state(state.clone());
    let private_api_routes = Router::new()
        .route(
            "/ping",
            get(|| async { response_success(StatusCode::OK, json!({"message":"pong"})) }),
        )
        .nest(
            "/newsletters",
            Router::new()
                .route("/", get(get_newsletters))
                .route("/", post(create_newsletter))
                .route("/{id}/send", post(send_newsletter)), // test route
        )
        .nest(
            "/contact_lists",
            Router::new()
                .route("/", post(create_contact_list))
                .route("/", get(list_contact_lists)),
        )
        .with_state(state.clone())
        .layer(middleware::from_fn(auth_middleware))
        .layer(middleware::from_fn(request_id_middleware));

    Router::new().nest("/api", public_api_routes.merge(private_api_routes))
}
