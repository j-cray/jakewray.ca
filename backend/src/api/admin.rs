use axum::{
    extract::{State, Json},
    http::{HeaderMap, StatusCode},
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use jsonwebtoken::{encode, Header, EncodingKey};
use chrono::{Utc, Duration};

fn get_jwt_secret() -> &'static [u8] {
    // In production, use environment variable: std::env::var("JWT_SECRET").unwrap_or_default().as_bytes()
    // For now using a default that should be changed
    b"change-this-secret-key-in-production-environment"
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    exp: usize,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    token: String,
}

pub fn router(state: crate::state::AppState) -> Router<crate::state::AppState> {
    Router::new()
        .route("/login", post(login))
        .route("/me", get(me))
        .with_state(state)
}

async fn login(
    State(pool): State<PgPool>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, (StatusCode, String)> {
    let user = sqlx::query!("SELECT id, password_hash FROM users WHERE username = $1", &req.username)
        .fetch_optional(&pool)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Database error".to_string()))?
        .ok_or((StatusCode::UNAUTHORIZED, "Invalid credentials".to_string()))?;

    // Verify password (using bcrypt in real implementation)
    if user.password_hash != req.password {
        return Err((StatusCode::UNAUTHORIZED, "Invalid credentials".to_string()));
    }

    let exp = (Utc::now() + Duration::hours(24)).timestamp() as usize;
    let claims = Claims {
        sub: user.id.to_string(),
        exp,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(get_jwt_secret()),
    )
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Token generation failed".to_string()))?;

    Ok(Json(LoginResponse { token }))
}

async fn me(headers: HeaderMap) -> Result<&'static str, StatusCode> {
    headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .ok_or(StatusCode::UNAUTHORIZED)?;

    Ok("Authenticated")
}
