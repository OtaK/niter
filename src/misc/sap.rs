#[derive(Debug, Clone, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
pub struct SimAccess {
    object_path: String,
}

impl std::str::FromStr for SimAccess {
    type Err = crate::NiterError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            object_path: s.into(),
        })
    }
}

crate::to_proxy_impl!(SimAccess, SimAccessProxy, "org.bluez");
crate::impl_tryfrom_zvariant!(SimAccess);

#[zbus::dbus_proxy(interface = "org.bluez.SimAccess1", default_service = "org.bluez")]
pub trait SimAccess {
    fn disconnect(&self) -> zbus::Result<()>;

    #[dbus_proxy(property)]
    fn connected(&self) -> zbus::fdo::Result<bool>;
}
