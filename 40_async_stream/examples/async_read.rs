use anyhow::Result;
use pin_project::pin_project;
use std::{
    pin::Pin,
    task::{Context, Poll},
};
use tokio::{
    fs::File,
    io::{AsyncRead, AsyncReadExt, AsyncWrite, ReadBuf},
};

#[pin_project]
struct FileWrapper {
    #[pin]
    file: File,
}

impl FileWrapper {
    pub async fn try_new(name: &str) -> Result<Self> {
        let file = File::open(name).await?;
        Ok(Self { file })
    }
}

impl AsyncRead for FileWrapper {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        self.project().file.poll_read(cx, buf)
    }
}

impl AsyncWrite for FileWrapper {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, std::io::Error>> {
        self.project().file.poll_write(cx, buf)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), std::io::Error>> {
        self.project().file.poll_flush(cx)
    }

    fn poll_shutdown(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), std::io::Error>> {
        self.project().file.poll_shutdown(cx)
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut file = FileWrapper::try_new("./Cargo.toml").await?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).await?;
    println!("{}", buffer);
    Ok(())
}
