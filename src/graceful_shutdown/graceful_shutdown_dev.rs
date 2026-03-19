use tracing::info;

pub async fn wait_for_shutdown() {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to listen for ctrl_c signal");
    info!("Received Ctrl-C, shutting down gracefully...");
}