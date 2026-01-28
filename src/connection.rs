use std::{io::Read, net::TcpStream, sync::Arc};
use dashmap::DashMap;
use crate::operations::{delete_key_value, get_value, set_key_value};
use tracing::{info, error, instrument};

#[instrument(skip(stream, shared_map))]
pub fn handle_connection(mut stream: TcpStream, shared_map: Arc<DashMap<Vec<u8>, Vec<u8>>>) {
    loop {
        let mut opcode = [0; 1];
        match stream.read(&mut opcode) {
            Ok(0) => {
                info!("Client disconnected successfully");
                break;
            }
            Ok(_) => match opcode[0] {
                1 => {
                    if !set_key_value(&mut stream, &shared_map) {
                        break;
                    }
                }
                2 => {
                    if !get_value(&mut stream, &shared_map) {
                        break;
                    }
                }
                3 => {
                    if !delete_key_value(&mut stream, &shared_map) {
                        break;
                    }
                }
                _ => {
                    error!("Unknown opcode: {}", opcode[0]);
                    break;
                }
            },
            Err(e) => {
                error!("Error in handle_connection {:?}", e);
            }
        }
    }
}
