// All methods are declared
use glib::{
    translate::{FromGlibContainer, FromGlibPtrFull, ToGlibPtr},
    wrapper,
};

wrapper! {
    pub struct FpImage(Object<libfprint_sys::FpImage, libfprint_sys::FpImageClass>);

    match fn {
        type_ => || libfprint_sys::fp_image_get_type() as usize,
    }
}

impl FpImage {
    pub fn new(width: u32, height: u32) -> Self {
        unsafe { FpImage::from_glib_full(libfprint_sys::fp_image_new(width as i32, height as i32)) }
    }

    pub fn width(&self) -> u32 {
        unsafe { libfprint_sys::fp_image_get_width(self.to_glib_none().0) as u32 }
    }

    pub fn height(&self) -> u32 {
        unsafe { libfprint_sys::fp_image_get_height(self.to_glib_none().0) as u32 }
    }

    pub fn ppmm(&self) -> f64 {
        unsafe { libfprint_sys::fp_image_get_ppmm(self.to_glib_none().0) }
    }

    fn _minutiae(&self) {
        unimplemented!()
    }
    fn _detect_minutiae(&self) {
        unimplemented!()
    }
    fn _detect_minutiae_finish(&self) {
        unimplemented!()
    }
    pub fn data(&self) -> Vec<u8> {
        unsafe {
            let mut len = 0;
            let data = libfprint_sys::fp_image_get_data(self.to_glib_none().0, &mut len);

            Vec::from_glib_none_num(data, len as usize)
        }
    }

    pub fn binarized(&self) -> Vec<u8> {
        unsafe {
            let mut len = 0;
            let data = libfprint_sys::fp_image_get_binarized(self.to_glib_none().0, &mut len);

            Vec::from_glib_none_num(data, len as usize)
        }
    }
}
