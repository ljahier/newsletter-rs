use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::{IntoResponse, Response as AxumResponse},
};
use axum_extra::extract::{CookieJar, cookie::Cookie};
use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::{Validate, ValidationErrors};

use bcrypt::verify;

use crate::{AppState, helpers::response::ApiResponse};

#[derive(Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email(message = "Invalid email address"))]
    pub email: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    pub password: String,
}

#[derive(Serialize)]
pub struct TokenResponse {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    exp: usize,
}

#[derive(FromRow, Debug)]
struct UserPassword {
    password: String,
}

/// POST /login
pub async fn login(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(payload): Json<LoginRequest>,
) -> AxumResponse {
    // Validate the incoming payload.
    if let Err(validation_errors) = payload.validate() {
        return response_err_with_cookie_jar(
            StatusCode::BAD_REQUEST,
            format!("Validation error: {}", extract_errors(validation_errors)),
            jar,
        );
    }
    let user: Option<UserPassword> =
        match sqlx::query_as::<_, UserPassword>("SELECT password FROM users WHERE email = ?")
            .bind(&payload.email.clone())
            .fetch_optional(&state.db_pool)
            .await
        {
            Ok(usr) => usr,
            Err(err) => {
                eprintln!("Database query error: {:?}", err);
                return response_err_with_cookie_jar(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Database error".to_string(),
                    jar,
                );
            }
        };

    let user = match user {
        Some(u) => u,
        None => {
            return response_err_with_cookie_jar(
                StatusCode::UNAUTHORIZED,
                "Invalid credentials".to_string(),
                jar,
            );
        }
    };

    if !verify(&payload.password, &user.password).unwrap_or(false) {
        return response_err_with_cookie_jar(
            StatusCode::UNAUTHORIZED,
            "Invalid credentials".to_string(),
            jar,
        );
    }

    let exp = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("Failed to compute expiration")
        .timestamp() as usize;
    let claims = Claims {
        sub: payload.email,
        exp,
    };

    let secret = "secret";
    let token = match encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    ) {
        Ok(t) => t,
        Err(_) => {
            // Todo(lucas): add tracing
            return response_err_with_cookie_jar(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Token generation failed".to_string(),
                jar,
            );
        }
    };

    let cookie = Cookie::build(("auth_token", token))
        .path("/")
        .http_only(true)
        .secure(true)
        .same_site(axum_extra::extract::cookie::SameSite::Strict)
        .max_age(time::Duration::hours(24));

    let updated_jar = jar.add(cookie);

    response_ok_with_cookie_jar(StatusCode::OK, (), updated_jar)
}

fn extract_errors(errors: ValidationErrors) -> String {
    errors
        .field_errors()
        .iter()
        .map(|(field, errs)| {
            let messages: Vec<String> = errs
                .iter()
                .filter_map(|e| e.message.clone().map(|msg| msg.into_owned()))
                .collect();
            format!("{}: {}", field, messages.join(", "))
        })
        .collect::<Vec<String>>()
        .join("; ")
}

fn response_ok_with_cookie_jar<T: Serialize>(
    status: StatusCode,
    data: T,
    jar: CookieJar,
) -> AxumResponse {
    (jar, (status, Json(ApiResponse::success(data)))).into_response()
}

fn response_err_with_cookie_jar(status: StatusCode, msg: String, jar: CookieJar) -> AxumResponse {
    (jar, (status, Json(ApiResponse::<()>::error(msg)))).into_response()
}
