use axum::Router;
use sqlx::PgPool;

mod admin;
mod public;

pub fn router(state: crate::state::AppState) -> Router {
    Router::new()
        .merge(public::router(state.clone()))
        .nest("/admin", admin::router(state.clone()))
}
