pub enum FpFinger {
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

impl From<FpFinger> for i32 {
    fn from(finger: FpFinger) -> Self {
        finger as i32
    }
}

impl From<u32> for FpFinger {
    fn from(value: u32) -> Self {
        match value {
            libfprint_sys::FpFinger_FP_FINGER_UNKNOWN => FpFinger::Unknown,
            libfprint_sys::FpFinger_FP_FINGER_LEFT_THUMB => FpFinger::LeftThumb,
            libfprint_sys::FpFinger_FP_FINGER_LEFT_INDEX => FpFinger::LeftIndex,
            libfprint_sys::FpFinger_FP_FINGER_LEFT_MIDDLE => FpFinger::LeftMiddle,
            libfprint_sys::FpFinger_FP_FINGER_LEFT_RING => FpFinger::LeftRing,
            libfprint_sys::FpFinger_FP_FINGER_LEFT_LITTLE => FpFinger::LeftLittle,
            libfprint_sys::FpFinger_FP_FINGER_RIGHT_THUMB => FpFinger::RightThumb,
            libfprint_sys::FpFinger_FP_FINGER_RIGHT_INDEX => FpFinger::RightIndex,
            libfprint_sys::FpFinger_FP_FINGER_RIGHT_MIDDLE => FpFinger::RightMiddle,
            libfprint_sys::FpFinger_FP_FINGER_RIGHT_RING => FpFinger::RightRing,
            libfprint_sys::FpFinger_FP_FINGER_RIGHT_LITTLE => FpFinger::RightLittle,
            _ => panic!("Invalid finger value"),
        }
    }
}

impl TryFrom<i32> for FpFinger {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value < 0 {
            return Err(());
        }
        let value = value as u32;
        match value {
            libfprint_sys::FpFinger_FP_FINGER_UNKNOWN => Ok(FpFinger::Unknown),
            libfprint_sys::FpFinger_FP_FINGER_LEFT_THUMB => Ok(FpFinger::LeftThumb),
            libfprint_sys::FpFinger_FP_FINGER_LEFT_INDEX => Ok(FpFinger::LeftIndex),
            libfprint_sys::FpFinger_FP_FINGER_LEFT_MIDDLE => Ok(FpFinger::LeftMiddle),
            libfprint_sys::FpFinger_FP_FINGER_LEFT_RING => Ok(FpFinger::LeftRing),
            libfprint_sys::FpFinger_FP_FINGER_LEFT_LITTLE => Ok(FpFinger::LeftLittle),
            libfprint_sys::FpFinger_FP_FINGER_RIGHT_THUMB => Ok(FpFinger::RightThumb),
            libfprint_sys::FpFinger_FP_FINGER_RIGHT_INDEX => Ok(FpFinger::RightIndex),
            libfprint_sys::FpFinger_FP_FINGER_RIGHT_MIDDLE => Ok(FpFinger::RightMiddle),
            libfprint_sys::FpFinger_FP_FINGER_RIGHT_RING => Ok(FpFinger::RightRing),
            libfprint_sys::FpFinger_FP_FINGER_RIGHT_LITTLE => Ok(FpFinger::RightLittle),
            _ => Err(()),
        }
    }
}
