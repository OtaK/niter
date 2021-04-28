mod utils;
pub use self::utils::*;

mod misc;
pub use self::misc::*;

pub mod adapter;
#[cfg(feature = "bluez-next")]
pub mod advertisement_monitor;
pub mod advertising;
pub mod agent;
pub mod device;
pub mod gatt;
pub mod media;
pub mod mesh;
// WIP
// pub mod obex;
pub mod profile;

#[macro_use]
mod macros;

#[derive(
    Debug,
    Clone,
    Copy,
    zvariant_derive::Type,
    strum::Display,
    strum::EnumString,
    serde::Serialize,
    serde::Deserialize,
)]
#[strum(serialize_all = "kebab-case")]
pub enum AddressType {
    Public,
    Random,
}

crate::impl_tryfrom_zvariant!(AddressType);

#[derive(Debug, Clone, Copy, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
/// Forward Compat BlueZ Dummy struct
pub struct BlueZDummy;

pub type ServiceData = ZvariantableMap<String, Vec<u8>>;
pub type ManufacturerData = ZvariantableMap<u16, Vec<u8>>;
pub type AdvertisingData = ZvariantableMap<u8, Vec<u8>>;
