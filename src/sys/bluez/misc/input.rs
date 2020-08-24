#[derive(Debug, Clone, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
pub struct Input {
    object_path: String,
}

impl std::str::FromStr for Input {
    type Err = crate::NiterError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            object_path: s.into(),
        })
    }
}

crate::to_proxy_impl!(Input, InputProxy, "org.bluez");
crate::impl_tryfrom_zvariant!(Input);

#[derive(
    Debug,
    Clone,
    Copy,
    strum::EnumString,
    strum::Display,
    zvariant_derive::Type,
    serde::Serialize,
    serde::Deserialize,
)]
#[strum(serialize_all = "lowercase")]
pub enum InputReconnectMode {
    None,
    Host,
    Device,
    Any,
}

crate::impl_tryfrom_zvariant!(InputReconnectMode);

#[zbus::dbus_proxy(interface = "org.bluez.Input1", default_service = "org.bluez")]
pub trait Input {
    #[dbus_proxy(property)]
    fn reconnect_mode(&self) -> zbus::fdo::Result<InputReconnectMode>;
}
