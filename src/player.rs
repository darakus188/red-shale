use futures::SinkExt;
use tokio::{net::TcpStream, stream::StreamExt};
use tokio_util::codec::LinesCodec;

type LineStream = tokio_util::codec::Framed<TcpStream, LinesCodec>;

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub conn: LineStream,
}

impl Player {

}

pub async fn new(mut conn: LineStream) -> Player {
    let _ = conn.send("What is your name? ").await;
    let name = conn.next().await.unwrap().unwrap();
    let _ = conn.send(format!("Welcome, {}", name)).await;
    Player {
        name,
        conn,
    }
}
pub fn new_with_name(name: String, conn: LineStream) -> Player {
    //TODO
    Player {
        name,
        conn,
    }
}