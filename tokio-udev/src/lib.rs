#![cfg(target_os = "linux")]

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

pub use mio_udev::{
    Attribute, Attributes, Context, Device, Enumerator, Event, EventType,
    Properties, Property, UdevError,
};

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
    pub fn new(context: &mio_udev::Context) -> io::Result<Self> {
        Ok(MonitorBuilder {
            builder: mio_udev::MonitorBuilder::new(context)?,
        })
    }

    /// Adds a filter that matches events for devices with the given subsystem.
    #[inline(always)]
    pub fn match_subsystem<T>(&mut self, subsystem: T) -> io::Result<()>
    where
        T: AsRef<OsStr>,
    {
        Ok(self.builder.match_subsystem::<T>(subsystem)?)
    }

    /// Adds a filter that matches events for devices with the given subsystem
    /// and device type.
    #[inline(always)]
    pub fn match_subsystem_devtype<T, U>(
        &mut self,
        subsystem: T,
        devtype: U,
    ) -> io::Result<()>
    where
        T: AsRef<OsStr>,
        U: AsRef<OsStr>,
    {
        Ok(self
            .builder
            .match_subsystem_devtype::<T, U>(subsystem, devtype)?)
    }

    /// Adds a filter that matches events for devices with the given tag.
    #[inline(always)]
    pub fn match_tag<T>(&mut self, tag: T) -> io::Result<()>
    where
        T: AsRef<OsStr>,
    {
        Ok(self.builder.match_tag::<T>(tag)?)
    }

    /// Removes all filters currently set on the monitor.
    #[inline(always)]
    pub fn clear_filters(&mut self) -> io::Result<()> {
        Ok(self.builder.clear_filters()?)
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
    io: PollEvented<mio_udev::MonitorSocket>,
}

impl Inner {
    fn new(monitor: mio_udev::MonitorSocket) -> io::Result<Inner> {
        Ok(Inner {
            io: PollEvented::new(monitor)?,
        })
    }

    fn poll_receive(
        &mut self,
        cx: &mut std::task::Context,
    ) -> Poll<Option<mio_udev::Event>> {
        if let Poll::Pending =
            self.io.poll_read_ready(cx, mio::Ready::readable())
        {
            return Poll::Pending;
        }

        match self.io.get_mut().next() {
            Some(event) => Poll::Ready(Some(event)),
            None => {
                self.io
                    .clear_read_ready(cx, mio::Ready::readable())
                    .unwrap();
                Poll::Pending
            }
        }
    }
}
