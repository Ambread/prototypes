use anyhow::Result;
use tokio::{
    net::{TcpListener, TcpStream},
    sync::broadcast,
};

use crate::{message::Message, NetChannels};

pub async fn server(mut channels: NetChannels, addr: String) -> Result<()> {
    let (tx, mut rx) = broadcast::channel(16);

    let mut id = 0;

    {
        let tx = tx.clone();
        id += 1;
        let id = id;

        channels.to_game.send(Message::SelfJoined { id })?;
        tx.send(Message::PlayerJoined { id })?;

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
        let (mut read_stream, mut write_stream) = stream.into_split();

        let mut rx = tx.subscribe();
        let tx = tx.clone();

        id += 1;
        let id = id;

        Message::SelfJoined { id }.write(&mut write_stream).await?;
        tx.send(Message::PlayerJoined { id })?;

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
