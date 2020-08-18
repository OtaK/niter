#[derive(Debug, Clone, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
pub struct GattDescriptorReadOptions {
    offset: u16,
    device: crate::device::Device,
    link: super::GattLinkType,
}

#[derive(Debug, Clone, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
pub struct GattDescriptorWriteOptions {
    offset: u16,
    device: crate::device::Device,
    link: super::GattLinkType,
    prepare_authorize: bool,
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
pub enum GattDescriptorFlags {
    Read,
    Write,
    EncryptRead,
    EncryptWrite,
    EncryptAuthenticatedRead,
    EncryptAuthenticatedWrite,
    SecureRead,
    SecureWrite,
    Authorize,
}

crate::impl_tryfrom_zvariant!(GattDescriptorFlags);

#[zbus::dbus_proxy(interface = "org.bluez.GattDescriptor1", default_service = "org.bluez")]
#[derive(Debug, Clone, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
pub trait GattDescriptor {
    fn read_value(&self, flags: GattDescriptorReadOptions) -> zbus::Result<Vec<u8>>;
    fn write_value(&self, value: Vec<u8>, flags: GattDescriptorWriteOptions) -> zbus::Result<()>;

    #[dbus_proxy(property, name = "UUID")]
    fn uuid(&self) -> zbus::fdo::Result<crate::UuidArray>;
    #[dbus_proxy(property)]
    fn characteristic(&self) -> zbus::fdo::Result<super::GattCharacteristic>;
    #[dbus_proxy(property)]
    fn value(&self) -> zbus::fdo::Result<Vec<u8>>;
    #[dbus_proxy(property)]
    fn flags(&self) -> zbus::fdo::Result<crate::ZvariantableArray<GattDescriptorFlags>>;
    #[dbus_proxy(property)]
    fn handle(&self) -> zbus::fdo::Result<u16>;
}
