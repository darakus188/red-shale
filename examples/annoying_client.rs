use std::error::Error;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::prelude::*;
use tokio::time::delay_for;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = "::1:10113";
    let mut socket = TcpStream::connect(addr).await?;

    println!("Chunk 1");
    socket.write_all("this is".as_bytes()).await?;

    delay_for(Duration::from_millis(500)).await;

    println!("Chunk 2");
    socket.write_all("not a full".as_bytes()).await?;

    delay_for(Duration::from_millis(500)).await;

    println!("Chunk 3");
    socket.write_all("command all at once".as_bytes()).await?;

    Ok(())
}
