use dashmap::DashMap;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use crate::authenticate::authenticate;
use crate::choose_operation::choose_operation;

pub async fn handle_connection(stream: TcpStream, mut arc_dashmap: Arc<DashMap<Vec<u8>, Vec<u8>>>) {
    let (mut readhalf, mut writehalf) = stream.into_split();

    if let Err(e) = authenticate(&mut readhalf, &mut writehalf).await {
        eprintln!("Error {:?}", e);
        let _ = writehalf.shutdown().await;
        return;
        }
        

    loop {
        match readhalf.read_u8().await {
            Ok(opcode) => {
                if let Err(e) =
                    choose_operation(&mut readhalf, &mut writehalf, &mut arc_dashmap, opcode).await
                {
                    eprintln!("Error {:?}", e);
                    let _ = writehalf.shutdown().await;
                    return;
                }
            }
            Err(e) =>{
                eprintln!("Error {:?}", e);
                return
            }
            
        };
    }
}
