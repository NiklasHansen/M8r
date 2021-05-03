use futures_util::stream::StreamExt;
use tokio_socketcan::{CANSocket, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut socket_rx = CANSocket::open("vcan0")?;
    
    //socket_rx.set_filter(filters: &[socketcan::CANFilter]);
    while let Some(Ok(_frame)) = socket_rx.next().await {
        println!("Ayo");
    }
    Ok(())
}
