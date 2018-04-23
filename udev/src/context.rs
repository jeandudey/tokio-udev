use {std::ptr, udev_sys};

/// Udev context.
///
/// This is a ptr to the **udev** library context. This context is reference
/// counted internally so a call to `Context::clone` points to the same
/// `Context`.
#[derive(Debug)]
pub struct Context {
    ptr: *mut udev_sys::udev,
}

unsafe impl Send for Context {}
unsafe impl Sync for Context {}

impl Context {
    /// Creates a new udev context.
    ///
    /// # Panics
    ///
    /// This function panics if the returned context ptr is invalid.
    pub fn new() -> Context {
        trace!("creating new udev context, calling `udev_new`");
        let ptr = unsafe { udev_sys::udev_new() };
        if ptr == ptr::null_mut() {
            panic!("udev_new returned `std::ptr::null_mut()`.");
        }

        Context {
            ptr,
        }
    }

    pub(crate) fn as_ptr(&self) -> *mut udev_sys::udev {
        assert!(self.ptr != ptr::null_mut());
        self.ptr
    }
}

impl Clone for Context {
    /// Increments the reference count.
    fn clone(&self) -> Context {
        trace!("incrementing udev context refence count, calling `udev_ref`");

        assert!(self.ptr != ptr::null_mut());
        let ptr = unsafe { udev_sys::udev_ref(self.ptr) };
        assert!(ptr != ptr::null_mut());

        Context {
            ptr,
        }
    }
}

impl Drop for Context {
    /// Decrements the reference count, once it reaches 0 it's dropped.
    fn drop(&mut self) {
        trace!("dropping udev context, calling `udev_unref`");

        if self.ptr != ptr::null_mut() {
            unsafe { udev_sys::udev_unref(self.ptr) };
        }
    }
}
