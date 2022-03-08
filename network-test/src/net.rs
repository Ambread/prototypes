use anyhow::Result;
use std::mem::size_of;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    sync::broadcast,
};

use crate::{Message, NetChannels};

pub async fn server(mut channels: NetChannels) -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878").await?;
    println!("[server] Listener started");

    let (tx, _) = broadcast::channel(16);
    let mut id = 0;

    {
        id += 1;
        let id = id;
        let mut rx = tx.subscribe();

        let msg = Message::Connected { id };
        println!("[server] {msg:?}");
        tx.send(msg).unwrap();

        tokio::spawn(async move {
            let data = rx.recv().await.unwrap();
            channels.to_game.send(data).unwrap();
        });

        let tx = tx.clone();

        tokio::spawn(async move {
            let data = channels.from_game.recv().await.unwrap();
            tx.send(data).unwrap();
        });
    }

    loop {
        let (mut stream, _) = listener.accept().await?;

        id += 1;
        let id = id;
        let mut rx = tx.subscribe();

        let msg = Message::Connected { id };
        println!("[server] {msg:?}");
        tx.send(msg).unwrap();

        tokio::spawn(async move {
            loop {
                let value = rx.recv().await.unwrap();
                let value = bincode::serialize(&value).unwrap();
                let length = (value.len() as u32).to_be_bytes();

                stream.write_all(&length).await.unwrap();
                stream.write_all(&value).await.unwrap();
            }
        });
    }
}

pub async fn client(mut channels: NetChannels) -> Result<()> {
    let (mut read_stream, mut write_stream) =
        TcpStream::connect("127.0.0.1:7878").await?.into_split();
    println!("[client] Connection established");

    tokio::spawn(async move {
        loop {
            let mut length = [0; size_of::<u32>()];
            read_stream.read_exact(&mut length).await.unwrap();
            let length = u32::from_be_bytes(length);

            let mut data = vec![0; length as usize];
            read_stream.read_exact(&mut data).await.unwrap();
            let data = bincode::deserialize(&data).unwrap();

            channels.to_game.send(data).unwrap();
        }
    });

    tokio::spawn(async move {
        loop {
            let data = channels.from_game.recv().await.unwrap();

            let data = bincode::serialize(&data).unwrap();
            let length = (data.len() as u32).to_be_bytes();

            write_stream.write_all(&length).await.unwrap();
            write_stream.write_all(&data).await.unwrap();
        }
    });

    Ok(())
}
