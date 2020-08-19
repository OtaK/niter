mod error;
pub use self::error::*;

mod uuid;
pub use self::uuid::*;

pub mod adapter;
pub mod advertisement_monitor;
pub mod advertising;
pub mod agent;
pub mod device;
pub mod gatt;
pub mod profile;
pub mod spec;

#[macro_use]
mod macros;

mod utils;
pub use self::utils::*;

mod misc;
pub use self::misc::*;

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

// pub type ServiceData = std::collections::HashMap<String, Vec<u8>>;
pub type ManufacturerData = std::collections::HashMap<u16, Vec<u8>>;
pub type AdvertisingData = std::collections::HashMap<u8, Vec<u8>>;

pub type ServiceData = ZvariantableMap<String, Vec<u8>>;

// pub type ServiceData = zvariant::Dict<'static, 'static>;
// pub type ManufacturerData = zvariant::Dict<'static, 'static>;
// pub type AdvertisingData = zvariant::Dict<'static, 'static>;
