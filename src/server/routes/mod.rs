use axum::Router;

mod emails;
mod notifications;

pub fn router() -> Router {
    Router::new()
        .merge(emails::router())
        .merge(notifications::router())
}
