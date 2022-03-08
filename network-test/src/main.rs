mod net;
mod render;

use std::thread::spawn;

use anyhow::Result;
use clap::Parser;
use net::server;
use render::render;
use tokio::runtime::Builder;

#[derive(Parser, Debug)]
enum Mode {
    Server,
    Client,
}

fn main() -> Result<()> {
    let rt = Builder::new_current_thread().enable_all().build()?;

    spawn(move || {
        rt.block_on(async move {
            server().await.unwrap();
        });
    });

    render()
}
