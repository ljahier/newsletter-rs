use axum::{
    extract::Request,
    http::{StatusCode, header},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{DecodingKey, Validation, decode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

#[derive(Clone)]
pub struct Session {
    pub user_email: String,
}

impl Session {
    pub fn new(user_email: String) -> Self {
        Self { user_email }
    }
}

pub async fn auth_middleware(
    mut req: Request,
    next: Next,
) -> Result<Response, (StatusCode, String)> {
    let cookie_header = req
        .headers()
        .get(header::COOKIE)
        .and_then(|hv| hv.to_str().ok());
    let token = cookie_header.and_then(|cookies| {
        cookies.split(';').find_map(|s| {
            let s = s.trim();
            if s.starts_with("auth_token=") {
                Some(s.trim_start_matches("auth_token=").to_string())
            } else {
                None
            }
        })
    });

    let token = match token {
        Some(t) => t,
        None => return Err((StatusCode::UNAUTHORIZED, "Invalid auth token".to_string())),
    };

    let secret = "secret";
    let token_data = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid auth token".to_string()))?;

    req.extensions_mut()
        .insert(Session::new(token_data.claims.sub));

    Ok(next.run(req).await)
}
