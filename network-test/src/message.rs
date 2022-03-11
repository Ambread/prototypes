use std::mem::size_of;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::tcp::{OwnedReadHalf, OwnedWriteHalf},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Message {
    SelfJoined { id: u32 },
    PlayerJoined { id: u32 },
    SetColor { color: [f32; 4] },
}

impl Message {
    pub async fn write(&self, stream: &mut OwnedWriteHalf) -> Result<()> {
        let data = bincode::serialize(self)?;
        let length = (data.len() as u32).to_be_bytes();

        stream.write_all(&length).await?;
        stream.write_all(&data).await?;

        Ok(())
    }

    pub async fn read(steam: &mut OwnedReadHalf) -> Result<Self> {
        let mut length = [0; size_of::<u32>()];
        steam.read_exact(&mut length).await?;
        let length = u32::from_be_bytes(length);

        let mut data = vec![0; length as usize];
        steam.read_exact(&mut data).await?;
        let data = bincode::deserialize(&data)?;

        Ok(data)
    }
}
