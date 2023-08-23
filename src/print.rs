// All methods are declared
use glib::{
    translate::FromGlibPtrFull,
    translate::{FromGlibContainer, FromGlibPtrNone, ToGlibPtr},
    wrapper, ObjectExt,
};

use crate::{device::FpDevice, finger::FpFinger, image::FpImage};

wrapper! {
    /// Struct representing a fingerprint.
    pub struct FpPrint(Object<libfprint_sys::FpPrint, libfprint_sys::FpPrintClass>)
        @extends glib::object::InitiallyUnowned;

    match fn {
        type_ => || libfprint_sys::fp_print_get_type() as usize,
    }
}

impl FpPrint {
    /// Create a new `FpPrint`. This is only useful to prepare an enrollment of a new print using `FpDevice::enroll_sync`.
    /// For this you should first create a new print, fill in the relevant metadata, and then start the enrollment
    pub fn new(dev: &FpDevice) -> Self {
        unsafe {
            let ptr = libfprint_sys::fp_print_new(dev.to_glib_none().0);
            Self::from_glib_full(ptr)
        }
    }

    /// Returns the driver that the print was created for.
    pub fn driver(&self) -> String {
        unsafe {
            let ptr = libfprint_sys::fp_print_get_driver(self.to_glib_none().0);
            String::from_glib_none(ptr)
        }
    }
    /// Returns the device ID that the print was created for.
    pub fn device_id(&self) -> String {
        unsafe {
            let ptr = libfprint_sys::fp_print_get_device_id(self.to_glib_none().0);
            String::from_glib_none(ptr)
        }
    }
    /// Whether the print is actually stored on the device and this is just a handle to use that references the device stored data.
    pub fn device_stored(&self) -> bool {
        unsafe {
            libfprint_sys::fp_print_get_device_stored(self.to_glib_none().0) == glib::ffi::GTRUE
        }
    }
    /// Returns the image that the print was created from, or None
    pub fn image(&self) -> Option<FpImage> {
        unsafe {
            let ptr = libfprint_sys::fp_print_get_image(self.to_glib_none().0);
            if ptr.is_null() {
                None
            } else {
                Some(FpImage::from_glib_none(ptr))
            }
        }
    }
    /// Returns the finger that the print was created for.
    pub fn finger(&self) -> FpFinger {
        let raw_finger = unsafe { libfprint_sys::fp_print_get_finger(self.to_glib_none().0) };
        FpFinger::from(raw_finger)
    }
    /// Returns the user defined username for the print.
    pub fn username(&self) -> Option<String> {
        unsafe {
            let ptr = libfprint_sys::fp_print_get_username(self.to_glib_none().0);
            if ptr.is_null() {
                None
            } else {
                Some(String::from_glib_none(ptr))
            }
        }
    }
    /// Returns the user defined description for the print.
    pub fn description(&self) -> Option<String> {
        unsafe {
            let ptr = libfprint_sys::fp_print_get_description(self.to_glib_none().0);
            if ptr.is_null() {
                None
            } else {
                Some(String::from_glib_none(ptr))
            }
        }
    }
    /// Returns the user defined enroll date for the print.
    pub fn enroll_date(&self) -> Option<crate::GDate> {
        unsafe {
            let ptr = libfprint_sys::fp_print_get_enroll_date(self.to_glib_none().0);
            if ptr.is_null() {
                None
            } else {
                Some(glib::Date::from_glib_none(ptr.cast()))
            }
        }
    }

    /// Set the finger that the print is for.
    pub fn set_finger(&self, finger: FpFinger) {
        unsafe { libfprint_sys::fp_print_set_finger(self.to_glib_none().0, finger as u32) };
    }
    /// Set the username for the print.
    pub fn set_username(&self, username: &str) {
        unsafe {
            libfprint_sys::fp_print_set_username(self.to_glib_none().0, username.to_glib_none().0);
        }
    }
    /// Set the description for the print.
    pub fn set_description(&self, description: &str) {
        unsafe {
            libfprint_sys::fp_print_set_description(
                self.to_glib_none().0,
                description.to_glib_none().0,
            );
        }
    }

    /// Set the enroll date for the print.
    pub fn set_enroll_date(&self, enroll_date: crate::GDate) {
        unsafe {
            libfprint_sys::fp_print_set_enroll_date(
                self.to_glib_none().0,
                enroll_date.to_glib_none().0.cast(),
            );
        }
    }
    /// Tests whether the prints is compatible with the given device.
    pub fn compatible(&self, device: &FpDevice) -> bool {
        unsafe {
            libfprint_sys::fp_print_compatible(self.to_glib_none().0, device.to_glib_none().0)
                == glib::ffi::GTRUE
        }
    }
    /// Tests whether the prints can be considered equal. This only compares the actual information about the print, not the metadata.
    pub fn equal(&self, other: &FpPrint) -> bool {
        unsafe {
            libfprint_sys::fp_print_equal(self.to_glib_none().0, other.to_glib_none().0)
                == glib::ffi::GTRUE
        }
    }
    /// Serialize a print definition for permanent storage. Note that this is lossy in the sense that e.g. the image data is discarded.
    pub fn serialize(&self) -> Result<Vec<u8>, glib::Error> {
        unsafe {
            let mut content = std::ptr::null_mut();
            let mut len = 0;
            let mut error = std::ptr::null_mut();

            libfprint_sys::fp_print_serialize(
                self.to_glib_none().0,
                &mut content,
                &mut len,
                &mut error,
            );

            if error.is_null() {
                Ok(Vec::from_glib_full_num(content, len as usize))
            } else {
                Err(glib::Error::from_glib_full(error.cast()))
            }
        }
    }

    /// Deserialize a print definition from permanent storage.
    pub fn deserialize(data: &[u8]) -> Result<FpPrint, glib::Error> {
        let len = data.len();
        let ptr = unsafe {
            let ptr = glib::translate::ToGlibPtr::to_glib_none(data);
            let mut error = std::ptr::null_mut();

            libfprint_sys::fp_print_deserialize(ptr.0, len as u64, &mut error)
        };

        if ptr.is_null() {
            Err(unsafe { glib::Error::from_glib_full(ptr.cast()) })
        } else {
            let print = unsafe { FpPrint::from_glib_full(ptr) };
            unsafe { print.set_data("set", true) };
            Ok(print)
        }
    }
}
