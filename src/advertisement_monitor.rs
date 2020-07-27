use crate::adapter::Device;

#[derive(Debug, Clone, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
pub struct AdvertisementMonitor {}

#[zbus::dbus_proxy(
    interface = "org.bluez.AdvertisementMonitor1",
    default_service = "org.bluez"
)]
#[derive(Debug, Clone, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
pub trait AdvertisementMonitor {
    fn release(&self) -> zbus::Result<()>;
    fn activate(&self) -> zbus::Result<()>;
    fn device_found(&self, device: Device) -> zbus::Result<()>;
    fn device_lost(&self, device: Device) -> zbus::Result<()>;

    #[zbus::dbus_proxy(property)]
    fn r#type(&self) -> zbus::fdo::Result<String>;
    #[zbus::dbus_proxy(property)]
    fn rssi_thresholds_and_timers(&self) -> zbus::fdo::Result<(u16, u16, u16, u16)>;
    #[zbus::dbus_proxy(property)]
    fn patterns(&self) -> zbus::fdo::Result<Vec<(u8, u8, String)>>;
}

#[derive(Debug, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
pub enum MonitorType {
    OrPatterns,
}

#[derive(Debug, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
pub enum MonitorFeature {
    ControllerPatterns,
}

#[zbus::dbus_proxy(
    interface = "org.bluez.AdvertisementMonitorManager1",
    default_service = "org.bluez"
)]
#[derive(Debug, Clone, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
pub trait AdvertisementMonitorManager {
    fn register_monitor(&self, application: AdvertisementMonitor) -> zbus::Result<()>;
    fn unregister_monitor(&self, application: AdvertisementMonitor) -> zbus::Result<()>;

    #[zbus::dbus_proxy(property)]
    fn supported_monitor_types(&self) -> zbus::fdo::Result<Vec<MonitorType>>;
    #[zbus::dbus_proxy(property)]
    fn supported_features(&self) -> zbus::fdo::Result<Vec<MonitorFeature>>;
}
