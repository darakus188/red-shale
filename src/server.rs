use futures::{StreamExt, SinkExt};
use std::error::Error;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{broadcast}
use tokio_util::codec::{Framed, LinesCodec};

pub async fn serve() -> Result<(), Box<dyn Error>> {
    let binding = ":::10113";
    let mut listener = TcpListener::bind(&binding).await?;
    let (client_tx, respond_rx) = broadcast::channel(100);
    let client_rx = client_tx.subscribe();

    tokio::spawn(async move {
        let _ = respond(respond_rx, respond_tx).await;
    });

    loop {
        let (socket, _) = listener.accept().await?;
        let tx = client_tx.clone();
        let rx = client_rx.clone();
        tokio::spawn(async move {
            let _ = handle_client(socket, tx, rx).await;
        });
    }
}

async fn handle_client(socket: TcpStream, mut tx: Sender<String>, rx: Receiver<String>) -> Result<(), Box<dyn Error>> {
    let remote_ip = socket.peer_addr()?.ip();
    println!("Incoming connection from {}", remote_ip);

    let mut client = Framed::new(socket, LinesCodec::new_with_max_length(1024));
    loop {
        let query = match client.next().await {
            Some(Ok(q)) => q,
            _ => return Err("no query received".into()),
        };

        // Echo back to client temporarily
        client.send(query.clone()).await?;
        let _ = tx.send(query).await;
    }
}

pub async fn respond(mut rx: Receiver<String>, tx: Sender<String>) -> Result<(), Box<dyn Error>> {
    loop {
        let received = rx.recv().await;
        let received = match received {
            Some(x) => x,
            None => panic!("received nothing and exploded."),
        };
        println!("Receiver got: {}", received);
    }
}
