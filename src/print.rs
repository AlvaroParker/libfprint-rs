// All methods are declared
use glib::{
    translate::FromGlibPtrFull,
    translate::{FromGlibContainer, FromGlibPtrNone, ToGlibPtr},
    wrapper,
};

use crate::{device::FpDevice, finger::FpFinger, image::FpImage};

wrapper! {
    pub struct FpPrint(Object<libfprint_sys::FpPrint, libfprint_sys::FpPrintClass>)
        @extends glib::object::InitiallyUnowned;

    match fn {
        type_ => || libfprint_sys::fp_print_get_type() as usize,
    }
}

impl FpPrint {
    pub fn new(dev: &FpDevice) -> Self {
        unsafe {
            let ptr = libfprint_sys::fp_print_new(dev.to_glib_none().0);
            Self::from_glib_full(ptr)
        }
    }

    pub fn driver(&self) -> String {
        unsafe {
            let ptr = libfprint_sys::fp_print_get_driver(self.to_glib_none().0);
            String::from_glib_none(ptr)
        }
    }
    pub fn device_id(&self) -> String {
        unsafe {
            let ptr = libfprint_sys::fp_print_get_device_id(self.to_glib_none().0);
            String::from_glib_none(ptr)
        }
    }
    pub fn device_stored(&self) -> bool {
        unsafe {
            libfprint_sys::fp_print_get_device_stored(self.to_glib_none().0) == glib::ffi::GTRUE
        }
    }
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
    pub fn finger(&self) -> FpFinger {
        let raw_finger = unsafe { libfprint_sys::fp_print_get_finger(self.to_glib_none().0) };
        FpFinger::from(raw_finger)
    }
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
    pub fn enroll_date(&self) -> Option<glib::Date> {
        unsafe {
            let ptr = libfprint_sys::fp_print_get_enroll_date(self.to_glib_none().0);
            if ptr.is_null() {
                None
            } else {
                Some(glib::Date::from_glib_none(ptr.cast()))
            }
        }
    }

    pub fn set_finger(&self, finger: FpFinger) {
        unsafe { libfprint_sys::fp_print_set_finger(self.to_glib_none().0, finger as u32) };
    }
    pub fn set_username(&self, username: &str) {
        unsafe {
            libfprint_sys::fp_print_set_username(self.to_glib_none().0, username.to_glib_none().0);
        }
    }
    pub fn set_description(&self, description: &str) {
        unsafe {
            libfprint_sys::fp_print_set_description(
                self.to_glib_none().0,
                description.to_glib_none().0,
            );
        }
    }

    pub fn set_enroll_date(&self, enroll_date: glib::Date) {
        unsafe {
            libfprint_sys::fp_print_set_enroll_date(
                self.to_glib_none().0,
                enroll_date.to_glib_none().0.cast(),
            );
        }
    }
    pub fn compatible(&self, device: &FpDevice) -> bool {
        unsafe {
            libfprint_sys::fp_print_compatible(self.to_glib_none().0, device.to_glib_none().0)
                == glib::ffi::GTRUE
        }
    }
    pub fn equal(&self, other: &FpPrint) -> bool {
        unsafe {
            libfprint_sys::fp_print_equal(self.to_glib_none().0, other.to_glib_none().0)
                == glib::ffi::GTRUE
        }
    }
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
            Ok(unsafe { FpPrint::from_glib_full(ptr) })
        }
    }
}
