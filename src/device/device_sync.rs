use crate::device::{callback::fp_match_cb, fn_pointer, UserData};
use crate::image::FpImage;
use gio::Cancellable;
use glib::translate::FromGlibPtrNone;
use glib::translate::{FromGlibPtrFull, ToGlibPtr};
use glib::ObjectExt;
use std::sync::Arc;

use crate::print::FpPrint;

use super::FpDevice;

/// This type represents the callback function for the `FpDevice::enroll` implementation and will be called for each stage of the enrollment process.
pub type FpEnrollProgress<T> =
    fn(&FpDevice, i32, Option<FpPrint>, Option<crate::GError>, &Option<T>) -> ();
/// This type represents the callback function for the `FpDevice::verify` and `FpDevice::identify` implementations and will be called when a print is matched.
pub type FpMatchCb<T> =
    fn(&FpDevice, Option<FpPrint>, FpPrint, Option<crate::GError>, &Option<T>) -> ();

impl FpDevice {
    #[cfg(not(doctest))]
    /// Open the device synchronously.
    /// # Example:
    /// ```no_run
    /// use libfprint_rs::{FpDevice, FpContext};
    ///
    /// let ctx = FpContext::new();
    /// let devices = ctx.devices();
    /// let dev = devices.get(0).unwrap();
    ///
    /// dev.open_sync(None).unwrap();
    /// ```
    pub fn open_sync(&self, cancellable: Option<&Cancellable>) -> Result<(), crate::GError> {
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
    #[cfg(not(doctest))]
    /// Close the device synchronously.
    /// # Example:
    /// ```no_run
    /// use libfprint_rs::{FpDevice, FpContext};
    ///
    /// let ctx = FpContext::new();
    /// let devices = ctx.devices();
    /// let dev = devices.get(0).unwrap();
    ///
    /// dev.open_sync(None).unwrap();
    /// dev.close_sync(None).unwrap();
    /// ```
    pub fn close_sync(&self, cancellable: Option<&Cancellable>) -> Result<(), crate::GError> {
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

    #[cfg(not(doctest))]
    /// Enroll a new print.
    /// Enrolls a print, `progress_cb` will be called for each stage of the enrollment process.
    /// # Example:
    /// ```no_run
    /// use libfprint_rs::{FpDevice, FpContext, FpPrint};
    ///
    /// pub fn enroll_cb(device: &FpDevice,enroll_stage: i32, print: Option<FpPrint>, error: Option<libfprint_rs::GError>, data: &Option<i32>,) -> () {
    ///     println!("Enroll stage: {}", enroll_stage);
    /// }
    ///
    /// let ctx = FpContext::new();
    /// let devices = ctx.devices();
    /// let dev = devices.get(0).unwrap();
    /// dev.open_sync(None).unwrap();
    ///
    /// let template = FpPrint::new(&dev);
    /// let new_print = dev.enroll_sync(template, None, Some(enroll_cb), Some(10)).unwrap();
    ///
    /// dev.close_sync(None).unwrap();
    /// ```
    pub fn enroll_sync<T>(
        &self,
        template: FpPrint,
        cancellable: Option<&Cancellable>,
        progress_cb: Option<FpEnrollProgress<T>>,
        progress_data: Option<T>,
    ) -> Result<FpPrint, crate::GError> {
        let mut error = std::ptr::null_mut();

        let template = self.check_print(template);

        let raw_dev = self.to_glib_none().0;
        let raw_cancel = match cancellable {
            Some(p) => p.to_glib_none().0,
            None => std::ptr::null_mut(),
        };

        let user_ptr = fn_pointer!(progress_cb, progress_data);

        // Raw template: transfer full
        let raw_template: *mut libfprint_sys::FpPrint = template.to_glib_full();

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
            let _: Arc<UserData<FpEnrollProgress<T>, T>> =
                unsafe { Arc::from_raw(user_ptr.cast()) };
        }

        if !ptr.is_null() {
            let fp = unsafe { FpPrint::from_glib_full(ptr) };
            unsafe {
                fp.set_data("set", true);
            }
            return Ok(fp);
        } else {
            return Err(unsafe { glib::Error::from_glib_full(error.cast()) });
        }
    }

    #[cfg(not(doctest))]
    /// Verify a given print synchronously.
    /// `match_cb` will be called when the verification is done.
    /// # Example:
    /// ```no_run
    /// use libfprint_rs::{FpDevice, FpContext, FpPrint, GError};
    ///
    /// pub fn match_cb(device: &FpDevice, matched_print: Option<FpPrint>, enrolled_print: FpPrint,
    /// error: Option<GError>, data: &Option<i32>) {
    ///     if matched_print.is_some() {
    ///         println!("Matched print: {:?}", matched_print);
    ///     }
    /// }
    /// let ctx = FpContext::new();
    /// let devices = ctx.devices();
    /// let dev = devices.get(0).unwrap();
    /// dev.open_sync(None).unwrap();
    ///
    /// let some_print: FpPrint = foreign_function_that_gets_print();
    /// let mut new_print = FpPrint::new(&dev); // The variable that will hold the new print
    /// let verified = dev.verify_sync(&some_print, None, Some(match_cb), Some(10), Some(&mut
    /// new_print)).unwrap();
    /// if verified {
    ///    println!("Print verified");println
    /// }
    /// ```
    pub fn verify_sync<T>(
        &self,
        enrolled_print: &FpPrint,
        cancellable: Option<&Cancellable>,
        match_cb: Option<FpMatchCb<T>>,
        match_data: Option<T>,
        print: Option<&mut FpPrint>, // TODO: Handle initialized
    ) -> Result<bool, crate::GError> {
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
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    /// Prepare device for suspend.
    pub fn suspend_sync(&self, cancellable: Option<&Cancellable>) -> Result<(), crate::GError> {
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

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    /// Resume device after suspend.
    pub fn resume_sync(&self, cancellable: Option<&Cancellable>) -> Result<(), crate::GError> {
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
    #[cfg(not(doctest))]
    /// Identify a print synchronously.
    ///
    /// `match_cb` will be called when a print matches or at the end of the operation.
    /// # Example:
    /// ```no_run
    /// use libfprint_rs::{FpDevice, FpContext, FpPrint, GError};
    ///
    /// pub fn match_cb(device: &FpDevice, matched_print: Option<FpPrint>, enrolled_print: FpPrint,
    /// error: Option<GError>, data: &Option<i32>) {
    ///     if matched_print.is_some() {
    ///         println!("Matched print: {:?}", matched_print);
    ///     }
    /// }
    ///
    /// let ctx = FpContext::new();
    /// let devices = ctx.devices();
    /// let dev = devices.get(0).unwrap();
    /// dev.open_sync(None).unwrap();
    ///
    /// let vec_prints: Vec<FpPrint> = function_returning_Vec_prints();
    /// let mut new_print = FpPrint::new(&dev); // The variable that will hold the new print
    /// let print_identified = dev.identify_sync(&vec_prints, None, Some(match_cb), Some(10), Some(&mut
    /// new_print)).unwrap();
    /// if print_identified.is_some() {
    ///     println!("Found matching print on vector passed");
    /// }
    /// ```
    pub fn identify_sync<T>(
        &self,
        prints: &Vec<FpPrint>,
        cancellable: Option<&Cancellable>,
        match_cb: Option<FpMatchCb<T>>,
        match_data: Option<T>,
        print: Option<&mut FpPrint>, // TODO: Handle initialized
    ) -> Result<Option<FpPrint>, crate::GError> {
        // Arc the function content and the data, get the pointer. If no function is provided
        // then a null pointer is returned.

        use glib::translate::ToGlibContainerFromSlice;
        let ptr = fn_pointer!(match_cb, match_data);

        // Create a GPtrArray from the vector of prints
        let raw_prints: (*mut glib::ffi::GPtrArray, _) =
            ToGlibContainerFromSlice::to_glib_container_from_slice(&prints);

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
                raw_prints.0.cast(),
                raw_cancel.cast(),
                Some(fp_match_cb::<FpMatchCb<T>, T>),
                ptr,
                new_print_ptr,
                std::ptr::addr_of_mut!(print_match),
                std::ptr::addr_of_mut!(error),
            )
        };
        unsafe { libfprint_sys::g_ptr_array_free(raw_prints.0.cast(), 1) };

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
    #[cfg(not(doctest))]
    /// Start an synchronous operation to capture an image.
    /// # Example:
    /// ```no_run
    /// let ctx = FpContext::new();
    /// let devices = ctx.devices();
    /// let dev = devices.get(0).unwrap();
    /// dev.open_sync(None).unwrap();
    ///
    /// let image = dev.capture_sync(true, None).unwrap();
    /// ```
    pub fn capture_sync(
        &self,
        wait_for_finger: bool,
        cancellable: Option<&Cancellable>,
    ) -> Result<FpImage, crate::GError> {
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

    /// Delete a given print from the device.
    pub fn delete_print_sync() {
        unimplemented!()
    }
    /// List device stored prints synchronously.
    pub fn list_prints_sync() {
        unimplemented!()
    }
    /// Clear sensor storage.
    pub fn clear_storage_sync() {
        unimplemented!()
    }

    fn check_print(&self, template: FpPrint) -> FpPrint {
        // This checks if the template was created with FpPrint::new() or not
        let set: Option<bool> = unsafe { template.steal_data("set") };
        if set == Some(true) {
            let empty_template = FpPrint::new(&self);
            if let Some(username) = template.username() {
                empty_template.set_username(&username);
            }
            if let Some(description) = template.description() {
                empty_template.set_description(&description);
            }
            empty_template.set_finger(template.finger());
            if let Some(date) = template.enroll_date() {
                empty_template.set_enroll_date(date);
            }
            drop(template);
            return empty_template;
        } else {
            return template;
        };
    }
}
