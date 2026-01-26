
use std::{net::TcpListener, thread, sync::Arc};
use dashmap::DashMap;
mod connection;
mod operations;
mod lib;
use connection::handle_connection;
use lib::envvariables::loaddotenv;

fn main() {
    match TcpListener::bind(loaddotenv()) {
        Ok(listener) => {
            for stream_result in listener.incoming() {
                match stream_result {
                    Ok(stream) => {
                        let shared_map: Arc<DashMap<Vec<u8>, Vec<u8>>> = Arc::new(DashMap::new());
                        let db_clone = shared_map.clone();
                        thread::spawn(|| {
                            handle_connection(stream, db_clone);
                        });
                    }
                    Err(e) => {
                        eprintln!("Error {:?}", e);
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Error {:?}", e);
        }
    }
}
