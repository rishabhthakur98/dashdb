use std::io::{Error, ErrorKind, Result};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tracing::{error, info};

static PASSWORD_ARRAY: [u8; 8] = [1, 2, 3, 4, 5, 6, 7, 8];

pub async fn authenticate(
    readhalf_mutable_reference: &mut OwnedReadHalf,
    writehalf_mutable_reference: &mut OwnedWriteHalf,
) -> Result<()> {
    let opcode: u8 = readhalf_mutable_reference.read_u8().await?;
    if opcode != 255 {
        error!("Authentication failed: Invalid initial opcode {}", opcode);
        return Err(Error::new(
            ErrorKind::InvalidData,
            "Invalid opcode received",
        ));
    }
    let password_length = readhalf_mutable_reference.read_u8().await?;

    let mut password_vector: Vec<u8> = vec![0u8; password_length as usize];

    let _no_of_bytes_read = readhalf_mutable_reference
        .read_exact(&mut password_vector)
        .await?;

    if &PASSWORD_ARRAY[..] != &password_vector[..] {
        writehalf_mutable_reference.write_u8(2).await?;
        error!("Authentication failed: Invalid password provided");
        return Err(Error::new(
            ErrorKind::PermissionDenied,
            "Authentication failed: Invalid password",
        ));
    }

    info!("Client authenticated successfully");
    writehalf_mutable_reference.write_u8(1).await?;
    writehalf_mutable_reference.flush().await?;
    Ok(())
}