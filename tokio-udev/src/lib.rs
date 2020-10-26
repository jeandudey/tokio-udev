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

pub use mio_udev::{
    Attribute, Attributes, Device, Enumerator, Event, EventType, Properties,
    Property,
};

use mio::{Events, Interest, Poll as MioPoll, Token};
use std::ffi::OsStr;
use std::io;
use std::pin::Pin;
use std::sync::Mutex;
use std::task::Poll;

use futures_core::stream::Stream;
use tokio::io::PollEvented;

/// Monitors for device events.
///
/// A monitor communicates with the kernel over a socket. Filtering events is
/// performed efficiently in the kernel, and only events that match the filters
/// are received by the socket. Filters must be setup before listening for
/// events.
pub struct MonitorBuilder {
    builder: mio_udev::MonitorBuilder,
}

impl MonitorBuilder {
    /// Creates a new `MonitorSocket`.
    #[inline(always)]
    pub fn new() -> io::Result<Self> {
        Ok(MonitorBuilder {
            builder: mio_udev::MonitorBuilder::new()?,
        })
    }

    fn map(builder: mio_udev::MonitorBuilder) -> Self {
        MonitorBuilder { builder }
    }

    /// Adds a filter that matches events for devices with the given subsystem.
    #[inline(always)]
    pub fn match_subsystem<T>(self, subsystem: T) -> io::Result<Self>
    where
        T: AsRef<OsStr>,
    {
        self.builder.match_subsystem::<T>(subsystem).map(Self::map)
    }

    /// Adds a filter that matches events for devices with the given subsystem
    /// and device type.
    #[inline(always)]
    pub fn match_subsystem_devtype<T, U>(
        self,
        subsystem: T,
        devtype: U,
    ) -> io::Result<Self>
    where
        T: AsRef<OsStr>,
        U: AsRef<OsStr>,
    {
        self.builder
            .match_subsystem_devtype::<T, U>(subsystem, devtype)
            .map(Self::map)
    }

    /// Adds a filter that matches events for devices with the given tag.
    #[inline(always)]
    pub fn match_tag<T>(self, tag: T) -> io::Result<Self>
    where
        T: AsRef<OsStr>,
    {
        self.builder.match_tag::<T>(tag).map(Self::map)
    }

    /// Removes all filters currently set on the monitor.
    #[inline(always)]
    pub fn clear_filters(self) -> io::Result<Self> {
        self.builder.clear_filters().map(Self::map)
    }

    /// Listens for events matching the current filters.
    ///
    /// This method consumes the `MonitorBuilder`.
    pub fn listen(self) -> io::Result<MonitorSocket> {
        MonitorSocket::new(self.builder.listen()?)
    }
}

/// Asynchronous stream of device events.
pub struct MonitorSocket {
    inner: Mutex<Inner>,
}

impl MonitorSocket {
    fn new(monitor: mio_udev::MonitorSocket) -> io::Result<MonitorSocket> {
        Ok(MonitorSocket {
            inner: Mutex::new(Inner::new(monitor)?),
        })
    }
}

unsafe impl Send for MonitorSocket {}
unsafe impl Sync for MonitorSocket {}

impl Stream for MonitorSocket {
    type Item = mio_udev::Event;

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context,
    ) -> Poll<Option<Self::Item>> {
        self.inner.lock().unwrap().poll_receive(cx)
    }
}

struct Inner {
    poll: MioPoll,
    monitor: mio_udev::MonitorSocket,
}

impl Inner {
    fn new(mut monitor: mio_udev::MonitorSocket) -> io::Result<Inner> {
        let mut poll = MioPoll::new()?;
        poll.registry()
            .register(&mut monitor, Token(0), Interest::READABLE)?;
        Ok(Inner { poll, monitor })
    }

    fn poll_receive(
        &mut self,
        cx: &mut std::task::Context,
    ) -> Poll<Option<mio_udev::Event>> {
        let mut events = Events::with_capacity(1);
        self.poll.poll(&mut events, None).unwrap();
        let mut events = events.into_iter();
        let poll_result = match events.next() {
            Some(event) => {
                if event.is_error() {
                    Poll::Ready(None)
                } else if !event.is_readable() {
                    Poll::Pending
                } else {
                    Poll::Ready(self.monitor.next())
                }
            }
            None => Poll::Pending,
        };
        poll_result
    }
}
