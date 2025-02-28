use axum::Router;
use axum::body::Body;
use axum::http::{Response, StatusCode, header};
use axum::routing::{get, post};
use tokio::fs;

use crate::AppState;
use crate::handlers::newsletters::list_newsletters;
use crate::handlers::{
    app::{app_handler, app_login_handler},
    auth::login,
};

pub fn create_routes(state: AppState) -> Router {
    let api_routes = Router::new()
        .route("/auth/login", post(login))
        .route("/newsletters", get(list_newsletters))
        .with_state(state);

    Router::new().nest("/api", api_routes)
}
