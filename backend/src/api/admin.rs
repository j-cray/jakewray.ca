use axum::{
    routing::{get, post},
    Router, Json,
    http::StatusCode,
    extract::State,
    response::IntoResponse,
};
use axum_extra::extract::cookie::{Cookie, SignedCookieJar};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use bcrypt::verify;
use std::sync::Arc;

pub fn router(state: crate::state::AppState) -> Router
{
    Router::new()
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/me", get(me))
        .route("/setup/status", get(check_setup_status))
        .route("/setup", post(perform_setup))
        .with_state(state)
}

#[derive(Serialize)]
struct SetupStatus {
    required: bool,
}

async fn check_setup_status(State(pool): State<PgPool>) -> impl IntoResponse {
    let count = sqlx::query!("SELECT COUNT(*) as count FROM users")
        .fetch_one(&pool)
        .await
        .map(|r| r.count.unwrap_or(0))
        .unwrap_or(0);

    Json(SetupStatus { required: count == 0 })
}

use bcrypt::{hash, DEFAULT_COST};

async fn perform_setup(
    State(pool): State<PgPool>,
    Json(payload): Json<LoginPayload>,
) -> impl IntoResponse {
    // 1. Verify no users exist (race condition possible but low risk for this specific use case)
    let count = sqlx::query!("SELECT COUNT(*) as count FROM users")
        .fetch_one(&pool)
        .await
        .map(|r| r.count.unwrap_or(0))
        .unwrap_or(0);

    if count > 0 {
        return (StatusCode::FORBIDDEN, "Setup already completed").into_response();
    }

    // 2. Create user
    let hashed_password = hash(payload.password, DEFAULT_COST).unwrap();

    let result = sqlx::query!(
        "INSERT INTO users (username, password_hash) VALUES ($1, $2)",
        payload.username,
        hashed_password
    )
    .execute(&pool)
    .await;

    match result {
        Ok(_) => (StatusCode::OK, "Admin created").into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

#[derive(Deserialize)]
struct LoginPayload {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct UserResponse {
    username: String,
}

async fn login(
    State(pool): State<PgPool>,
    jar: SignedCookieJar,
    Json(payload): Json<LoginPayload>,
) ->  impl IntoResponse { // Changed from Result<..., (StatusCode, String)> for simplicity in this snippet, but better to be explicit
    let user = sqlx::query!(
        "SELECT id, password_hash FROM users WHERE username = $1",
        payload.username
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()));

    match user {
        Ok(Some(record)) => {
            if verify(&payload.password, &record.password_hash).unwrap_or(false) {
                // Password correct
                let cookie = Cookie::build(("auth_token", record.id.to_string()))
                    .path("/")
                    .http_only(true)
                    .secure(true) // Should be true in prod
                    .same_site(axum_extra::extract::cookie::SameSite::Lax)
                    .build();

                (StatusCode::OK, jar.add(cookie))
            } else {
                (StatusCode::UNAUTHORIZED, jar)
            }
        }
        Ok(None) => (StatusCode::UNAUTHORIZED, jar),
        Err(e) => (e.0, jar),
    }
}

async fn logout(jar: SignedCookieJar) -> (StatusCode, SignedCookieJar) {
    (StatusCode::OK, jar.remove(Cookie::from("auth_token")))
}

async fn me(jar: SignedCookieJar, State(pool): State<PgPool>) -> impl IntoResponse {
    if let Some(cookie) = jar.get("auth_token") {
        let user_id = cookie.value();
        // convert string to uuid
        let uuid = match uuid::Uuid::parse_str(user_id) {
            Ok(u) => u,
            Err(_) => return (StatusCode::UNAUTHORIZED, Json(None::<UserResponse>)).into_response(),
        };

        let user = sqlx::query!("SELECT username FROM users WHERE id = $1", uuid)
            .fetch_optional(&pool)
            .await
            .unwrap_or(None);

        if let Some(u) = user {
            return (StatusCode::OK, Json(Some(UserResponse { username: u.username }))).into_response();
        }
    }

    (StatusCode::UNAUTHORIZED, Json(None::<UserResponse>)).into_response()
}
