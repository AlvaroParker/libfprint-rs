use glib::translate::FromGlibPtrArrayContainerAsVec;
use glib::{translate::ToGlibPtr, wrapper};

use crate::device::FpDevice;

wrapper! {
    pub struct FpContext(Object<libfprint_sys::FpContext, libfprint_sys::FpContextClass>);
    match fn {
        type_ => || libfprint_sys::fp_context_get_type() as usize,
    }
}
impl FpContext {
    pub fn new() -> Self {
        unsafe { glib::translate::from_glib_none(libfprint_sys::fp_context_new()) }
    }
    pub fn devices(&self) -> Vec<FpDevice> {
        unsafe {
            let devs = libfprint_sys::fp_context_get_devices(self.to_glib_none().0);

            let devs = devs.cast::<glib::ffi::GPtrArray>();
            FromGlibPtrArrayContainerAsVec::from_glib_none_as_vec(devs)
        }
    }

    pub fn enumerate(&self) {
        unsafe {
            libfprint_sys::fp_context_enumerate(self.to_glib_none().0);
        }
    }
}
