use anyhow::Result;
use futures::{SinkExt, StreamExt};
use tokio::net::TcpListener;
use tokio_util::codec::{Framed, LinesCodec};

#[tokio::main]
async fn main() -> Result<()> {
    let addr = "0.0.0.0:8080";
    let listener = TcpListener::bind(addr).await?;
    println!("listen to: {}", addr);
    loop {
        let (stream, addr) = listener.accept().await?;
        println!("Accepted: {:?}", addr);
        tokio::spawn(async move {
            // 使用 LinesCodec 把 TCP 数据切成一行行字符串处理
            let framed = Framed::new(stream, LinesCodec::new());
            // split 成 writer 和 reader
            let (mut w, mut r) = framed.split();
            while let Some(Ok(line)) = r.next().await {
                // 每读到一行就加个前缀发回
                w.send(format!("I got: {}", line)).await?;
            }
            Ok::<_, anyhow::Error>(())
        });
    }
}
