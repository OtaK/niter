#[zbus::dbus_proxy(interface = "org.bluez.GattService1", default_service = "org.bluez")]
#[derive(Debug, Clone, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
pub trait GattService {
    #[dbus_proxy(property, name = "UUID")]
    fn uuid(&self) -> zbus::fdo::Result<crate::Uuid>;
    #[dbus_proxy(property)]
    fn primary(&self) -> zbus::fdo::Result<bool>;
    #[dbus_proxy(property)]
    fn device(&self) -> zbus::fdo::Result<crate::device::Device>;
    #[dbus_proxy(property)]
    fn includes(&self) -> zbus::fdo::Result<crate::ZvariantableArray<crate::advertising::SystemInclude>>;
    #[dbus_proxy(property)]
    fn handle(&self) -> zbus::fdo::Result<u16>;
}

#[derive(Debug, Clone, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
pub struct GattService {
    object_path: String,
}

impl std::str::FromStr for GattService {
    type Err = crate::NiterError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self { object_path: s.into() })
    }
}

crate::impl_tryfrom_zvariant!(GattService);
