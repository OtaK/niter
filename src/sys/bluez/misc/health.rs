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
pub enum HealthApplicationRole {
    Source,
    Sink,
}

crate::impl_tryfrom_zvariant!(HealthApplicationRole);

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HealthApplicationConfiguration {
    data_type: u16,
    role: HealthApplicationRole,
    description: Option<String>,
    channel_type: Option<HealthChannelType>,
}

impl zvariant::Type for HealthApplicationConfiguration {
    fn signature() -> zvariant::Signature<'static> {
        zvariant::Signature::from_str_unchecked("a{sv}")
    }
}

impl std::convert::TryInto<zvariant::Dict<'_, '_>> for HealthApplicationConfiguration {
    type Error = crate::NiterError;
    fn try_into(mut self) -> crate::NiterResult<zvariant::Dict<'static, 'static>> {
        use std::string::ToString as _;
        use zvariant::Type as _;
        let mut dict = zvariant::Dict::new(String::signature(), zvariant::Value::signature());
        dict.add("DataType", self.data_type)?;
        dict.add("Role", zvariant::Value::Str(self.role.to_string().into()))?;
        if let Some(description) = self.description.take() {
            dict.add("Description", zvariant::Value::Str(description.into()))?;
        }

        if let Some(channel_type) = self.channel_type.take() {
            dict.add(
                "ChannelType",
                zvariant::Value::Str(channel_type.to_string().into()),
            )?;
        }

        Ok(dict)
    }
}

impl std::convert::TryInto<zvariant::OwnedValue> for HealthApplicationConfiguration {
    type Error = crate::NiterError;
    fn try_into(self) -> Result<zvariant::OwnedValue, Self::Error> {
        let dict: zvariant::Dict = self.try_into()?;
        Ok(zvariant::Value::Dict(dict).into())
    }
}

#[derive(Debug, Clone, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
pub struct HealthApplication {
    object_path: String,
}

impl std::str::FromStr for HealthApplication {
    type Err = crate::NiterError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            object_path: s.into(),
        })
    }
}

crate::to_proxy_impl!(HealthApplication, HealthManagerProxy, "org.bluez");
crate::impl_tryfrom_zvariant!(HealthApplication);

#[zbus::dbus_proxy(
    interface = "org.bluez.HealthManager",
    default_service = "org.bluez",
    default_path = "/org/bluez"
)]
pub trait HealthManager {
    fn create_application(
        &self,
        config: HealthApplicationConfiguration,
    ) -> zbus::Result<HealthApplication>;
    fn destroy_application(&self, application: HealthApplication) -> zbus::Result<()>;
}

#[derive(Debug, Clone, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
pub struct HealthDevice {
    object_path: String,
}

impl std::str::FromStr for HealthDevice {
    type Err = crate::NiterError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            object_path: s.into(),
        })
    }
}

crate::to_proxy_impl!(HealthDevice, HealthDeviceProxy, "org.bluez");
crate::impl_tryfrom_zvariant!(HealthDevice);

#[zbus::dbus_proxy(interface = "org.bluez.HealthDevice1", default_service = "org.bluez")]
pub trait HealthDevice {
    fn echo(&self) -> zbus::Result<bool>;
    fn create_channel(&self) -> zbus::Result<HealthChannel>;
    fn destroy_channel(&self, channel: HealthChannel) -> zbus::Result<()>;

    #[dbus_proxy(signal)]
    fn channel_connected(&self, channel: HealthChannel) -> zbus::fdo::Result<()>;
    #[dbus_proxy(signal)]
    fn channel_deleted(&self, channel: HealthChannel) -> zbus::fdo::Result<()>;
    #[dbus_proxy(property)]
    fn main_channel(&self) -> zbus::fdo::Result<HealthChannel>;
}

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
pub enum HealthChannelType {
    Reliable,
    Streaming,
}

crate::impl_tryfrom_zvariant!(HealthChannelType);

#[derive(Debug, Clone, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
pub struct HealthChannel {
    object_path: String,
}

impl std::str::FromStr for HealthChannel {
    type Err = crate::NiterError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            object_path: s.into(),
        })
    }
}

crate::to_proxy_impl!(HealthChannel, HealthChannelProxy, "org.bluez");
crate::impl_tryfrom_zvariant!(HealthChannel);

#[zbus::dbus_proxy(interface = "org.bluez.HealthChannel", default_service = "org.bluez")]
pub trait HealthChannel {
    fn acquire(&self) -> zbus::Result<std::os::unix::io::RawFd>;
    fn release(&self) -> zbus::Result<()>;

    #[dbus_proxy(property, name = "Type")]
    fn r#type(&self) -> zbus::fdo::Result<HealthChannelType>;
    #[dbus_proxy(property)]
    fn device(&self) -> zbus::fdo::Result<HealthDevice>;
    #[dbus_proxy(property)]
    fn application(&self) -> zbus::fdo::Result<HealthApplication>;
}
