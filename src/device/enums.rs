/// The scan type of the device.
#[derive(Debug, Clone, Copy)]
pub enum FpScanType {
    Swipe = libfprint_sys::FpScanType_FP_SCAN_TYPE_SWIPE as isize,
    Press = libfprint_sys::FpScanType_FP_SCAN_TYPE_PRESS as isize,
}

/// The finger status flags for the device.
#[derive(Debug, Clone, Copy)]
pub enum FpFingerStatus {
    None = libfprint_sys::FpFingerStatusFlags_FP_FINGER_STATUS_NONE as isize,
    Needed = libfprint_sys::FpFingerStatusFlags_FP_FINGER_STATUS_NEEDED as isize,
    Present = libfprint_sys::FpFingerStatusFlags_FP_FINGER_STATUS_PRESENT as isize,
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

impl FpDeviceFeature {
    pub fn try_from(n: u32) -> Result<Self, ()> {
        match n {
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
            _ => Err(()),
        }
    }
}
