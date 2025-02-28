use axum::{
    extract::FromRequestParts,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use axum_extra::extract::cookie::{Cookie, CookieJar};
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Claims {
    // Exemples de champs à récupérer depuis le token JWT
    sub: String,
    exp: usize,
}

pub async fn require_auth<B>(req: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
    // Extraction du CookieJar
    let jar = CookieJar::from_request(&req)
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    // Récupérer le cookie contenant le token (ici nommé "token")
    if let Some(token_cookie) = jar.get("token") {
        let token = token_cookie.value();

        // Définir la clé secrète et la configuration de validation
        let decoding_key = DecodingKey::from_secret("votre_secret".as_ref());
        let validation = Validation::new(Algorithm::HS256);

        // Décoder et vérifier le token
        match decode::<Claims>(token, &decoding_key, &validation) {
            Ok(_token_data) => {
                // Token valide, on passe à la suite
                Ok(next.run(req).await)
            }
            Err(_) => Err(StatusCode::UNAUTHORIZED),
        }
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}
