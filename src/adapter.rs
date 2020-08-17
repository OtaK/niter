use crate::error::*;
use crate::impl_tryfrom_zvariant;

#[repr(u8)]
#[derive(Debug, Clone, Copy, strum::Display, strum::EnumString, serde_repr::Serialize_repr, serde_repr::Deserialize_repr, zvariant_derive::Type)]
#[strum(serialize_all = "lowercase")]
pub enum TransportFilter {
    Auto,
    BrEdr,
    Le,
}

impl Default for TransportFilter {
    fn default() -> Self {
        Self::Auto
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct DiscoveryFilter {
    uuids: Vec<crate::Uuid>,
    rssi: Option<u16>,
    pathloss: Option<u16>,
    transport: TransportFilter,
    duplicate_data: bool,
    discoverable: bool,
    pattern: Option<String>,
}

impl Default for DiscoveryFilter {
    fn default() -> Self {
        DiscoveryFilter {
            uuids: vec![],
            rssi: None,
            pathloss: None,
            transport: TransportFilter::Auto,
            duplicate_data: true,
            discoverable: false,
            pattern: None,
        }
    }
}

impl DiscoveryFilter {
    pub fn get_filter_names() -> Vec<String> {
        vec![
            "uuids".into(),
            "rssi".into(),
            "pathloss".into(),
            "transport".into(),
            "duplicate_data".into(),
            "discoverable".into(),
            "pattern".into(),
        ]
    }
}

impl std::convert::TryInto<zvariant::Value<'_>> for DiscoveryFilter {
    type Error = NiterError;
    fn try_into(mut self) -> NiterResult<zvariant::Value<'static>> {
        use zvariant::Type as _;

        let mut dict: std::collections::HashMap<&str, zvariant::Value<'_>> =
            std::collections::HashMap::new();

        if !self.uuids.is_empty() {
            let tmp = self
                .uuids
                .into_iter()
                .map(|uuid| {
                    uuid.to_hyphenated()
                        .encode_lower(&mut uuid::Uuid::encode_buffer())
                        .to_string()
                        .into()
                })
                .try_fold(
                    zvariant::Array::new(String::signature()),
                    |mut acc, s: zvariant::Str| -> NiterResult<zvariant::Array> {
                        acc.append(zvariant::Value::Str(s))?;
                        Ok(acc)
                    },
                )?;

            dict.insert("UUIDs", tmp.into());
        }
        if let Some(rssi) = self.rssi.take() {
            dict.insert("RSSI", rssi.into());
        }
        if let Some(pathloss) = self.pathloss.take() {
            dict.insert("Pathloss", pathloss.into());
        }
        dict.insert(
            "Transport",
            zvariant::Value::Str(self.transport.to_string().into()),
        );
        dict.insert("DuplicateData", self.duplicate_data.into());
        dict.insert("Discoverable", self.discoverable.into());
        if let Some(pat) = self.pattern.take() {
            dict.insert("Pattern", zvariant::Value::Str(pat.into()));
        }

        Ok(zvariant::Value::Dict(dict.into()))
    }
}

#[derive(Debug, Clone, Copy, strum::Display, strum::EnumString, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
#[strum(serialize_all = "kebab-case")]
pub enum AdapterRole {
    Central,
    Peripheral,
    CentralPeripheral,
}

impl_tryfrom_zvariant!(AdapterRole);

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, zvariant_derive::Type)]
pub struct Adapter {
    object_path: String
}

crate::to_proxy_impl!(Adapter, AdapterProxy, "org.bluez");

impl Adapter {
    pub fn advertising_manager<'a>(&'a self, connection: &'a zbus::Connection) -> NiterResult<crate::advertising::AdvertisingManagerProxy<'a>> {
        Ok(crate::advertising::AdvertisingManagerProxy::new_for(
            connection,
            "org.bluez",
            &self.object_path
        )?)
    }
}

#[zbus::dbus_proxy(
    interface = "org.bluez.Adapter1",
    default_service = "org.bluez",
    default_path = "/org/bluez/hci0"
)]
#[derive(Debug, Clone, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
pub trait Adapter {
    fn start_discovery(&self) -> zbus::Result<()>;
    fn stop_discovery(&self) -> zbus::Result<()>;
    fn remove_device(&self, device: crate::device::Device) -> zbus::Result<()>;
    fn set_discovery_filter(&self, filter: zvariant::Value) -> zbus::Result<()>;
    fn get_discovery_filters(&self) -> zbus::Result<Vec<String>>;
    fn connect_device(&self, device: zvariant::Value) -> zbus::Result<()>;

    #[dbus_proxy(property)]
    fn address(&self) -> zbus::fdo::Result<String>;
    #[dbus_proxy(property)]
    fn address_type(&self) -> zbus::fdo::Result<crate::AddressType>;
    #[dbus_proxy(property)]
    fn name(&self) -> zbus::fdo::Result<String>;
    #[dbus_proxy(property)]
    fn alias(&self) -> zbus::fdo::Result<String>;
    #[dbus_proxy(property)]
    fn class(&self) -> zbus::fdo::Result<u32>;
    #[dbus_proxy(property)]
    fn powered(&self) -> zbus::fdo::Result<bool>;
    #[dbus_proxy(property)]
    fn discoverable(&self) -> zbus::fdo::Result<bool>;
    #[dbus_proxy(property)]
    fn pairable(&self) -> zbus::fdo::Result<bool>;
    #[dbus_proxy(property)]
    fn pairable_timeout(&self) -> zbus::fdo::Result<u32>;
    #[dbus_proxy(property)]
    fn discoverable_timeout(&self) -> zbus::fdo::Result<u32>;
    #[dbus_proxy(property)]
    fn discovering(&self) -> zbus::fdo::Result<bool>;
    #[dbus_proxy(property, name = "UUIDs")]
    fn uuids(&self) -> zbus::fdo::Result<crate::UuidArray>;
    #[dbus_proxy(property)]
    fn modalias(&self) -> zbus::fdo::Result<String>;
    #[dbus_proxy(property)]
    // fn roles(&self) -> zbus::fdo::Result<Vec<AdapterRole>>;
    fn roles(&self) -> zbus::fdo::Result<Vec<String>>;
}

impl<'a> std::ops::Deref for AdapterProxy<'a> {
    type Target = zbus::Proxy<'a>;
    fn deref(&self) -> &Self::Target { &self.0 }
}

// type ObjectManagerItemList = std::collections::HashMap<
//     String,
//     std::collections::HashMap<
//         String,
//         std::collections::HashMap<
//             String,
//             zvariant::OwnedValue
//         >
//     >
// >;

// impl<'a> AdapterProxy<'a> {
    // FIXME: This doesn't work because lifetimes are fundamentally broken on zbus's side
    // pub fn enumerate_adapters(connection: &'a zbus::Connection) -> NiterResult<Vec<Self>> {
    //     let object_manager = zbus::fdo::ObjectManagerProxy::new_for(connection, "org.bluez", "/")?;
    //     let managed_objects = object_manager.get_managed_objects()?;
    //     let iter = managed_objects
    //         .iter()
    //         .filter(|(_, contents)| contents.contains_key("org.bluez.Adapter1"))
    //         .map(move |(path, _)| zbus::Proxy::new(connection, "org.bluez.Adapter1", "org.bluez", path))
    //         .map(|proxy| Self(proxy.unwrap()))
    //         .collect();

    //     Ok(iter)

    // }
// }

