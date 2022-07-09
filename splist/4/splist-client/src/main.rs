use futures_channel::mpsc;
use futures_util::{future, pin_mut, StreamExt};
use log::info;
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio_tungstenite::{connect_async, tungstenite::Message};

#[tokio::main]
async fn main() {
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"),
    );

    let addr = "ws://127.0.0.1:3000";

    let (stdin_tx, stdin_rx) = mpsc::unbounded();
    tokio::spawn(read_stdin(stdin_tx));

    let (ws_stream, _) = connect_async(addr).await.unwrap();
    info!("WebSocket connected to {addr}");

    let (write, read) = ws_stream.split();
    let stdin_to_ws = stdin_rx.map(Ok).forward(write);
    let ws_to_stdout = read.for_each(|message| async {
        let data = message.unwrap().into_data();
        let SplistPacket::Text(text) = bincode::deserialize::<SplistPacket>(&data).unwrap();
        tokio::io::stdout()
            .write_all(text.as_bytes())
            .await
            .unwrap();
    });

    pin_mut!(stdin_to_ws, ws_to_stdout);
    future::select(stdin_to_ws, ws_to_stdout).await;
}

async fn read_stdin(tx: mpsc::UnboundedSender<Message>) {
    let mut stdin = BufReader::new(tokio::io::stdin());
    loop {
        let mut buffer = String::new();
        stdin.read_line(&mut buffer).await.unwrap();
        let packet = bincode::serialize(&SplistPacket::Text(buffer)).unwrap();
        tx.unbounded_send(Message::binary(packet)).unwrap();
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum SplistPacket {
    Text(String),
}
