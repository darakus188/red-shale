use std::error::Error;

mod server;
mod player;
mod world;
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    server::serve().await?;
    Ok(())
}
