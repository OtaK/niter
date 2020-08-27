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
pub enum ProvisionerAddNodeFailedReason {
    Aborted,
    Timeout,
    BadPdu,
    ConfirmationFailed,
    OutOfResources,
    DecryptionError,
    UnexpectedError,
    CannotAssignAddresses,
}

crate::impl_tryfrom_zvariant!(ProvisionerAddNodeFailedReason);

pub trait ProvisionerDelegate: zvariant::Type {
    fn scan_result(
        &self,
        rssi: i16,
        data: Vec<u8>,
        options: std::collections::HashMap<String, String>,
    );
    fn request_prov_data(&self, count: u8) -> (u16, u16);
    fn add_node_complete(&self, uuid: crate::Uuid, unicast: u16, count: u8);
    fn add_node_failed(&self, uuid: crate::Uuid, reason: ProvisionerAddNodeFailedReason);
}

#[derive(Debug, Clone, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
pub struct Provisioner<T: ProvisionerDelegate> {
    service_name: String,
    object_path: String,
    delegate: T,
}

#[zbus::dbus_interface(name = "org.bluez.mesh.Provisioner1")]
impl<T: ProvisionerDelegate> Provisioner<T> {
    fn scan_result(
        &self,
        rssi: i16,
        data: Vec<u8>,
        options: std::collections::HashMap<String, String>,
    ) {
        self.delegate.scan_result(rssi, data, options)
    }
    fn request_prov_data(&self, count: u8) -> (u16, u16) {
        self.delegate.request_prov_data(count)
    }
    fn add_node_complete(&self, uuid: crate::Uuid, unicast: u16, count: u8) {
        self.delegate.add_node_complete(uuid, unicast, count)
    }
    fn add_node_failed(&self, uuid: crate::Uuid, reason: ProvisionerAddNodeFailedReason) {
        self.delegate.add_node_failed(uuid, reason)
    }
}
