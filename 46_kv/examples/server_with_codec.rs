use anyhow::Result;
use bytes::BytesMut;
use futures::prelude::*;
use kv6::{CommandRequest, MemTable, Service, ServiceInner};
use prost::Message;
use tokio::net::TcpListener;
use tokio_util::codec::{Framed, LengthDelimitedCodec};
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let service: Service = ServiceInner::new(MemTable::new()).into();
    let addr = "127.0.0.1:9527";
    let listener = TcpListener::bind(addr).await?;
    info!("Start listening on {}", addr);
    loop {
        let (stream, addr) = listener.accept().await?;
        info!("Client {:?} connected", addr);
        let svc = service.clone();
        tokio::spawn(async move {
            let mut stream = Framed::new(stream, LengthDelimitedCodec::new());
            while let Some(Ok(mut buf)) = stream.next().await {
                let cmd = CommandRequest::decode(&buf[..]).unwrap();
                info!("Got a new command: {:?}", cmd);
                let mut res = svc.execute(cmd);

                buf.clear();
                while let Some(data) = res.next().await {
                    let mut buf = BytesMut::new();
                    data.encode(&mut buf).unwrap();
                    stream.send(buf.freeze()).await.unwrap();
                }
            }
            info!("Client {:?} disconnected", addr);
        });
    }
}
