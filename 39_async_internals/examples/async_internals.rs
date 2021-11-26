use std::{future::Future, pin::Pin};
use tokio::{fs, io::AsyncWriteExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let filename = "/tmp/async_internals";
    write_hello_file_async(filename).await?;

    Ok(())
}

async fn write_hello_file_async(name: &str) -> anyhow::Result<()> {
    let mut file = fs::File::create(name).await?;
    file.write_all(b"hello world!").await?;

    Ok(())
}

#[allow(dead_code)]
enum WriteHelloFile {
    Init(String),
    AwaitingCreate(Pin<Box<dyn Future<Output = Result<fs::File, std::io::Error>>>>),
    AwaitingWrite(Pin<Box<dyn Future<Output = Result<(), std::io::Error>>>>),
    Done,
}

#[allow(dead_code)]
impl WriteHelloFile {
    pub fn new(name: impl Into<String>) -> Self {
        Self::Init(name.into())
    }
}

#[allow(dead_code)]
fn write_hello_file_async1(name: &str) -> WriteHelloFile {
    WriteHelloFile::new(name)
}

// impl Future for WriteHelloFile {
//     type Output = Result<(), std::io::Error>;

//     fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
//         let this = self.get_mut();
//         loop {
//             match this {
//                 // 如果状态是 Init，那么就生成 create Future，把状态切换到 AwaitingCreate
//                 WriteHelloFile::Init(name) => {
//                     let fut = Box::new(fs::File::create(name));
//                     *self = WriteHelloFile::AwaitingCreate(fut);
//                 }
//                 // 如果状态是 AwaitingCreate，那么 poll create Future
//                 // 如果返回 Poll::Ready(Ok(_))，那么创建 write Future
//                 // 并把状态切换到 Awaiting
//                 WriteHelloFile::AwaitingCreate(fut) => match fut.poll(cx) {
//                     Poll::Ready(Ok(v)) => {
//                         let fut = Box::new(v.write_all(b"hello world!"));
//                         *self = WriteHelloFile::AwaitingWrite(fut);
//                     }
//                     Poll::Ready(Err(e)) => return Poll::Ready(Err(e)),
//                     Poll::Pending => return Poll::Pending,
//                 },
//                 // 如果状态是 AwaitingWrite，那么 poll write Future
//                 // 如果返回 Poll::Ready(_)，那么状态切换到 Done，整个 Future 执行成功
//                 WriteHelloFile::AwaitingWrite(fut) => match fut.poll(cx) {
//                     Poll::Ready(result) => {
//                         *self = WriteHelloFile::Done;
//                         return Poll::Ready(result);
//                     }
//                     Poll::Pending => return Poll::Pending,
//                 },
//                 // 整个 Future 已经执行完毕
//                 WriteHelloFile::Done => return Poll::Ready(Ok(())),
//             }
//         }
//     }
// }

// impl Future for WriteHelloFile {
//     type Output = Result<(), std::io::Error>;

//     fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
//         let this = self.get_mut();
//         loop {
//             match this {
//                 WriteHelloFile::Init(name) => {
//                     let fut = Box::pin(fs::File::create(name));
//                     *self = WriteHelloFile::AwaitingCreate(fut);
//                 }
//                 WriteHelloFile::AwaitingCreate(fut) => match fut.poll_unpin(cx) {
//                     Poll::Ready(Ok(v)) => {
//                         let fut = Box::pin(v.write_all(b"hello world!"));
//                         *self = WriteHelloFile::AwaitingWrite(fut);
//                     }
//                     Poll::Ready(Err(e)) => return Poll::Ready(Err(e)),
//                     Poll::Pending => return Poll::Pending,
//                 },
//                 WriteHelloFile::AwaitingWrite(fut) => match fut.poll_unpin(cx) {
//                     Poll::Ready(Ok(_)) => {
//                         *self = WriteHelloFile::Done;
//                         return Poll::Ready(Ok(()));
//                     }
//                     Poll::Ready(Err(e)) => return Poll::Ready(Err(e)),
//                     Poll::Pending => return Poll::Pending,
//                 },
//                 WriteHelloFile::Done => return Poll::Ready(Ok(())),
//             }
//         }
//     }
// }

// enum WriteFileAsync {
//     WaitForCreate(CreateFut),
//     WaitForWrite(WriteFut),
// }

// struct CreateFut(Pin<Box<dyn Future<Output = Result<fs::File, std::io::Error>>>>);

// struct WriteFut {
//     file: fs::File,
//     f: Pin<Box<dyn Future<Output = Result<(), std::io::Error>>>>,
// }

// impl Future for WriteFileAsync {
//     type Output = Result<(), std::io::Error>;

//     fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
//         let this = unsafe { self.get_unchecked_mut() };
//         match this {
//             WriteFileAsync::WaitForCreate(fut) => match Pin::new(&mut fut.0).poll(cx) {
//                 Poll::Ready(Ok(f)) => {
//                     let fut = WriteFut {
//                         f: Box::pin(f.write_all(b"hello world!")),
//                         file: f,
//                     };

//                     *this = WriteFileAsync::WaitForWrite(fut);
//                     return Pin::new(this).poll(cx);
//                 }
//                 Poll::Ready(Err(e)) => return Poll::Ready(Err(e)),
//                 Poll::Pending => return Poll::Pending,
//             },
//             WriteFileAsync::WaitForWrite(_) => todo!(),
//         }
//     }
// }
