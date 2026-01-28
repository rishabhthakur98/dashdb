
use std::{net::TcpListener, thread, sync::Arc};
use dashmap::DashMap;
use tracing::{error, info};
mod connection;
mod operations;
mod utilities;
mod trace_logs;
use connection::handle_connection;
use utilities::envvariables::loaddotenv;
use trace_logs::initialize_tracing::init_tracing;
fn main() {
    let env_data = loaddotenv();

    let _guard = init_tracing();

    info!("Application starting");

    let shared_map: Arc<DashMap<Vec<u8>, Vec<u8>>> = Arc::new(DashMap::new());
    match TcpListener::bind(env_data) {
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
