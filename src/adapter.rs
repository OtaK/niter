use crate::error::*;

#[repr(u8)]
#[derive(Debug, Clone, Copy, serde_repr::Serialize_repr, serde_repr::Deserialize_repr, zvariant_derive::Type)]
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

impl std::str::FromStr for TransportFilter {
    type Err = NiterError;
    fn from_str(s: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> {
        match &*s.to_lowercase() {
            "auto" => Ok(Self::Auto),
            "bredr" => Ok(Self::BrEdr),
            "le" => Ok(Self::Le),
            _ => Err(NiterError::Other(anyhow::anyhow!(
                "Unrecognized Tranport Filter: {}",
                s
            ))),
        }
    }
}

impl std::string::ToString for TransportFilter {
    fn to_string(&self) -> String {
        match *self {
            Self::Le => "le".into(),
            Self::BrEdr => "bredr".into(),
            Self::Auto => "auto".into(),
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct DiscoveryFilter {
    uuids: Vec<uuid::Uuid>,
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
                    zvariant::Array::new(zvariant::Signature::from_str_unchecked("s")),
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

#[derive(Debug, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
pub enum AddressType {
    Public,
    Random,
}

#[derive(Debug, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
pub enum AdapterRole {
    Central,
    Peripheral,
    CentralPeripheral,
}

#[derive(Debug, Clone, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
pub struct Device {}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, zvariant_derive::Type)]
pub struct Adapter<'a, 'c> {
    #[serde(skip)]
    connection: &'c zbus::Connection,
    object_path: &'a zvariant::ObjectPath<'a>,
};

impl std::str::FromStr for Adapter {
    type Err = NiterError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use std::convert::TryFrom as _;
        let object_path = zvariant::ObjectPath::try_from(s)?;
        Ok(Self {
            connection,
            object_path
        })
    }
}

impl<'a> std::convert::TryInto<AdapterProxy<'a>> for Adapter {
    type Error = NiterError;
    fn try_into(self) -> NiterResult<AdapterProxy<'a>> {
        Ok(AdapterProxy::new_for(self.0.into())?)
    }

}

#[zbus::dbus_proxy(
    interface = "org.bluez.Adapter1",
    default_service = "org.bluez",
    default_path = "/org/bluez/Adapter1"
)]
#[derive(Debug, Clone, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
pub trait Adapter {
    fn start_discovery(&self) -> zbus::Result<()>;
    fn stop_discovery(&self) -> zbus::Result<()>;
    fn remove_device(&self, device: Device) -> zbus::Result<()>;
    fn set_discovery_filter(&self, filter: zvariant::Value) -> zbus::Result<()>;
    fn get_discovery_filters(&self) -> zbus::Result<Vec<String>>;
    fn connect_device(&self, device: zvariant::Value) -> zbus::Result<()>;

    #[zbus::dbus_proxy(property)]
    fn address(&self) -> zbus::fdo::Result<String>;
    #[zbus::dbus_proxy(property)]
    fn address_type(&self) -> zbus::fdo::Result<AddressType>;
    #[zbus::dbus_proxy(property)]
    fn name(&self) -> zbus::fdo::Result<String>;
    #[zbus::dbus_proxy(property)]
    fn alias(&self) -> zbus::fdo::Result<String>;
    #[zbus::dbus_proxy(property)]
    fn class(&self) -> zbus::fdo::Result<u32>;
    #[zbus::dbus_proxy(property)]
    fn powered(&self) -> zbus::fdo::Result<bool>;
    #[zbus::dbus_proxy(property)]
    fn discoverable(&self) -> zbus::fdo::Result<bool>;
    #[zbus::dbus_proxy(property)]
    fn pairable(&self) -> zbus::fdo::Result<bool>;
    #[zbus::dbus_proxy(property)]
    fn pairable_timeout(&self) -> zbus::fdo::Result<u32>;
    #[zbus::dbus_proxy(property)]
    fn discoverable_timeout(&self) -> zbus::fdo::Result<u32>;
    #[zbus::dbus_proxy(property)]
    fn discovering(&self) -> zbus::fdo::Result<bool>;
    #[zbus::dbus_proxy(property)]
    fn uuids(&self) -> zbus::fdo::Result<Vec<String>>;
    #[zbus::dbus_proxy(property)]
    fn modalias(&self) -> zbus::fdo::Result<String>;
    #[zbus::dbus_proxy(property)]
    fn roles(&self) -> zbus::fdo::Result<Vec<AdapterRole>>;
}
