#[zbus::dbus_proxy(interface = "org.bluez.GattProfile1", default_service = "org.bluez")]
#[derive(Debug, Clone, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
pub trait GattProfile {
    fn release(&self) -> zbus::Result<()>;

    #[zbus::dbus_proxy(property)]
    fn uuids(&self) -> zbus::fdo::Result<Vec<String>>;
}
