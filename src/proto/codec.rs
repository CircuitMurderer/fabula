use std::{marker::PhantomData};

use futures::{SinkExt, StreamExt};
use serde::{Serialize, de::DeserializeOwned};
use tokio::net::TcpStream;
use tokio_util::codec::{Framed, LengthDelimitedCodec};

use crate::proto::types::{ClientMessage, ServerMessage};

struct JsonCodec<T> {
    inner: Framed<TcpStream, LengthDelimitedCodec>,
    marker: PhantomData<T>,
}

impl<T> JsonCodec<T> {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            inner: Framed::new(stream, LengthDelimitedCodec::new()),
            marker: PhantomData,
        }
    }
}

impl<T: Serialize + DeserializeOwned> JsonCodec<T> {
    pub async fn send(&mut self, msg: &T) -> anyhow::Result<()> {
        let bytes = serde_json::to_vec(msg)?;
        self.inner.send(bytes.into()).await?;
        Ok(())
    }

    pub async fn recv(&mut self) -> anyhow::Result<T> {
        let frame = self
            .inner
            .next()
            .await
            .ok_or_else(|| {
                anyhow::anyhow!("connection closed")
            })?;
        
        let bytes = frame?;
        let msg = serde_json::from_slice(&bytes)?;
        Ok(msg)
    }
}

pub type ClientCodec = JsonCodec<ClientMessage>;
pub type ServerCodec = JsonCodec<ServerMessage>;