#[derive(Debug, Clone, Copy, zvariant_derive::Type, serde::Deserialize, serde::Serialize)]
pub struct UnprovisionedScanOptions {
    seconds: u16,
}

#[derive(
    Debug,
    Clone,
    Copy,
    zvariant_derive::Type,
    serde_repr::Deserialize_repr,
    serde_repr::Serialize_repr,
)]
#[repr(u8)]
pub enum KeyPhase {
    CancelKeyRefresh = 0,
    InvalidArgument = 1,
    GoToPhase2 = 2,
    CompleteKeyRefreshProcedure = 3,
}

#[zbus::dbus_proxy(
    interface = "org.bluez.mesh.Management1",
    default_service = "org.bluez.mesh",
    // default_path = "/org/bluez/mesh/node{uuid}"
)]
pub trait MeshProvisioningManager {
    fn unprovisioned_scan(&self, options: UnprovisionedScanOptions) -> zbus::Result<()>;
    fn unprovisioned_scan_cancel(&self) -> zbus::Result<()>;
    fn add_node(&self, uuid: crate::Uuid, options: crate::BlueZDummy) -> zbus::Result<()>;
    fn create_subnet(&self, net_index: u16) -> zbus::Result<()>;
    fn import_subnet(&self, net_index: u16, net_key: &[u8; 16]) -> zbus::Result<()>;
    fn update_subnet(&self, net_index: u16) -> zbus::Result<()>;
    fn delete_subnet(&self, net_index: u16) -> zbus::Result<()>;
    fn set_key_phase(&self, index: u16, phase: KeyPhase) -> zbus::Result<()>;
    fn create_app_key(&self, net_index: u16, app_index: u16) -> zbus::Result<()>;
    fn import_app_key(
        &self,
        net_index: u16,
        app_index: u16,
        app_key: &[u8; 16],
    ) -> zbus::Result<()>;
    fn update_app_key(&self, app_index: u16) -> zbus::Result<()>;
    fn delete_app_key(&self, app_index: u16) -> zbus::Result<()>;
    fn import_remote_node(
        &self,
        primary: u16,
        count: u8,
        device_key: &[u8; 16],
    ) -> zbus::Result<()>;
    fn delete_remote_node(&self, primary: u16, count: u8) -> zbus::Result<()>;
}
