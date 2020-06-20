// Copyright 2020 Jean Pierre Dudey. See the LICENSE-MIT and
// LICENSE-APACHE files at the top-level directory of this
// distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! # mio-udev
//!
//! This library implements abstractions around `udev` to make it usable
//! with `mio` event loop.
//!
//! # Usage
//!
//! First put the dependency on your crate's `Cargo.toml`. For example:
//!
//! ```toml
//! [dependencies]
//! mio-udev = "0.1"
//! ```
//!
//! Then import it in your crate root as:
//!
//! ```rust
//! use mio_udev;
//! ```

#![cfg(target_os = "linux")]

pub use udev::{
    Attribute, Attributes, Context, Device, Enumerator, Error as UdevError,
    Event, EventType, Properties, Property,
};

mod util;

use std::ffi::OsStr;
use std::io;
use std::os::unix::io::{AsRawFd, RawFd};

use mio::event::Evented;
use mio::unix::EventedFd;
use mio::{Poll, PollOpt, Ready, Token};

/// Monitors for device events.
///
/// A monitor communicates with the kernel over a socket. Filtering events is
/// performed efficiently in the kernel, and only events that match the filters
/// are received by the socket. Filters must be setup before listening for
/// events.
pub struct MonitorBuilder {
    builder: udev::MonitorBuilder,
}

impl MonitorBuilder {
    /// Creates a new `MonitorSocket`.
    #[inline(always)]
    pub fn new(context: &Context) -> io::Result<Self> {
        Ok(MonitorBuilder {
            builder: udev::MonitorBuilder::new(context)?,
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
        Ok(MonitorSocket::new(self.builder.listen()?)?)
    }
}

/// A wrapper around an `udev::MonitorSocket` that adds the required `mio`
/// functionality.
pub struct MonitorSocket {
    monitor: udev::MonitorSocket,
}

impl MonitorSocket {
    fn new(monitor: udev::MonitorSocket) -> io::Result<MonitorSocket> {
        use crate::util::cvt;
        use libc::{
            fcntl, FD_CLOEXEC, F_GETFD, F_GETFL, F_SETFD, F_SETFL, O_NONBLOCK,
        };

        let fd = monitor.as_raw_fd();

        // Make sure the udev file descriptor is marked as CLOEXEC.
        let r = unsafe { cvt(fcntl(fd, F_GETFD))? };

        if (r & FD_CLOEXEC) != FD_CLOEXEC {
            unsafe { cvt(fcntl(fd, F_SETFD, r | FD_CLOEXEC))? };
        }

        // Some older versions of udev are not non-blocking by default,
        // so make sure this is set
        let r = unsafe { cvt(fcntl(fd, F_GETFL))? };

        if (r & O_NONBLOCK) != O_NONBLOCK {
            unsafe { cvt(fcntl(fd, F_SETFL, r | O_NONBLOCK))? };
        }

        Ok(MonitorSocket { monitor })
    }

    #[inline(always)]
    fn fd(&self) -> RawFd {
        self.monitor.as_raw_fd()
    }
}

impl Evented for MonitorSocket {
    fn register(
        &self,
        poll: &Poll,
        token: Token,
        interest: Ready,
        opts: PollOpt,
    ) -> io::Result<()> {
        EventedFd(&self.fd()).register(poll, token, interest, opts)
    }

    fn reregister(
        &self,
        poll: &Poll,
        token: Token,
        interest: Ready,
        opts: PollOpt,
    ) -> io::Result<()> {
        EventedFd(&self.fd()).reregister(poll, token, interest, opts)
    }

    fn deregister(&self, poll: &Poll) -> io::Result<()> {
        EventedFd(&self.fd()).deregister(poll)
    }
}

impl Iterator for MonitorSocket {
    type Item = Event;

    fn next(&mut self) -> Option<Self::Item> {
        self.monitor.next()
    }
}
