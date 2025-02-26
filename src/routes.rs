use axum::Router;
use axum::body::Body;
use axum::http::{Response, StatusCode, header};
use axum::routing::{get, post};
use tokio::fs;

use crate::AppState;
use crate::handlers::{
    app::{app_handler, app_login_handler},
    auth::login,
};

pub fn create_routes(state: AppState) -> Router {
    Router::new()
        .route("/", get(app_handler))
        .route("/login", get(app_login_handler))
        .route("/auth/login", post(login))
        .route(
            "/public/embed.js",
            get(|| async {
                match fs::read("./res/public/embed.js").await {
                    Ok(content) => Response::builder()
                        .status(StatusCode::OK)
                        .header(header::CONTENT_TYPE, "application/javascript")
                        .body(Body::from(content))
                        .unwrap(),
                    Err(err) => {
                        println!("debug err = {}", err);
                        return Response::builder()
                            .status(StatusCode::NOT_FOUND)
                            .body(Body::from("File not found"))
                            .unwrap();
                    }
                }
            }),
        )
        .with_state(state)
}
