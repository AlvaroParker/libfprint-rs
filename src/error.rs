use std::{fmt, marker::PhantomData};

use crate::{
    context::FpContext,
    utils::{ptr_to_str, ptr_to_str_static},
};

#[derive(Debug)]
pub struct GError<'a> {
    code: i32,
    message: &'a str,
    source: GErrorSource,
}

impl<'a> std::error::Error for GError<'a> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.source)
    }
}

impl<'a> fmt::Display for GError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
impl GError<'_> {
    pub fn code(&self) -> i32 {
        self.code
    }
}

#[derive(Debug)]
pub struct GErrorSource {
    source: &'static str,
}
impl std::error::Error for GErrorSource {}
impl fmt::Display for GErrorSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.source)
    }
}

impl GErrorSource {
    pub fn new(domain: u32) -> GErrorSource {
        let raw_str = unsafe { libfprint_sys::g_quark_to_string(domain) };
        GErrorSource {
            source: ptr_to_str_static(raw_str.cast()),
        }
    }
}

pub unsafe fn from_libfprint<'a>(
    context: PhantomData<&'a FpContext>,
    error: *mut libfprint_sys::GError,
) -> GError<'a> {
    GError {
        message: ptr_to_str(context, error.read().message.cast()),
        code: error.read().code,
        source: GErrorSource::new(unsafe { error.read().domain }),
    }
}
pub unsafe fn from_libfprint_static(error: *mut libfprint_sys::GError) -> GError<'static> {
    GError {
        message: ptr_to_str_static(error.read().message.cast()),
        code: error.read().code,
        source: GErrorSource::new(unsafe { error.read().domain }),
    }
}
