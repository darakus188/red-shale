use futures::StreamExt;
use std::error::Error;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio_util::codec::{Framed, LinesCodec};

pub async fn serve() -> Result<(), Box<dyn Error>> {
    let binding = ":::10113";
    let mut listener = TcpListener::bind(&binding).await?;
    let (tx, rx) = channel(100);

    tokio::spawn(async move {
        let _ = respond(rx).await;
    });

    loop {
        let (socket, _) = listener.accept().await?;
        let tx = tx.clone();
        tokio::spawn(async move {
            let _ = handle_client(socket, tx).await;
        });
    }
}

async fn handle_client(socket: TcpStream, mut tx: Sender<String>) -> Result<(), Box<dyn Error>> {
    let remote_ip = socket.peer_addr()?.ip();
    println!("Incoming connection from {}", remote_ip);

    let mut client = Framed::new(socket, LinesCodec::new_with_max_length(1024));
    loop {
        let query = match client.next().await {
            Some(Ok(q)) => q,
            _ => return Err("no query received".into()),
        };

        let _ = tx.send(query).await;
    }
}

pub async fn respond(mut rx: Receiver<String>) -> Result<(), Box<dyn Error>> {
    loop {
        let received = rx.recv().await;
        let received = match received {
            Some(x) => x,
            None => panic!("received nothing and exploded."),
        };
        println!("Receiver got: {}", received);
    }
}
