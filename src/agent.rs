use crate::device::Device;

#[derive(Debug, Clone, Copy, strum::Display, strum::EnumString, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
#[strum(serialize_all = "PascalCase")]
pub enum AgentCapability {
    DisplayOnly,
    DisplayYesNo,
    KeyboardOnly,
    NoInputNoOutput,
    KeyboardDisplay,
}

impl Default for AgentCapability {
    fn default() -> Self {
        Self::KeyboardDisplay
    }
}

#[derive(Debug, Clone, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
pub struct Agent(String);

#[zbus::dbus_proxy(
    interface = "org.bluez.AgentManager1",
    default_service = "org.bluez",
    default_path = "/org/bluez/AgentManager1"
)]
#[derive(Debug, Clone, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
pub trait AgentManager {
    fn register_agent(&self, agent: Agent, capability: AgentCapability) -> zbus::Result<()>;
    fn unregister_agent(&self, agent: Agent) -> zbus::Result<()>;
    fn request_default_agent(&self, agent: Agent) -> zbus::Result<()>;
}

#[zbus::dbus_proxy(interface = "org.bluez.Agent1")]
#[derive(Debug, Clone, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
pub trait Agent {
    fn release(&self) -> zbus::Result<()>;
    fn request_pin_code(&self, device: Device) -> zbus::Result<String>;
    fn display_pin_code(&self, device: Device, pincode: String) -> zbus::Result<()>;
    fn request_passkey(&self, device: Device) -> zbus::Result<u32>;
    fn display_passkey(&self, device: Device, passkey: u32, entered: u16) -> zbus::Result<()>;
    fn request_confirmation(&self, device: Device, passkey: u32) -> zbus::Result<()>;
    fn request_authorization(&self, device: Device) -> zbus::Result<()>;
    fn authorize_service(&self, uuid: crate::Uuid) -> zbus::Result<()>;
    fn cancel(&self) -> zbus::Result<()>;
}
