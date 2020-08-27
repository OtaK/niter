pub mod transport;
pub mod endpoint;
pub mod item;
pub mod folder;
pub mod player;
pub mod control;

#[derive(Debug, Clone, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
pub struct MediaEndpointRegisterProperties {
    #[serde(rename = "UUID")]
    uuid: crate::Uuid,
    codec: u8,
    capabilities: endpoint::MediaEndpointCapabilities,
}

#[zbus::dbus_proxy(
    interface = "org.bluez.Media1",
    default_service = "org.bluez",
    // default_path = "[variable prefix]/{hci0,hci1,...}"
)]
pub trait Media {
    fn register_endpoint(
        &self,
        endpoint: endpoint::MediaEndpointServer<impl endpoint::MediaEndpointDelegate<zbus::fdo::Error> + serde::Serialize>,
        properties: MediaEndpointRegisterProperties,
    ) -> zbus::Result<()>;

    fn unregister_endpoint(&self, endpoint: endpoint::MediaEndpointServer<impl endpoint::MediaEndpointDelegate<zbus::fdo::Error> + serde::Serialize>) -> zbus::Result<()>;

    fn register_player<
        K: serde::Serialize + Eq + std::hash::Hash + zvariant::Type,
        V: serde::Serialize + zvariant::Type
    >(&self, player: player::MediaPlayer, properties: std::collections::HashMap<K, V>) -> zbus::Result<()>;

    fn unregister_player(&self, player: player::MediaPlayer) -> zbus::Result<()>;

    fn register_application<
        K: serde::Serialize + Eq + std::hash::Hash + zvariant::Type,
        V: serde::Serialize + zvariant::Type
    >(&self, root: zvariant::ObjectPath, options: std::collections::HashMap<K, V>) -> zbus::Result<()>;

    fn unregister_application(&self, application: zvariant::ObjectPath) -> zbus::Result<()>;
}
