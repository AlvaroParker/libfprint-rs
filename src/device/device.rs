/*
# Not added
FpDeviceFeature 	fp_device_get_features ()
gboolean 	fp_device_identify_sync ()

# Added but todo!
gboolean 	fp_device_verify_sync ()
GPtrArray * 	fp_device_list_prints_sync ()
*/

use std::{cell::RefCell, marker::PhantomData, rc::Rc};

pub type FpEnrollProgress<T> = fn(&FpDevice, i32, FpPrint, Option<GError>, &mut Option<T>) -> ();
pub type FpMatchCb<T> = fn(&FpDevice, Option<FpPrint>, FpPrint, Option<GError>, &Option<T>) -> ();

use crate::{
    context::FpContext,
    device::FpDeviceFeature,
    device::FpScanType,
    error::{self, GError},
    finger::FpFingerStatusFlags,
    image::FpImage,
    print::{self, FpPrint},
    utils::{ptr_to_str, ptr_to_str_static},
};

use super::{
    extern_fn::{fb_enroll_progress, fp_match_cb},
    FpDevice, UserData,
};

impl<'a> FpDevice<'a> {
    #[cfg(not(doctest))]
    /// Identify a print from a given vector of prints.
    /// # Example:
    ///
    /// ```
    /// use libfprint_rs::{device::FpDevice, error::GError, print::FpPrint, context::FpContext};
    ///
    /// let ctx = FpContext::new();
    /// let dev = ctx.get_devices().iter().next().unwrap();

    ///
    /// fn callback_function(
    ///     device: &FpDevice,              // The fingerprint scanner device
    ///     matched_print: Option<FpPrint>, // The matched print, if any.
    ///     new_print: FpPrint,             // New print scanned.
    ///     error: Option<GError>,          // Optinal error in case of an error.
    ///     match_data: &Option<()>         // User data can be any data type.
    /// )
    /// {
    ///     if matched_print.is_some() {
    ///         println!("Found matched print!");
    ///     }
    /// }
    /// let prints: Vec<FpPrint> = function_to_get_prints(&dev);
    /// dev.identify(prints, Some(callback_function), None::<()>, None, None);
    /// ```
    pub fn identify<T>(
        &self,
        mut prints: Vec<FpPrint>,
        callback_fn: Option<FpMatchCb<T>>,
        user_data: Option<T>,
        matched_print: Option<&mut FpPrint>,
        new_print: Option<&mut FpPrint>,
    ) -> Result<(), GError<'static>> {
        // Create pointer to callback function
        let ptr_cb = match callback_fn {
            Some(cb) => {
                let data = UserData {
                    function: cb,
                    data: user_data,
                };
                let boxed = Box::new(data);
                Box::into_raw(boxed).cast()
            }
            None => std::ptr::null_mut(),
        };

        // Create array of pointers
        let ptr_arr: libfprint_sys::GPtrArray_autoptr = unsafe { libfprint_sys::g_ptr_array_new() }; // Autoptr GArray
        prints.iter_mut().for_each(|print| unsafe {
            libfprint_sys::g_ptr_array_add(ptr_arr, (*print.print.borrow()).cast())
        });

        // Create gerror
        let mut gerror = std::ptr::null_mut();

        // Matched print pointer
        let matched_print_raw = get_print_ptr(matched_print);

        // New print pointer
        let new_print_raw = get_print_ptr(new_print);

        // Call the function with the given arguments
        unsafe {
            libfprint_sys::fp_device_identify_sync(
                *self.device,
                ptr_arr,
                std::ptr::null_mut(),
                Some(fp_match_cb::<FpMatchCb<T>, T>),
                ptr_cb,
                matched_print_raw.cast(),
                new_print_raw.cast(),
                std::ptr::addr_of_mut!(gerror),
            )
        };

        // Cleanup
        unsafe { libfprint_sys::g_ptr_array_free(ptr_arr.cast(), 1) };

        // Return Ok or Err if error
        if gerror.is_null() {
            Ok(())
        } else {
            Err(unsafe { error::from_libfprint_static(gerror) })
        }
    }
    /// Returns the name of the device.
    pub fn get_name(&self) -> &str {
        let raw_name = unsafe { libfprint_sys::fp_device_get_name(*self.device) };
        ptr_to_str(self.context, raw_name.cast())
    }
    /// Open the device.
    pub fn open(&self) -> Result<(), GError<'static>> {
        let mut gerror = std::ptr::null_mut();
        let res = unsafe {
            libfprint_sys::fp_device_open_sync(
                *self.device,
                std::ptr::null_mut(),
                std::ptr::addr_of_mut!(gerror),
            )
        };
        if res == 1 {
            Ok(())
        } else {
            Err(unsafe { error::from_libfprint_static(gerror) })
        }
    }
    /// Close the device.
    pub fn close(&self) -> Result<(), GError<'static>> {
        let mut gerror = std::ptr::null_mut();
        let res = unsafe {
            libfprint_sys::fp_device_close_sync(
                *self.device,
                std::ptr::null_mut(),
                std::ptr::addr_of_mut!(gerror),
            )
        };
        if res == 1 {
            Ok(())
        } else {
            Err(unsafe { error::from_libfprint_static(gerror) })
        }
    }
    /// Enroll a new print. Template print is a print with relevant metadata filled in.
    /// # Example:
    /// ```
    ///
    /// use libfprint_rs::{device::FpDevice, error::GError, print::FpPrint, context::FpContext};
    ///
    /// let ctx = FpContext::new();
    /// let dev = ctx.get_devices().iter().next().unwrap();
    ///
    /// fn callback_fn(
    ///     device: &FpDevice,
    ///     completed_stages: i32,
    ///     print: FpPrint,
    ///     error: Option<GError>,
    ///     user_data: &mut Option<()>
    /// ) {
    ///     if error.is_none() {
    ///         println!("Enrolling: {} of {}", completed_stages, device.get_nr_enroll_stages() );
    ///     }
    /// }
    /// dev.open().unwrap();
    /// let template_print = FpPrint::new(&dev);
    ///
    /// dev.enroll(template_print, Some(callback_fn), None::<()>);
    /// ```
    pub fn enroll<T>(
        &self,
        mut template_print: FpPrint,
        callback_fn: Option<FpEnrollProgress<T>>,
        user_data: Option<T>,
    ) -> Result<FpPrint<'static>, GError<'static>> {
        let (ptr, ptr_clone) = match callback_fn {
            Some(cb) => {
                let data = UserData {
                    function: cb,
                    data: user_data,
                };
                let boxed = Box::new(data);
                let ptr = Box::into_raw(boxed);
                (ptr, ptr.clone())
            }
            None => (std::ptr::null_mut(), std::ptr::null_mut()),
        };

        let mut gerror = std::ptr::null_mut();
        // let template = template_print.print;
        let raw_print = unsafe {
            libfprint_sys::fp_device_enroll_sync(
                *self.device,
                *template_print.print.borrow(),
                std::ptr::null_mut(),
                Some(fb_enroll_progress::<FpEnrollProgress<T>, T>),
                ptr.cast(),
                std::ptr::addr_of_mut!(gerror),
            )
        };
        // println!("{:p}", ptr_clone);
        if !ptr_clone.is_null() {
            let boxed = unsafe { Box::from_raw(ptr_clone) };
            drop(boxed);
        }
        if raw_print.is_null() {
            Err(unsafe { error::from_libfprint_static(gerror) })
        } else {
            template_print.print = Rc::new(RefCell::new(std::ptr::null_mut()));
            let print = unsafe { print::from_libfprint_static(raw_print) };
            Ok(print)
        }
    }

    #[cfg(not(doctest))]
    /// Verify a given print.
    /// # Example:
    /// ```
    /// use libfprint_rs::{device::FpDevice, error::GError, print::FpPrint, context::FpContext};
    ///
    /// let ctx = FpContext::new();
    /// let dev = ctx.get_devices().iter().next().unwrap();
    ///
    /// fn match_callback_fn(
    ///     device: &FpDevice,              // The fingerprint scanner device
    ///     matched_print: Option<FpPrint>, // The matched print, if any.
    ///     new_print: FpPrint,             // New print scanned.
    ///     error: Option<GError>,          // Error, if any.
    ///     match_data: &Option<()>         // User data to pass to this function.
    /// )
    /// {
    ///     if matched_print.is_some() {
    ///         println!("Found matched print!");
    ///     }
    /// }
    /// // This dummy fn gets an existing print
    /// let enrolled_print = get_enrolled_print();
    /// // Where we saved the new scanned print
    /// let mut scanned_print = FpPrint::new(&dev);
    /// dev.verify(enrolled_print, Some(match_callback_fn), None::<()>, Some(&mut scanned_print));
    /// ```
    pub fn verify<T>(
        &self,
        enrolled_print: FpPrint,
        callback_fn: Option<FpMatchCb<T>>,
        match_data: Option<T>,
        scanned_print: Option<&mut FpPrint>,
    ) -> Result<bool, GError<'static>> {
        let ptr = match callback_fn {
            Some(cb) => {
                let data = UserData {
                    function: cb,
                    data: match_data,
                };
                let boxed = Box::new(data);
                let ptr = Box::into_raw(boxed);
                ptr
            }
            None => std::ptr::null_mut(),
        };

        let mut gerror = std::ptr::null_mut();
        let mut matched = 0;

        let raw_scanned_print = get_print_ptr(scanned_print);

        let res = unsafe {
            libfprint_sys::fp_device_verify_sync(
                *self.device,
                *enrolled_print.print.borrow(),
                std::ptr::null_mut(),
                Some(fp_match_cb::<FpMatchCb<T>, T>),
                ptr.cast(),
                std::ptr::addr_of_mut!(matched),
                raw_scanned_print.cast(),
                std::ptr::addr_of_mut!(gerror),
            )
        };
        if res == 1 {
            return Ok(matched == 1);
        } else {
            Err(unsafe { error::from_libfprint_static(gerror) })
        }
    }
    /// Start operation to capture an image.
    pub fn capture(&self) -> Result<FpImage, GError<'static>> {
        let mut raw_error = std::ptr::null_mut();
        let raw_image = unsafe {
            libfprint_sys::fp_device_capture_sync(
                *self.device,
                1,
                std::ptr::null_mut(),
                std::ptr::addr_of_mut!(raw_error),
            )
        };
        if raw_image.is_null() {
            Err(unsafe { error::from_libfprint_static(raw_error) })
        } else {
            Ok(FpImage { image: raw_image })
        }
    }
    /// Retrieves the number of enroll stages for this device.
    pub fn get_nr_enroll_stages(&self) -> i32 {
        unsafe { libfprint_sys::fp_device_get_nr_enroll_stages(*self.device) }
    }
    /// Retrieves the finger status flags for the device. This can be used by the UI to present the relevant feedback, although it is not guaranteed to be a relevant value when not performing any action.
    pub fn get_finger_status(&self) -> FpFingerStatusFlags {
        let status_raw = unsafe { libfprint_sys::fp_device_get_finger_status(*self.device) };
        FpFingerStatusFlags::from(status_raw)
    }
    pub fn get_features(&self) {
        todo!()
    }
    pub fn get_driver(&self) -> &str {
        let raw_str = unsafe { libfprint_sys::fp_device_get_driver(*self.device) };
        ptr_to_str_static(raw_str.cast())
    }
    pub fn get_device_id(&self) -> &str {
        let raw_id = unsafe { libfprint_sys::fp_device_get_device_id(*self.device) };
        ptr_to_str_static(raw_id.cast())
    }
    /// Retrieves the scan type of the device (`FpScanType`)
    pub fn get_scan_type(&self) -> FpScanType {
        let raw_type = unsafe { libfprint_sys::fp_device_get_scan_type(*self.device) };
        FpScanType::from(raw_type)
    }
    /// Checks if the `FpDevice` supports the requested `FpDeviceFeature`.
    #[cfg(target_arch = "x86_64")]
    pub fn has_feature(&self, features: FpDeviceFeature) -> bool {
        let res = unsafe { libfprint_sys::fp_device_has_feature(*self.device, features as u32) };
        if res == 1 {
            true
        } else {
            false
        }
    }
    // response of gboolean are equivalent to true if they are 1, else they are false (usually 0)
    // todo: check the docs for if this is stable
    pub fn is_open(&self) -> bool {
        let res = unsafe { libfprint_sys::fp_device_is_open(*self.device) };
        if res == 1 {
            true
        } else {
            false
        }
    }
    // todo: check if this is true lol:
    /// Delete a print from the device. This only makes sense on devices that store prints on-chip, but is safe to always call.
    pub fn delete_print(&self, print: FpPrint) -> Result<(), GError> {
        let mut raw_error = std::ptr::null_mut();
        let res = unsafe {
            libfprint_sys::fp_device_delete_print_sync(
                *self.device,
                *print.print.borrow(),
                std::ptr::null_mut(),
                std::ptr::addr_of_mut!(raw_error),
            )
        };
        if res == 1 {
            Ok(())
        } else {
            Err(unsafe { error::from_libfprint_static(raw_error) })
        }
    }
    pub fn list_prints(&self) {
        todo!();
    }
    /// Deletes all prints from the device. This only makes sense on devices that store prints on-chip, but is safe to always call.
    #[cfg(target_arch = "x86_64")]
    pub fn clear_storage(&self) -> Result<(), GError> {
        let mut raw_error = std::ptr::null_mut();
        let res = unsafe {
            libfprint_sys::fp_device_clear_storage_sync(
                *self.device,
                std::ptr::null_mut(),
                std::ptr::addr_of_mut!(raw_error),
            )
        };
        if res == 1 {
            Ok(())
        } else {
            Err(unsafe { error::from_libfprint_static(raw_error) })
        }
    }
    /// Prepare device for suspend.
    #[cfg(target_arch = "x86_64")]
    pub fn suspend(&self) -> Result<(), GError> {
        let mut raw_error = std::ptr::null_mut();
        let res = unsafe {
            libfprint_sys::fp_device_suspend_sync(
                *self.device,
                std::ptr::null_mut(),
                std::ptr::addr_of_mut!(raw_error),
            )
        };
        if res == 1 {
            Ok(())
        } else {
            Err(unsafe { error::from_libfprint_static(raw_error) })
        }
    }

    /// Resume device after suspend.
    #[cfg(target_arch = "x86_64")]
    pub fn resume(&self) -> Result<(), GError> {
        let mut raw_error = std::ptr::null_mut();
        let res = unsafe {
            libfprint_sys::fp_device_resume_sync(
                *self.device,
                std::ptr::null_mut(),
                std::ptr::addr_of_mut!(raw_error),
            )
        };
        if res == 1 {
            Ok(())
        } else {
            Err(unsafe { error::from_libfprint_static(raw_error) })
        }
    }
}

fn get_print_ptr(print: Option<&mut FpPrint>) -> *mut *mut std::ffi::c_void {
    let raw_print;
    if let Some(print) = print {
        let raw = *print.print.borrow();
        if !raw.is_null() {
            unsafe { libfprint_sys::g_object_unref(raw.cast()) };
        }
        let null_ptr: libfprint_sys::FpPrint_autoptr = std::ptr::null_mut();
        print.print = Rc::new(RefCell::new(null_ptr));
        raw_print = std::ptr::addr_of_mut!(*print.print.borrow_mut())
    } else {
        raw_print = std::ptr::null_mut();
    };
    raw_print.cast()
}

pub(crate) unsafe fn from_libfprint<'a>(
    context: PhantomData<&'a FpContext>,
    device: *mut libfprint_sys::FpDevice,
) -> FpDevice<'a> {
    FpDevice {
        context,
        device: Rc::new(device),
    }
}