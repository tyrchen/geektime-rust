use anyhow::Result;
use bytes::{BufMut, BytesMut};
use futures::{Sink, SinkExt};
use pin_project::pin_project;
use std::{
    pin::Pin,
    task::{Context, Poll},
};
use tokio::{fs::File, io::AsyncWrite};

#[pin_project]
struct FileSink {
    #[pin]
    file: File,
    buf: BytesMut,
}

impl FileSink {
    pub fn new(file: File) -> Self {
        Self {
            file,
            buf: BytesMut::new(),
        }
    }
}

impl Sink<&str> for FileSink {
    type Error = std::io::Error;

    fn poll_ready(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn start_send(self: Pin<&mut Self>, item: &str) -> Result<(), Self::Error> {
        let this = self.project();
        eprint!("{}", item);
        this.buf.put(item.as_bytes());
        Ok(())
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        // 如果想 project() 多次，需要先把 self reborrow 一下
        let this = self.as_mut().project();
        let buf = this.buf.split_to(this.buf.len());
        if buf.is_empty() {
            return Poll::Ready(Ok(()));
        }

        // 写入文件
        if let Err(e) = futures::ready!(this.file.poll_write(cx, &buf[..])) {
            return Poll::Ready(Err(e));
        }
        // 刷新文件
        self.project().file.poll_flush(cx)
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        let this = self.project();
        // 结束写入
        this.file.poll_shutdown(cx)
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let file_sink = FileSink::new(File::create("/tmp/hello").await?);
    // pin_mut 可以把变量 pin 住
    futures::pin_mut!(file_sink);
    file_sink.send("hello\n").await?;
    file_sink.send("world!\n").await?;
    file_sink.send("Tyr!\n").await?;

    Ok(())
}
