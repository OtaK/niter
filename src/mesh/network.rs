#![allow(clippy::too_many_arguments)]

use crate::mesh::{
    application::{MeshApplication, MeshApplicationDelegate},
    element::ModelConfiguration,
    node::MeshNode,
};

#[derive(Debug, Clone, Copy, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
pub struct MeshNetworkImportFlags {
    iv_update: bool,
    key_refresh: bool,
}

pub type NodeConfiguration = Vec<(u8, ModelConfigurationList)>;
pub type ModelConfigurationList = Vec<(u16, ModelConfiguration)>;

#[zbus::dbus_proxy(
    interface = "org.bluez.mesh.Network1",
    default_service = "org.bluez.mesh",
    default_path = "/org/bluez/mesh"
)]
pub trait MeshNetwork {
    fn join(
        &self,
        app_root: MeshApplication<impl MeshApplicationDelegate>,
        uuid: crate::RawUuid,
    ) -> zbus::Result<()>;
    fn cancel(&self) -> zbus::Result<()>;
    fn attach(
        &self,
        app_root: MeshApplication<impl MeshApplicationDelegate>,
        token: u64,
    ) -> zbus::Result<(MeshNode, NodeConfiguration)>;
    fn leave(&self, token: u64) -> zbus::Result<()>;
    fn create_network(
        &self,
        app_root: MeshApplication<impl MeshApplicationDelegate>,
        uuid: crate::RawUuid,
    ) -> zbus::Result<u64>;
    fn import(
        &self,
        app_root: MeshApplication<impl MeshApplicationDelegate>,
        uuid: crate::RawUuid,
        dev_key: crate::RawUuid,
        net_key: crate::RawUuid,
        net_index: u16,
        flags: MeshNetworkImportFlags,
        iv_index: u32,
        unicast: u16,
    ) -> zbus::Result<u64>;
}
