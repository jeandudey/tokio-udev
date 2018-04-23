use std::{io, ptr, path::PathBuf, str::FromStr};

use udev_sys;

macro_rules! call_cstring {
    ($call:expr) => {
        {
            let r = unsafe { $call };
            if r == $crate::std::ptr::null_mut() {
                Err($crate::std::io::Error::new($crate::std::io::ErrorKind::Other,
                                   concat!("call to `", stringify!($fn), "` failed")))
            } else { 
                Ok(unsafe { $crate::std::ffi::CStr::from_ptr(r).to_owned() })
            }
        }
    }
}

/// An Udev device.
#[derive(Debug)]
pub struct Device {
    ptr: *mut udev_sys::udev_device,
}

unsafe impl Send for Device {}
unsafe impl Sync for Device {}

impl Device {
    #[doc(hidden)]
    pub fn from_raw_part(ptr: *mut udev_sys::udev_device) -> Device {
        assert!(ptr != ptr::null_mut());

        Device { ptr }
    }

    pub fn get_devpath(&self) -> io::Result<PathBuf> {
        assert!(self.ptr != ptr::null_mut());

        call_cstring!(udev_sys::udev_device_get_devpath(self.ptr))?
            .to_str()
            .map_err(|e| {
                io::Error::new(io::ErrorKind::Other, Box::new(e))
            })
            .and_then(|devpath_str| {
                PathBuf::from_str(devpath_str)
                    .map_err(|_| io::Error::new(io::ErrorKind::Other, "couldn't parse path"))
            })
    }

    pub fn get_action(&self) -> io::Result<String> {
        assert!(self.ptr != ptr::null_mut());

        call_cstring!(udev_sys::udev_device_get_devpath(self.ptr))?
            .to_str()
            .map_err(|e| {
                io::Error::new(io::ErrorKind::Other, Box::new(e))
            })
            .map(|action| {
                action.to_string()
            })
    }
}

impl Clone for Device {
    fn clone(&self) -> Device {
        assert!(self.ptr != ptr::null_mut());
        let ptr = unsafe { udev_sys::udev_device_ref(self.ptr) };
        assert!(ptr != ptr::null_mut());

        Device {
            ptr,
        }
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        if self.ptr != ptr::null_mut() {
            unsafe { udev_sys::udev_device_unref(self.ptr) };
        }
    }
}
