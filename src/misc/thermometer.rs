#[derive(Debug, Clone, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
pub struct ThermometerManager {
    object_path: String
}

impl std::str::FromStr for ThermometerManager {
    type Err = crate::NiterError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self { object_path: s.into() })
    }
}

crate::to_proxy_impl!(ThermometerManager, ThermometerManagerProxy, "org.bluez");
crate::impl_tryfrom_zvariant!(ThermometerManager);

#[zbus::dbus_proxy(
    interface = "org.bluez.ThermometerManager1",
    default_service = "org.bluez"
)]
pub trait ThermometerManager {
    fn register_watcher(&self, agent: ThermometerWatcher) -> zbus::Result<()>;
    fn unregister_watcher(&self, agent: ThermometerWatcher) -> zbus::Result<()>;
    fn enable_intermediate_measurement(&self, agent: ThermometerWatcher) -> zbus::Result<()>;
    fn disable_intermediate_measurement(&self, agent: ThermometerWatcher) -> zbus::Result<()>;
}

#[derive(Debug, Clone, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
pub struct Thermometer {
    object_path: String
}

impl std::str::FromStr for Thermometer {
    type Err = crate::NiterError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self { object_path: s.into() })
    }
}

crate::to_proxy_impl!(Thermometer, ThermometerProxy, "org.bluez");
crate::impl_tryfrom_zvariant!(Thermometer);

#[zbus::dbus_proxy(
    interface = "org.bluez.Thermometer1",
    default_service = "org.bluez"
)]
pub trait Thermometer {
    #[dbus_proxy(property)]
    fn intermediate(&self) -> zbus::fdo::Result<bool>;
    #[dbus_proxy(property)]
    fn interval(&self) -> zbus::fdo::Result<u16>;
    #[dbus_proxy(property)]
    fn set_interval(&self, interval_seconds: u16) -> zbus::fdo::Result<()>;
    #[dbus_proxy(property)]
    fn maximum(&self) -> zbus::fdo::Result<u16>;
    #[dbus_proxy(property)]
    fn minimum(&self) -> zbus::fdo::Result<u16>;
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ThermometerWatcher {
    object_path: String,
    current_measurement: Option<ThermometerMeasurement>,
}

impl zvariant::Type for ThermometerWatcher {
    fn signature() -> zvariant::Signature<'static> {
        zvariant::Signature::from_str_unchecked("s")
    }
}

#[zbus::dbus_interface(name = "org.bluez.ThermometerWatcher1")]
impl ThermometerWatcher {
    fn measurement_received(&mut self, measurement: ThermometerMeasurement) -> zbus::fdo::Result<()> {
        self.current_measurement = Some(measurement);
        Ok(())
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ThermometerMeasurement {
    exponent: i16,
    mantissa: i32,
    unit: ThermometerMeasurementUnit,
    time: Option<u64>,
    r#type: Option<ThermometerMeasurementType>,
    measurement: ThermometerMeasurementKind,
}

impl zvariant::Type for ThermometerMeasurement {
    fn signature() -> zvariant::Signature<'static> {
        zvariant::Signature::from_str_unchecked("a{sv}")
    }
}

impl std::convert::TryFrom<zvariant::Dict<'_, '_>> for ThermometerMeasurement {
    type Error = crate::NiterError;
    fn try_from(dict: zvariant::Dict<'_, '_>) -> crate::NiterResult<Self> {
        use std::str::FromStr as _;
        let exponent = dict.get("Exponent")?.ok_or_else(|| zvariant::Error::IncorrectType)?;
        let mantissa = dict.get("Mantissa")?.ok_or_else(|| zvariant::Error::IncorrectType)?;
        let unit: &str = dict.get("Unit")?.ok_or_else(|| zvariant::Error::IncorrectType)?;
        let time = dict.get("Time")?;
        let measurement_type: Option<&str> = dict.get("Type")?;
        let measurement_kind: &str = dict.get("Measurement")?.ok_or_else(|| zvariant::Error::IncorrectType)?;

        Ok(Self {
            exponent: *exponent,
            mantissa: *mantissa,
            unit: ThermometerMeasurementUnit::from_str(&unit)?,
            time: time.map(|t| *t),
            r#type: measurement_type.and_then(|s| ThermometerMeasurementType::from_str(s).ok()),
            measurement: ThermometerMeasurementKind::from_str(measurement_kind)?,
        })
    }
}

impl std::convert::TryFrom<zvariant::OwnedValue> for ThermometerMeasurement {
    type Error = crate::NiterError;
    fn try_from(v: zvariant::OwnedValue) -> Result<Self, Self::Error> {
        use std::convert::TryInto as _;
        let dict: zvariant::Dict = v.try_into()?;
        Self::try_from(dict)
    }
}

#[derive(Debug, Clone, Copy, strum::EnumString, strum::Display, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
#[strum(serialize_all = "lowercase")]
pub enum ThermometerMeasurementUnit {
    Celsius,
    Farenheit,
}

crate::impl_tryfrom_zvariant!(ThermometerMeasurementUnit);

#[derive(Debug, Clone, Copy, strum::EnumString, strum::Display, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
#[strum(serialize_all = "lowercase")]
pub enum ThermometerMeasurementType {
    Armpit,
    Body,
    Ear,
    Finger,
    Intestines,
    Mouth,
    Rectum,
    Toe,
    Tympanum,
}

crate::impl_tryfrom_zvariant!(ThermometerMeasurementType);

#[derive(Debug, Clone, Copy, strum::EnumString, strum::Display, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
#[strum(serialize_all = "lowercase")]
pub enum ThermometerMeasurementKind {
    Final,
    Intermediate,
}

crate::impl_tryfrom_zvariant!(ThermometerMeasurementKind);
