use dashmap::DashMap;
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::{error, info};

mod connection;
mod operations;
mod env_vars;
mod choose_operation;
mod authenticate;
pub mod dashdb_logging;

use connection::handle_connection;
use env_vars::load_and_log_env;

#[tokio::main]
async fn main() {
    let env_vars = load_and_log_env();

    let arc_dashmap: Arc<DashMap<Vec<u8>, Vec<u8>>> = Arc::new(DashMap::new());

    match TcpListener::bind(&env_vars.server_url).await {
        Ok(listener) => {
            info!("Server listening on {}", env_vars.server_url);
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
        },

        Err(e) => {
            error!("Error {:?}", e);
        }
    }
}