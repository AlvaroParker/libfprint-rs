use super::{FpDevice, UserData};
use crate::{error::GError, print::FpPrint};

impl<F, T> UserData<F, T>
where
    F: Fn(&FpDevice, i32, FpPrint, Option<GError>, &Option<T>) -> (),
{
    pub(crate) fn callback_enroll(
        &self,
        device: &FpDevice,
        enroll_stage: i32,
        print: FpPrint,
        error: Option<GError>,
    ) -> () {
        (self.function)(device, enroll_stage, print, error, &self.data);
    }
}

impl<F, T> UserData<F, T>
where
    F: Fn(&FpDevice, Option<FpPrint>, FpPrint, Option<GError>, &Option<T>),
{
    pub(crate) fn callback_match(
        &self,
        device: &FpDevice,
        matched_print: Option<FpPrint>,
        print: FpPrint,
        error: Option<GError>,
    ) -> () {
        (self.function)(device, matched_print, print, error, &self.data);
    }
}
