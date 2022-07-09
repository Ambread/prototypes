use futures_util::{future, StreamExt, TryStreamExt};
use log::info;
use serde::{Deserialize, Serialize};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite::Message;

mod prisma;

#[tokio::main]
async fn main() {
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"),
    );

    let addr = "127.0.0.1:3000";

    // let client = prisma::new_client().await;

    let listener = TcpListener::bind(addr).await.unwrap();
    info!("Listening on: {addr}");

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(accept_connection(stream));
    }
}

async fn accept_connection(stream: TcpStream) {
    let addr = stream.peer_addr().unwrap();
    info!("Peer address: {addr}");

    let ws_stream = tokio_tungstenite::accept_async(stream).await.unwrap();
    info!("New WebSocket connection: {addr}");

    let (write, read) = ws_stream.split();

    read.try_filter(|msg| future::ready(msg.is_binary()))
        .map(|msg| {
            let msg = msg?;
            let SplistPacket::Text(text) =
                bincode::deserialize::<SplistPacket>(&msg.into_data()).unwrap();
            let packet = SplistPacket::Text(text);
            let msg = Message::binary(bincode::serialize(&packet).unwrap());
            Ok(msg)
        })
        .forward(write)
        .await
        .unwrap();
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum SplistPacket {
    Text(String),
}
