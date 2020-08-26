#[derive(
    Debug,
    Clone,
    Copy,
    Eq,
    PartialEq,
    num_enum::TryFromPrimitive,
    num_enum::IntoPrimitive,
    serde_repr::Serialize_repr,
    serde_repr::Deserialize_repr,
)]
#[cfg_attr(target_os = "linux", derive(zvariant_derive::Type))]
#[repr(u16)]
pub enum CharacteristicNumber {
    AerobicHeartRateLowerLimit = 0x2A7E,
    AerobicHeartRateHigherLimit = 0x2A84,
    AerobicThreshold = 0x2A7F,
    Age = 0x2A80,
    Aggregate = 0x2A5A,
    AlertCategoryID = 0x2A43,
    AlertCategoryIDBitMask = 0x2A42,
    AlertLevel = 0x2A06,
    AlertNotificationControlPoint = 0x2A44,
    AlertStatus = 0x2A3F,
    Altitude = 0x2AB3,
    // TODO:
}
