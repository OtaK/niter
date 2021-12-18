use crate::platform::bluez;
use crate::error::*;

#[repr(u8)]
#[derive(
    Debug,
    Clone,
    Copy,
    strum::Display,
    strum::EnumString,
    serde_repr::Serialize_repr,
    serde_repr::Deserialize_repr,
    zvariant_derive::Type,
)]
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
    type Error = crate::NiterError;
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

#[derive(
    Debug,
    Clone,
    Copy,
    strum::Display,
    strum::EnumString,
    zvariant_derive::Type,
    serde::Serialize,
    serde::Deserialize,
)]
#[strum(serialize_all = "kebab-case")]
pub enum AdapterRole {
    Central,
    Peripheral,
    CentralPeripheral,
}

crate::impl_tryfrom_zvariant!(AdapterRole);

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, zvariant_derive::Type)]
pub struct Adapter {
    pub(crate) object_path: String,
}

impl<'a> From<std::borrow::Cow<'a, String>> for Adapter {
    fn from(s: std::borrow::Cow<'a, String>) -> Self {
        Self {
            object_path: s.to_string(),
        }
    }
}

impl std::str::FromStr for Adapter {
    type Err = crate::NiterError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            object_path: s.into(),
        })
    }
}

crate::impl_tryfrom_zvariant!(Adapter);
crate::to_proxy_impl!(Adapter, AdapterProxy, "org.bluez");

impl Adapter {
    pub fn advertising_manager<'a>(
        &'a self,
        connection: &'a zbus::Connection,
    ) -> NiterResult<bluez::advertising::AdvertisingManagerProxy<'a>> {
        Ok(bluez::advertising::AdvertisingManagerProxy::new_for(
            connection,
            "org.bluez",
            &self.object_path,
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
    fn remove_device(&self, device: bluez::device::Device) -> zbus::Result<()>;
    fn set_discovery_filter(&self, filter: zvariant::Value) -> zbus::Result<()>;
    fn get_discovery_filters(&self) -> zbus::Result<Vec<String>>;
    fn connect_device(&self, device: zvariant::Value) -> zbus::Result<()>;

    #[dbus_proxy(property)]
    fn address(&self) -> zbus::fdo::Result<String>;
    #[dbus_proxy(property)]
    fn address_type(&self) -> zbus::fdo::Result<bluez::AddressType>;
    #[dbus_proxy(property)]
    fn name(&self) -> zbus::fdo::Result<String>;
    #[dbus_proxy(property)]
    fn alias(&self) -> zbus::fdo::Result<String>;
    #[dbus_proxy(property)]
    fn set_alias(&self, alias: &str) -> zbus::fdo::Result<()>;
    #[dbus_proxy(property)]
    fn class(&self) -> zbus::fdo::Result<u32>;
    #[dbus_proxy(property)]
    fn powered(&self) -> zbus::fdo::Result<bool>;
    #[dbus_proxy(property)]
    fn set_powered(&self, powered: bool) -> zbus::fdo::Result<()>;
    #[dbus_proxy(property)]
    fn discoverable(&self) -> zbus::fdo::Result<bool>;
    #[dbus_proxy(property)]
    fn set_discoverable(&self, discoverable: bool) -> zbus::fdo::Result<()>;
    #[dbus_proxy(property)]
    fn pairable(&self) -> zbus::fdo::Result<bool>;
    #[dbus_proxy(property)]
    fn set_pairable(&self, pairable: bool) -> zbus::fdo::Result<()>;
    #[dbus_proxy(property)]
    fn pairable_timeout(&self) -> zbus::fdo::Result<u32>;
    #[dbus_proxy(property)]
    fn set_pairable_timeout(&self, timeout: u32) -> zbus::fdo::Result<()>;
    #[dbus_proxy(property)]
    fn discoverable_timeout(&self) -> zbus::fdo::Result<u32>;
    #[dbus_proxy(property)]
    fn set_discoverable_timeout(&self, timeout: u32) -> zbus::fdo::Result<()>;
    #[dbus_proxy(property)]
    fn discovering(&self) -> zbus::fdo::Result<bool>;
    #[dbus_proxy(property, name = "UUIDs")]
    fn uuids(&self) -> zbus::fdo::Result<crate::UuidArray>;
    #[dbus_proxy(property)]
    fn modalias(&self) -> zbus::fdo::Result<String>;
    #[dbus_proxy(property)]
    fn roles(&self) -> zbus::fdo::Result<bluez::ZvariantableArray<AdapterRole>>;
}

#[derive(Debug)]
pub struct AdapterEnumerator<'a> {
    paths: Vec<std::borrow::Cow<'a, String>>,
    connection: &'a zbus::Connection,
}

impl std::iter::Iterator for AdapterEnumerator<'_> {
    type Item = Adapter;
    fn next(&mut self) -> std::option::Option<Self::Item> {
        self.paths.pop().map(Into::into)
    }
}

impl<'a> AdapterProxy<'a> {
    pub fn enumerate_adapters(
        connection: &'a zbus::Connection,
    ) -> NiterResult<AdapterEnumerator<'a>> {
        let object_manager = zbus::fdo::ObjectManagerProxy::new_for(connection, "org.bluez", "/")?;
        let managed_objects = object_manager.get_managed_objects()?;
        let mut paths: Vec<std::borrow::Cow<String>> = managed_objects
            .into_iter()
            .filter(|(_, contents)| contents.contains_key("org.bluez.Adapter1"))
            .map(|(path, _)| std::borrow::Cow::Owned(path.to_string()))
            .collect();

        paths.reverse();

        Ok(AdapterEnumerator { connection, paths })
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn get_default_adapter() {
        let connection = zbus::Connection::new_system().unwrap();
        let adapter_proxy = niter::platform::adapter::AdapterProxy::new(&connection).unwrap();
        adapter_proxy.name().unwrap();
    }

    #[test]
    fn get_default_adapter_uuids() {
        let connection = zbus::Connection::new_system().unwrap();
        let adapter_proxy = niter::platform::adapter::AdapterProxy::new(&connection).unwrap();
        adapter_proxy.uuids().unwrap();
    }

    #[test]
    fn enumerate_adapters() {
        let connection = zbus::Connection::new_system().unwrap();
        let enumerator = niter::platform::adapter::AdapterProxy::enumerate_adapters(&connection).unwrap();
        println!("{:#?}", enumerator);
    }

    #[test]
    fn enumerate_devices() {
        let connection = zbus::Connection::new_system().unwrap();
        let enumerator = niter::platform::device::DeviceProxy::enumerate_devices(&connection).unwrap();
        println!("{:#?}", enumerator);
    }

    #[test]
    fn connect_to_device() {
        let connection = zbus::Connection::new_system().unwrap();
        let adapter = niter::platform::adapter::AdapterProxy::new(&connection).unwrap();
        adapter.start_discovery().unwrap();
        std::thread::sleep(std::time::Duration::from_secs(5));
        let mut devices = niter::platform::device::DeviceProxy::enumerate_devices(&connection).unwrap();
        if let Some(d) = devices.next() {
            println!("Device: {:#?}", d);
            let path: String = d.object_path;
            let device = niter::platform::device::DeviceProxy::new_for_owned(connection, "org.bluez".into(), path).unwrap();
            device.connect().unwrap();
        }
    }
}
