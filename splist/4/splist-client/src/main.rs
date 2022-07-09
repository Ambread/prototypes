use futures_channel::mpsc;
use futures_util::{future, pin_mut, StreamExt};
use log::info;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
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
    info!("WebSocket connected");

    let (write, read) = ws_stream.split();
    let stdin_to_ws = stdin_rx.map(Ok).forward(write);
    let ws_to_stdout = read.for_each(|message| async {
        let data = message.unwrap().into_data();
        tokio::io::stdout().write_all(&data).await.unwrap();
    });

    pin_mut!(stdin_to_ws, ws_to_stdout);
    future::select(stdin_to_ws, ws_to_stdout).await;
}

async fn read_stdin(tx: mpsc::UnboundedSender<Message>) {
    let mut stdin = tokio::io::stdin();
    loop {
        let mut buffer = vec![0; 1024];
        let n = match stdin.read(&mut buffer).await {
            Err(_) | Ok(0) => break,
            Ok(n) => n,
        };
        buffer.truncate(n);
        tx.unbounded_send(Message::binary(buffer)).unwrap();
    }
}
