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

crate::impl_tryfrom_zvariant!(AddressType);

// pub type ServiceData = std::collections::HashMap<String, zvariant::OwnedValue>;
// pub type ManufacturerData = std::collections::HashMap<u16, zvariant::OwnedValue>;
// pub type AdvertisingData = std::collections::HashMap<u8, zvariant::OwnedValue>;

pub type ServiceData<'a, 'b> = zvariant::Dict<'a, 'b>;
pub type ManufacturerData<'a, 'b> = zvariant::Dict<'a, 'b>;
pub type AdvertisingData<'a, 'b> = zvariant::Dict<'a, 'b>;

#[derive(Debug, Clone, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
pub struct ZvariantableArray<T: zvariant::Type>(Vec<T>);

impl<T: zvariant::Type> std::ops::Deref for ZvariantableArray<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: zvariant::Type> From<Vec<T>> for ZvariantableArray<T> {
    fn from(value: Vec<T>) -> Self {
        Self(value)
    }
}

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

        impl<'a> std::convert::TryFrom<zvariant::Value<'a>> for $thing {
            type Error = crate::NiterError;
            fn try_from(v: zvariant::Value<'a>) -> crate::NiterResult<Self> {
                use std::str::FromStr as _;
                let s: String = v.downcast().ok_or_else(|| zvariant::Error::IncorrectType)?;
                Ok(Self::from_str(&s)?)
            }
        }
        impl std::convert::TryFrom<zvariant::OwnedValue> for $thing {
            type Error = crate::NiterError;
            fn try_from(v: zvariant::OwnedValue) -> crate::NiterResult<Self> {
                use std::str::FromStr as _;
                use std::convert::TryInto as _;
                let s: String = v.try_into()?;
                Ok(Self::from_str(&s)?)
            }
        }

        impl std::convert::TryFrom<zvariant::OwnedValue> for crate::ZvariantableArray<$thing> {
            type Error = crate::NiterError;
            fn try_from(v: zvariant::OwnedValue) -> crate::NiterResult<Self> {
                use std::convert::TryInto as _;
                let zva: zvariant::Array = v.try_into()?;
                let zva_len = zva.len();
                let inner: Vec<$thing> = zva.iter().cloned().try_fold(
                    Vec::with_capacity(zva_len),
                    |mut acc, item| -> crate::NiterResult<Vec<$thing>> {
                        acc.push(item.try_into()?);
                        Ok(acc)
                    })?;
                Ok(Self(inner))
            }
        }
    };
}


