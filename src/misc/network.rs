#[derive(Debug, Clone, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
pub struct Network {
    object_path: String,
}

impl std::str::FromStr for Network {
    type Err = crate::NiterError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self { object_path: s.into() })
    }
}

crate::to_proxy_impl!(Network, NetworkProxy, "org.bluez");
crate::impl_tryfrom_zvariant!(Network);

#[derive(Debug, Clone, strum::EnumString, strum::Display, serde::Serialize, serde::Deserialize)]
pub enum NetworkUuid {
    Uuid(crate::Uuid),
    Gn,
    Panu,
    Nap,
}

impl std::convert::TryFrom<zvariant::OwnedValue> for NetworkUuid {
    type Error = crate::NiterError;
    fn try_from(v: zvariant::OwnedValue) -> crate::NiterResult<Self> {
        use std::convert::TryInto as _;
        use std::str::FromStr as _;
        let s: String = v.try_into()?;
        if let Ok(uuid) = crate::Uuid::from_str(&s) {
            Ok(Self::Uuid(uuid))
        } else {
            Ok(Self::from_str(&s)?)
        }
    }
}

impl zvariant::Type for NetworkUuid {
    fn signature() -> zvariant::Signature<'static> {
        zvariant::Signature::from_str_unchecked("s")
    }
}

#[zbus::dbus_proxy(
    interface = "org.bluez.Network1",
    default_service = "org.bluez"
)]
pub trait Network {
    fn connect(&self, uuid: NetworkUuid) -> zbus::Result<String>;
    fn disconnect(&self) -> zbus::Result<()>;

    #[dbus_proxy(property)]
    fn connected(&self) -> zbus::fdo::Result<bool>;
    #[dbus_proxy(property)]
    fn interface(&self) -> zbus::fdo::Result<String>;
    #[dbus_proxy(property, name = "UUID")]
    fn uuid(&self) -> zbus::fdo::Result<NetworkUuid>;
}

#[derive(Debug, Clone, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
pub struct NetworkServer {
    object_path: String
}

impl std::str::FromStr for NetworkServer {
    type Err = crate::NiterError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self { object_path: s.into() })
    }
}

crate::to_proxy_impl!(NetworkServer, NetworkServerProxy, "org.bluez");
crate::impl_tryfrom_zvariant!(NetworkServer);

#[zbus::dbus_proxy(
    interface = "org.bluez.NetworkServer1",
    default_service = "org.bluez",
    default_path = "/org/bluez/hci0"
)]
pub trait NetworkServer {
    fn register(&self, uuid: NetworkUuid, bridge: String) -> zbus::Result<()>;
    fn unregsiter(&self, uuid: NetworkUuid) -> zbus::Result<()>;
}
