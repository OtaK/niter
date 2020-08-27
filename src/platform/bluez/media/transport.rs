pub type MediaTransportConfiguration = Vec<u8>;

#[derive(
    Debug,
    Clone,
    Copy,
    strum::Display,
    strum::EnumString,
    serde::Serialize,
    serde::Deserialize,
    zvariant_derive::Type,
)]
#[strum(serialize_all = "lowercase")]
pub enum MediaTransportState {
    Idle,
    Pending,
    Active,
}

crate::impl_tryfrom_zvariant!(MediaTransportState);

type MediaEndpoint = bool;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, zvariant_derive::Type)]
pub struct MediaTransport {
    object_path: String
}

impl MediaTransport {
    pub fn configuration(&self, connection: &zbus::Connection) -> crate::NiterResult<MediaTransportConfiguration> {
        Ok(self.get_proxy(connection)?.configuration()?)
    }
}

impl std::str::FromStr for MediaTransport {
    type Err = crate::NiterError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            object_path: s.into(),
        })
    }
}

crate::impl_tryfrom_zvariant!(MediaTransport);
crate::to_proxy_impl!(MediaTransport, MediaTransportProxy, "org.bluez");

#[zbus::dbus_proxy(
    interface = "org.bluez.MediaTransport1",
    default_service = "org.bluez",
    // default_path = "[variable prefix]/{hci0,hci1,...}/dev_XX_XX_XX_XX_XX_XX/fdX"
)]
pub trait MediaTransport {
    fn acquire(&self) -> zbus::Result<(std::os::unix::io::RawFd, u16, u16)>;
    fn try_acquire(&self) -> zbus::Result<(std::os::unix::io::RawFd, u16, u16)>;
    fn release(&self) -> zbus::Result<()>;

    #[dbus_proxy(property)]
    fn device(&self) -> zbus::fdo::Result<crate::platform::bluez::device::Device>;
    #[dbus_proxy(property, name = "UUID")]
    fn uuid(&self) -> zbus::fdo::Result<crate::Uuid>;
    #[dbus_proxy(property)]
    fn codec(&self) -> zbus::fdo::Result<u8>;
    #[dbus_proxy(property)]
    fn configuration(&self) -> zbus::fdo::Result<MediaTransportConfiguration>;
    #[dbus_proxy(property)]
    fn state(&self) -> zbus::fdo::Result<MediaTransportState>;
    #[dbus_proxy(property)]
    fn delay(&self) -> zbus::fdo::Result<u16>;
    #[dbus_proxy(property)]
    fn set_delay(&self, delay: u16) -> zbus::fdo::Result<()>;
    #[dbus_proxy(property)]
    fn volume(&self) -> zbus::fdo::Result<u16>;
    #[dbus_proxy(property)]
    fn set_volume(&self, volume: u16) -> zbus::fdo::Result<()>;
    #[dbus_proxy(property)]
    fn endpoint(&self) -> zbus::fdo::Result<MediaEndpoint>;
}
