use dashmap::DashMap;
use std::io::Result;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};

pub async fn set_key_value(
    readhalf_mutable_reference: &mut OwnedReadHalf,
    writehalf_mutable_reference: &mut OwnedWriteHalf,
    arc_dashmap_mutable_reference: &mut Arc<DashMap<Vec<u8>, Vec<u8>>>,
) -> Result<()> {
    let key_length = readhalf_mutable_reference.read_u16().await?;
    let value_length = readhalf_mutable_reference.read_u32().await?;
    let mut key: Vec<u8> = vec![0u8; key_length as usize];
    let mut value: Vec<u8> = vec![0u8; value_length as usize];
    readhalf_mutable_reference.read_exact(&mut key).await?;
    readhalf_mutable_reference.read_exact(&mut value).await?;
    arc_dashmap_mutable_reference.insert(key, value);
    writehalf_mutable_reference.write_u8(1).await?;
    writehalf_mutable_reference.flush().await?;
    Ok(())
}

pub async fn get_value(
    readhalf_mutable_reference: &mut OwnedReadHalf,
    writehalf_mutable_reference: &mut OwnedWriteHalf,
    arc_dashmap_mutable_reference: &mut Arc<DashMap<Vec<u8>, Vec<u8>>>,
) -> Result<()> {
    let key_length = readhalf_mutable_reference.read_u16().await?;
    let mut key: Vec<u8> = vec![0u8; key_length as usize];
    readhalf_mutable_reference.read_exact(&mut key).await?;

    match arc_dashmap_mutable_reference.get(&key) {
        Some(key_value) => {
            writehalf_mutable_reference.write_u8(1).await?;
            let value = key_value.value();
            let value_length: usize = value.len();
            writehalf_mutable_reference
                .write_u32(value_length as u32)
                .await?;
            writehalf_mutable_reference.write_all(value).await?;
        }
        None => {
            writehalf_mutable_reference.write_u8(2).await?;
        }
    }
    writehalf_mutable_reference.flush().await?;
    Ok(())
}

pub async fn delete_key_value(
    readhalf_mutable_reference: &mut OwnedReadHalf,
    writehalf_mutable_reference: &mut OwnedWriteHalf,
    arc_dashmap_mutable_reference: &mut Arc<DashMap<Vec<u8>, Vec<u8>>>,
) -> Result<()> {
    let key_length = readhalf_mutable_reference.read_u16().await?;
    let mut key: Vec<u8> = vec![0u8; key_length as usize];
    readhalf_mutable_reference.read_exact(&mut key).await?;

    match arc_dashmap_mutable_reference.remove(&key) {
        Some(_) => {
            writehalf_mutable_reference.write_u8(1).await?;
        }
        None => {
            writehalf_mutable_reference.write_u8(2).await?;
        }
    }
    writehalf_mutable_reference.flush().await?;
    Ok(())
}
