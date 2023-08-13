mod context;
mod device;
mod finger;
mod image;
mod print;

#[cfg(test)]
mod tests {

    use crate::{context::FpContext, device::FpDevice, print::FpPrint};

    #[test]
    fn get_names() {
        let ctx = FpContext::new();
        let devs = ctx.devices();

        let dev = devs[0].clone();

        dev.open_sync(None).unwrap();
        if let Err(e) = dev.open_sync(None) {
            println!("Error while trying to open fingerprint device: {}", e);
        }
        println!("Enrolling fingerprint devices");

        let p = FpPrint::new(&dev);
        let print1 = dev
            .enroll_sync(p.clone(), None, Some(enroll_cb), Some(1))
            .unwrap();

        let print2 = dev
            .enroll_sync(p.clone(), None, Some(enroll_cb), Some(1))
            .unwrap();

        let prints = vec![print1, print2];

        let mut enrolled = FpPrint::new(&dev);
        dev.identify_sync(&prints, None, Some(match_cb), Some(1), Some(&mut enrolled))
            .unwrap();

        let print = &prints[0];
        let s = print.serialize().unwrap();
        println!("{:?}", s);
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
