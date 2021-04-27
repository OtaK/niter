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
pub enum MeshApplicationJoinFailureReason {
    Timeout,
    BadPdu,
    ConfirmationFailed,
    OutOfResources,
    DecryptionError,
    UnexpectedError,
    CannotAssignAddresses,
}

crate::impl_tryfrom_zvariant!(MeshApplicationJoinFailureReason);

pub trait MeshApplicationDelegate: zvariant::Type + serde::Serialize + 'static {
    fn join_complete(&mut self, token: u64);
    fn join_failed(&mut self, reason: MeshApplicationJoinFailureReason);
}

#[derive(Debug, Clone, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
pub struct MeshApplication<T: MeshApplicationDelegate> {
    company_id: u16,
    product_id: u16,
    version_id: u16,
    crpl: u16,
    service_name: String,
    object_path: String,
    delegate: T,
}

#[zbus::dbus_interface(name = "org.bluez.mesh.Application1")]
impl<T: MeshApplicationDelegate> MeshApplication<T> {
    fn join_complete(&mut self, token: u64) {
        self.delegate.join_complete(token)
    }
    fn join_failed(&mut self, reason: MeshApplicationJoinFailureReason) {
        self.delegate.join_failed(reason)
    }

    #[dbus_interface(property)]
    fn company_id(&self) -> u16 {
        self.company_id
    }
    #[dbus_interface(property)]
    fn product_id(&self) -> u16 {
        self.product_id
    }
    #[dbus_interface(property)]
    fn version_id(&self) -> u16 {
        self.version_id
    }
    #[dbus_interface(property, name = "CRPL")]
    fn crpl(&self) -> u16 {
        self.crpl
    }
}
