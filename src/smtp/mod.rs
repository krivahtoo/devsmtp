use listenfd::ListenFd;
use tokio::net::TcpListener;
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info};

mod handler;

const BIND_ADDRESS: &str = "127.0.0.1:2525";

pub async fn start_smtp(token: CancellationToken) {
    //let mut listenfd = ListenFd::from_env();
    // try to first get a socket from listenfd, if that does not give us
    // one (eg: no systemd or systemfd), open on port 2525 instead.
    //let listener = match listenfd.take_tcp_listener(1).unwrap() {
    //    Some(listener) => TcpListener::from_std(listener),
    //    None => TcpListener::bind(BIND_ADDRESS).await,
    //}
    //.unwrap();
    let listener = TcpListener::bind(BIND_ADDRESS).await.unwrap();
    let addr = listener.local_addr().unwrap();
    info!("SMTP server running on {}", addr);
    tokio::select! {
        _ = token.cancelled() => {}
        _ = async {
            loop {
                let (stream, addr) = match listener.accept().await {
                    Ok(v) => v,
                    Err(e) => {
                        error!("{e}");
                        break;
                    }
                };
                debug!("New connection from: {}", addr);
                tokio::spawn(async move {
                    if let Err(e) = handler::handle_client(stream).await {
                        error!("Error handling client {}: {}", addr, e);
                    }
                });
            }
        } => {}
    }
}
