use axum::{routing::get, Json, Router};
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct Email {
    id: u32,
    from: &'static str,
    to: &'static str,
    subject: &'static str,
    body: &'static str,
    received_at: &'static str,
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
            from: "alerts@devsmtp.local",
            to: "developer@example.com",
            subject: "Welcome to DevSMTP",
            body: "Your local SMTP server is up and running.",
            received_at: "2026-02-21T09:00:00Z",
        },
        Email {
            id: 2,
            from: "noreply@service.local",
            to: "developer@example.com",
            subject: "Daily summary",
            body: "No new activity in the last 24 hours.",
            received_at: "2026-02-21T10:15:00Z",
        },
    ]
}
