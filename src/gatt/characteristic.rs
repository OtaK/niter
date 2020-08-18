#[derive(Debug, Clone, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
pub struct GattCharacteristicReadOptions {
    offset: u16,
    mtu: u16,
    device: crate::device::Device,
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
#[strum(serialize_all = "lowercase")]
pub enum GattWriteType {
    Command,
    Request,
    Reliable,
}

crate::impl_tryfrom_zvariant!(GattWriteType);

#[derive(Debug, Clone, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
pub struct GattCharacteristicWriteOptions {
    offset: u16,
    r#type: GattWriteType,
    mtu: u16,
    device: crate::device::Device,
    link: super::GattLinkType,
    prepare_authorize: bool,
}

#[derive(Debug, Clone, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
pub struct GattCharacteristicAcquireOptions {
    mtu: u16,
    device: crate::device::Device,
    link: super::GattLinkType,
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

crate::impl_tryfrom_zvariant!(GattCharacteristicFlags);

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

    #[dbus_proxy(property, name = "UUID")]
    fn uuid(&self) -> zbus::fdo::Result<crate::Uuid>;
    #[dbus_proxy(property)]
    fn service(&self) -> zbus::fdo::Result<super::GattService>;
    #[dbus_proxy(property)]
    fn value(&self) -> zbus::fdo::Result<Vec<u8>>;
    #[dbus_proxy(property)]
    fn write_acquired(&self) -> zbus::fdo::Result<bool>;
    #[dbus_proxy(property)]
    fn notify_acquired(&self) -> zbus::fdo::Result<bool>;
    #[dbus_proxy(property)]
    fn notifying(&self) -> zbus::fdo::Result<bool>;
    #[dbus_proxy(property)]
    fn flags(&self) -> zbus::fdo::Result<crate::ZvariantableArray<GattCharacteristicFlags>>;
    #[dbus_proxy(property)]
    fn handle(&self) -> zbus::fdo::Result<u16>;
}

#[derive(Debug, Clone, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
pub struct GattCharacteristic {
    object_path: String,
}

impl std::str::FromStr for GattCharacteristic {
    type Err = crate::NiterError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            object_path: s.into(),
        })
    }
}

crate::impl_tryfrom_zvariant!(GattCharacteristic);
