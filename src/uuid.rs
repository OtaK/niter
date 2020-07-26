const BASE_UUID: (u64, u64) = (0x000000000000_1000, 0x8000_00805F9B34FB);

#[repr(align(64))]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
pub struct Uuid {
    u1: u64,
    u2: u64,
}

impl From<(u64, u64)> for Uuid {
    #[inline(always)]
    fn from((u1, u2): (u64, u64)) -> Self {
        Self { u1, u2 }
    }
}

impl std::fmt::Display for Uuid {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:016X}_{:016X}", self.u1, self.u2)
    }
}

pub trait SdpShortUUID<T: Into<u32>> {
    #[inline(always)]
    fn from_sdp_short_uuid(uuid: T) -> Uuid {
        let uuid: u32 = uuid.into();
        ((((uuid as u64) << 32) ^ BASE_UUID.0), BASE_UUID.1).into()
    }
}

impl SdpShortUUID<u16> for Uuid {}
impl SdpShortUUID<u32> for Uuid {}

