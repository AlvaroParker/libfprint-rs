use crate::print::FpPrint;

use super::{FpDevice, UserData};

impl<F, T> UserData<F, T>
where
    F: Fn(&FpDevice, i32, Option<FpPrint>, Option<glib::Error>, &Option<T>),
{
    pub(crate) fn callback_enroll(
        &self,
        device: &FpDevice,
        enroll_stage: i32,
        print: Option<FpPrint>,
        error: Option<glib::Error>,
    ) {
        (self.function)(device, enroll_stage, print, error, &self.data);
    }
}

impl<F, T> UserData<F, T>
where
    F: Fn(&FpDevice, Option<FpPrint>, FpPrint, Option<glib::Error>, &Option<T>),
{
    pub(crate) fn callback_match(
        &self,
        device: &FpDevice,
        match_print: Option<FpPrint>,
        print: FpPrint,
        error: Option<glib::Error>,
    ) {
        (self.function)(device, match_print, print, error, &self.data);
    }
}
