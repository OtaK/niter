#[derive(Debug, Clone, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
pub struct Battery {
    object_path: String,
}

impl std::str::FromStr for Battery {
    type Err = crate::NiterError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            object_path: s.into(),
        })
    }
}

crate::to_proxy_impl!(Battery, BatteryProxy, "org.bluez");
crate::impl_tryfrom_zvariant!(Battery);

#[zbus::dbus_proxy(interface = "org.bluez.Battery1", default_service = "org.bluez")]
pub trait Battery {
    #[dbus_proxy(property)]
    fn percentage(&self) -> zbus::fdo::Result<u8>;
}
