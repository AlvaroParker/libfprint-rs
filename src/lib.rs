mod context;
mod device;
mod finger;
mod image;
mod print;

pub use gio::traits::CancellableExt;
pub use gio::Cancellable;
pub use glib::Date as GDate;
pub use glib::Error as GError;

pub use context::FpContext;
pub use device::{FpDevice, FpEnrollProgress, FpMatchCb};
pub use finger::FpFinger;
pub use image::FpImage;
pub use print::FpPrint;

#[cfg(test)]
mod tests {

    use crate::{FpContext, FpDevice, FpFinger, FpPrint};

    #[test]
    fn get_names() {
        let ctx = FpContext::new();
        let devices = ctx.devices();
        let dev = devices.get(0).unwrap();

        dev.open_sync(None).unwrap();
        let print = FpPrint::new(&dev);
        print.set_finger(FpFinger::RightRing);
        print.set_username("testing_username");

        let print = dev.enroll_sync(print, None, Some(enroll_cb), Some(10));
        if let Ok(print) = print {
            if print.image().is_none() {
                println!("Failed to get image from the scanned print");
            }
        }
    }
    pub fn enroll_cb(
        _device: &FpDevice,
        enroll_stage: i32,
        _print: Option<FpPrint>,
        _error: Option<glib::Error>,
        _data: &Option<i32>,
    ) -> () {
        println!("Enroll stage: {}", enroll_stage);
    }
    pub fn _match_cb(
        _device: &FpDevice,
        matched_print: Option<FpPrint>,
        _print: FpPrint,
        _error: Option<glib::Error>,
        _data: &Option<i32>,
    ) -> () {
        if matched_print.is_some() {
            println!("Matched");
        } else {
            println!("Not matched");
        }
    }
}
