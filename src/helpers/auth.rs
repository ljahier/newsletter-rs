use axum::{
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
};
use axum_extra::extract::cookie::CookieJar;
use jsonwebtoken::{DecodingKey, Validation, decode};

use crate::handlers::auth::Claims;

#[derive(Debug)]
pub struct Session {
    pub email: String,
}

impl<S> FromRequestParts<S> for Session
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let cookie_jar = CookieJar::from_request_parts(parts, state)
            .await
            .expect("L'extraction des cookies ne doit pas échouer");

        // On cherche le cookie "auth_token"
        if let Some(cookie) = cookie_jar.get("auth_token") {
            let token = cookie.value().to_string();
            let secret = "secret";
            let token_data = decode::<Claims>(
                &token,
                &DecodingKey::from_secret(secret.as_ref()),
                &Validation::default(),
            )
            .map_err(|_| {
                (
                    StatusCode::UNAUTHORIZED,
                    "Invalid token, redirecting to /login".into(),
                )
            })?;
            Ok(Session {
                email: token_data.claims.sub,
            })
        } else {
            Err((
                StatusCode::UNAUTHORIZED,
                "Missing auth_token, redirecting to /login".into(),
            ))
        }
    }
}
