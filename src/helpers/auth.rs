use axum::{
    extract::Request,
    http::{StatusCode, header},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{DecodingKey, Validation, decode};

use crate::models::types::{Claims, Session};

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
        None => return Err((StatusCode::UNAUTHORIZED, "Invalid auth 2 token".to_string())),
    };

    let secret = "secret";
    let token_data = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|err| {
        (
            StatusCode::UNAUTHORIZED,
            format!("Invalid auth 3 token, {err}").to_string(),
        )
    })?;

    req.extensions_mut().insert(Session::new(
        token_data.claims.sub.user_email,
        token_data.claims.sub.user_id,
    ));

    Ok(next.run(req).await)
}
