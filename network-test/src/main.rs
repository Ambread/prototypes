mod net;
mod render;

use std::thread::spawn;

use anyhow::Result;
use clap::Parser;
use net::{client, server};
use render::render;
use serde::{Deserialize, Serialize};
use tokio::runtime::Builder;

#[derive(Parser, Debug, Clone, Copy)]
pub enum Mode {
    Server,
    Client,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum Message {
    Connected { id: u32 },
    SetColor { color: [f32; 4] },
}

fn main() -> Result<()> {
    let mode = Mode::parse();
    let rt = Builder::new_current_thread().enable_all().build()?;
    let (game_channels, net_channels) = create_channels();

    spawn(move || {
        rt.block_on(async move {
            match mode {
                Mode::Server => server(net_channels).await.unwrap(),
                Mode::Client => client(net_channels).await.unwrap(),
            }
        });
    });

    render(game_channels, mode)
}

#[derive(Debug)]
pub struct GameChannels {
    from_net: std::sync::mpsc::Receiver<Message>,
    to_net: tokio::sync::mpsc::UnboundedSender<Message>,
}

#[derive(Debug)]
pub struct NetChannels {
    from_game: tokio::sync::mpsc::UnboundedReceiver<Message>,
    to_game: std::sync::mpsc::Sender<Message>,
}

fn create_channels() -> (GameChannels, NetChannels) {
    // async to sync
    let (to_game, from_net) = std::sync::mpsc::channel::<Message>();
    // sync to async
    let (to_net, from_game) = tokio::sync::mpsc::unbounded_channel::<Message>();

    (
        GameChannels { from_net, to_net },
        NetChannels { from_game, to_game },
    )
}
