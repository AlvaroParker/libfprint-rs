use glib::{translate::ToGlibPtr, wrapper};

use crate::FpDevice;

wrapper! {
    /// Discovers connected fingerprint readers.
    pub struct FpContext(Object<libfprint_sys::FpContext, libfprint_sys::FpContextClass>);
    match fn {
        type_ => || libfprint_sys::fp_context_get_type() as usize,
    }
}
impl FpContext {
    /// Create a new `FpContext`
    /// # Examples:
    /// ```no_run
    /// use libfprint_rs::FpContext;
    ///
    /// let context = FpContext::new();
    /// ```
    pub fn new() -> Self {
        unsafe { glib::translate::from_glib_none(libfprint_sys::fp_context_new()) }
    }
    /// Get the list of devices connected to the system
    /// # Examples:
    /// ```no_run
    /// use libfprint_rs::FpContext;
    ///
    /// let context = FpContext::new();
    /// let devices = context.devices();
    /// ```
    pub fn devices(&self) -> Vec<FpDevice> {
        use glib::translate::FromGlibPtrContainer;

        unsafe {
            let devs = libfprint_sys::fp_context_get_devices(self.to_glib_none().0);

            let devs = devs.cast::<glib::ffi::GPtrArray>();
            FromGlibPtrContainer::from_glib_none(devs)
        }
    }

    /// Enumerate all the devices connected to the system
    ///
    /// This function will enumerate all the devices connected to the system and add them to the context.
    pub fn enumerate(&self) {
        unsafe {
            libfprint_sys::fp_context_enumerate(self.to_glib_none().0);
        }
    }
}

impl Default for FpContext {
    fn default() -> Self {
        Self::new()
    }
}
