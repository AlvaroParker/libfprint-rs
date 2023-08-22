mod context;
mod device;
mod finger;
mod image;
mod print;

pub use gio::traits::CancellableExt;
/// Re-export `gio::Cancellable`, it provides a way to cancel sync operations, i.e
/// `FpDevice::enroll_sync`
pub use gio::Cancellable;
/// Re-export `glib::Error`, it provides a way to pass enroll dates to `FpPrint` metadata
pub use glib::Date as GDate;
/// Re-export `glib::Error`, it provides error handling for sync operations
pub use glib::Error as GError;

pub use context::FpContext;
pub use device::{FpDevice, FpEnrollProgress, FpMatchCb};
pub use finger::FpFinger;
pub use image::FpImage;
pub use print::FpPrint;

#[cfg(test)]
mod tests {

    use std::io::{Read, Write};

    use crate::{FpContext, FpDevice, FpPrint};

    #[test]
    fn get_names() {
        let ctx = FpContext::new();
        let devices = ctx.devices();
        let dev = devices.get(0).unwrap();

        dev.open_sync(None).unwrap();
        let mut prints = Vec::new();

        for i in 0..3 {
            save_prints(&dev, i);
        }

        for i in 0..3 {
            let print = read_prints(i);
            prints.push(print);
        }

        let mut new_print = FpPrint::new(&dev);
        let matched = dev
            .identify_sync(&prints, None, Some(match_cb), None, Some(&mut new_print))
            .unwrap();

        if matched.is_some() {
            println!("Matched");
        } else {
            println!("Not matched");
        }
    }
    pub fn _enroll_print(dev: &FpDevice) -> FpPrint {
        let template = FpPrint::new(&dev);
        let print = dev.enroll_sync(template, None, Some(enroll_cb), None);
        print.unwrap()
    }
    pub fn save_prints(dev: &FpDevice, id: u32) {
        let template = FpPrint::new(&dev);
        let print = dev
            .enroll_sync(template, None, Some(enroll_cb), None)
            .unwrap();
        let data = print.serialize().unwrap();
        let name = format!("prints/print{}", id);
        let mut file = std::fs::File::create(name).unwrap();
        file.write_all(&data).unwrap();
    }
    pub fn read_prints(id: u32) -> FpPrint {
        let name = format!("prints/print{}", id);
        let mut file = std::fs::File::open(name).unwrap();
        let mut buf = Vec::new();
        file.read_to_end(&mut buf).unwrap();

        FpPrint::deserialize(&buf).unwrap()
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
    pub fn match_cb(
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
