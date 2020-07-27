#[derive(Debug, Clone, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
pub struct GattCharacteristicReadOptions {
    offset: u16,
    mtu: u16,
    device: crate::adapter::Device,
}

#[derive(Debug, Clone, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
pub enum GattWriteType {
    Command,
    Request,
    Reliable,
}

#[derive(Debug, Clone, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
pub struct GattCharacteristicWriteOptions {
    offset: u16,
    r#type: GattWriteType,
    mtu: u16,
    device: crate::adapter::Device,
    link: super::GattLinkType,
    prepare_authorize: bool,
}

#[derive(Debug, Clone, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
pub struct GattCharacteristicAcquireOptions {
    mtu: u16,
    device: crate::adapter::Device,
    link: super::GattLinkType,
}

#[derive(Debug, Clone, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
pub enum GattCharacteristicFlags {
    Broadcast,
    Read,
    WriteWithoutResponse,
    Write,
    Notify,
    Indicate,
    AuthenticatedSignedWrites,
    ExtendedProperties,
    ReliableWrite,
    WritableAuxiliaries,
    EncryptRead,
    EncryptWrite,
    EncryptAuthenticatedRead,
    EncryptAuthenticatedWrite,
    SecureRead,
    SecureWrite,
    Authorize,
}

#[zbus::dbus_proxy(
    interface = "org.bluez.GattCharacteristic1",
    default_service = "org.bluez"
)]
#[derive(Debug, Clone, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
pub trait GattCharacteristic {
    fn read_value(&self, options: GattCharacteristicReadOptions) -> zbus::Result<Vec<u8>>;
    fn write_value(
        &self,
        value: Vec<u8>,
        options: GattCharacteristicWriteOptions,
    ) -> zbus::Result<bool>;
    fn acquire_write(
        &self,
        options: GattCharacteristicAcquireOptions,
    ) -> zbus::Result<(std::os::unix::io::RawFd, u16)>;
    fn acquire_notify(
        &self,
        options: GattCharacteristicAcquireOptions,
    ) -> zbus::Result<(std::os::unix::io::RawFd, u16)>;
    fn start_notify(&self) -> zbus::Result<()>;
    fn stop_notify(&self) -> zbus::Result<()>;
    fn confirm(&self) -> zbus::Result<()>;

    #[zbus::dbus_proxy(property)]
    fn uuid(&self) -> zbus::fdo::Result<String>;
    #[zbus::dbus_proxy(property)]
    fn service(&self) -> zbus::fdo::Result<super::GattService>;
    #[zbus::dbus_proxy(property)]
    fn value(&self) -> zbus::fdo::Result<Vec<u8>>;
    #[zbus::dbus_proxy(property)]
    fn write_acquired(&self) -> zbus::fdo::Result<bool>;
    #[zbus::dbus_proxy(property)]
    fn notify_acquired(&self) -> zbus::fdo::Result<bool>;
    #[zbus::dbus_proxy(property)]
    fn notifying(&self) -> zbus::fdo::Result<bool>;
    #[zbus::dbus_proxy(property)]
    fn flags(&self) -> zbus::fdo::Result<Vec<GattCharacteristicFlags>>;
    #[zbus::dbus_proxy(property)]
    fn handle(&self) -> zbus::fdo::Result<u16>;
}

#[derive(Debug, Clone, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
pub struct GattCharacteristic;
