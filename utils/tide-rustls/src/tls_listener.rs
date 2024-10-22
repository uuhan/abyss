use crate::custom_tls_acceptor::StandardTlsAcceptor;
use crate::{
    CustomTlsAcceptor, TcpConnection, TlsListenerBuilder, TlsListenerConfig, TlsStreamWrapper,
};

use tide::listener::ListenInfo;
use tide::listener::{Listener, ToListener};
use tide::Server;

use async_std::net::{TcpListener, TcpStream};
use async_std::prelude::*;
use async_std::{io, task};

use async_rustls::TlsAcceptor;
use rustls::internal::pemfile::{certs, pkcs8_private_keys, rsa_private_keys};
use rustls::{Certificate, NoClientAuth, PrivateKey, ServerConfig};

use std::fmt::{self, Debug, Display, Formatter};
use std::fs::File;
use std::io::{BufReader, Seek, SeekFrom};
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

/// The primary type for this crate
pub struct TlsListener<State> {
    connection: TcpConnection,
    config: TlsListenerConfig,
    server: Option<Server<State>>,
    tcp_nodelay: Option<bool>,
    tcp_ttl: Option<u32>,
}

impl<State> Debug for TlsListener<State> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("TlsListener")
            .field(&"connection", &self.connection)
            .field(&"config", &self.config)
            .field(
                &"server",
                if self.server.is_some() {
                    &"Some(Server<State>)"
                } else {
                    &"None"
                },
            )
            .field("tcp_ttl", &self.tcp_ttl)
            .field("tcp_nodelay", &self.tcp_nodelay)
            .finish()
    }
}

impl<State> TlsListener<State> {
    pub(crate) fn new(
        connection: TcpConnection,
        config: TlsListenerConfig,
        tcp_nodelay: Option<bool>,
        tcp_ttl: Option<u32>,
    ) -> Self {
        Self {
            connection,
            config,
            server: None,
            tcp_nodelay,
            tcp_ttl,
        }
    }
    /// The primary entrypoint to create a TlsListener. See
    /// [TlsListenerBuilder](crate::TlsListenerBuilder) for more
    /// configuration options.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use tide_rustls::TlsListener;
    /// let listener = TlsListener::<()>::build()
    ///     .addrs("localhost:4433")
    ///     .cert("./tls/localhost-4433.cert")
    ///     .key("./tls/localhost-4433.key")
    ///     .finish();
    /// ```
    pub fn build() -> TlsListenerBuilder<State> {
        TlsListenerBuilder::new()
    }

    async fn configure(&mut self) -> io::Result<()> {
        self.config = match std::mem::take(&mut self.config) {
            TlsListenerConfig::Paths { cert, key } => {
                let certs = load_certs(&cert)?;
                let mut keys = load_keys(&key)?;
                let mut config = ServerConfig::new(NoClientAuth::new());
                config
                    .set_single_cert(certs, keys.remove(0))
                    .map_err(|err| io::Error::new(io::ErrorKind::InvalidInput, err))?;

                TlsListenerConfig::Acceptor(Arc::new(StandardTlsAcceptor(TlsAcceptor::from(
                    Arc::new(config),
                ))))
            }

            TlsListenerConfig::ServerConfig(config) => TlsListenerConfig::Acceptor(Arc::new(
                StandardTlsAcceptor(TlsAcceptor::from(Arc::new(config))),
            )),

            other @ TlsListenerConfig::Acceptor(_) => other,

            TlsListenerConfig::Unconfigured => {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    "could not configure tlslistener",
                ));
            }
        };

        Ok(())
    }

    fn acceptor(&self) -> Option<&Arc<dyn CustomTlsAcceptor>> {
        match self.config {
            TlsListenerConfig::Acceptor(ref a) => Some(a),
            _ => None,
        }
    }

    fn tcp(&self) -> Option<&TcpListener> {
        match self.connection {
            TcpConnection::Connected(ref t) => Some(t),
            _ => None,
        }
    }

    async fn connect(&mut self) -> io::Result<()> {
        if let TcpConnection::Addrs(addrs) = &self.connection {
            let tcp = TcpListener::bind(&addrs[..]).await?;
            self.connection = TcpConnection::Connected(tcp);
        }
        Ok(())
    }
}

fn handle_tls<State: Clone + Send + Sync + 'static>(
    app: Server<State>,
    stream: TcpStream,
    acceptor: Arc<dyn CustomTlsAcceptor>,
) {
    task::spawn(async move {
        let local_addr = stream.local_addr().ok();
        let peer_addr = stream.peer_addr().ok();

        match acceptor.accept(stream).await {
            Ok(None) => {}

            Ok(Some(tls_stream)) => {
                let stream = TlsStreamWrapper::new(tls_stream);
                let fut = async_h1::accept(stream, |mut req| async {
                    if req.url_mut().set_scheme("https").is_err() {
                        tide::log::error!("unable to set https scheme on url", { url: req.url().to_string() });
                    }

                    req.set_local_addr(local_addr);
                    req.set_peer_addr(peer_addr);
                    app.respond(req).await
                });

                if let Err(error) = fut.await {
                    tide::log::error!("async-h1 error", { error: error.to_string() });
                }
            }

            Err(tls_error) => {
                tide::log::error!("tls error", { error: tls_error.to_string() });
            }
        }
    });
}

impl<State: Clone + Send + Sync + 'static> ToListener<State> for TlsListener<State> {
    type Listener = Self;
    fn to_listener(self) -> io::Result<Self::Listener> {
        Ok(self)
    }
}

impl<State: Clone + Send + Sync + 'static> ToListener<State> for TlsListenerBuilder<State> {
    type Listener = TlsListener<State>;
    fn to_listener(self) -> io::Result<Self::Listener> {
        self.finish()
    }
}

#[tide::utils::async_trait]
impl<State: Clone + Send + Sync + 'static> Listener<State> for TlsListener<State> {
    async fn bind(&mut self, server: Server<State>) -> io::Result<()> {
        self.configure().await?;
        self.connect().await?;
        self.server = Some(server);
        Ok(())
    }

    async fn accept(&mut self) -> io::Result<()> {
        let listener = self.tcp().unwrap();
        let mut incoming = listener.incoming();
        let acceptor = self.acceptor().unwrap();
        let server = self.server.as_ref().unwrap();

        while let Some(stream) = incoming.next().await {
            match stream {
                Err(ref e) if is_transient_error(e) => continue,

                Err(error) => {
                    let delay = Duration::from_millis(500);
                    tide::log::error!("Error: {}. Pausing for {:?}.", error, delay);
                    task::sleep(delay).await;
                    continue;
                }

                Ok(stream) => {
                    if let Some(nodelay) = self.tcp_nodelay {
                        stream.set_nodelay(nodelay)?;
                    }

                    if let Some(ttl) = self.tcp_ttl {
                        stream.set_ttl(ttl)?;
                    }

                    handle_tls(server.clone(), stream, acceptor.clone())
                }
            };
        }
        Ok(())
    }

    fn info(&self) -> Vec<ListenInfo> {
        vec![ListenInfo::new(
            self.connection.to_string(),
            String::from("tcp"),
            true,
        )]
    }
}

fn is_transient_error(e: &io::Error) -> bool {
    use io::ErrorKind::*;
    matches!(
        e.kind(),
        ConnectionRefused | ConnectionAborted | ConnectionReset
    )
}

impl<State> Display for TlsListener<State> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.connection)
    }
}

fn load_certs(path: &Path) -> io::Result<Vec<Certificate>> {
    certs(&mut BufReader::new(File::open(path)?))
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "invalid cert"))
}

fn load_keys(path: &Path) -> io::Result<Vec<PrivateKey>> {
    let mut bufreader = BufReader::new(File::open(path)?);
    if let Ok(pkcs8) = pkcs8_private_keys(&mut bufreader) {
        if !pkcs8.is_empty() {
            return Ok(pkcs8);
        }
    }

    bufreader.seek(SeekFrom::Start(0))?;

    if let Ok(rsa) = rsa_private_keys(&mut bufreader) {
        if !rsa.is_empty() {
            return Ok(rsa);
        }
    }

    Err(io::Error::new(io::ErrorKind::InvalidInput, "invalid key"))
}
