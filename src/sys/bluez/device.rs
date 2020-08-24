use crate::sys::bluez;

#[derive(Debug, Clone, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
pub struct Device {
    object_path: String,
}

impl std::str::FromStr for Device {
    type Err = crate::NiterError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            object_path: s.into(),
        })
    }
}

crate::impl_tryfrom_zvariant!(Device);

#[zbus::dbus_proxy(
    interface = "org.bluez.Device1",
    default_service = "org.bluez",
    default_path = "/org/bluez/Device1"
)]
#[derive(Debug, Clone, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
pub trait Device {
    fn connect(&self) -> zbus::Result<()>;
    fn disconnect(&self) -> zbus::Result<()>;
    fn connect_profile(&self, uuid: crate::Uuid) -> zbus::Result<()>;
    fn disconnect_profile(&self, uuid: crate::Uuid) -> zbus::Result<()>;
    fn pair(&self) -> zbus::Result<()>;
    fn cancel_pairing(&self) -> zbus::Result<()>;

    #[dbus_proxy(property)]
    fn address(&self) -> zbus::fdo::Result<String>;
    #[dbus_proxy(property)]
    fn address_type(&self) -> zbus::fdo::Result<bluez::AddressType>;
    #[dbus_proxy(property)]
    fn name(&self) -> zbus::fdo::Result<String>;
    #[dbus_proxy(property)]
    fn icon(&self) -> zbus::fdo::Result<String>;
    #[dbus_proxy(property)]
    fn class(&self) -> zbus::fdo::Result<u32>;
    #[dbus_proxy(property)]
    fn appearance(&self) -> zbus::fdo::Result<bluez::spec::ble_appearance::BLEAppearance>;
    #[dbus_proxy(property)]
    fn uuids(&self) -> zbus::fdo::Result<crate::UuidArray>;
    #[dbus_proxy(property)]
    fn paired(&self) -> zbus::fdo::Result<bool>;
    #[dbus_proxy(property)]
    fn connected(&self) -> zbus::fdo::Result<bool>;
    #[dbus_proxy(property)]
    fn trusted(&self) -> zbus::fdo::Result<bool>;
    #[dbus_proxy(property)]
    fn blocked(&self) -> zbus::fdo::Result<bool>;
    #[dbus_proxy(property)]
    fn wake_allowed(&self) -> zbus::fdo::Result<bool>;
    #[dbus_proxy(property)]
    fn alias(&self) -> zbus::fdo::Result<String>;
    #[dbus_proxy(property)]
    fn adapter(&self) -> zbus::fdo::Result<bluez::adapter::Adapter>;
    #[dbus_proxy(property)]
    fn legacy_pairing(&self) -> zbus::fdo::Result<bool>;
    #[dbus_proxy(property)]
    fn modalias(&self) -> zbus::fdo::Result<String>;
    #[dbus_proxy(property, name = "RSSI")]
    fn rssi(&self) -> zbus::fdo::Result<u16>;
    #[dbus_proxy(property)]
    fn tx_power(&self) -> zbus::fdo::Result<u16>;
    #[dbus_proxy(property)]
    fn manufacturer_data(&self) -> zbus::fdo::Result<bluez::ManufacturerData>;
    #[dbus_proxy(property)]
    fn service_data(&self) -> zbus::fdo::Result<bluez::ServiceData>;
    #[dbus_proxy(property)]
    fn services_resolved(&self) -> zbus::fdo::Result<bool>;
    #[dbus_proxy(property)]
    fn advertising_flags(&self) -> zbus::fdo::Result<Vec<u8>>;
    #[dbus_proxy(property)]
    fn advertising_data(&self) -> zbus::fdo::Result<bluez::AdvertisingData>;
}