use crate::device::Device;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, zvariant_derive::Type)]
pub struct AdvertisementRssiThresholdsAndTimers(u16, u16, u16, u16);

impl std::convert::TryFrom<zvariant::OwnedValue> for AdvertisementRssiThresholdsAndTimers {
    type Error = crate::NiterError;
    fn try_from(value: zvariant::OwnedValue) -> Result<Self, Self::Error> {
        use std::convert::TryInto as _;
        let zstruct: zvariant::Structure = value.try_into()?;
        let (
            high_rssi_threshold,
            high_rssi_threshold_timer,
            low_rssi_threshold,
            low_rssi_threshold_timer,
        ) = zstruct.try_into()?;
        Ok(Self(
            high_rssi_threshold,
            high_rssi_threshold_timer,
            low_rssi_threshold,
            low_rssi_threshold_timer,
        ))
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, zvariant_derive::Type)]
pub struct AdvertisementPattern(u8, u8, String);

impl std::convert::TryFrom<zvariant::OwnedValue> for AdvertisementPattern {
    type Error = crate::NiterError;
    fn try_from(value: zvariant::OwnedValue) -> Result<Self, Self::Error> {
        use std::convert::TryInto as _;
        let zstruct: zvariant::Structure = value.try_into()?;
        let (start_position, ad_data_type, contents) = zstruct.try_into()?;
        Ok(Self(start_position, ad_data_type, contents))
    }
}

impl<'a> std::convert::TryFrom<zvariant::Value<'a>> for AdvertisementPattern {
    type Error = crate::NiterError;
    fn try_from(value: zvariant::Value<'a>) -> Result<Self, Self::Error> {
        use std::convert::TryInto as _;
        let zstruct: zvariant::Structure = value
            .downcast()
            .ok_or_else(|| zvariant::Error::IncorrectType)?;
        let (start_position, ad_data_type, contents) = zstruct.try_into()?;
        Ok(Self(start_position, ad_data_type, contents))
    }
}

impl std::convert::TryFrom<zvariant::OwnedValue>
    for crate::ZvariantableArray<AdvertisementPattern>
{
    type Error = crate::NiterError;
    fn try_from(v: zvariant::OwnedValue) -> crate::NiterResult<Self> {
        use std::convert::TryInto as _;
        let zva: zvariant::Array = v.try_into()?;
        let zva_len = zva.len();
        let inner: Vec<AdvertisementPattern> = zva.iter().cloned().try_fold(
            Vec::with_capacity(zva_len),
            |mut acc, item| -> crate::NiterResult<Vec<AdvertisementPattern>> {
                acc.push(item.try_into()?);
                Ok(acc)
            },
        )?;
        Ok(Self(inner))
    }
}

#[derive(Debug, Clone, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
pub struct AdvertisementMonitor {
    object_path: String,
}

#[zbus::dbus_proxy(
    interface = "org.bluez.AdvertisementMonitor1",
    default_service = "org.bluez"
)]
#[derive(Debug, Clone, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
pub trait AdvertisementMonitor {
    fn release(&self) -> zbus::Result<()>;
    fn activate(&self) -> zbus::Result<()>;
    fn device_found(&self, device: Device) -> zbus::Result<()>;
    fn device_lost(&self, device: Device) -> zbus::Result<()>;

    #[dbus_proxy(property, name = "Type")]
    fn r#type(&self) -> zbus::fdo::Result<String>;
    #[dbus_proxy(property)]
    fn rssi_thresholds_and_timers(&self)
        -> zbus::fdo::Result<AdvertisementRssiThresholdsAndTimers>;
    #[dbus_proxy(property)]
    fn patterns(&self) -> zbus::fdo::Result<crate::ZvariantableArray<AdvertisementPattern>>;
}

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
#[strum(serialize_all = "snake_case")]
pub enum MonitorType {
    OrPatterns,
}

crate::impl_tryfrom_zvariant!(MonitorType);

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
pub enum MonitorFeature {
    ControllerPatterns,
}

crate::impl_tryfrom_zvariant!(MonitorFeature);

#[zbus::dbus_proxy(
    interface = "org.bluez.AdvertisementMonitorManager1",
    default_service = "org.bluez"
)]
#[derive(Debug, Clone, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
pub trait AdvertisementMonitorManager {
    fn register_monitor(&self, application: AdvertisementMonitor) -> zbus::Result<()>;
    fn unregister_monitor(&self, application: AdvertisementMonitor) -> zbus::Result<()>;

    #[dbus_proxy(property)]
    fn supported_monitor_types(&self) -> zbus::fdo::Result<crate::ZvariantableArray<MonitorType>>;
    #[dbus_proxy(property)]
    fn supported_features(&self) -> zbus::fdo::Result<crate::ZvariantableArray<MonitorFeature>>;
}
