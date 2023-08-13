use crate::device::{callback::fp_match_cb, fn_pointer};
use crate::image::FpImage;
use gio::Cancellable;
use glib::translate::FromGlibPtrNone;
use glib::translate::{FromGlibPtrFull, ToGlibPtr};

use crate::print::FpPrint;

use super::FpDevice;

/// This type represents the callback function for the `FpDevice::enroll` implementation and will be called for each stage of the enrollment process.
pub type FpEnrollProgress<T> =
    fn(&FpDevice, i32, Option<FpPrint>, Option<glib::Error>, &Option<T>) -> ();
/// This type represents the callback function for the `FpDevice::verify` and `FpDevice::identify` implementations and will be called when a print is matched.
pub type FpMatchCb<T> =
    fn(&FpDevice, Option<FpPrint>, FpPrint, Option<glib::Error>, &Option<T>) -> ();

impl FpDevice {
    pub fn open_sync(&self, cancellable: Option<&Cancellable>) -> Result<(), glib::Error> {
        let raw_cancel = match cancellable {
            Some(p) => p.to_glib_none().0,
            None => std::ptr::null_mut(),
        };

        let mut error = std::ptr::null_mut();

        let res = unsafe {
            libfprint_sys::fp_device_open_sync(
                self.to_glib_none().0,
                raw_cancel.cast(),
                std::ptr::addr_of_mut!(error),
            )
        };
        if res == glib::ffi::GFALSE {
            return Err(unsafe { glib::Error::from_glib_full(error.cast()) });
        }
        Ok(())
    }
    pub fn close_sync(&self, cancellable: Option<&Cancellable>) -> Result<(), glib::Error> {
        let raw_cancel = match cancellable {
            Some(p) => p.to_glib_none().0,
            None => std::ptr::null_mut(),
        };

        let mut error = std::ptr::null_mut();

        let res = unsafe {
            libfprint_sys::fp_device_close_sync(
                self.to_glib_none().0,
                raw_cancel.cast(),
                std::ptr::addr_of_mut!(error),
            )
        };
        if res == glib::ffi::GFALSE {
            return Err(unsafe { glib::Error::from_glib_full(error.cast()) });
        }
        Ok(())
    }

    pub fn enroll_sync<T>(
        &self,
        template: FpPrint,
        cancellable: Option<&Cancellable>,
        progress_cb: Option<FpEnrollProgress<T>>,
        progress_data: Option<T>,
    ) -> Result<FpPrint, glib::Error> {
        let mut error = std::ptr::null_mut();

        let raw_dev = self.to_glib_none().0;
        let raw_cancel = match cancellable {
            Some(p) => p.to_glib_none().0,
            None => std::ptr::null_mut(),
        };

        let user_ptr = fn_pointer!(progress_cb, progress_data);

        // Raw template: transfer full
        let raw_template = template.to_glib_full();

        let ptr = unsafe {
            libfprint_sys::fp_device_enroll_sync(
                raw_dev,
                raw_template,
                raw_cancel.cast(),
                Some(crate::device::callback::fp_enroll_progress::<FpEnrollProgress<T>, T>),
                user_ptr,
                std::ptr::addr_of_mut!(error),
            )
        };

        if !user_ptr.is_null() {
            drop(unsafe { Box::from_raw(user_ptr) });
        }

        if !ptr.is_null() {
            let fp = unsafe { FpPrint::from_glib_full(ptr) };
            return Ok(fp);
        } else {
            return Err(unsafe { glib::Error::from_glib_full(error.cast()) });
        }
    }

    pub fn verify_sync<T>(
        &self,
        enrolled_print: &FpPrint,
        cancellable: Option<gio::Cancellable>,
        match_cb: Option<FpMatchCb<T>>,
        match_data: Option<T>,
        print: Option<&mut FpPrint>, // TODO: This variable is causing memory leaks
    ) -> Result<bool, glib::Error> {
        let ptr = fn_pointer!(match_cb, match_data);
        let mut error = std::ptr::null_mut();
        let mut matched = glib::ffi::GFALSE;

        let mut new_print: libfprint_sys::FpPrint_autoptr = std::ptr::null_mut();
        let new_print_ptr = match print {
            Some(_) => std::ptr::addr_of_mut!(new_print),
            None => std::ptr::null_mut(),
        };

        let raw_cancel = match cancellable {
            Some(p) => p.to_glib_none().0,
            None => std::ptr::null_mut(),
        };

        let res = unsafe {
            libfprint_sys::fp_device_verify_sync(
                self.to_glib_none().0,
                enrolled_print.to_glib_none().0,
                raw_cancel.cast(),
                Some(fp_match_cb::<FpMatchCb<T>, T>),
                ptr,
                &mut matched,
                new_print_ptr,
                &mut error,
            )
        };

        match print {
            Some(p) => {
                if !new_print.is_null() {
                    *p = unsafe { FpPrint::from_glib_full(new_print) };
                }
            }
            None => {}
        };

        // If res is false, the operation failed, so the `error` pointer must be pointing
        // to a valid error
        if res == glib::ffi::GFALSE {
            return Err(unsafe { glib::Error::from_glib_none(error.cast()) });
        }
        // Else there must be a response
        return Ok(matched == glib::ffi::GTRUE);
    }
    pub fn suspend_sync(&self, cancellable: Option<&Cancellable>) -> Result<(), glib::Error> {
        let raw_cancel = match cancellable {
            Some(p) => p.to_glib_none().0,
            None => std::ptr::null_mut(),
        };

        let mut error = std::ptr::null_mut();

        let res = unsafe {
            libfprint_sys::fp_device_suspend_sync(
                self.to_glib_none().0,
                raw_cancel.cast(),
                std::ptr::addr_of_mut!(error),
            )
        };
        if res == glib::ffi::GFALSE {
            return Err(unsafe { glib::Error::from_glib_full(error.cast()) });
        }
        Ok(())
    }

    pub fn resume_sync(&self, cancellable: Option<&Cancellable>) -> Result<(), glib::Error> {
        let raw_cancel = match cancellable {
            Some(p) => p.to_glib_none().0,
            None => std::ptr::null_mut(),
        };

        let mut error = std::ptr::null_mut();

        let res = unsafe {
            libfprint_sys::fp_device_resume_sync(
                self.to_glib_none().0,
                raw_cancel.cast(),
                std::ptr::addr_of_mut!(error),
            )
        };
        if res == glib::ffi::GFALSE {
            return Err(unsafe { glib::Error::from_glib_full(error.cast()) });
        }
        Ok(())
    }
    pub fn identify_sync<T>(
        &self,
        prints: &Vec<FpPrint>,
        cancellable: Option<&Cancellable>,
        match_cb: Option<FpMatchCb<T>>,
        match_data: Option<T>,
        print: Option<&mut FpPrint>,
    ) -> Result<Option<FpPrint>, glib::Error> {
        // Box the function content and the data, get the pointer. If no function is provided
        // then a null pointer is returned.
        let ptr = fn_pointer!(match_cb, match_data);

        // Create a GPtrArray from the vector of prints
        let raw_prints = unsafe {
            let raw_prints = glib::ffi::g_ptr_array_new();
            for print in prints {
                let raw_print: *mut libfprint_sys::FpPrint = print.to_glib_none().0;
                glib::ffi::g_ptr_array_add(raw_prints, raw_print.cast());
            }
            raw_prints
        };

        let raw_cancel = match cancellable {
            Some(p) => p.to_glib_none().0,
            None => std::ptr::null_mut(),
        };

        let mut new_print: libfprint_sys::FpPrint_autoptr = std::ptr::null_mut();
        let new_print_ptr = match print {
            Some(_) => std::ptr::addr_of_mut!(new_print),
            None => std::ptr::null_mut(),
        };

        let mut print_match = std::ptr::null_mut();

        let mut error = std::ptr::null_mut();

        let res = unsafe {
            libfprint_sys::fp_device_identify_sync(
                self.to_glib_none().0,
                raw_prints.cast(),
                raw_cancel.cast(),
                Some(fp_match_cb::<FpMatchCb<T>, T>),
                ptr,
                new_print_ptr,
                std::ptr::addr_of_mut!(print_match),
                std::ptr::addr_of_mut!(error),
            )
        };
        unsafe { glib::ffi::g_ptr_array_free(raw_prints, glib::ffi::GTRUE) };

        match print {
            Some(p) => {
                if !new_print.is_null() {
                    *p = unsafe { FpPrint::from_glib_full(new_print) };
                }
            }
            None => {}
        };

        if res == glib::ffi::GFALSE {
            return Err(unsafe { glib::Error::from_glib_full(error.cast()) });
        }
        if print_match.is_null() {
            return Ok(None);
        } else {
            return Ok(Some(unsafe { FpPrint::from_glib_full(print_match) }));
        }
    }
    pub fn capture_sync(
        &self,
        wait_for_finger: bool,
        cancellable: Option<&Cancellable>,
    ) -> Result<FpImage, glib::Error> {
        let raw_cancel = match cancellable {
            Some(p) => p.to_glib_none().0,
            None => std::ptr::null_mut(),
        };

        let mut raw_error = std::ptr::null_mut();

        let raw_image = unsafe {
            libfprint_sys::fp_device_capture_sync(
                self.to_glib_none().0,
                wait_for_finger as i32,
                raw_cancel.cast(),
                std::ptr::addr_of_mut!(raw_error),
            )
        };
        if raw_image.is_null() {
            return Err(unsafe { glib::Error::from_glib_full(raw_error.cast()) });
        }
        Ok(unsafe { FpImage::from_glib_full(raw_image) })
    }

    pub fn delete_print_sync() {
        unimplemented!()
    }
    pub fn list_prints_sync() {
        unimplemented!()
    }
    pub fn clear_storage_sync() {
        unimplemented!()
    }
}
