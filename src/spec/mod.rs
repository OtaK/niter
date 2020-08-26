pub mod assigned_numbers;
pub mod ble_appearance;
pub mod gap;
pub mod unit;
pub mod format;

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
pub enum GattAttribute {
    CharacteristicDeclaration = 0x2803,
    IncludeDeclaration = 0x2802,
    PrimaryService = 0x2800,
    SecondaryService = 0x2801,
}
