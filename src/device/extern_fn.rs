use std::{cell::RefCell, marker::PhantomData, rc::Rc};

use crate::{
    error::{self, GError},
    print::FpPrint,
};

use super::{FpDevice, UserData};
pub(crate) extern "C" fn fb_enroll_progress<F, T>(
    device: *mut libfprint_sys::FpDevice,
    completed_stages: libfprint_sys::gint,
    print: *mut libfprint_sys::FpPrint,
    user_data: libfprint_sys::gpointer,
    error: *mut libfprint_sys::GError,
) where
    F: Fn(&FpDevice, i32, FpPrint, Option<GError>, &Option<T>) -> (),
{
    // We use the user data pointer to pass the callback function and the user data
    if !user_data.is_null() {
        // We build the UserData struct from the user data pointer
        // Safety: We are the only ones who have access to the user data pointer
        // The pointer was created using Box::into_raw, so it's safe to cast it back to a Box
        // using Box::from_raw
        let callback_data: Box<UserData<F, T>> = unsafe { Box::from_raw(user_data.cast()) };
        // We build the structs needed by the Rust callback function passed by the user
        let device = FpDevice::<'static> {
            device: Rc::new(device),
            context: PhantomData,
        };
        let print = FpPrint::<'static> {
            print: Rc::new(RefCell::new(print)),
            context: PhantomData,
            auto_drop: true,
        };
        let gerror = if error.is_null() {
            None
        } else {
            // Transfer is none, so we pass false. This pointer will get drop at the end of
            // the callback function
            unsafe { Some(error::from_libfprint_static(error, false)) }
        };

        // We call the user-provided callback function
        callback_data.callback_enroll(&device, completed_stages, print, gerror);
        // We forget the memory where the callback function is stored, so it won't get dropped
        // we do this because the callback function will get called again
        // This is safe because the only copy to this pointer is in Device::enroll and the pointer
        // is freed at the end of the fp_device_enroll function
        _ = std::mem::forget(callback_data);
    }
}
pub(crate) extern "C" fn fp_match_cb<F, T>(
    device: *mut libfprint_sys::FpDevice,
    matched_print: *mut libfprint_sys::FpPrint,
    print: *mut libfprint_sys::FpPrint,
    user_data: libfprint_sys::gpointer,
    error: *mut libfprint_sys::GError,
) where
    F: Fn(&FpDevice, Option<FpPrint>, FpPrint, Option<GError>, &Option<T>),
{
    // We use the user data pointer to pass the callback function and the user data
    if !user_data.is_null() {
        // We build the UserData struct instance from the user data pointer
        // Safety: We are the only ones who have access to the user data pointer
        // and the pointer was created using Box::into_raw, so it's safe to cast it back to a Box
        // the Box will get dropped at the end of the callback function, and since the callback
        // function only gets called once, the Box will get dropped only once
        let callback_data: Box<UserData<F, T>> = unsafe { Box::from_raw(user_data.cast()) };
        // We build the structs needed by the Rust callback function passed by the user
        let device = FpDevice::<'static> {
            device: Rc::new(device),
            context: PhantomData,
        };
        let matched_print = if matched_print.is_null() {
            None
        } else {
            Some(FpPrint::<'static> {
                print: Rc::new(RefCell::new(matched_print)),
                context: PhantomData,
                auto_drop: true,
            })
        };
        let print = FpPrint::<'static> {
            print: Rc::new(RefCell::new(print)),
            context: PhantomData,
            auto_drop: true,
        };

        // Check for errors
        let gerror = if error.is_null() {
            None
        } else {
            unsafe { Some(error::from_libfprint_static(error, false)) }
        };
        // Run the callback function
        callback_data.callback_match(&device, matched_print, print, gerror);
    }
}
