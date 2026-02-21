use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::Response,
    routing::get,
    Router,
};
use tokio::time::{interval, Duration};
use tracing::debug;

use super::emails::dummy_emails;

pub fn router() -> Router {
    Router::new().route("/emails/notifications", get(notifications_ws))
}

async fn notifications_ws(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    let mut ticker = interval(Duration::from_secs(5));
    let mut next_id = dummy_emails().len() as u32 + 1;

    loop {
        tokio::select! {
            _ = ticker.tick() => {
                let payload = serde_json::json!({
                    "type": "new_email",
                    "email": {
                        "id": next_id,
                        "from": "system@devsmtp.local",
                        "to": "developer@example.com",
                        "subject": format!("Dummy email #{}", next_id),
                        "body": "This is a websocket notification from dummy data.",
                        "received_at": "2026-02-21T10:30:00Z"
                    }
                });

                if socket.send(Message::Text(payload.to_string().into())).await.is_err() {
                    debug!("WebSocket client disconnected while sending notification");
                    break;
                }

                next_id += 1;
            }
            msg = socket.recv() => {
                match msg {
                    Some(Ok(Message::Close(_))) | None => {
                        debug!("WebSocket client disconnected");
                        break;
                    }
                    Some(Ok(_)) => {}
                    Some(Err(err)) => {
                        debug!(?err, "WebSocket receive error");
                        break;
                    }
                }
            }
        }
    }
}
