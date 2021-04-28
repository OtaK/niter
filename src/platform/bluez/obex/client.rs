
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
#[strum(serialize_all = "lowercase")]
pub enum ObexSessionTarget {
    Ftp,
    Map,
    Opp,
    Pbap,
    Sync,
}

#[derive(Debug, Clone, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
pub struct ObexSessionArgs {
    target: ObexSessionTarget,
    source: String,
    channel: u8,
}

#[derive(Debug, Clone, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
pub struct ObexSession {
    object_path: String,
}

#[zbus::dbus_proxy(
    interface = "org.bluez.obex.Client1",
    default_service = "org.bluez.obex",
    default_path = "/org/bluez/obex"
)]
pub trait ObexClient {
    fn create_session(
        &self,
        destination: String,
        args: ObexSessionArgs,
    ) -> zbus::Result<ObexSession>;
    fn remove_session(&self, session: ObexSession) -> zbus::Result<()>;
}
