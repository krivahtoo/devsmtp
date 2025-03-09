use tokio_util::sync::CancellationToken;
use tracing::info;

mod command;
mod server;
mod smtp;
mod utils;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();

    let token = CancellationToken::new();

    let smtp_token = token.clone();
    let smtp_task = tokio::task::spawn(smtp::start_smtp(smtp_token));

    let http_token = token.clone();
    let http_task = server::start_server(http_token);

    tokio::task::spawn(async move {
        utils::shutdown_signal().await;
        info!("Shutting down");
        token.cancel();
    });

    http_task.await;
    smtp_task.await.unwrap();
    Ok(())
}
