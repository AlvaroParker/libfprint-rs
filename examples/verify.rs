use libfprint_rs::{FpContext, FpDevice, FpPrint};

fn main() {
    // Get devices
    let ctx = FpContext::new();
    let devices = ctx.devices();
    let dev = devices.get(0).unwrap();
    dev.open_sync(None).unwrap();

    // Create a template print
    let template = FpPrint::new(&dev);
    let enrolled_print = dev
        .enroll_sync(template, None, Some(progress_cb), None)
        .unwrap();

    // New print where we will store the next print
    let mut new_print = FpPrint::new(&dev);

    // Verify if the next print matches the previously enrolled print
    let matched = dev
        .verify_sync(
            &enrolled_print,
            None,
            Some(match_cb),
            None,
            Some(&mut new_print),
        )
        .unwrap();
    if matched {
        println!("Matched again");
    }
}

pub fn progress_cb(
    _device: &FpDevice,
    enroll_stage: i32,
    _print: Option<FpPrint>,
    _error: Option<glib::Error>,
    _: &Option<()>,
) -> () {
    println!("Enroll stage: {}", enroll_stage);
}

pub fn match_cb(
    _device: &FpDevice,
    matched_print: Option<FpPrint>,
    _print: FpPrint,
    _error: Option<glib::Error>,
    _data: &Option<()>,
) -> () {
    if matched_print.is_some() {
        println!("Matched");
    } else {
        println!("Not matched");
    }
}
