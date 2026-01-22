use std::net::{TcpListener, TcpStream};
use std::thread;
fn main() {
    match TcpListener::bind("127.0.0.1:8080") {
        Ok(listener) => {
            for stream_result in listener.incoming() {
                match stream_result {
                    Ok(stream) => {
                        thread::spawn(|| {
                            // handle_connection(stream);
                        });
                    }
                    Err(e) => {
                        eprintln!("Error {:?}", e);
                    }
                }
            }
        }
        Err(err) => {
            eprintln!("Error {:?}", err);
        }
    }
}

fn handle_connection() {
    todo!("code for handling connection");
}
