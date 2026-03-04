use dashmap::DashMap;
use std::io::{Error, ErrorKind, Result};
use std::sync::Arc;
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};

use crate::operations::{delete_key_value, get_value, set_key_value};


pub async fn choose_operation(
    readhalf_mutable_reference: &mut OwnedReadHalf,
    writehalf_mutable_reference: &mut OwnedWriteHalf,
    arc_dashmap_mutable_reference: &mut Arc<DashMap<Vec<u8>, Vec<u8>>>,
    opcode: u8,
) -> Result<()> {
    match opcode {
        1 => {
            return set_key_value(
                readhalf_mutable_reference,
                writehalf_mutable_reference,
                arc_dashmap_mutable_reference,
            )
            .await;
        }
        2 => {
            return get_value(
                readhalf_mutable_reference,
                writehalf_mutable_reference,
                arc_dashmap_mutable_reference,
            )
            .await;
        }
        3 => {
            return delete_key_value(
                readhalf_mutable_reference,
                writehalf_mutable_reference,
                arc_dashmap_mutable_reference,
            )
            .await;
        }
        _ => {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "Invalid opcode received",
            ));
        }
    }
}