use axum::response::Html;
use tokio::fs;

use crate::helpers::auth::Session;

pub async fn app_handler(session: Session) -> Html<String> {
    // // Pour l'exemple, nous simulons ici l'authentification
    // // Remplacer par une vérification réelle (ex. via des cookies ou un middleware)
    // let is_authenticated = false;

    println!("debug {}", session.email);

    let html = fs::read_to_string("res/app/app.html")
        .await
        .unwrap_or_else(|_| "<h1>Erreur lors du chargement de l'app</h1>".to_string());
    Html(html)
    // if is_authenticated {
    //     // L'utilisateur est authentifié, on charge le layout de l'app
    //     Ok(Html(html))
    // } else {
    //     // Non authentifié, redirection vers la page de login
    //     Err(Redirect::to("/login"))
    // }
}

pub async fn app_login_handler() -> Html<String> {
    let html = fs::read_to_string("res/app/login.html")
        .await
        .unwrap_or_else(|_| "<h1>Erreur lors du chargement de la page de login</h1>".to_string());
    Html(html)
}
