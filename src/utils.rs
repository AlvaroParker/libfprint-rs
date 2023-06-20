use std::{
    ffi::{c_void, CStr},
    marker::PhantomData,
};

use crate::context::FpContext;

pub fn ptr_to_str<'a>(_context: PhantomData<&'a FpContext>, ptr: *const c_void) -> &'a str {
    unsafe {
        let ptr = ptr.cast::<core::ffi::c_char>();
        CStr::from_ptr(ptr)
            .to_str()
            .expect("Error while parsing string")
    }
}

pub fn ptr_to_str_static(ptr: *const c_void) -> &'static str {
    unsafe {
        let ptr = ptr.cast::<core::ffi::c_char>();
        CStr::from_ptr(ptr)
            .to_str()
            .expect("Error while parsing string")
    }
}
