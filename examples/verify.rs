use std::{fs::File, io::Read};

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
    let print_matched = verify(dev).unwrap();

    if print_matched {
        println!("Print matched");
    } else {
        println!("Prints don't match!!");
    }
}

fn verify<'a>(dev: FpDevice<'a>) -> Result<bool, GError<'a>> {
    if !dev.is_open() {
        dev.open()?;
    };

    let mut buf = Vec::new();
    let mut raw_print = File::open("examples/print").unwrap();
    raw_print.read_to_end(&mut buf).unwrap();

    let enrolled_print = FpPrint::deserialize(&buf).unwrap();
    let mut scanned_print = FpPrint::new(&dev);

    let matched = dev
        .verify(
            enrolled_print,
            Some(verify_callback),
            None,
            Some(&mut scanned_print),
        )
        .unwrap();
    Ok(matched)
}

fn verify_callback(
    _device: &FpDevice,             // The fingerprint scanner device
    matched_print: Option<FpPrint>, // The matched print, if any.
    _new_print: FpPrint,            // New print scanned.
    error: Option<GError>,          // Error, if any.
    _match_data: &Option<()>,
) {
    if let Some(_matched_print) = matched_print {
        println!("Found matched print!");
    }
    if let Some(err) = error {
        eprintln!("Error: {}", err);
    }
}
