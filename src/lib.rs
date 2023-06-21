pub mod context;
pub mod device;
pub mod error;
pub mod finger;
pub mod image;
pub mod print;
pub(crate) mod utils;

#[cfg(test)]
mod tests {

    use crate::{context::FpContext, device::FpDevice, error::GError, print::FpPrint};
    struct UserData {
        _count: u32,
        _name: String,
    }
    fn generate_print<'a>(dev: &'a FpDevice) -> FpPrint<'a> {
        let mut user_data = UserData {
            _count: 304,
            _name: "Donda".into(),
        };

        let template = FpPrint::new(&dev);
        let print1 = dev
            .enroll(template, Some(callback_fn), Some(&mut user_data))
            .unwrap();
        let print2 = print1.clone();
        drop(print2);
        return print1;
    }

    fn callback_fn(
        device: &FpDevice,
        completed_stages: i32,
        _print: FpPrint,
        _error: Option<GError>,
        _user_data: &mut Option<&mut UserData>,
    ) {
        println!(
            "Enrolling: {} of {}",
            completed_stages,
            device.get_nr_enroll_stages()
        );
    }

    fn callback_function(
        _device: &FpDevice,             // The fingerprint scanner device
        matched_print: Option<FpPrint>, // The matched print, if any.
        _new_print: FpPrint,            // New print scanned.
        _error: Option<GError>,         // Optinal error in case of an error.
        _match_data: &Option<()>,       // User data can be any data type.
    ) {
        if matched_print.is_some() {
            println!("Found matched print!");
        }
    }
    #[test]
    fn ident_test() {
        let ctx = FpContext::new();
        let dev = ctx.get_devices().iter().next().unwrap(); // Throws errors if no device is connected

        dev.open().unwrap(); // Open the device

        let prints = vec![generate_print(&dev), generate_print(&dev)];

        let mut matched_print = FpPrint::new(&dev);
        let mut new_print = FpPrint::new(&dev);

        dev.identify(
            prints,
            Some(callback_function),
            None,
            Some(&mut matched_print),
            Some(&mut new_print),
        )
        .unwrap();
    }
}
