use axum::{routing::get, Json, Router};
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct Email {
    id: u32,
    from: String,
    to: String,
    subject: String,
    body: String,
    received_at: String,
}

pub fn router() -> Router {
    Router::new().route("/emails", get(get_all_emails))
}

async fn get_all_emails() -> Json<Vec<Email>> {
    Json(dummy_emails())
}

pub fn dummy_emails() -> Vec<Email> {
    vec![
        Email {
            id: 1,
            from: "alerts@devsmtp.local".to_string(),
            to: "developer@example.com".to_string(),
            subject: "Welcome to DevSMTP".to_string(),
            body: "Your local SMTP server is up and running.".to_string(),
            received_at: "2026-02-21T09:00:00Z".to_string(),
        },
        Email {
            id: 2,
            from: "noreply@service.local".to_string(),
            to: "developer@example.com".to_string(),
            subject: "Daily summary".to_string(),
            body: "No new activity in the last 24 hours.".to_string(),
            received_at: "2026-02-21T10:15:00Z".to_string(),
        },
    ]
}
