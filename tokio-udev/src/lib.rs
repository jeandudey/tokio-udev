// Copyright 2020 Jean Pierre Dudey. See the LICENSE-MIT and
// LICENSE-APACHE files at the top-level directory of this
// distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! # tokio-udev
//!
//! This library implements an stream of device events from `udev`
//! asynchronously.
//!
//! # Usage
//!
//! First put the dependency on your crate's `Cargo.toml`. For example:
//!
//! ```toml
//! [dependencies]
//! tokio-udev = "0.1"
//! ```
//!
//! Then import it in your crate root as:
//!
//! ```rust
//! use tokio_udev;
//! ```

#![cfg(target_os = "linux")]

pub use udev::{
    Attributes, Device, Enumerator, Event, EventType, MonitorBuilder,
    MonitorSocket, Properties,
};

use futures_core::stream::Stream;
use std::{convert::TryFrom, io, pin::Pin, sync::Mutex, task::Poll};
use tokio::io::unix::AsyncFd;

/// Asynchronous stream of device events.
pub struct AsyncMonitorSocket {
    inner: Mutex<Inner>,
}

impl AsyncMonitorSocket {
    /// Construct a tokio-udev [`AsyncMonitorSocket`] from an existing one.
    pub fn new(monitor: MonitorSocket) -> io::Result<AsyncMonitorSocket> {
        Ok(AsyncMonitorSocket {
            inner: Mutex::new(Inner::new(monitor)?),
        })
    }
}

impl TryFrom<MonitorSocket> for AsyncMonitorSocket {
    type Error = io::Error;

    fn try_from(
        monitor: MonitorSocket,
    ) -> Result<AsyncMonitorSocket, Self::Error> {
        AsyncMonitorSocket::new(monitor)
    }
}

impl Stream for AsyncMonitorSocket {
    type Item = Result<udev::Event, io::Error>;

    fn poll_next(
        self: Pin<&mut Self>,
        ctx: &mut std::task::Context,
    ) -> Poll<Option<Self::Item>> {
        self.inner.lock().unwrap().poll_receive(ctx)
    }
}

struct Inner {
    fd: AsyncFd<udev::MonitorSocket>,
}

impl Inner {
    fn new(monitor: udev::MonitorSocket) -> io::Result<Inner> {
        Ok(Inner {
            fd: AsyncFd::new(monitor)?,
        })
    }

    fn poll_receive(
        &mut self,
        ctx: &mut std::task::Context,
    ) -> Poll<Option<Result<udev::Event, io::Error>>> {
        match self.fd.poll_read_ready(ctx) {
            Poll::Ready(Ok(mut ready_guard)) => {
                ready_guard.clear_ready();
                Poll::Ready(self.fd.get_mut().next().map(Ok))
            }
            Poll::Ready(Err(err)) => Poll::Ready(Some(Err(err))),
            Poll::Pending => Poll::Pending,
        }
    }
}
