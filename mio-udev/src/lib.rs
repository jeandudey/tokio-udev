#![cfg(target_os = "linux")]

extern crate udev;
extern crate mio;
extern crate libc;

mod util;

use std::io;
use std::os::unix::io::{AsRawFd, RawFd};

use mio::{Ready, Poll, PollOpt, Token};
use mio::event::Evented;
use mio::unix::EventedFd;

#[derive(Debug)]
pub struct MonitorIo {
    monitor: udev::Monitor,
}

impl MonitorIo {
    /// Creates a new monitor io object from an existing udev monitor.
    ///
    /// # Notes
    ///
    /// It marks the file descriptor as `FD_CLOEXEC` and sets the `O_NONBLOCK`
    /// flag.
    pub fn from_monitor(monitor: udev::Monitor) -> io::Result<MonitorIo> {
        use libc::{fcntl, F_GETFD, FD_CLOEXEC, F_SETFD, F_GETFL, F_SETFL, O_NONBLOCK};
        use util::cvt;

        let fd = monitor.as_raw_fd();

        // Make sure the udev file descriptor is marked as CLOEXEC.
        let r = unsafe { cvt(fcntl(fd, F_GETFD))? };

        if !((r & FD_CLOEXEC) == FD_CLOEXEC) {
            unsafe { cvt(fcntl(fd, F_SETFD, r | FD_CLOEXEC))? };
        }

        // Some older versions of udev are not non-blocking by default,
        // so make sure this is set
        let r = unsafe { cvt(fcntl(fd, F_GETFL))? };

        if !((r & O_NONBLOCK) == O_NONBLOCK) {
            unsafe { cvt(fcntl(fd, F_SETFL, r | O_NONBLOCK))? };
        }

        Ok(MonitorIo { monitor })
    }

    pub fn receive_device(&self) -> io::Result<udev::Device> {
        self.monitor.receive_device()
    }

    #[inline(always)]
    fn fd(&self) -> RawFd {
        self.monitor.as_raw_fd()
    }
}

impl Evented for MonitorIo {
    fn register(&self, poll: &Poll, token: Token, interest: Ready, opts: PollOpt)
        -> io::Result<()>
    {
        EventedFd(&self.fd()).register(poll, token, interest, opts)
    }

    fn reregister(&self, poll: &Poll, token: Token, interest: Ready, opts: PollOpt)
        -> io::Result<()>
    {
        EventedFd(&self.fd()).reregister(poll, token, interest, opts)
    }

    fn deregister(&self, poll: &Poll) -> io::Result<()> {
        EventedFd(&self.fd()).deregister(poll)
    }
}
