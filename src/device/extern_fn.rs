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
    F: Fn(&FpDevice, i32, FpPrint, Option<GError>, &mut Option<T>) -> (),
{
    if !user_data.is_null() {
        //let ptr = user_data.cast::<UserData<F, T>>();
        let mut callback_data: Box<UserData<F, T>> = unsafe { Box::from_raw(user_data.cast()) };
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
            unsafe { Some(error::from_libfprint_static(error)) }
        };

        callback_data.callback_enroll(&device, completed_stages, print, gerror);
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
    if !user_data.is_null() {
        let callback_data: Box<UserData<F, T>> = unsafe { Box::from_raw(user_data.cast()) };
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

        let gerror = if error.is_null() {
            None
        } else {
            unsafe { Some(error::from_libfprint_static(error)) }
        };
        callback_data.callback_match(&device, matched_print, print, gerror);
    }
}
