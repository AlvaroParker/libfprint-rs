use libfprint_rs::{FpContext, FpDevice, FpPrint, GError};

fn main() {
    let context = FpContext::new();
    let devices = context.get_devices();

    let dev = match devices.iter().next() {
        Some(dev) => dev,
        None => {
            eprintln!("No devices detected.");
            std::process::exit(1);
        }
    };

    let print = enroll(dev);
    if let Ok(print) = print {
        let username = print.get_username().unwrap();
        println!("User provided username: \"{}\"", username);
    }
}

fn enroll<'a>(dev: FpDevice<'a>) -> Result<FpPrint<'a>, GError<'static>> {
    if !dev.is_open() {
        dev.open()?;
    };

    let template_print = FpPrint::new(&dev);
    template_print.set_username("MyUsername");

    dev.enroll(template_print, Some(enroll_callback), None)
}

fn enroll_callback(
    device: &FpDevice,
    completed_stages: i32,
    _print: FpPrint,
    error: Option<GError>,
    _user_data: &Option<()>,
) {
    println!(
        "Enrolling ... {} of {}",
        completed_stages,
        device.get_nr_enroll_stages()
    );
    if let Some(err) = error {
        eprintln!("Error: {}", err);
    }
}
