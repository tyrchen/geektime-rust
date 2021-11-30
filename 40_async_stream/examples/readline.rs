use futures::prelude::*;
use pin_project::pin_project;
use std::{
    pin::Pin,
    task::{Context, Poll},
};
use tokio::{
    fs,
    io::{AsyncBufReadExt, AsyncRead, BufReader, Lines},
};

/// LineStream 内部使用 tokio::io::Lines
#[pin_project]
struct LineStream<R> {
    #[pin]
    lines: Lines<BufReader<R>>,
}

impl<R: AsyncRead> LineStream<R> {
    /// 从 BufReader 创建一个 LineStream
    pub fn new(reader: BufReader<R>) -> Self {
        Self {
            lines: reader.lines(),
        }
    }
}

/// 为 LineStream 实现 Stream trait
impl<R: AsyncRead> Stream for LineStream<R> {
    type Item = std::io::Result<String>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.project()
            .lines
            .poll_next_line(cx)
            .map(Result::transpose)
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let file = fs::File::open("Cargo.toml").await?;
    let reader = BufReader::new(file);
    let mut st = LineStream::new(reader);
    while let Some(Ok(line)) = st.next().await {
        println!("Got: {}", line);
    }

    Ok(())
}
