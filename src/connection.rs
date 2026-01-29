use std::sync::Arc;
use tokio::{net::TcpStream, io::AsyncReadExt};
use dashmap::DashMap;

use crate::operations::{delete_key_value, get_value, set_key_value};
use tracing::{error, instrument};

#[instrument(skip(stream, shared_map))]
pub async fn handle_connection(mut stream: TcpStream, shared_map: Arc<DashMap<Vec<u8>, Vec<u8>>>) {
    loop {
        let mut opcode = [0; 1];
      match stream.read_exact(&mut opcode).await {
            Ok(_) => match opcode[0] {
                1 => if !set_key_value(&mut stream, &shared_map).await { 
                    break; 
                },
                2 => if !get_value(&mut stream, &shared_map).await { 
                    break; 
                },
                3 => if !delete_key_value(&mut stream, &shared_map).await { 
                    break;
                 },
                _ => {
                    error!("Unknown opcode: {}", opcode[0]);
                    break;
                }
            },
            Err(_) => break,
        }
    }
}
