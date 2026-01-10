use axum::Router;
use sqlx::PgPool;

mod admin;
mod public;

pub fn router<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
    PgPool: axum::extract::FromRef<S>,
{
    Router::new()
        .merge(public::router())
        .nest("/admin", admin::router())
}
