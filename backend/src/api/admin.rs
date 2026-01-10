use axum::{
    routing::{get, post},
    Router, Json,
};
use sqlx::PgPool;

pub fn router(state: crate::state::AppState) -> Router
{
    Router::new()
        .route("/login", post(login))
        .route("/me", get(me))
        .with_state(state)
}

async fn login() -> &'static str {
    "Login Placeholder"
}

async fn me() -> &'static str {
    "Admin User"
}
