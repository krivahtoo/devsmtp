use tokio::net::TcpListener;
use tracing::{debug, error, info};

mod command;
mod handler;

const BIND_ADDRESS: &str = "127.0.0.1:2525";

#[tokio::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();

    let listener = TcpListener::bind(BIND_ADDRESS).await?;
    info!("SMTP server running on {}", BIND_ADDRESS);

    loop {
        let (stream, addr) = listener.accept().await?;
        debug!("New connection from: {}", addr);
        tokio::spawn(async move {
            if let Err(e) = handler::handle_client(stream).await {
                error!("Error handling client {}: {}", addr, e);
            }
        });
    }
}
