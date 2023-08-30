use glib::translate::{FromGlibPtrFull, FromGlibPtrNone, ToGlibPtr};

use super::{
    enums::{FpDeviceFeature, FpFingerStatus, FpScanType},
    FpDevice,
};

impl FpDevice {
    /// The ID of the driver.
    pub fn driver(&self) -> String {
        unsafe {
            let driver = libfprint_sys::fp_device_get_driver(self.to_glib_none().0);
            glib::GString::from_glib_none(driver).to_string()
        }
    }
    /// The ID of the device.
    pub fn device_id(&self) -> String {
        unsafe {
            let driver = libfprint_sys::fp_device_get_device_id(self.to_glib_none().0);
            glib::GString::from_glib_none(driver).to_string()
        }
    }
    /// The human readable name of the device.
    pub fn name(&self) -> String {
        unsafe {
            let name = libfprint_sys::fp_device_get_name(self.to_glib_none().0);
            glib::GString::from_glib_full(name).to_string()
        }
    }
    /// Retrieves the scan type of the device.
    pub fn scan_type(&self) -> FpScanType {
        let scan_type = unsafe { libfprint_sys::fp_device_get_scan_type(self.to_glib_none().0) };
        match scan_type {
            libfprint_sys::FpScanType_FP_SCAN_TYPE_PRESS => FpScanType::Press,
            libfprint_sys::FpScanType_FP_SCAN_TYPE_SWIPE => FpScanType::Swipe,
            _ => panic!("Unknown scan type"),
        }
    }
    /// Retrieves the number of enroll stages for this device.
    pub fn nr_enroll_stage(&self) -> i32 {
        unsafe { libfprint_sys::fp_device_get_nr_enroll_stages(self.to_glib_none().0) }
    }
    /// Retrieves the finger status flags for the device. This can be used by the UI to present the relevant feedback, although it is not guaranteed to be a relevant value when not performing any action.
    pub fn finger_status(&self) -> FpFingerStatus {
        let status = unsafe { libfprint_sys::fp_device_get_finger_status(self.to_glib_none().0) };
        match status {
            libfprint_sys::FpFingerStatusFlags_FP_FINGER_STATUS_NONE => FpFingerStatus::None,
            libfprint_sys::FpFingerStatusFlags_FP_FINGER_STATUS_NEEDED => FpFingerStatus::Needed,
            libfprint_sys::FpFingerStatusFlags_FP_FINGER_STATUS_PRESENT => FpFingerStatus::Present,
            _ => panic!("Unknown finger status"),
        }
    }
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    /// Gets the FpDeviceFeature's supported by the device .
    pub fn features(&self) -> Vec<FpDeviceFeature> {
        // Unmask the features bitfield and return a vector of FpDeviceFeature
        let mut features = Vec::new();
        let x = unsafe { libfprint_sys::fp_device_get_features(self.to_glib_none().0) };
        if x == 0 {
            return vec![FpDeviceFeature::None];
        } else {
            (0..31).for_each(|i| {
                let mask = 1 << i;
                if (mask & x) > 0 {
                    if let Ok(feature) = FpDeviceFeature::try_from(2_u32.pow(i)) {
                        features.push(feature);
                    }
                }
            });
        }
        features
    }
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    /// Checks if device supports the requested FpDeviceFeature.
    pub fn has_feature(&self, feature: FpDeviceFeature) -> bool {
        let res =
            unsafe { libfprint_sys::fp_device_has_feature(self.to_glib_none().0, feature as u32) };
        res == glib::ffi::GTRUE
    }
    /// Whether the device is open or not
    pub fn is_open(&self) -> bool {
        unsafe { libfprint_sys::fp_device_is_open(self.to_glib_none().0) == glib::ffi::GTRUE }
    }
}
