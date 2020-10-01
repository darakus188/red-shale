use futures::prelude::*;
use std::error::Error;
use tokio::prelude::*;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{broadcast};
use tokio::sync::broadcast::{Sender, Receiver};
use tokio_util::codec::{Framed, LinesCodec};

pub async fn serve() -> Result<(), Box<dyn Error>> {
    // Setup socket
    let binding = ":::10113";
    let mut listener = TcpListener::bind(&binding).await?;

    // bidirectional communication channels
    let (client_tx, respond_rx) = broadcast::channel(100);
    let (respond_tx, client_rx) = broadcast::channel(100);
    
    // created to satisfy the borrow checker
    let r_tx = respond_tx.clone();
    let r_rx = client_tx.subscribe();

    // start responder loop
    tokio::spawn(async move {
        let _ = respond(r_rx, r_tx).await;
    });

    loop {
        let (socket, _) = listener.accept().await?;
        let tx = client_tx.clone();
        let rx: Receiver<String> = respond_tx.subscribe();
        tokio::spawn(async move {
            let _ = handle_client(socket, tx, rx).await;
        });
    }
}

async fn handle_client(socket: TcpStream, mut tx: Sender<String>, mut rx: Receiver<String>) -> Result<(), Box<dyn Error>> {
    let remote_ip = socket.peer_addr()?.ip();
    println!("Incoming connection from {}", remote_ip);

    let mut client = Framed::new(socket, LinesCodec::new_with_max_length(1024));
    loop {
        tokio::select! {
            query = client.next() => {
                println!("Hit client.next()");
                let _ = tx.send(query.unwrap().unwrap().clone());
            }
            query = rx.recv() => {
                // TODO
                println!("Hit rx.recv");
                let _ = client.send(query.unwrap().clone()).await?;
            }
        }
    }
}

async fn respond(mut rx: Receiver<String>, tx: Sender<String>) -> Result<(), Box<dyn Error>> {
    loop {
        let received = rx.recv().await;
        let received = match received {
            Ok(x) => x,
            Err(e) => panic!("received nothing and exploded."),
        };
        println!("Receiver got: {}", received);
        tx.send(received).unwrap();
    }
}
