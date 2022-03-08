use anyhow::Result;
use std::{mem::size_of, time::Duration};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    sync::broadcast,
};

use crate::{Message, NetChannels};

pub async fn server(mut channels: NetChannels) -> Result<()> {
    let (tx, mut rx) = broadcast::channel(16);

    tokio::spawn(async move {
        loop {
            dbg!("test1");
            let data = rx.recv().await.unwrap();
            channels.to_game.send(data).unwrap();
        }
    });

    tokio::spawn(async move {
        loop {
            dbg!("test2");
            let data = channels.from_game.recv().await.unwrap();
            tx.send(data).unwrap();
        }
    });

    loop {
        tokio::time::sleep(Duration::from_secs(5)).await;
    }

    // let listener = TcpListener::bind("127.0.0.1:7878").await.unwrap();
    // println!("[server] Listener started");

    // loop {
    //     let (mut stream, _) = listener.accept().await.unwrap();

    //     id += 1;
    //     let id = id;
    //     let mut rx = tx.subscribe();

    //     let msg = Message::Connected { id };
    //     println!("[server] {msg:?}");
    //     tx.send(msg).unwrap();

    //     tokio::spawn(async move {
    //         loop {
    //             let value = rx.recv().await.unwrap();
    //             let value = bincode::serialize(&value).unwrap();
    //             let length = (value.len() as u32).to_be_bytes();

    //             stream.write_all(&length).await.unwrap();
    //             stream.write_all(&value).await.unwrap();
    //         }
    //     });
    // }
}

pub async fn client(mut channels: NetChannels) -> Result<()> {
    let (mut read_stream, mut write_stream) = TcpStream::connect("127.0.0.1:7878")
        .await
        .unwrap()
        .into_split();
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
