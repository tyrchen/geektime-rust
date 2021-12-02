use futures::{future, Future, TryStreamExt};
use std::marker::PhantomData;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio_util::compat::{Compat, FuturesAsyncReadCompatExt, TokioAsyncReadCompatExt};
use yamux::{Config, Connection, ConnectionError, Control, Mode, WindowUpdateMode};

/// Yamux 控制结构
pub struct YamuxCtrl<S> {
    /// yamux control，用于创建新的 stream
    ctrl: Control,
    _conn: PhantomData<S>,
}

impl<S> YamuxCtrl<S>
where
    S: AsyncRead + AsyncWrite + Unpin + Send + 'static,
{
    /// 创建 yamux 客户端
    pub fn new_client(stream: S, config: Option<Config>) -> Self {
        Self::new(stream, config, true, |_stream| future::ready(Ok(())))
    }

    /// 创建 yamux 服务端，服务端我们需要具体处理 stream
    pub fn new_server<F, Fut>(stream: S, config: Option<Config>, f: F) -> Self
    where
        F: FnMut(yamux::Stream) -> Fut,
        F: Send + 'static,
        Fut: Future<Output = Result<(), ConnectionError>> + Send + 'static,
    {
        Self::new(stream, config, false, f)
    }

    // 创建 YamuxCtrl
    fn new<F, Fut>(stream: S, config: Option<Config>, is_client: bool, f: F) -> Self
    where
        F: FnMut(yamux::Stream) -> Fut,
        F: Send + 'static,
        Fut: Future<Output = Result<(), ConnectionError>> + Send + 'static,
    {
        let mode = if is_client {
            Mode::Client
        } else {
            Mode::Server
        };

        // 创建 config
        let mut config = config.unwrap_or_default();
        config.set_window_update_mode(WindowUpdateMode::OnRead);

        // 创建 config，yamux::Stream 使用的是 futures 的 trait 所以需要 compat() 到 tokio 的 trait
        let conn = Connection::new(stream.compat(), config, mode);

        // 创建 yamux ctrl
        let ctrl = conn.control();

        // pull 所有 stream 下的数据
        tokio::spawn(yamux::into_stream(conn).try_for_each_concurrent(None, f));

        Self {
            ctrl,
            _conn: PhantomData::default(),
        }
    }

    /// 打开一个新的 stream
    pub async fn open_stream(&mut self) -> Result<Compat<yamux::Stream>, ConnectionError> {
        let stream = self.ctrl.open_stream().await?;
        Ok(stream.compat())
    }
}

#[cfg(test)]
mod tests {
    use std::net::SocketAddr;

    use super::*;
    use crate::{
        assert_res_ok,
        network::tls::tls_utils::{tls_acceptor, tls_connector},
        utils::DummyStream,
        CommandRequest, KvError, MemTable, ProstClientStream, ProstServerStream, Service,
        ServiceInner, Storage, TlsServerAcceptor,
    };
    use anyhow::Result;
    use tokio::net::{TcpListener, TcpStream};
    use tokio_rustls::server;
    use tracing::warn;

    pub async fn start_server_with<Store>(
        addr: &str,
        tls: TlsServerAcceptor,
        store: Store,
        f: impl Fn(server::TlsStream<TcpStream>, Service) + Send + Sync + 'static,
    ) -> Result<SocketAddr, KvError>
    where
        Store: Storage,
        Service: From<ServiceInner<Store>>,
    {
        let listener = TcpListener::bind(addr).await.unwrap();
        let addr = listener.local_addr().unwrap();
        let service: Service = ServiceInner::new(store).into();

        tokio::spawn(async move {
            loop {
                match listener.accept().await {
                    Ok((stream, _addr)) => match tls.accept(stream).await {
                        Ok(stream) => f(stream, service.clone()),
                        Err(e) => warn!("Failed to process TLS: {:?}", e),
                    },
                    Err(e) => warn!("Failed to process TCP: {:?}", e),
                }
            }
        });

        Ok(addr)
    }

    /// 创建 ymaux server
    pub async fn start_yamux_server<Store>(
        addr: &str,
        tls: TlsServerAcceptor,
        store: Store,
    ) -> Result<SocketAddr, KvError>
    where
        Store: Storage,
        Service: From<ServiceInner<Store>>,
    {
        let f = |stream, service: Service| {
            YamuxCtrl::new_server(stream, None, move |s| {
                let svc = service.clone();
                async move {
                    let stream = ProstServerStream::new(s.compat(), svc);
                    stream.process().await.unwrap();
                    Ok(())
                }
            });
        };
        start_server_with(addr, tls, store, f).await
    }

    #[tokio::test]
    async fn yamux_ctrl_creation_should_work() -> Result<()> {
        let s = DummyStream::default();
        let mut ctrl = YamuxCtrl::new_client(s, None);
        let stream = ctrl.open_stream().await;

        assert!(stream.is_ok());
        Ok(())
    }

    #[tokio::test]
    async fn yamux_ctrl_client_server_should_work() -> Result<()> {
        // 创建使用了 TLS 的 yamux server
        let acceptor = tls_acceptor(false)?;
        let addr = start_yamux_server("127.0.0.1:0", acceptor, MemTable::new()).await?;

        let connector = tls_connector(false)?;
        let stream = TcpStream::connect(addr).await?;
        let stream = connector.connect(stream).await?;
        // 创建使用了 TLS 的 yamux client
        let mut ctrl = YamuxCtrl::new_client(stream, None);

        // 从 client ctrl 中打开一个新的 yamux stream
        let stream = ctrl.open_stream().await?;
        // 封装成 ProstClientStream
        let mut client = ProstClientStream::new(stream);

        let cmd = CommandRequest::new_hset("t1", "k1", "v1".into());
        client.execute_unary(&cmd).await.unwrap();

        let cmd = CommandRequest::new_hget("t1", "k1");
        let res = client.execute_unary(&cmd).await.unwrap();
        assert_res_ok(&res, &["v1".into()], &[]);

        Ok(())
    }
}
