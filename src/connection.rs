use dashmap::DashMap;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use tracing::{error, debug};

use crate::authenticate::authenticate;
use crate::choose_operation::choose_operation;

pub async fn handle_connection(stream: TcpStream, dashmap: Arc<DashMap<Vec<u8>, Vec<u8>>>, auth_token: String) {

    debug!(
    "New Connection | src_addr: {:?} | dest_addr: {:?}",
    stream.peer_addr(),
    stream.local_addr()
    );



    let (mut readhalf, mut writehalf) = stream.into_split();


    if let Err(e) = authenticate(&mut readhalf, &mut writehalf, &auth_token).await {
        error!("Error {:?}", e);
        let _ = writehalf.shutdown().await;
        return;
    }

    loop {
        match readhalf.read_u8().await {
            Ok(opcode) => {
                if let Err(e) =
                    choose_operation(&mut readhalf, &mut writehalf, &dashmap, opcode).await
                {
                    error!("Error {:?}", e);
                    let _ = writehalf.shutdown().await;
                    return;
                }
            }
            Err(e) => {
                error!("Error {:?}", e);
                return;
            }
        };
    }
}