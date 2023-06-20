use libfprint_sys;

use crate::device::{device_list, DeviceList};

pub struct FpContext {
    context: *mut libfprint_sys::FpContext,
}

impl FpContext {
    pub fn new() -> FpContext {
        FpContext {
            context: unsafe { libfprint_sys::fp_context_new() },
        }
    }

    pub fn enumerate(&self) {
        unsafe {
            libfprint_sys::fp_context_enumerate(self.context);
        }
    }
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
