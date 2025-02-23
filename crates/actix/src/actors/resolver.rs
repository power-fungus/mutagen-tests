//! DNS resolver and connector utility actor
//!
//! ## Example
//!
//! ```rust
//! #![recursion_limit="128"]
//! # use futures::{future, Future};
//! use actix::prelude::*;
//! use actix::actors::resolver;
//!
//! fn main() {
//!     System::run(|| {
//!
//!         actix_rt::spawn({
//!             let resolver = resolver::Resolver::from_registry();
//!
//!             resolver.send(
//!                 // resolve "localhost"
//!                 resolver::Resolve::host("localhost"))
//!                     .then(|addrs| {
//!                         println!("RESULT: {:?}", addrs);
//! #                       System::current().stop();
//!                         Ok::<_, ()>(())
//!                     })
//!         });
//!
//!         actix_rt::spawn({
//!             let resolver = resolver::Resolver::from_registry();
//!
//!             resolver.send(
//!                 // connect to a "localhost:5000"
//!                 resolver::Connect::host_and_port("localhost", 5000))
//!                     .then(|stream| {
//!                         println!("RESULT: {:?}", stream);
//!                         Ok::<_, ()>(())
//!                     })
//!        });
//!    });
//! }
//! ```
use std::collections::VecDeque;
use std::io;
use std::net::SocketAddr;
use std::time::Duration;

use derive_more::Display;
use futures::{Async, Future, Poll};
use log::warn;
use tokio_tcp::{ConnectFuture, TcpStream};
use tokio_timer::Delay;
use trust_dns_resolver::config::{ResolverConfig, ResolverOpts};
use trust_dns_resolver::AsyncResolver;
use trust_dns_resolver::BackgroundLookupIp;

use crate::clock;
use crate::fut::wrap_future;
use crate::prelude::*;

#[deprecated(since = "0.7.0", note = "please use `Resolver` instead")]
pub type Connector = Resolver;

#[deprecated(since = "0.7.0", note = "please use `ResolverError` instead")]
pub type ConnectorError = ResolverError;

#[derive(Eq, PartialEq, Debug)]
pub struct Resolve {
    pub name: String,
    pub port: Option<u16>,
}

#[cfg_attr(test, ::mutagen::mutate)] impl Resolve {
    pub fn host<T: AsRef<str>>(host: T) -> Resolve {
        Resolve {
            name: host.as_ref().to_owned(),
            port: None,
        }
    }

    pub fn host_and_port<T: AsRef<str>>(host: T, port: u16) -> Resolve {
        Resolve {
            name: host.as_ref().to_owned(),
            port: Some(port),
        }
    }
}

#[cfg_attr(test, ::mutagen::mutate)] impl Message for Resolve {
    type Result = Result<VecDeque<SocketAddr>, ResolverError>;
}

#[derive(Eq, PartialEq, Debug)]
pub struct Connect {
    pub name: String,
    pub port: Option<u16>,
    pub timeout: Duration,
}

#[cfg_attr(test, ::mutagen::mutate)] impl Connect {
    pub fn host<T: AsRef<str>>(host: T) -> Connect {
        Connect {
            name: host.as_ref().to_owned(),
            port: None,
            timeout: Duration::from_secs(1),
        }
    }

    pub fn host_and_port<T: AsRef<str>>(host: T, port: u16) -> Connect {
        Connect {
            name: host.as_ref().to_owned(),
            port: Some(port),
            timeout: Duration::from_secs(1),
        }
    }

    /// Set connect timeout
    ///
    /// By default timeout is set to a 1 second.
    pub fn timeout(mut self, timeout: Duration) -> Connect {
        self.timeout = timeout;
        self
    }
}

#[cfg_attr(test, ::mutagen::mutate)] impl Message for Connect {
    type Result = Result<TcpStream, ResolverError>;
}

#[derive(Eq, PartialEq, Debug)]
pub struct ConnectAddr(pub SocketAddr);

#[cfg_attr(test, ::mutagen::mutate)] impl Message for ConnectAddr {
    type Result = Result<TcpStream, ResolverError>;
}

#[derive(Debug, Display)]
pub enum ResolverError {
    /// Failed to resolve the hostname
    #[display(fmt = "Failed resolving hostname: {}", _0)]
    Resolver(String),

    /// Address is invalid
    #[display(fmt = "Invalid input: {}", _0)]
    InvalidInput(&'static str),

    /// Connecting took too long
    #[display(fmt = "Timeout out while establishing connection")]
    Timeout,

    /// Connection io error
    #[display(fmt = "{}", _0)]
    IoError(io::Error),
}

/// `InternalServerError` for `actix::MailboxError`
#[cfg(feature = "http")]
#[cfg_attr(test, ::mutagen::mutate)] impl actix_http::ResponseError for ResolverError {}

pub struct Resolver {
    resolver: Option<AsyncResolver>,
    cfg: Option<(ResolverConfig, ResolverOpts)>,
    err: Option<String>,
}

#[cfg_attr(test, ::mutagen::mutate)] impl Resolver {
    pub fn new(config: ResolverConfig, options: ResolverOpts) -> Resolver {
        Resolver {
            resolver: None,
            cfg: Some((config, options)),
            err: None,
        }
    }

    fn start_resolver<F>(
        &self,
        ctx: &mut <Self as Actor>::Context,
        parts: (AsyncResolver, F),
    ) -> AsyncResolver
    where
        F: 'static + Future<Item = (), Error = ()>,
    {
        ctx.spawn(wrap_future::<_, Self>(parts.1));
        parts.0
    }
}

#[cfg_attr(test, ::mutagen::mutate)] impl Actor for Resolver {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        // AsyncResolver::new() returns the AsyncResolver itself, plus an anonymous
        // future which gets spawned as a background task for doing DNS
        // resolution. So we use our litle `start_resolver` wrapper to spawn
        // the background task (which gets cleaned up automatically if no
        // outstanding AsyncResolvers stil have a handle to it).
        let resolver = if let Some(cfg) = self.cfg.take() {
            self.start_resolver(ctx, AsyncResolver::new(cfg.0, cfg.1))
        } else {
            match AsyncResolver::from_system_conf() {
                Ok(resolver) => self.start_resolver(ctx, resolver),
                Err(err) => {
                    warn!("Can not create system dns resolver: {}", err);
                    self.start_resolver(
                        ctx,
                        AsyncResolver::new(
                            ResolverConfig::default(),
                            ResolverOpts::default(),
                        ),
                    )
                }
            }
        };

        // Keep the resolver itself.
        self.resolver = Some(resolver);
    }
}

#[cfg_attr(test, ::mutagen::mutate)] impl Supervised for Resolver {}

#[cfg_attr(test, ::mutagen::mutate)] impl SystemService for Resolver {}

#[cfg_attr(test, ::mutagen::mutate)] impl Default for Resolver {
    fn default() -> Resolver {
        Resolver {
            resolver: None,
            cfg: None,
            err: None,
        }
    }
}

#[cfg_attr(test, ::mutagen::mutate)] impl Handler<Resolve> for Resolver {
    type Result = ResponseActFuture<Self, VecDeque<SocketAddr>, ResolverError>;

    fn handle(&mut self, msg: Resolve, _: &mut Self::Context) -> Self::Result {
        if let Some(ref err) = self.err {
            Box::new(ResolveFut::err(err.clone()))
        } else {
            Box::new(ResolveFut::new(
                msg.name,
                msg.port.unwrap_or(0),
                self.resolver.as_ref().unwrap(),
            ))
        }
    }
}

#[cfg_attr(test, ::mutagen::mutate)] impl Handler<Connect> for Resolver {
    type Result = ResponseActFuture<Self, TcpStream, ResolverError>;

    fn handle(&mut self, msg: Connect, _: &mut Self::Context) -> Self::Result {
        let timeout = msg.timeout;
        Box::new(
            ResolveFut::new(
                msg.name,
                msg.port.unwrap_or(0),
                self.resolver.as_ref().unwrap(),
            )
            .and_then(move |addrs, _, _| TcpConnector::with_timeout(addrs, timeout)),
        )
    }
}

#[cfg_attr(test, ::mutagen::mutate)] impl Handler<ConnectAddr> for Resolver {
    type Result = ResponseActFuture<Self, TcpStream, ResolverError>;

    fn handle(&mut self, msg: ConnectAddr, _: &mut Self::Context) -> Self::Result {
        let mut v = VecDeque::new();
        v.push_back(msg.0);
        Box::new(TcpConnector::new(v))
    }
}

/// A resolver future.
struct ResolveFut {
    lookup: Option<BackgroundLookupIp>,
    port: u16,
    addrs: Option<VecDeque<SocketAddr>>,
    error: Option<ResolverError>,
    error2: Option<String>,
}

#[cfg_attr(test, ::mutagen::mutate)] impl ResolveFut {
    pub fn new<S: AsRef<str>>(
        addr: S,
        port: u16,
        resolver: &AsyncResolver,
    ) -> ResolveFut {
        // try to parse as a regular SocketAddr first
        if let Ok(addr) = addr.as_ref().parse() {
            let mut addrs = VecDeque::new();
            addrs.push_back(addr);

            ResolveFut {
                port,
                lookup: None,
                addrs: Some(addrs),
                error: None,
                error2: None,
            }
        } else {
            // we need to do dns resolution
            match ResolveFut::parse(addr.as_ref(), port) {
                Ok((host, port)) => ResolveFut {
                    port,
                    lookup: Some(resolver.lookup_ip(host)),
                    addrs: None,
                    error: None,
                    error2: None,
                },
                Err(err) => ResolveFut {
                    port,
                    lookup: None,
                    addrs: None,
                    error: Some(err),
                    error2: None,
                },
            }
        }
    }

    pub fn err(err: String) -> ResolveFut {
        ResolveFut {
            port: 0,
            lookup: None,
            addrs: None,
            error: None,
            error2: Some(err),
        }
    }

    fn parse(addr: &str, port: u16) -> Result<(&str, u16), ResolverError> {
        macro_rules! try_opt {
            ($e:expr, $msg:expr) => {
                match $e {
                    Some(r) => r,
                    None => return Err(ResolverError::InvalidInput($msg)),
                }
            };
        }

        // split the string by ':' and convert the second part to u16
        let mut parts_iter = addr.splitn(2, ':');
        let host = try_opt!(parts_iter.next(), "invalid socket address");
        let port_str = parts_iter.next().unwrap_or("");
        let port: u16 = port_str.parse().unwrap_or(port);

        Ok((host, port))
    }
}

#[cfg_attr(test, ::mutagen::mutate)] impl ActorFuture for ResolveFut {
    type Item = VecDeque<SocketAddr>;
    type Error = ResolverError;
    type Actor = Resolver;

    fn poll(
        &mut self,
        _: &mut Resolver,
        _: &mut Context<Resolver>,
    ) -> Poll<Self::Item, Self::Error> {
        if let Some(err) = self.error.take() {
            Err(err)
        } else if let Some(err) = self.error2.take() {
            Err(ResolverError::Resolver(err))
        } else if let Some(addrs) = self.addrs.take() {
            Ok(Async::Ready(addrs))
        } else {
            match self.lookup.as_mut().unwrap().poll() {
                Ok(Async::NotReady) => Ok(Async::NotReady),
                Ok(Async::Ready(ips)) => {
                    let addrs: VecDeque<_> = ips
                        .iter()
                        .map(|ip| SocketAddr::new(ip, self.port))
                        .collect();
                    if addrs.is_empty() {
                        Err(ResolverError::Resolver(
                            "Expect at least one A dns record".to_owned(),
                        ))
                    } else {
                        Ok(Async::Ready(addrs))
                    }
                }
                Err(err) => Err(ResolverError::Resolver(format!("{}", err))),
            }
        }
    }
}

/// A TCP stream connector.
pub struct TcpConnector {
    addrs: VecDeque<SocketAddr>,
    timeout: Delay,
    stream: Option<ConnectFuture>,
}

#[cfg_attr(test, ::mutagen::mutate)] impl TcpConnector {
    pub fn new(addrs: VecDeque<SocketAddr>) -> TcpConnector {
        TcpConnector::with_timeout(addrs, Duration::from_secs(1))
    }

    pub fn with_timeout(addrs: VecDeque<SocketAddr>, timeout: Duration) -> TcpConnector {
        TcpConnector {
            addrs,
            stream: None,
            timeout: Delay::new(clock::now() + timeout),
        }
    }
}

#[cfg_attr(test, ::mutagen::mutate)] impl ActorFuture for TcpConnector {
    type Item = TcpStream;
    type Error = ResolverError;
    type Actor = Resolver;

    fn poll(
        &mut self,
        _: &mut Resolver,
        _: &mut Context<Resolver>,
    ) -> Poll<Self::Item, Self::Error> {
        // timeout
        if let Ok(Async::Ready(_)) = self.timeout.poll() {
            return Err(ResolverError::Timeout);
        }

        // connect
        loop {
            if let Some(new) = self.stream.as_mut() {
                match new.poll() {
                    Ok(Async::Ready(sock)) => return Ok(Async::Ready(sock)),
                    Ok(Async::NotReady) => return Ok(Async::NotReady),
                    Err(err) => {
                        if self.addrs.is_empty() {
                            return Err(ResolverError::IoError(err));
                        }
                    }
                }
            }

            // try to connect
            let addr = self.addrs.pop_front().unwrap();
            self.stream = Some(TcpStream::connect(&addr));
        }
    }
}
