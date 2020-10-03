use std::error::Error;

mod server;
mod player;
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    server::serve().await?;
    Ok(())
}
