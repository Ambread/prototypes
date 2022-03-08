use std::{mem::size_of, vec};

use anyhow::Result;
use clap::Parser;
use serde::{Deserialize, Serialize};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    sync::broadcast,
};

#[derive(Parser, Debug)]
enum Command {
    Server,
    Client,
}

#[tokio::main]
async fn main() -> Result<()> {
    match Command::parse() {
        Command::Server => server().await,
        Command::Client => client().await,
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum Message {
    Connected { id: u32 },
}

async fn server() -> Result<()> {
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
            let msg = rx.recv().await.unwrap();
            println!("[client] {msg:?}");
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

async fn client() -> Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:7878").await?;
    println!("[client] Connection established");

    loop {
        let mut length = [0; size_of::<u32>()];
        stream.read_exact(&mut length).await?;
        let length = u32::from_be_bytes(length);

        let mut data = vec![0; length as usize];
        stream.read_exact(&mut data).await?;
        let data: Message = bincode::deserialize(&data)?;

        println!("[client] {data:?}");
    }
}
