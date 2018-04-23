#![cfg(target_os = "linux")]

//! # tokio udev

extern crate libc;

extern crate udev;
extern crate mio_udev;
extern crate mio;

extern crate futures;
extern crate tokio_reactor;

mod monitor;

pub use udev::{Subsystem, DeviceType, MonitorName, USB_SUBSYSTEM, USB_DEVICE, UDEV_MONITOR};

pub use monitor::{Builder, Monitor};
