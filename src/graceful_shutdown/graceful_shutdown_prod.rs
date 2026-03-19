use tokio::signal::unix::{signal, SignalKind};
use tracing::info;

pub async fn wait_for_shutdown() {
    let mut sigterm = signal(SignalKind::terminate()).expect("Failed to listen for SIGTERM");
    let mut sigint = signal(SignalKind::interrupt()).expect("Failed to listen for SIGINT");

    tokio::select! {
        _ = sigterm.recv() => {
            info!("Received SIGTERM, shutting down gracefully...");
        },
        _ = sigint.recv() => {
            info!("Received SIGINT, shutting down gracefully...");
        },
    }
}