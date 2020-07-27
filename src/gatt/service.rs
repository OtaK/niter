#[zbus::dbus_proxy(interface = "org.bluez.GattService1", default_service = "org.bluez")]
#[derive(Debug, Clone, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
pub trait GattService {
    #[zbus::dbus_proxy(property)]
    fn uuid(&self) -> zbus::fdo::Result<String>;
    #[zbus::dbus_proxy(property)]
    fn primary(&self) -> zbus::fdo::Result<bool>;
    #[zbus::dbus_proxy(property)]
    fn device(&self) -> zbus::fdo::Result<crate::adapter::Device>;
    #[zbus::dbus_proxy(property)]
    fn includes(&self) -> zbus::fdo::Result<Vec<String>>;
    #[zbus::dbus_proxy(property)]
    fn handle(&self) -> zbus::fdo::Result<u16>;
}

#[derive(Debug, Clone, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
pub struct GattService;
