mod error;
pub use self::error::*;

mod uuid;
pub use self::uuid::*;

pub mod adapter;
pub mod advertisement_monitor;
pub mod advertising;
pub mod agent;
pub mod assigned_numbers;
pub mod device;
pub mod gatt;
pub mod profile;

// TODO: Fix all proxies

#[derive(Debug, Clone, Copy, zvariant_derive::Type, strum::Display, strum::EnumString, serde::Serialize, serde::Deserialize)]
#[strum(serialize_all = "kebab-case")]
pub enum AddressType {
    Public,
    Random,
}

impl_tryfrom_zvariant!(AddressType);

// pub type ServiceData = std::collections::HashMap<String, zvariant::OwnedValue>;
// pub type ManufacturerData = std::collections::HashMap<u16, zvariant::OwnedValue>;
// pub type AdvertisingData = std::collections::HashMap<u8, zvariant::OwnedValue>;

pub type ServiceData<'a, 'b> = zvariant::Dict<'a, 'b>;
pub type ManufacturerData<'a, 'b> = zvariant::Dict<'a, 'b>;
pub type AdvertisingData<'a, 'b> = zvariant::Dict<'a, 'b>;

#[macro_export(local_inner_macros)]
#[doc(hidden)]
macro_rules! to_proxy_impl {
    ($struct: ident, $proxy: ident, $service: expr) => {
        impl $struct {
            pub fn into_proxy<'a>(&'a self, connection: &'a zbus::Connection) -> NiterResult<$proxy<'a>> {
                Ok($proxy::new_for(connection, "org.bluez", &self.object_path)?)
            }
        }
    };
}

#[macro_export(local_inner_macros)]
#[doc(hidden)]
macro_rules! impl_tryfrom_zvariant {
    ($thing:ident) => {
        impl std::convert::TryFrom<zvariant::OwnedValue> for $thing {
            type Error = NiterError;
            fn try_from(v: zvariant::OwnedValue) -> NiterResult<Self> {
                use std::str::FromStr as _;
                use std::convert::TryInto as _;
                let s: String = v.try_into()?;
                Ok(Self::from_str(&s)?)
            }
        }
    };
}


