use libfprint_sys;

use crate::DeviceList;

use crate::device_list;

/// This struct allows you to discover fingerprint scanning hardware. This is the starting point when integrating libfprint-rs into your software.
pub struct FpContext {
    context: *mut libfprint_sys::FpContext,
}

impl FpContext {
    /// Create a new FpContext
    /// # Examples:
    /// ```rust
    /// use libfprint_rs::FpContext;
    ///
    /// let context = FpContext::new();
    /// ```
    pub fn new() -> FpContext {
        FpContext {
            context: unsafe { libfprint_sys::fp_context_new() },
        }
    }

    /// Enumerate all the devices connected to the system
    ///
    /// This function will enumerate all the devices connected to the system and add them to the context.
    pub fn enumerate(&self) {
        unsafe {
            libfprint_sys::fp_context_enumerate(self.context);
        }
    }
    /// Get the list of devices connected to the system
    /// # Examples:
    /// ```rust
    /// use libfprint_rs::FpContext;
    ///
    /// let context = FpContext::new();
    /// let devices = context.get_devices();
    /// ```
    pub fn get_devices<'a>(&'a self) -> DeviceList<'a> {
        let raw_devs = unsafe { libfprint_sys::fp_context_get_devices(self.context) };
        let device_list = unsafe {
            device_list::from_libfprint(
                &self,
                raw_devs.read().pdata.cast(),
                raw_devs.read().len as usize,
            )
        };
        device_list
    }
}
