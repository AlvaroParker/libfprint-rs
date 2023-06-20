#[derive(Debug)]
pub enum Finger {
    Unknown = libfprint_sys::FpFinger_FP_FINGER_UNKNOWN as isize,
    LeftThumb = libfprint_sys::FpFinger_FP_FINGER_LEFT_THUMB as isize,
    LeftIndex = libfprint_sys::FpFinger_FP_FINGER_LEFT_INDEX as isize,
    LeftMiddle = libfprint_sys::FpFinger_FP_FINGER_LEFT_MIDDLE as isize,
    LeftRing = libfprint_sys::FpFinger_FP_FINGER_LEFT_RING as isize,
    LeftLittle = libfprint_sys::FpFinger_FP_FINGER_LEFT_LITTLE as isize,
    RightThumb = libfprint_sys::FpFinger_FP_FINGER_RIGHT_THUMB as isize,
    RightIndex = libfprint_sys::FpFinger_FP_FINGER_RIGHT_INDEX as isize,
    RightMiddle = libfprint_sys::FpFinger_FP_FINGER_RIGHT_MIDDLE as isize,
    RightRing = libfprint_sys::FpFinger_FP_FINGER_RIGHT_RING as isize,
    RightLittle = libfprint_sys::FpFinger_FP_FINGER_RIGHT_LITTLE as isize,
}

impl From<u32> for Finger {
    fn from(value: u32) -> Self {
        match value {
            libfprint_sys::FpFinger_FP_FINGER_UNKNOWN => Finger::Unknown,
            libfprint_sys::FpFinger_FP_FINGER_LEFT_THUMB => Finger::LeftThumb,
            libfprint_sys::FpFinger_FP_FINGER_LEFT_INDEX => Finger::LeftIndex,
            libfprint_sys::FpFinger_FP_FINGER_LEFT_MIDDLE => Finger::LeftMiddle,
            libfprint_sys::FpFinger_FP_FINGER_LEFT_RING => Finger::LeftRing,
            libfprint_sys::FpFinger_FP_FINGER_LEFT_LITTLE => Finger::LeftLittle,
            libfprint_sys::FpFinger_FP_FINGER_RIGHT_THUMB => Finger::RightThumb,
            libfprint_sys::FpFinger_FP_FINGER_RIGHT_INDEX => Finger::RightIndex,
            libfprint_sys::FpFinger_FP_FINGER_RIGHT_MIDDLE => Finger::RightMiddle,
            libfprint_sys::FpFinger_FP_FINGER_RIGHT_RING => Finger::RightRing,
            libfprint_sys::FpFinger_FP_FINGER_RIGHT_LITTLE => Finger::RightLittle,
            _ => panic!("Invalid finger value"),
        }
    }
}

pub enum FpFingerStatusFlags {
    Non = libfprint_sys::FpFingerStatusFlags_FP_FINGER_STATUS_NONE as isize,
    Needed = libfprint_sys::FpFingerStatusFlags_FP_FINGER_STATUS_NEEDED as isize,
    Present = libfprint_sys::FpFingerStatusFlags_FP_FINGER_STATUS_PRESENT as isize,
}

impl From<u32> for FpFingerStatusFlags {
    fn from(value: u32) -> Self {
        match value {
            libfprint_sys::FpFingerStatusFlags_FP_FINGER_STATUS_NONE => FpFingerStatusFlags::Non,
            libfprint_sys::FpFingerStatusFlags_FP_FINGER_STATUS_NEEDED => {
                FpFingerStatusFlags::Needed
            }
            libfprint_sys::FpFingerStatusFlags_FP_FINGER_STATUS_PRESENT => {
                FpFingerStatusFlags::Present
            }
            _ => panic!("Invalid FpFingerStatusFlags"),
        }
    }
}
