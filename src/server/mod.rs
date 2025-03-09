use axum::{routing::get, Router};
use listenfd::ListenFd;
use tokio::net::TcpListener;
use tokio_util::sync::CancellationToken;
use tracing::{debug, info};

async fn root() -> &'static str {
    "Hello World!"
}

pub async fn start_server(token: CancellationToken) {
    let app = Router::new().route("/", get(root));

    let mut listenfd = ListenFd::from_env();
    // try to first get a socket from listenfd, if that does not give us
    // one (eg: no systemd or systemfd), open on port 3000 instead.
    let listener = match listenfd.take_tcp_listener(0).unwrap() {
        Some(listener) => TcpListener::from_std(listener),
        None => TcpListener::bind("0.0.0.0:3000").await,
    }
    .unwrap();
    let addr = listener.local_addr().unwrap();
    info!("Http server listening on http://{}", addr);
    axum::serve(listener, app)
        .with_graceful_shutdown(async move {
            token.cancelled().await;
            debug!("Stopping http server");
        })
        .await
        .unwrap();
}
