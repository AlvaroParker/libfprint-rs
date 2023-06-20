use crate::context::FpContext;
use std::marker::PhantomData;
use std::rc::Rc;

pub use crate::device::device_list::{DeviceList, Devices};

pub(crate) mod device;
pub(crate) mod device_list;
mod extern_fn;
mod user_data;

pub use crate::device::device::FpEnrollProgress;
pub use crate::device::device::FpMatchCb;

#[derive(Clone)]
pub struct FpDevice<'a> {
    pub(crate) context: PhantomData<&'a FpContext>,
    pub(crate) device: Rc<*mut libfprint_sys::FpDevice>,
}

pub(crate) struct UserData<F, T> {
    function: F,
    data: Option<T>,
}

#[derive(Debug, Clone, Copy)]
pub enum FpDeviceFeature {
    /// Device does not support any feature
    None = 0,
    /// Supports image capture
    Capture = 1,
    /// Supports finger identification
    Identify = 2,
    /// Supports finger verification
    Verify = 4,
    /// Device has a persistent storage
    Storage = 8,
    /// Supports listing the storage templates
    StorageList = 16,
    /// Supports deleting stored templates
    StorageDelete = 32,
    /// Supports clearing the whole storage
    StorageClear = 64,
    /// Natively supports duplicates detection
    DuplicatesCheck = 128,
    /// Whether the device can run continuously
    AlwaysOn = 256,
    /// Supports updating an existing print record using new scans
    UpdatePrint = 512,
}

#[derive(Debug)]
pub enum FpScanType {
    Swipe = libfprint_sys::FpScanType_FP_SCAN_TYPE_SWIPE as isize,
    Press = libfprint_sys::FpScanType_FP_SCAN_TYPE_PRESS as isize,
}

impl From<u32> for FpScanType {
    fn from(value: u32) -> Self {
        match value {
            libfprint_sys::FpScanType_FP_SCAN_TYPE_PRESS => FpScanType::Press,
            libfprint_sys::FpScanType_FP_SCAN_TYPE_SWIPE => FpScanType::Swipe,
            _ => panic!("Invalid FpScanType"),
        }
    }
}
