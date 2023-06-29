//! Rust bindings for [libfprint](https://gitlab.freedesktop.org/libfprint/libfprint)
//!
//! This crate provides a wrapper around the libfprint library, which allows you to use fingerprint scanners in your Rust applications.
//!
//! ## Enrolling a new fingerprint
//! ```rust
//! use libfprint_rs::{FpContext, FpPrint, GError};
//!
//! let context = FpContext::new();
//! let devices = context.get_devices();
//!
//! let dev = devices.iter().next().unwrap();
//! dev.open()?;
//!
//! let template = FpPrint::new(&dev);
//! template.set_username("Bruce Banner");
//!
//! dev.enroll(template, None, None::<()>)?;
//! ```
//! ## Verifying a fingerprint
//! ```rust
//! use libfprint_rs::{FpContext, FpPrint, GError};
//! let context = FpContext::new();
//! let devices = context.get_devices();
//!
//! let dev = devices.iter().next().unwrap();
//! dev.open()?;
//!
//! let enrolled_print = load_print_from_file();
//!
//! dev.verify(enrolled_print, None, None::<()>, None)?;
//! ```
mod context;
mod device;
mod error;
mod finger;
mod image;
mod print;
pub(crate) mod utils;

pub use crate::{
    context::FpContext,
    // import all from device mod
    device::*,
    error::{GError, GErrorSource},
    finger::{Finger, FpFingerStatusFlags},
    image::{FpImage, FpImageData},
    print::{FpPrint, SerializedPrint},
};

#[cfg(test)]
mod tests {

    use std::{cell::RefCell, rc::Rc};

    use crate::{context::FpContext, device::FpDevice, error::GError, print::FpPrint};
    struct UserData {
        _count: u32,
        _name: String,
    }
    fn generate_print<'a>(dev: &'a FpDevice) -> FpPrint<'a> {
        let user_data = UserData {
            _count: 304,
            _name: "Donda".into(),
        };

        let user_data = Rc::new(RefCell::new(user_data));

        let template = FpPrint::new(&dev);
        let print1 = dev
            .enroll(template, Some(callback_fn), Some(user_data.clone()))
            .unwrap();
        println!("{}", user_data.borrow()._count);

        return print1;
    }

    fn callback_fn(
        device: &FpDevice,
        completed_stages: i32,
        _print: FpPrint,
        _error: Option<GError>,
        _user_data: &Option<Rc<RefCell<UserData>>>,
    ) {
        if let Some(user_data) = _user_data {
            user_data.borrow_mut()._count += 1;
            // Mutate the user data
        }
        println!(
            "Enrolling: {} of {}",
            completed_stages,
            device.get_nr_enroll_stages()
        );
    }

    fn match_cb_function(
        _device: &FpDevice,                         // The fingerprint scanner device
        matched_print: Option<FpPrint>,             // The matched print, if any.
        _new_print: FpPrint,                        // New print scanned.
        _error: Option<GError>,                     // Optinal error in case of an error.
        match_data: &Option<Rc<RefCell<UserData>>>, // User data can be any data type.
    ) {
        if let Some(user_data) = match_data {
            user_data.borrow_mut()._count += 1;
            user_data.borrow_mut()._name = "Kanye".into();
        }
        if matched_print.is_some() {
            println!("Found matched print!");
        }
    }
    #[test]
    fn ident_test() {
        let ctx = FpContext::new();
        let dev = match ctx.get_devices().iter().next() {
            Some(dev) => dev,
            None => {
                return;
            }
        }; // Throws errors if no device is connected
        let f = dev.get_features();
        println!("{:?}", f);

        dev.open().unwrap(); // Open the device

        let prints = vec![generate_print(&dev), generate_print(&dev)];

        let mut matched_print = FpPrint::new(&dev);
        matched_print.set_username("Some username should be here");
        let mut new_print = FpPrint::new(&dev);

        dev.identify(
            prints,
            Some(match_cb_function),
            None,
            Some(&mut matched_print),
            Some(&mut new_print),
        )
        .unwrap();
    }
}
