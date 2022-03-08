use anyhow::Result;
use tokio::{
    net::{TcpListener, TcpStream},
    sync::broadcast,
};

use crate::{message::Message, NetChannels};

pub async fn server(mut channels: NetChannels, addr: String) -> Result<()> {
    let (tx, mut rx) = broadcast::channel(16);

    {
        let tx = tx.clone();

        tokio::spawn(async move {
            loop {
                let data = rx.recv().await.unwrap();
                channels.to_game.send(data).unwrap();
            }
        });

        tokio::spawn(async move {
            loop {
                let data = channels.from_game.recv().await.unwrap();
                tx.send(data).unwrap();
            }
        });
    }

    let listener = TcpListener::bind(addr).await.unwrap();
    println!("[server] Listener started");

    loop {
        let (stream, _) = listener.accept().await.unwrap();
        let mut rx = tx.subscribe();
        let tx = tx.clone();
        let (mut read_stream, mut write_stream) = stream.into_split();

        tokio::spawn(async move {
            loop {
                let data = Message::read(&mut read_stream).await.unwrap();
                tx.send(data).unwrap();
            }
        });

        tokio::spawn(async move {
            loop {
                let data = rx.recv().await.unwrap();
                data.write(&mut write_stream).await.unwrap();
            }
        });
    }
}

pub async fn client(mut channels: NetChannels, addr: String) -> Result<()> {
    let (mut read_stream, mut write_stream) = TcpStream::connect(addr).await.unwrap().into_split();
    println!("[client] Connection established");

    tokio::spawn(async move {
        loop {
            let data = Message::read(&mut read_stream).await.unwrap();
            channels.to_game.send(data).unwrap();
        }
    });

    loop {
        let data = channels.from_game.recv().await.unwrap();
        data.write(&mut write_stream).await.unwrap();
    }
}
