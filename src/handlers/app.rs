use axum::{
    http::{StatusCode, Uri},
    response::{Html, IntoResponse},
};
use tokio::fs;

use crate::helpers::auth::Session;

pub async fn app_handler(session: Session, uri: Uri) -> impl IntoResponse {
    println!("debug {}", session.email);

    if uri.path().starts_with("/api/") {
        return StatusCode::NOT_FOUND.into_response();
    }

    let html = fs::read_to_string("res/app/app.html")
        .await
        .unwrap_or_else(|_| "<h1>Erreur lors du chargement de l'app</h1>".to_string());
    Html(html).into_response()
}

pub async fn app_login_handler() -> Html<String> {
    let html = fs::read_to_string("res/app/login.html")
        .await
        .unwrap_or_else(|_| "<h1>Erreur lors du chargement de la page de login</h1>".to_string());
    Html(html)
}
