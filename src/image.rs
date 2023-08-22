// All methods are declared
use glib::{
    translate::{FromGlibContainer, FromGlibPtrFull, ToGlibPtr},
    wrapper,
};

wrapper! {
#[cfg(not(doctest))]
/// Struct representing an image of a fingerprint. Not all devices support this feature.
/// # Examples:
/// ```rust
/// use libfprint_rs::FpContext;
/// use std::fs::File;
/// use std::io::Write;
///
/// let context = FpContext::new();
/// let devices = context.devices();
/// let device = devices.get(0).unwrap();
///
/// device.open_sync(None).unwrap();
/// let image = device.capture_sync(true, None).unwrap();
/// let data = image.data();
///
/// let mut file = File::create("image.pgm").unwrap();
/// let header = format!("P5\n{} {}\n255\n", image.width(), image.height());
/// file.write_all(header.as_bytes()).unwrap();
/// file.write_all(data.as_slice()).unwrap();
/// ```
    pub struct FpImage(Object<libfprint_sys::FpImage, libfprint_sys::FpImageClass>);

    match fn {
        type_ => || libfprint_sys::fp_image_get_type() as usize,
    }
}

impl FpImage {
    pub fn new(width: u32, height: u32) -> Self {
        unsafe { FpImage::from_glib_full(libfprint_sys::fp_image_new(width as i32, height as i32)) }
    }

    /// Gets the pixel width of an image.
    pub fn width(&self) -> u32 {
        unsafe { libfprint_sys::fp_image_get_width(self.to_glib_none().0) as u32 }
    }

    /// Gets the pixel height of an image.
    pub fn height(&self) -> u32 {
        unsafe { libfprint_sys::fp_image_get_height(self.to_glib_none().0) as u32 }
    }

    /// Gets the resolution of the image. Note that this is assumed to be fixed to 500 points per inch (~19.685 p/mm) for most drivers.
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
    /// Gets the greyscale data for an image.
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
