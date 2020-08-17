#[derive(Debug, Clone, Copy, strum::Display, strum::EnumString, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
#[strum(serialize_all = "lowercase")]
pub enum AdvertisementType {
    Broadcast,
    Peripheral,
}

#[derive(Debug, Clone, Copy, strum::Display, strum::EnumString, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
pub enum SecondaryChannel {
    #[strum(serialize = "1M")]
    OneM,
    #[strum(serialize = "2M")]
    TwoM,
    Coded,
}

#[derive(Debug, Clone, Copy, strum::Display, strum::EnumString, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
#[strum(serialize_all = "kebab-case")]
pub enum SystemInclude {
    TxPower,
    Appearance,
    LocalName,
}

#[derive(Debug, Clone, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
pub struct Advertisement {
    object_path: String
}

#[zbus::dbus_proxy(
    interface = "org.bluez.LEAdvertisement1",
    default_service = "org.bluez"
)]
#[derive(Debug, Clone, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
pub trait Advertisement {
    fn release(&self) -> zbus::Result<()>;

    #[zbus::dbus_proxy(property)]
    fn r#type(&self) -> zbus::fdo::Result<AdvertisementType>;
    #[zbus::dbus_proxy(property)]
    fn service_uuids(&self) -> zbus::fdo::Result<Vec<crate::Uuid>>;
    #[zbus::dbus_proxy(property)]
    fn manufacturer_data(&self) -> zbus::fdo::Result<crate::ManufacturerData>;
    #[zbus::dbus_proxy(property)]
    fn solicit_uuids(&self) -> zbus::fdo::Result<Vec<crate::Uuid>>;
    #[zbus::dbus_proxy(property)]
    fn service_data(&self) -> zbus::fdo::Result<crate::ServiceData>;
    #[zbus::dbus_proxy(property)]
    fn data(&self) -> zbus::fdo::Result<crate::AdvertisingData>;
    #[zbus::dbus_proxy(property)]
    fn discoverable(&self) -> zbus::fdo::Result<bool>;
    #[zbus::dbus_proxy(property)]
    fn discoverable_timeout(&self) -> zbus::fdo::Result<u16>;
    #[zbus::dbus_proxy(property)]
    fn includes(&self) -> zbus::fdo::Result<Vec<SystemInclude>>;
    #[zbus::dbus_proxy(property)]
    fn local_name(&self) -> zbus::fdo::Result<String>;
    #[zbus::dbus_proxy(property)]
    fn appearance(&self) -> zbus::fdo::Result<u16>;
    #[zbus::dbus_proxy(property)]
    fn duration(&self) -> zbus::fdo::Result<u16>;
    #[zbus::dbus_proxy(property)]
    fn timeout(&self) -> zbus::fdo::Result<u16>;
    #[zbus::dbus_proxy(property)]
    fn secondary_channel(&self) -> zbus::fdo::Result<SecondaryChannel>;
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, zvariant_derive::Type)]
pub struct AdvertisingManager {
    object_path: String
}

#[zbus::dbus_proxy(
    interface = "org.bluez.LEAdvertisingManager1",
    default_service = "org.bluez"
)]
#[derive(Debug, Clone, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
pub trait AdvertisingManager {
    fn register_advertisement(
        &self,
        advertisement: Advertisement,
        options: std::collections::HashMap<String, String>,
    ) -> zbus::Result<()>;
    fn unregister_advertisement(&self, advertisement: Advertisement) -> zbus::Result<()>;

    #[zbus::dbus_proxy(property)]
    fn active_instances(&self) -> zbus::fdo::Result<u8>;
    #[zbus::dbus_proxy(property)]
    fn supported_instances(&self) -> zbus::fdo::Result<u8>;
    #[zbus::dbus_proxy(property)]
    fn supported_includes(&self) -> zbus::fdo::Result<Vec<SystemInclude>>;
    #[zbus::dbus_proxy(property)]
    fn supported_secondary_channels(&self) -> zbus::fdo::Result<Vec<SecondaryChannel>>;
}
