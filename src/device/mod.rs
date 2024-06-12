mod callback;
mod device;
mod device_sync;
mod enums;
mod user_data;

pub use device_sync::{FpEnrollProgress, FpMatchCb};
use gio::AsyncInitable;
use glib::wrapper;

wrapper! {
#[cfg(not(doctest))]
/// Fingerpint device routines. You can interact with fingerprint devices using this struct.
///
/// # Examples:
/// ```rust
/// use libfprint_rs::FpContext;
///
/// let context = FpContext::new();
/// let devices = context.devices();
/// let device = devices.get(0).unwrap();
///
/// device.open_sync(None).unwrap();
/// let name = device.name().unwrap();
/// println!("Device name: {}", name);
/// ```
    pub struct FpDevice(Object<libfprint_sys::FpDevice, libfprint_sys::FpDeviceClass>)
        @implements AsyncInitable;

    match fn {
        type_ => || libfprint_sys::fp_device_get_type() as usize,
    }
}

pub(crate) struct UserData<F, T> {
    function: F,
    data: Option<T>,
}

impl<F, T> Drop for UserData<F, T> {
    fn drop(&mut self) {
        if !self.data.is_none() {
            drop(self.data.take())
        }
    }
}

macro_rules! fn_pointer {
    ($function:ident, $struct:ident) => {{
        let ptr: *mut std::ffi::c_void = match $function {
            Some(cb) => {
                let data = crate::device::UserData {
                    function: cb,
                    data: $struct,
                };
                let boxed = std::sync::Arc::new(data);
                std::sync::Arc::into_raw(boxed) as *mut std::ffi::c_void
            }
            None => std::ptr::null_mut(),
        };
        ptr
    }};
}

use fn_pointer;
