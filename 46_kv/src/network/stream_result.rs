use std::{
    convert::TryInto,
    ops::{Deref, DerefMut},
    pin::Pin,
};

use futures::{Stream, StreamExt};

use crate::{CommandResponse, KvError};

/// 创建时之间取得 subscription id，并使用 Deref/DerefMut 使其用起来和 Stream 一致
pub struct StreamResult {
    pub id: u32,
    inner: Pin<Box<dyn Stream<Item = Result<CommandResponse, KvError>> + Send>>,
}

impl StreamResult {
    pub async fn new<T>(mut stream: T) -> Result<Self, KvError>
    where
        T: Stream<Item = Result<CommandResponse, KvError>> + Send + Unpin + 'static,
    {
        let id = match stream.next().await {
            Some(Ok(CommandResponse {
                status: 200,
                values: v,
                ..
            })) => {
                if v.is_empty() {
                    return Err(KvError::Internal("Invalid stream".into()));
                }
                let id: i64 = (&v[0]).try_into().unwrap();
                Ok(id as u32)
            }
            _ => Err(KvError::Internal("Invalid stream".into())),
        };

        Ok(StreamResult {
            inner: Box::pin(stream),
            id: id?,
        })
    }
}

impl Deref for StreamResult {
    type Target = Pin<Box<dyn Stream<Item = Result<CommandResponse, KvError>> + Send>>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for StreamResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
