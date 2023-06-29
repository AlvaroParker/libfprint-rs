/// Struct representing an image of a fingerprint. Not all devices support this feature.
/// # Examples:
/// ```rust
/// use libfprint_rs::FpContext;
/// use std::fs::File;
/// use std::io::Write;
///
/// let context = FpContext::new();
/// let devices = context.get_devices();
/// let device = devices.iter().next().unwrap();
///
/// device.open().unwrap();
/// let image = device.capture_image().unwrap();
/// let data = image.get_data();
///
/// let mut file = File::create("image.pgm").unwrap();
/// let header = format!("P5\n{} {}\n255\n", image.get_width(), image.get_height());
/// file.write_all(header.as_bytes()).unwrap();
/// file.write_all(data.as_slice()).unwrap();
/// ```
pub struct FpImage {
    pub(crate) image: *mut libfprint_sys::FpImage,
}
#[derive(Debug)]
pub struct FpImageData<'a> {
    data: &'a [u8],
}
/// Struct representing an the binary data of an image of a fingerprint. Not all devices support this feature.
impl FpImageData<'_> {
    pub fn as_slice(&self) -> &[u8] {
        self.data
    }
}

impl Drop for FpImage {
    fn drop(&mut self) {
        if !self.image.is_null() {
            unsafe {
                libfprint_sys::g_object_unref(self.image.cast());
            }
        }
    }
}

impl FpImage {
    pub fn get_data(&self) -> FpImageData {
        let mut len = 0;
        let data = unsafe {
            let raw_data =
                libfprint_sys::fp_image_get_data(self.image, std::ptr::addr_of_mut!(len));
            std::slice::from_raw_parts(raw_data, len as usize)
        };
        // This shouln't be freed | See libfprint docs about fp_image_get_data
        FpImageData { data }
    }
    pub fn get_width(&self) -> u32 {
        unsafe { libfprint_sys::fp_image_get_width(self.image) }
    }
    pub fn get_height(&self) -> u32 {
        unsafe { libfprint_sys::fp_image_get_height(self.image) }
    }
}
