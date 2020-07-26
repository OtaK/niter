// #[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize, zvariant_derive::Type)]
// pub struct Adapter {

// }
use crate::error::*;

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
#[repr(u8)]
pub enum TransportFilter {
    Auto,
    BrEdr,
    Le
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
            _ => Err(NiterError::Other(anyhow::anyhow!("Unrecognized Tranport Filter: {}", s)))
        }
    }
}

impl std::string::ToString for TransportFilter {
    fn to_string(&self) -> String {
        match *self {
            Self::Le => "le".into(),
            Self::BrEdr => "bredr".into(),
            Self::Auto => "auto".into()
        }
    }
}

#[derive(Debug, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
pub struct DiscoveryFilter {
    uuids: Vec<crate::uuid::Uuid>,
    rssi: Option<u16>,
    pathloss: Option<u16>,
    transport: TransportFilter,
    duplicate_data: bool,
    discoverable: bool,
    pattern: Option<String>
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
            pattern: None
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
        let mut dict = zvariant::Dict::new(zvariant::Signature::from_str_unchecked("s"), zvariant::Signature::from_str_unchecked("v"));
        if !self.uuids.is_empty() {
            // FIXME: meh
            //dict.add("UUIDs", zvariant::Value::Array(self.uuids.into()))?;
        }
        if let Some(rssi) = self.rssi.take() {
            dict.add("RSSI", rssi)?;
        }
        if let Some(pathloss) = self.pathloss.take() {
            dict.add("Pathloss", pathloss)?;
        }
        dict.add("Transport", zvariant::Value::Str(self.transport.to_string().into()))?;
        dict.add("DuplicateData", self.duplicate_data)?;
        dict.add("Discoverable", self.discoverable)?;
        if let Some(pat) = self.pattern.take() {
            dict.add("Pattern", zvariant::Value::Str(pat.into()))?;
        }

        Ok(zvariant::Value::Dict(dict))
    }
}


#[derive(Debug, Clone, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
pub struct Device {}

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
}
