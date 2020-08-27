#[derive(
    Debug, Clone, Default, Copy, zvariant_derive::Type, serde::Serialize, serde::Deserialize,
)]
pub struct MeshNodeFeatures {
    friend: bool,
    low_power: bool,
    proxy: bool,
    relay: bool,
}

impl std::convert::TryFrom<zvariant::OwnedValue> for MeshNodeFeatures {
    type Error = crate::NiterError;
    fn try_from(value: zvariant::OwnedValue) -> Result<Self, Self::Error> {
        use std::convert::TryInto as _;
        let mut ret = Self::default();
        let dict: zvariant::Dict = value.try_into()?;
        ret.friend = (dict.get("Friend")?.copied() as Option<bool>).unwrap_or_default();
        ret.low_power = (dict.get("LowPower")?.copied() as Option<bool>).unwrap_or_default();
        ret.proxy = (dict.get("Proxy")?.copied() as Option<bool>).unwrap_or_default();
        ret.relay = (dict.get("Relay")?.copied() as Option<bool>).unwrap_or_default();
        Ok(ret)
    }
}

#[derive(
    Debug, Clone, Default, Copy, zvariant_derive::Type, serde::Serialize, serde::Deserialize,
)]
pub struct MeshNode {}

#[zbus::dbus_proxy(interface = "org.bluez.mesh.Node1", default_service = "org.bluez.mesh")]
pub trait MeshNode {
    fn send(
        &self,
        element_path: zvariant::ObjectPath,
        destination: u16,
        key_index: u16,
        data: Vec<u8>,
    ) -> zbus::Result<()>;
    fn dev_key_send(
        &self,
        element_path: zvariant::ObjectPath,
        destination: u16,
        remote: bool,
        net_index: u16,
        data: Vec<u8>,
    ) -> zbus::Result<()>;
    fn add_net_key(
        &self,
        element_path: zvariant::ObjectPath,
        destination: u16,
        subnet_index: u16,
        net_index: u16,
        update: bool,
    ) -> zbus::Result<()>;
    fn add_app_key(
        &self,
        element_path: zvariant::ObjectPath,
        destination: u16,
        app_index: u16,
        net_index: u16,
        update: bool,
    ) -> zbus::Result<()>;
    fn publish(
        &self,
        element_path: zvariant::ObjectPath,
        model: u16,
        data: Vec<u8>,
    ) -> zbus::Result<()>;
    fn vendor_publish(
        &self,
        element_path: zvariant::ObjectPath,
        vendor: u16,
        model_id: u16,
        data: Vec<u8>,
    ) -> zbus::Result<()>;

    #[dbus_proxy(property)]
    fn features(&self) -> zbus::fdo::Result<MeshNodeFeatures>;
    #[dbus_proxy(property)]
    fn beacon(&self) -> zbus::fdo::Result<bool>;
    #[dbus_proxy(property)]
    fn beacon_flags(&self) -> zbus::fdo::Result<u8>;
    #[dbus_proxy(property)]
    fn iv_index(&self) -> zbus::fdo::Result<u32>;
    #[dbus_proxy(property)]
    fn seconds_since_last_heard(&self) -> zbus::fdo::Result<u32>;
    #[dbus_proxy(property)]
    fn addresses(&self) -> zbus::fdo::Result<Vec<u16>>;
    #[dbus_proxy(property)]
    fn sequence_number(&self) -> zbus::fdo::Result<u32>;
}
