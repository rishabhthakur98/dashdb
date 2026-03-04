use dashmap::DashMap;
use std::sync::Arc;
use tokio::net::TcpListener;
mod connection;
mod operations;
mod utilities;
mod choose_operation;
mod authenticate;
use connection::handle_connection;
use utilities::envvariables::loaddotenv;

#[tokio::main]
async fn main() {
    let env_data = loaddotenv();

    let arc_dashmap: Arc<DashMap<Vec<u8>, Vec<u8>>> = Arc::new(DashMap::new());

    match TcpListener::bind(env_data).await {
        Ok(listener) => loop {
            match listener.accept().await {
                Ok((stream, _addr)) => {
                    let dashmap_arc_clone = arc_dashmap.clone();
                    tokio::spawn(async move {
                        handle_connection(stream, dashmap_arc_clone).await;
                    });
                }
                Err(e) => {
                    eprintln!("Error {:?}", e);
                }
            }
        },

        Err(e) => {
            eprintln!("Error {:?}", e);
        }
    }
}
