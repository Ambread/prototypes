mod message;
mod net;
mod render;

use std::{net::SocketAddr, thread::spawn};

use anyhow::Result;
use clap::{ArgEnum, Parser};
use message::Message;
use net::{client, server};
use render::render;
use tokio::runtime::Builder;

#[derive(Parser, Debug, Clone)]
pub struct Args {
    #[clap(arg_enum)]
    mode: Mode,
    addr: String,
}

#[derive(ArgEnum, Debug, Clone)]
pub enum Mode {
    Server,
    Client,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let rt = Builder::new_current_thread().enable_all().build()?;
    let (game_channels, net_channels) = create_channels();

    let title = match &args.mode {
        Mode::Server => "Server",
        Mode::Client { .. } => "Client",
    };

    spawn(move || {
        rt.block_on(async move {
            match args.mode {
                Mode::Server => server(net_channels, args.addr).await.unwrap(),
                Mode::Client => client(net_channels, args.addr).await.unwrap(),
            }
        });
    });

    render(game_channels, title)
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
