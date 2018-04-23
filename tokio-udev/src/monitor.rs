use std::io;

use {mio, udev, mio_udev};
use tokio_reactor::PollEvented;
use futures::{Async, Poll, stream::Stream};

#[derive(Debug)]
pub struct Builder {
    match_filters: Vec<(udev::Subsystem, udev::DeviceType)>,
}

impl Builder {
    pub fn new() -> Builder {
        Builder {
            match_filters: Vec::new(),
        }
    }

    pub fn add_match(mut self,
                     subsystem: udev::Subsystem,
                     devtype: udev::DeviceType) -> Builder {
        self.match_filters.push((subsystem, devtype));
        self
    }

    pub fn build(self, name: udev::MonitorName) -> io::Result<Monitor> {
        let context = udev::Context::new();
        let monitor = udev::Monitor::new_from_netlink(&context, name);

        for filter in self.match_filters {
            monitor.filter_add_match_subsystem_devtype(filter.0, filter.1)?;
        }

        monitor.enable_receiving()?;

        Monitor::new(monitor)
    }
}

#[derive(Debug)]
pub struct Monitor {
    io: PollEvented<mio_udev::MonitorIo>,
}

impl Monitor {
    pub fn new(monitor: udev::Monitor) -> io::Result<Monitor> {
        let io = PollEvented::new(mio_udev::MonitorIo::from_monitor(monitor)?);
        Ok(Monitor { io })
    }

    pub fn poll_receive(&self) -> Poll<Option<udev::Device>, io::Error> {
        if let Async::NotReady = self.io.poll_read_ready(mio::Ready::readable())? {
            return Ok(Async::NotReady);
        }

        match self.io.get_ref().receive_device() {
            Ok(device) => Ok(Async::Ready(Some(device))),
            Err(e) => {
                if e.kind() == io::ErrorKind::WouldBlock {
                    self.io.clear_read_ready(mio::Ready::readable())?;
                    Ok(Async::NotReady)
                } else {
                    Err(e)
                }
            },
        }

    }
}

impl Stream for Monitor {
    type Item = udev::Device;
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        self.poll_receive()
    }
}
