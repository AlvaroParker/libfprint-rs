use std::marker::PhantomData;

use crate::{context::FpContext, device, device::FpDevice};

/// List of fingerprint devices. This struct will allow you to iterate over the fingerprint devices.
pub struct DeviceList<'a> {
    ctx: PhantomData<&'a FpContext>,
    list: *const *mut libfprint_sys::FpDevice,
    len: usize,
}

impl<'a> DeviceList<'a> {
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn iter<'b>(&'b self) -> Devices<'a, 'b> {
        Devices {
            ctx: self.ctx,
            devices: unsafe { std::slice::from_raw_parts(self.list, self.len) },
            index: 0,
        }
    }
}

pub(crate) unsafe fn from_libfprint<'a>(
    _context: &'a FpContext,
    list: *const *mut libfprint_sys::FpDevice,
    len: usize,
) -> DeviceList<'a> {
    DeviceList {
        ctx: PhantomData,
        list,
        len,
    }
}

/// Collection of fingerprint devices
/// This struct will allow you to iterate with the fingerprint devices
/// available in the system.
///
/// To get an iterator of the devices, use the `iter` method.
pub struct Devices<'a, 'b> {
    ctx: PhantomData<&'a FpContext>,
    devices: &'b [*mut libfprint_sys::FpDevice],
    index: usize,
}

impl<'a, 'b> Iterator for Devices<'a, 'b> {
    type Item = FpDevice<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.devices.len() {
            let device = self.devices[self.index];

            self.index += 1;
            Some(unsafe { device::device::from_libfprint(self.ctx, device) })
        } else {
            None
        }
    }
}
