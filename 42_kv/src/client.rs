use std::time::Duration;

use anyhow::Result;
use futures::StreamExt;
use kv5::{CommandRequest, KvError, ProstClientStream, TlsClientConnector, YamuxCtrl};
use tokio::{net::TcpStream, time};
use tokio_util::compat::Compat;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    // 以后用配置替换
    let ca_cert = include_str!("../fixtures/ca.cert");

    let addr = "127.0.0.1:9527";
    // 连接服务器
    let connector = TlsClientConnector::new("kvserver.acme.inc", None, Some(ca_cert))?;
    let stream = TcpStream::connect(addr).await?;
    let stream = connector.connect(stream).await?;

    // 打开一个 stream
    let mut ctrl = YamuxCtrl::new_client(stream, None);

    let channel = "lobby";
    start_publishing(ctrl.open_stream().await?, channel)?;

    let stream = ctrl.open_stream().await?;

    let mut client = ProstClientStream::new(stream);

    // 生成一个 HSET 命令
    let cmd = CommandRequest::new_hset("table1", "hello", "world".to_string().into());

    // 发送 HSET 命令
    let data = client.execute_unary(&cmd).await?;
    info!("Got response {:?}", data);

    // 生成一个 Subscribe 命令
    let cmd = CommandRequest::new_subscribe(channel);
    let mut stream = client.execute_streaming(&cmd).await?;
    let id = stream.id;
    start_unsubscribe(ctrl.open_stream().await?, channel, id)?;

    while let Some(Ok(data)) = stream.next().await {
        println!("Got published data: {:?}", data);
    }

    println!("Done!");

    Ok(())
}

fn start_publishing(stream: Compat<yamux::Stream>, name: &str) -> Result<(), KvError> {
    let cmd = CommandRequest::new_publish(name, vec![1.into(), 2.into(), "hello".into()]);
    tokio::spawn(async move {
        time::sleep(Duration::from_millis(1000)).await;
        let mut client = ProstClientStream::new(stream);
        let res = client.execute_unary(&cmd).await.unwrap();
        println!("Finished publishing: {:?}", res);
    });

    Ok(())
}

fn start_unsubscribe(stream: Compat<yamux::Stream>, name: &str, id: u32) -> Result<(), KvError> {
    let cmd = CommandRequest::new_unsubscribe(name, id as _);
    tokio::spawn(async move {
        time::sleep(Duration::from_millis(2000)).await;
        let mut client = ProstClientStream::new(stream);
        let res = client.execute_unary(&cmd).await.unwrap();
        println!("Finished unsubscribing: {:?}", res);
    });

    Ok(())
}
