use dashmap::DashMap;
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::{error, info};

mod authenticate;
mod choose_operation;
mod connection;
pub mod dashdb_logging;
mod env_vars;
mod graceful_shutdown;
mod operations;

use connection::handle_connection;
use env_vars::load_and_log_env;
use graceful_shutdown::wait_for_shutdown;

#[tokio::main]
async fn main() {
    let env_vars = load_and_log_env();
    let arc_dashmap: Arc<DashMap<Vec<u8>, Vec<u8>>> = Arc::new(DashMap::new());

    match TcpListener::bind(&env_vars.server_url).await {
        Ok(listener) => {
            info!("Server listening on {}", env_vars.server_url);
            tokio::select! {
                _ = async {
                    loop {
                        match listener.accept().await {
                            Ok((stream, _addr)) => {
                                let dashmap_arc_clone = arc_dashmap.clone();
                                let auth_token = env_vars.auth_token.clone();
                                tokio::spawn(async move {
                                    handle_connection(stream, dashmap_arc_clone, auth_token).await;
                                });
                            }
                            Err(e) => {
                                error!("Error {:?}", e);
                            }
                        }
                    }
                } => {}
                _ = wait_for_shutdown() => {}
            }
        }
        Err(e) => {
            error!("Error {:?}", e);
        }
    }
}