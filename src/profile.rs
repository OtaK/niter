use crate::device::Device;
use crate::assigned_numbers::AssignedRfcommNumber;

#[derive(Debug, Clone, Copy, strum::EnumString, strum::Display, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
#[strum(serialize_all = "lowercase")]
pub enum ServiceRole {
    Client,
    Server,
}

#[derive(Debug, Clone, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
pub struct ProfileManagerRegisterOptions {
    name: String,
    service: String,
    role: ServiceRole,
    channel: AssignedRfcommNumber,
    psm: u16,
    require_authentication: bool,
    auto_connect: bool,
    service_record: String,
    version: u16,
    features: u16,
}

#[derive(Debug, Clone, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
pub struct Profile(String);

#[zbus::dbus_proxy(
    interface = "org.bluez.ProfileManager1",
    default_service = "org.bluez",
    default_path = "/org/bluez/ProfileManager1"
)]
#[derive(Debug, Clone, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
pub trait ProfileManager {
    fn register_profile(
        &self,
        profile: Profile,
        uuid: crate::Uuid,
        options: ProfileManagerRegisterOptions,
    ) -> zbus::Result<()>;
    fn unregister_profile(&self, profile: Profile) -> zbus::Result<()>;
}

#[derive(Debug, Clone, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
pub struct FdProperties {
    version: u16,
    features: u16,
}

#[zbus::dbus_proxy(interface = "org.bluez.Profile1")]
#[derive(Debug, Clone, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
pub trait Profile {
    fn release(&self) -> zbus::Result<()>;
    fn new_connection(
        &self,
        device: Device,
        fd: std::os::unix::io::RawFd,
        fd_properties: FdProperties,
    ) -> zbus::Result<()>;
    fn request_disconnection(&self, device: Device) -> zbus::Result<()>;
}
