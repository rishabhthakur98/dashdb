
use std::{net::TcpListener, thread, sync::Arc};
use dashmap::DashMap;
use tracing::{info, error, instrument};
mod connection;
mod operations;
mod lib;
use connection::handle_connection;
use lib::envvariables::loaddotenv;

#[instrument(level = "trace")]
fn main() {

    tracing_subscriber::fmt()
    .with_max_level(tracing::Level::TRACE)
    .init();

    let shared_map: Arc<DashMap<Vec<u8>, Vec<u8>>> = Arc::new(DashMap::new());
    match TcpListener::bind(loaddotenv()) {
        Ok(listener) => {
            for stream_result in listener.incoming() {
                match stream_result {
                    Ok(stream) => {
                        let db_clone = shared_map.clone();
                        thread::spawn(|| {
                            handle_connection(stream, db_clone);
                        });
                    }
                    Err(e) => {
                        error!("Error {:?}", e);
                    }
                }
            }
        }
        Err(e) => {
            error!("Error {:?}", e);
        }
    }
}
