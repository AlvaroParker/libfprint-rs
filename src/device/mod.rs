use crate::context::FpContext;
use std::error::Error;
use std::fmt::Display;
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
#[derive(Debug, Clone, Copy)]
pub enum ParseError {
    InvalidFormat,
    Overflow,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ParseError::InvalidFormat => write!(f, "Invalid format"),
            ParseError::Overflow => write!(f, "Overflow"),
        }
    }
}

impl Error for ParseError {}

impl TryFrom<u32> for FpDeviceFeature {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(FpDeviceFeature::None),
            1 => Ok(FpDeviceFeature::Capture),
            2 => Ok(FpDeviceFeature::Identify),
            4 => Ok(FpDeviceFeature::Verify),
            8 => Ok(FpDeviceFeature::Storage),
            16 => Ok(FpDeviceFeature::StorageList),
            32 => Ok(FpDeviceFeature::StorageDelete),
            64 => Ok(FpDeviceFeature::StorageClear),
            128 => Ok(FpDeviceFeature::DuplicatesCheck),
            256 => Ok(FpDeviceFeature::AlwaysOn),
            512 => Ok(FpDeviceFeature::UpdatePrint),
            _ => Err(ParseError::Overflow),
        }
    }
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

macro_rules! fn_pointer {
    ($function:ident, $struct:ident) => {{
        let ptr: *mut std::ffi::c_void = match $function {
            Some(cb) => {
                let data = UserData {
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

macro_rules! return_sucess {
    ($res:ident, $ptr: ident) => {{
        let res: i32 = $res;
        let ptr: *mut libfprint_sys::_GError = $ptr;
        if res == 1 {
            Ok(())
        } else {
            Err(unsafe { crate::error::from_libfprint_static(ptr) })
        }
    }};
}

pub(crate) use fn_pointer;
pub(crate) use return_sucess;
