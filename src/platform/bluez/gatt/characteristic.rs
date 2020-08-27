#[derive(Debug, Clone, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
pub struct GattCharacteristicReadOptions {
    offset: u16,
    mtu: u16,
    device: crate::platform::bluez::device::Device,
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
    device: crate::platform::bluez::device::Device,
    link: super::GattLinkType,
    prepare_authorize: bool,
}

#[derive(Debug, Clone, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
pub struct GattCharacteristicAcquireOptions {
    mtu: u16,
    device: crate::platform::bluez::device::Device,
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
    default_service = "org.bluez",
    // default_path = "[variable prefix]/{hci0,hci1,...}/dev_XX_XX_XX_XX_XX_XX/serviceXX/charYYYY"
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
    fn flags(&self) -> zbus::fdo::Result<crate::platform::bluez::ZvariantableArray<GattCharacteristicFlags>>;
    #[dbus_proxy(property)]
    fn handle(&self) -> zbus::fdo::Result<u16>;
}

// TODO: Create an interface
#[derive(Debug, Clone, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
pub struct GattCharacteristic {
    object_path: String,
    value: Vec<u8>,
    uuid: crate::Uuid,
}

impl std::str::FromStr for GattCharacteristic {
    type Err = crate::NiterError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            object_path: s.into(),
            value: vec![],
            uuid: crate::Uuid::default()
        })
    }
}

// #[zbus::dbus_interface(name = "org.bluez.GattCharacteristic1")]
// impl GattCharacteristic {
//     fn read_value(&self, options: GattCharacteristicReadOptions) -> zbus::fdo::Result<Vec<u8>> {
//         todo!()
//     }
//     fn write_value(
//         &self,
//         value: Vec<u8>,
//         options: GattCharacteristicWriteOptions,
//     ) -> zbus::fdo::Result<bool> {
//         todo!()
//     }
//     fn acquire_write(
//         &self,
//         options: GattCharacteristicAcquireOptions,
//     ) -> zbus::fdo::Result<(std::os::unix::io::RawFd, u16)> {
//         todo!()
//     }
//     fn acquire_notify(
//         &self,
//         options: GattCharacteristicAcquireOptions,
//     ) -> zbus::fdo::Result<(std::os::unix::io::RawFd, u16)> {
//         todo!()
//     }
//     fn start_notify(&self) -> zbus::fdo::Result<()> {
//         todo!()
//     }
//     fn stop_notify(&self) -> zbus::fdo::Result<()> {
//         todo!()
//     }
//     fn confirm(&self) -> zbus::fdo::Result<()> {
//         todo!()
//     }

//     #[dbus_interface(property, name = "UUID")]
//     fn uuid(&self) -> &str {
//         &self.uuid.to_hyphenated().to_string()
//     }
//     #[dbus_interface(property)]
//     fn service(&self) -> super::GattService {
//         todo!()
//     }
//     #[dbus_interface(property)]
//     fn value(&self) -> Vec<u8> {
//         todo!()
//     }
//     #[dbus_interface(property)]
//     fn write_acquired(&self) -> bool {
//         todo!()
//     }
//     #[dbus_interface(property)]
//     fn notify_acquired(&self) -> bool {
//         todo!()
//     }
//     #[dbus_interface(property)]
//     fn notifying(&self) -> bool {
//         todo!()
//     }
//     #[dbus_interface(property)]
//     fn flags(&self) -> crate::platform::bluez::ZvariantableArray<GattCharacteristicFlags> {
//         todo!()
//     }
//     #[dbus_interface(property)]
//     fn handle(&self) -> u16 {
//         todo!()
//     }
// }

crate::impl_tryfrom_zvariant!(GattCharacteristic);
