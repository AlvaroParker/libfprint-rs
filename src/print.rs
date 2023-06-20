use std::{cell::RefCell, marker::PhantomData, rc::Rc};

use crate::{
    context::FpContext,
    device::FpDevice,
    error::{self, GError},
    finger::Finger,
    image::FpImage,
    utils::ptr_to_str_static,
};

#[derive(Debug, Clone)]
pub struct FpPrint<'a> {
    pub(crate) context: PhantomData<&'a FpContext>,
    pub(crate) print: Rc<RefCell<*mut libfprint_sys::FpPrint>>,
    pub(crate) auto_drop: bool,
}

pub struct SerializedPrint<'a> {
    pub(crate) data: &'a mut [u8],
}

impl SerializedPrint<'_> {
    pub fn as_slice(&self) -> &[u8] {
        self.data
    }
}

impl Drop for SerializedPrint<'_> {
    fn drop(&mut self) {
        unsafe {
            libfprint_sys::g_free(self.data.as_mut_ptr().cast());
        }
    }
}

impl Drop for FpPrint<'_> {
    fn drop(&mut self) {
        let count = Rc::strong_count(&self.print);
        if !self.auto_drop && !(*self.print.borrow()).is_null() && count == 1 {
            unsafe { libfprint_sys::g_object_unref((*self.print.borrow()).cast()) };
        }
    }
}

impl FpPrint<'_> {
    pub fn new<'a>(device: &'a FpDevice) -> FpPrint<'a> {
        // Possible memory leak
        let raw_print = unsafe { libfprint_sys::fp_print_new(*device.device) };
        FpPrint {
            context: device.context,
            print: Rc::new(RefCell::new(raw_print)),
            auto_drop: false,
        }
    }
    pub fn serialize(&self) -> Result<SerializedPrint, GError> {
        let mut raw_data = std::ptr::null_mut();
        let mut len = 0;
        let mut gerror = std::ptr::null_mut();
        let res = unsafe {
            libfprint_sys::fp_print_serialize(
                *self.print.borrow(),
                std::ptr::addr_of_mut!(raw_data),
                std::ptr::addr_of_mut!(len),
                std::ptr::addr_of_mut!(gerror),
            )
        };
        if res == 1 {
            let print = SerializedPrint {
                data: unsafe { std::slice::from_raw_parts_mut(raw_data, len as usize) },
            };
            Ok(print)
        } else {
            Err(unsafe { error::from_libfprint(self.context, gerror) })
        }
    }
    pub fn deserialize(data: &[u8]) -> Result<FpPrint<'static>, GError> {
        let mut gerror = std::ptr::null_mut();
        let raw_print = unsafe {
            libfprint_sys::fp_print_deserialize(
                data.as_ptr(),
                data.len().try_into().unwrap(),
                std::ptr::addr_of_mut!(gerror),
            )
        };
        if raw_print.is_null() {
            Err(unsafe { error::from_libfprint(PhantomData, gerror) })
        } else {
            Ok(FpPrint {
                context: PhantomData,
                print: Rc::new(RefCell::new(raw_print)),
                auto_drop: false,
            })
        }
    }
    pub fn driver(&self) -> &str {
        let driver = unsafe { libfprint_sys::fp_print_get_driver(*self.print.borrow()) };
        ptr_to_str_static(driver.cast())
    }
    pub fn device_id(&self) -> &str {
        let device_id = unsafe { libfprint_sys::fp_print_get_device_id(*self.print.borrow()) };
        ptr_to_str_static(device_id.cast())
    }
    pub fn device_stored(&self) -> bool {
        let res = unsafe { libfprint_sys::fp_print_get_device_stored(*self.print.borrow()) };
        if res == 1 {
            true
        } else {
            false
        }
    }
    pub fn get_image(&self) -> Option<FpImage> {
        // Untested because of device doesn't support this funcionality
        let raw_image = unsafe { libfprint_sys::fp_print_get_image(*self.print.borrow()) };
        if raw_image.is_null() {
            None
        } else {
            Some(FpImage { image: raw_image })
        }
    }
    pub fn get_finger(&self) -> Finger {
        let raw_finger = unsafe { libfprint_sys::fp_print_get_finger(*self.print.borrow()) };
        Finger::from(raw_finger)
    }
    pub fn get_username(&self) -> Option<&str> {
        let raw_username = unsafe { libfprint_sys::fp_print_get_username(*self.print.borrow()) };
        if raw_username.is_null() {
            None
        } else {
            Some(ptr_to_str_static(raw_username.cast()))
        }
    }
    pub fn get_description(&self) -> Option<&str> {
        let raw_desc = unsafe { libfprint_sys::fp_print_get_description(*self.print.borrow()) };
        if raw_desc.is_null() {
            None
        } else {
            Some(ptr_to_str_static(raw_desc.cast()))
        }
    }
    pub fn get_enroll_date() {
        todo!()
    }
    pub fn set_finger(&self, finger: Finger) {
        unsafe {
            libfprint_sys::fp_print_set_finger(*self.print.borrow(), finger as u32);
        }
    }
    pub fn set_username(&self, username: &str) {
        let username =
            std::ffi::CString::new(username).expect("Error settings username for fingerprint");
        unsafe {
            libfprint_sys::fp_print_set_username(*self.print.borrow(), username.as_ptr());
        }
    }
    pub fn set_description(&self, description: &str) {
        let description = std::ffi::CString::new(description)
            .expect("Error settings description for fingerprint");
        unsafe {
            libfprint_sys::fp_print_set_description(*self.print.borrow(), description.as_ptr());
        }
    }
    pub fn set_enroll_date() {
        todo!()
    }
    pub fn compatible(&self, device: &FpDevice) -> bool {
        let res =
            unsafe { libfprint_sys::fp_print_compatible(*self.print.borrow(), *device.device) };
        if res == 1 {
            true
        } else {
            false
        }
    }
    pub fn equal(&self, other: &FpPrint) -> bool {
        let res =
            unsafe { libfprint_sys::fp_print_equal(*self.print.borrow(), *other.print.borrow()) };
        if res == 1 {
            true
        } else {
            false
        }
    }
}

#[allow(dead_code)]
pub(crate) unsafe fn from_libfprint<'a>(
    context: PhantomData<&'a FpContext>,
    print: *mut libfprint_sys::FpPrint,
) -> FpPrint<'a> {
    FpPrint {
        context,
        print: Rc::new(RefCell::new(print)),
        auto_drop: true,
    }
}
pub(crate) unsafe fn from_libfprint_static(print: *mut libfprint_sys::FpPrint) -> FpPrint<'static> {
    FpPrint {
        context: PhantomData,
        print: Rc::new(RefCell::new(print)),
        auto_drop: false,
    }
}
