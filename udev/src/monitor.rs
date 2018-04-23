use std::{io, ptr};
use std::os::unix::io::{AsRawFd, RawFd};

use {context::Context, device::Device, udev_sys};
use libc::c_char;

/// Name of an udev monitor.
#[derive(Debug)]
pub struct MonitorName(&'static [u8]);

impl MonitorName {
    /// The name as a nul terminated pointer to C string.
    fn as_ptr(&self) -> *const u8 {
        self.0.as_ptr()
    }
}

/// Udev monitor name (a.k.a `"udev"`).
pub const UDEV_MONITOR: MonitorName = MonitorName(b"udev\0");

/// Udev subsystem.
#[derive(Debug)]
pub struct Subsystem(&'static [u8]);

impl Subsystem {
    /// The name as a nul terminated pointer to C string.
    fn as_ptr(&self) -> *const u8 {
        self.0.as_ptr()
    }
}

/// USB udev subsytem (a.k.a `"usb"`).
pub const USB_SUBSYSTEM: Subsystem = Subsystem(b"usb\0");

/// Udev device type.
#[derive(Debug)]
pub struct DeviceType(&'static [u8]);

impl DeviceType {
    /// The name as a nul terminated pointer to C string.
    fn as_ptr(&self) -> *const u8 {
        self.0.as_ptr()
    }
}

/// USB device type (a.k.a `"usb_device"`)
pub const USB_DEVICE: DeviceType = DeviceType(b"usb_device\0");

/// An udev monitor.
#[derive(Debug)]
pub struct Monitor {
    ptr: *mut udev_sys::udev_monitor,
}

unsafe impl Send for Monitor {}
unsafe impl Sync for Monitor {}

impl Monitor {
    /// Create a new monitor.
    ///
    /// # Notes
    ///
    /// This is equivalent to the udev C function
    /// `udev_monitor_new_from_netlink`.
    pub fn new_from_netlink(context: &Context, name: MonitorName) -> Monitor {
        trace!("creating new monitor, calling `udev_monitor_new_from_netlink`.");

        let name_ptr = name.as_ptr() as *const c_char;
        let ptr = unsafe {
            udev_sys::udev_monitor_new_from_netlink(context.as_ptr(), name_ptr)
        };

        if ptr == ptr::null_mut() {
            panic!("`udev_monitor_new_from_netlink` returned `std::ptr::null_mut()`.");
        }

        Monitor {
            ptr,
        }
    }

    // TODO: documentation.
    pub fn filter_add_match_subsystem_devtype(&self,
                                              subsystem: Subsystem,
                                              devtype: DeviceType) -> io::Result<()> {
        assert!(self.ptr != ptr::null_mut());

        let subsystem_ptr = subsystem.as_ptr() as *const c_char;
        let devtype_ptr = devtype.as_ptr() as *const c_char;

        let r = unsafe {
            udev_sys::udev_monitor_filter_add_match_subsystem_devtype(self.ptr,
                                                                      subsystem_ptr,
                                                                      devtype_ptr)
        };

        if r < 0 {
            return Err(io::Error::new(io::ErrorKind::Other,
                                      "couldn't add new subsystem-devtype match filter"));
        }

        Ok(())
    }

    pub fn enable_receiving(&self) -> io::Result<()> {
        assert!(self.ptr != ptr::null_mut());

        let r = unsafe {
            udev_sys::udev_monitor_enable_receiving(self.ptr)
        };

        if r < 0 {
            return Err(io::Error::new(io::ErrorKind::Other,
                                      "couldn't enable receiving on udev monitor"));
        }

        Ok(())
    }

    pub fn receive_device(&self) -> io::Result<Device> {
        assert!(self.ptr != ptr::null_mut());

        let r = unsafe {
            udev_sys::udev_monitor_receive_device(self.ptr)
        };

        if r == ptr::null_mut() {
            return Err(io::Error::new(io::ErrorKind::WouldBlock,
                                      "couldn't receive device from monitor"));
        }

        Ok(Device::from_raw_part(r))
    }
}

impl Clone for Monitor {
    /// Increments the reference count of the `Monitor`.
    fn clone(&self) -> Monitor {
        trace!("incrementing reference count.");
        assert!(self.ptr != ptr::null_mut());
        let ptr = unsafe { udev_sys::udev_monitor_ref(self.ptr) };
        assert!(ptr != ptr::null_mut());

        Monitor {
            ptr,
        }
    }
}

impl Drop for Monitor {
    /// Decrements the reference count, once it reaches 0 it's dropped.
    fn drop(&mut self) {
        if self.ptr != ptr::null_mut() {
            unsafe { udev_sys::udev_monitor_unref(self.ptr) };
        } else {
            trace!("monitor is already null.");
        }
    }
}

impl AsRawFd for Monitor {
    fn as_raw_fd(&self) -> RawFd {
        assert!(self.ptr != ptr::null_mut());

        unsafe {
            udev_sys::udev_monitor_get_fd(self.ptr) as RawFd
        }
    }
}
