#[derive(Debug, Clone, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
pub struct Application {}

#[derive(Debug, Clone, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
pub struct RegisterApplicationOptions {}

#[zbus::dbus_proxy(interface = "org.bluez.GattManager1", default_service = "org.bluez")]
#[derive(Debug, Clone, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
pub trait GattManager {
    fn register_application(
        &self,
        application: Application,
        options: RegisterApplicationOptions,
    ) -> zbus::Result<Vec<u8>>;
    fn unregister_application(&self, application: Application) -> zbus::Result<bool>;
}
