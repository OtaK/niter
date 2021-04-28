
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
pub enum ObexMessageReceptionStatus {
    Complete,
    Fractioned,
    Notification,
}

crate::impl_tryfrom_zvariant!(ObexMessageReceptionStatus);

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
pub enum ObexMessageType {
    Email,
    SmsGsm,
    SmsCdma,
    Mms,
}

crate::impl_tryfrom_zvariant!(ObexMessageType);

#[zbus::dbus_proxy(
    interface = "org.bluez.obex.Message1",
    default_service = "org.bluez.obex"
)]
#[derive(Debug, Clone, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
pub trait ObexMessage {
    fn get(&self, targetfile: String, attachment: bool) -> zbus::Result<(String, )>;

    #[dbus_proxy(property)]
    fn folder(&self) -> zbus::fdo::Result<String>;
    #[dbus_proxy(property)]
    fn subject(&self) -> zbus::fdo::Result<String>;
    #[dbus_proxy(property)]
    fn timestamp(&self) -> zbus::fdo::Result<String>;
    #[dbus_proxy(property)]
    fn sender(&self) -> zbus::fdo::Result<String>;
    #[dbus_proxy(property)]
    fn sender_address(&self) -> zbus::fdo::Result<String>;
    #[dbus_proxy(property)]
    fn reply_to(&self) -> zbus::fdo::Result<String>;
    #[dbus_proxy(property)]
    fn recipient(&self) -> zbus::fdo::Result<String>;
    #[dbus_proxy(property)]
    fn recipient_address(&self) -> zbus::fdo::Result<String>;
    #[dbus_proxy(property, name = "Type")]
    fn r#type(&self) -> zbus::fdo::Result<ObexMessageType>;
    #[dbus_proxy(property)]
    fn status(&self) -> zbus::fdo::Result<ObexMessageReceptionStatus>;
    #[dbus_proxy(property)]
    fn priority(&self) -> zbus::fdo::Result<bool>;
    #[dbus_proxy(property, name = "Read")]
    fn is_read(&self) -> zbus::fdo::Result<bool>;
    #[dbus_proxy(property)]
    fn set_read(&self) -> zbus::fdo::Result<String>;
    #[dbus_proxy(property)]
    fn set_deleted(&self) -> zbus::fdo::Result<String>;
    #[dbus_proxy(property)]
    fn sent(&self) -> zbus::fdo::Result<bool>;
    #[dbus_proxy(property)]
    fn protected(&self) -> zbus::fdo::Result<bool>;
}
