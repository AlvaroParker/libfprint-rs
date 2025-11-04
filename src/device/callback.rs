use std::{os::raw::c_int, os::raw::c_void, sync::Arc};

use glib::translate::{FromGlibPtrBorrow, FromGlibPtrNone};

use crate::print::FpPrint;

use super::{FpDevice, UserData};

pub(crate) extern "C" fn fp_enroll_progress<F, T>(
    device: *mut libfprint_sys::FpDevice,
    completed_stages: c_int,
    print: *mut libfprint_sys::FpPrint, // The last scanned print
    user_data: *mut c_void,
    error: *mut libfprint_sys::GError,
) where
    F: Fn(&FpDevice, i32, Option<FpPrint>, Option<glib::Error>, &Option<T>),
{
    // If user data is not null, a callback function was provided
    if !user_data.is_null() {
        // We "reconstruct" the UserData from the void pointer
        // Safety: This pointer is created on FpDevice::enroll_sync, and is only
        // used here. After the callback is called, the pointer is forgotten. Then
        // back in FpDevice::enroll_sync, the pointer is dropped. So we are the only
        // "owners" of the pointer. The user does not have access to this pointer
        let callback_data: Arc<UserData<F, T>> = unsafe { Arc::from_raw(user_data.cast()) };

        // We borrow the device
        let device = unsafe { FpDevice::from_glib_borrow(device) };
        // Convert the raw pointer to a Rust struct
        let print = match print.is_null() {
            true => None,
            false => Some(unsafe { FpPrint::from_glib_none(print) }),
        };

        let error = match error.is_null() {
            true => None,
            false => Some(unsafe { glib::Error::from_glib_none(error.cast()) }),
        };

        callback_data.callback_enroll(&device, completed_stages, print, error);

        std::mem::forget(callback_data);
    }
}

pub(crate) extern "C" fn fp_match_cb<F, T>(
    device: *mut libfprint_sys::FpDevice,
    match_print: *mut libfprint_sys::FpPrint,
    print: *mut libfprint_sys::FpPrint,
    user_data: *mut c_void,
    error: *mut libfprint_sys::GError,
) where
    F: Fn(&FpDevice, Option<FpPrint>, FpPrint, Option<glib::Error>, &Option<T>),
{
    if !user_data.is_null() {
        // We reconstruct the UserData struct from the pointer
        // Safety: We are the only ones who have access to the pointer,
        // which is created either at verify_sync or identify_sync. Either way, the pointer is
        // forgotten after the callback is called, so we aer the only "owners" of the pointer.
        let callback_data: Arc<UserData<F, T>> = unsafe { Arc::from_raw(user_data.cast()) };

        let device = unsafe { FpDevice::from_glib_none(device) };

        let match_print = match match_print.is_null() {
            true => None,
            false => Some(unsafe { FpPrint::from_glib_none(match_print) }),
        };
        let print = unsafe { FpPrint::from_glib_none(print) };
        let error = match error.is_null() {
            true => None,
            false => Some(unsafe { glib::Error::from_glib_none(error.cast()) }),
        };

        callback_data.callback_match(&device, match_print, print, error);
    }
}
