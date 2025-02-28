use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{Router, middleware};
use serde_json::json;

use crate::AppState;
use crate::handlers::auth::login;
use crate::handlers::newsletters::{create_newsletter, get_newsletters};
use crate::helpers::{auth::auth_middleware, response::response_success};

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
                .route("/", post(create_newsletter)),
        )
        .with_state(state.clone())
        .layer(middleware::from_fn(auth_middleware));

    Router::new().nest("/api", public_api_routes.merge(private_api_routes))
}
