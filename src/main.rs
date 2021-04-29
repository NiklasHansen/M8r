use futures_util::stream::StreamExt;
use tokio_socketcan::{CANSocket, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut socket_rx = CANSocket::open("vcan0")?;
    let socket_tx = CANSocket::open("vcan0")?;
    while let Some(Ok(frame)) = socket_rx.next().await {
        socket_tx.write_frame(frame)?.await?;
    }
    Ok(())
}