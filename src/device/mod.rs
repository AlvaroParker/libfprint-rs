mod callback;
mod device;
mod device_sync;
mod enums;
mod user_data;

use gio::AsyncInitable;
use glib::wrapper;

wrapper! {
    pub struct FpDevice(Object<libfprint_sys::FpDevice, libfprint_sys::FpDeviceClass>)
        @implements AsyncInitable;

    match fn {
        type_ => || libfprint_sys::fp_device_get_type() as usize,
    }
}

unsafe impl Send for FpDevice {}
unsafe impl Sync for FpDevice {}

pub struct UserData<F, T> {
    function: F,
    data: Option<T>,
}

macro_rules! fn_pointer {
    ($function:ident, $struct:ident) => {{
        let ptr: *mut std::ffi::c_void = match $function {
            Some(cb) => {
                let data = crate::device::UserData {
                    function: cb,
                    data: $struct,
                };
                let boxed = std::boxed::Box::new(data);
                std::boxed::Box::into_raw(boxed).cast()
            }
            None => std::ptr::null_mut(),
        };
        ptr
    }};
}

use fn_pointer;
