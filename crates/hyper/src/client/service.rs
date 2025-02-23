//! Utilities used to interact with the Tower ecosystem.
//!
//! This module provides exports of `Service`, `MakeService` and `Connect` which
//! all provide hook-ins into the Tower ecosystem.

use super::conn::{SendRequest, Builder};
use std::marker::PhantomData;
use crate::{common::{Poll, task, Pin}, body::Payload};
use std::future::Future;
use std::error::Error as StdError;
use tower_make::MakeConnection;

pub use tower_service::Service;
pub use tower_make::MakeService;

/// Creates a connection via `SendRequest`.
///
/// This accepts a `hyper::client::conn::Builder` and provides
/// a `MakeService` implementation to create connections from some
/// target `T`.
#[derive(Debug)]
pub struct Connect<C, B, T> {
    inner: C,
    builder: Builder,
    _pd: PhantomData<fn(T, B)>
}

#[cfg_attr(test, ::mutagen::mutate)] impl<C, B, T> Connect<C, B, T> {
    /// Create a new `Connect` with some inner connector `C` and a connection
    /// builder.
    pub fn new(inner: C, builder: Builder) -> Self {
        Self {
            inner,
            builder,
            _pd: PhantomData
        }
    }
}

#[cfg_attr(test, ::mutagen::mutate)] impl<C, B, T> Service<T> for Connect<C, B, T>
where
    C: MakeConnection<T>,
    C::Connection: Unpin + Send + 'static,
    C::Future: Send + 'static,
    C::Error: Into<Box<dyn StdError + Send + Sync>> + Send,
    B: Payload + Unpin + 'static,
    B::Data: Unpin,
{
    type Response = SendRequest<B>;
    type Error = crate::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    fn poll_ready(&mut self, cx: &mut task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx).map_err(|e| crate::Error::new(crate::error::Kind::Connect).with(e.into()))
    }

    fn call(&mut self, req: T) -> Self::Future {
        let builder = self.builder.clone();
        let io = self.inner.make_connection(req);

        let fut = async move {
            match io.await {
                Ok(io) => {
                    match builder.handshake(io).await {
                        Ok((sr, conn)) => {
                            builder.exec.execute(async move {
                                if let Err(e) = conn.await {
                                    debug!("connection error: {:?}", e);
                                }
                            })?;
                            Ok(sr)
                        },
                        Err(e) => Err(e)
                    }
                },
                Err(e) => {
                    let err = crate::Error::new(crate::error::Kind::Connect).with(e.into());
                    Err(err)
                }
            }
        };

        Box::pin(fut)
    }
}
