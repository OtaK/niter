use crate::platform::bluez;
use crate::spec::ble_appearance::BLEAppearance;

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
pub enum AdvertisementType {
    Broadcast,
    Peripheral,
}

crate::impl_tryfrom_zvariant!(AdvertisementType);
crate::impl_enumto_zstruct!(AdvertisementType);

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
pub enum SecondaryChannel {
    #[strum(serialize = "1M")]
    OneM,
    #[strum(serialize = "2M")]
    TwoM,
    Coded,
}

crate::impl_tryfrom_zvariant!(SecondaryChannel);
crate::impl_enumto_zstruct!(SecondaryChannel);

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
pub enum SystemInclude {
    TxPower,
    Appearance,
    LocalName,
}

crate::impl_tryfrom_zvariant!(SystemInclude);
crate::impl_enumto_zstruct!(SystemInclude);

#[derive(Debug, Clone, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
pub struct Advertisement {
    #[serde(skip)]
    object_path: String,
    r#type: AdvertisementType,
    service_uuids: Vec<crate::Uuid>,
    manufacturer_data: bluez::ManufacturerData,
    solicit_uuids: Vec<crate::Uuid>,
    service_data: bluez::ServiceData,
    data: bluez::AdvertisingData,
    discoverable: bool,
    discoverable_timeout: u16,
    includes: Vec<SystemInclude>,
    local_name: String,
    appearance: BLEAppearance,
    duration: u16,
    timeout: u16,
    secondary_channel: SecondaryChannel,
}

//#[zbus::dbus_interface(name = "org.bluez.LEAdvertisement1")]
#[allow(dead_code)]
impl Advertisement {
    fn release(&self) -> zbus::fdo::Result<()> {
        Ok(())
    }

    fn advertisement_type(&self) -> AdvertisementType {
        self.r#type
    }
    fn service_uuids(&self) -> &Vec<crate::Uuid> {
        &self.service_uuids
    }
    fn manufacturer_data(&self) -> zvariant::Dict<'_, '_> {
        use zvariant::Type as _;
        let mut dict = zvariant::Dict::new(u16::signature(), <Vec<u8>>::signature());
        for (k, v) in self.manufacturer_data.iter() {
            let _ = dict.append(zvariant::Value::U16(*k), zvariant::Value::Array(v.into()));
        }
        dict
    }
    fn solicit_uuids(&self) -> &Vec<crate::Uuid> {
        &self.solicit_uuids
    }
    fn service_data(&self) -> zvariant::Dict<'_, '_> {
        use zvariant::Type as _;
        let mut dict = zvariant::Dict::new(String::signature(), <Vec<u8>>::signature());
        for (k, v) in self.service_data.iter() {
            let _ = dict.append(
                zvariant::Value::Str(k.clone().into()),
                zvariant::Value::Array(v.into()),
            );
        }
        dict
    }
    fn data(&self) -> zvariant::Dict<'_, '_> {
        use zvariant::Type as _;
        let mut dict = zvariant::Dict::new(u8::signature(), <Vec<u8>>::signature());
        for (k, v) in self.data.iter() {
            let _ = dict.append(zvariant::Value::U8(*k), zvariant::Value::Array(v.into()));
        }
        dict
    }
    fn discoverable(&self) -> bool {
        self.discoverable
    }
    fn discoverable_timeout(&self) -> u16 {
        self.discoverable_timeout
    }
    fn includes(&self) -> &Vec<SystemInclude> {
        &self.includes
    }
    fn local_name(&self) -> &str {
        &self.local_name
    }
    fn appearance(&self) -> BLEAppearance {
        self.appearance
    }
    fn duration(&self) -> u16 {
        self.duration
    }
    fn timeout(&self) -> u16 {
        self.timeout
    }
    fn secondary_channel(&self) -> SecondaryChannel {
        self.secondary_channel
    }
}

impl zbus::Interface for Advertisement {
    fn name() -> &'static str {
        "org.bluez.LEAdvertisement1"
    }
    fn get(&self, property_name: &str) -> Option<zbus::fdo::Result<zvariant::OwnedValue>> {
        match property_name {
            "Type" => Some(Ok(zvariant::Value::from(self.advertisement_type()).into())),
            "ServiceUUIDs" => Some(Ok(zvariant::Value::from(crate::UuidArray::from(
                self.service_uuids.clone(),
            ))
            .into())),
            "ManufacturerData" => Some(Ok(zvariant::Value::from(self.manufacturer_data()).into())),
            "SolicitUUIDs" => Some(Ok(zvariant::Value::from(crate::UuidArray::from(
                self.solicit_uuids.clone(),
            ))
            .into())),
            "ServiceData" => Some(Ok(zvariant::Value::from(self.service_data()).into())),
            "Data" => Some(Ok(zvariant::Value::from(self.data()).into())),
            "Discoverable" => Some(Ok(zvariant::Value::from(self.discoverable()).into())),
            "DiscoverableTimeout" => {
                Some(Ok(zvariant::Value::from(self.discoverable_timeout()).into()))
            }
            "Includes" => Some(Ok(zvariant::Value::from(bluez::ZvariantableArray::from(
                self.includes.clone(),
            ))
            .into())),
            "LocalName" => Some(Ok(zvariant::Value::from(self.local_name()).into())),
            "Appearance" => Some(Ok(zvariant::Value::from(self.appearance()).into())),
            "Duration" => Some(Ok(zvariant::Value::from(self.duration()).into())),
            "Timeout" => Some(Ok(zvariant::Value::from(self.timeout()).into())),
            "SecondaryChannel" => Some(Ok(zvariant::Value::from(self.secondary_channel()).into())),
            _ => None,
        }
    }
    fn get_all(&self) -> std::collections::HashMap<String, zvariant::OwnedValue> {
        let mut props: std::collections::HashMap<String, zvariant::OwnedValue> =
            std::collections::HashMap::new();
        props.insert(
            "Type".to_string(),
            zvariant::Value::from(self.advertisement_type()).into(),
        );
        props.insert(
            "ServiceUUIDs".to_string(),
            zvariant::Value::from(crate::UuidArray::from(self.service_uuids.clone())).into(),
        );
        props.insert(
            "ManufacturerData".to_string(),
            zvariant::Value::from(self.manufacturer_data()).into(),
        );
        props.insert(
            "SolicitUUIDs".to_string(),
            zvariant::Value::from(crate::UuidArray::from(self.solicit_uuids.clone())).into(),
        );
        props.insert(
            "ServiceData".to_string(),
            zvariant::Value::from(self.service_data()).into(),
        );
        props.insert(
            "Data".to_string(),
            zvariant::Value::from(self.data()).into(),
        );
        props.insert(
            "Discoverable".to_string(),
            zvariant::Value::from(self.discoverable()).into(),
        );
        props.insert(
            "DiscoverableTimeout".to_string(),
            zvariant::Value::from(self.discoverable_timeout()).into(),
        );
        props.insert(
            "Includes".to_string(),
            zvariant::Value::from(bluez::ZvariantableArray::from(self.includes.clone())).into(),
        );
        props.insert(
            "LocalName".to_string(),
            zvariant::Value::from(self.local_name()).into(),
        );
        props.insert(
            "Appearance".to_string(),
            zvariant::Value::from(self.appearance()).into(),
        );
        props.insert(
            "Duration".to_string(),
            zvariant::Value::from(self.duration()).into(),
        );
        props.insert(
            "Timeout".to_string(),
            zvariant::Value::from(self.timeout()).into(),
        );
        props.insert(
            "SecondaryChannel".to_string(),
            zvariant::Value::from(self.secondary_channel()).into(),
        );
        props
    }
    fn set(
        &mut self,
        _property_name: &str,
        _value: &zvariant::Value,
    ) -> Option<zbus::fdo::Result<()>> {
        None
    }
    fn call(
        &self,
        c: &zbus::Connection,
        m: &zbus::Message,
        name: &str,
    ) -> std::option::Option<zbus::Result<u32>> {
        match name {
            "Release" => {
                let reply = self.release();
                Some(match &reply {
                    Ok(r) => c.reply(m, r),
                    Err(e) => e.reply(c, m),
                })
            }
            _ => None,
        }
    }
    fn call_mut(
        &mut self,
        _c: &zbus::Connection,
        _m: &zbus::Message,
        _name: &str,
    ) -> std::option::Option<zbus::Result<u32>> {
        None
    }

    fn introspect_to_writer(&self, writer: &mut dyn std::fmt::Write, level: usize) {
        writeln!(
            writer,
            r#"{:indent$}<interface name="{}">"#,
            "",
            Self::name(),
            indent = level
        )
        .unwrap();
        {
            use zvariant::Type as _;
            let level = level + 2;
            writeln!(
                writer,
                "{:indent$}<method name=\"Release\">",
                indent = level
            )
            .unwrap();
            writeln!(writer, "{:indent$}</method>", indent = level).unwrap();
            writeln!(
                writer,
                "{:indent$}<property name=\"Appearance\" type=\"{}\" access=\"read\"/>",
                BLEAppearance::signature(),
                indent = level
            )
            .unwrap();
            writeln!(
                writer,
                "{:indent$}<property name=\"Duration\" type=\"{}\" access=\"read\"/>",
                u16::signature(),
                indent = level
            )
            .unwrap();
            writeln!(
                writer,
                "{:indent$}<property name=\"Timeout\" type=\"{}\" access=\"read\"/>",
                u16::signature(),
                indent = level
            )
            .unwrap();
            writeln!(
                writer,
                "{:indent$}<property name=\"SecondaryChannel\" type=\"{}\" access=\"read\"/>",
                SecondaryChannel::signature(),
                indent = level
            )
            .unwrap();
            writeln!(
                writer,
                "{:indent$}<property name=\"ManufacturerData\" type=\"{}\" access=\"read\"/>",
                self.manufacturer_data().signature(),
                indent = level
            )
            .unwrap();
            writeln!(
                writer,
                "{:indent$}<property name=\"DiscoverableTimeout\" type=\"{}\" access=\"read\"/>",
                u16::signature(),
                indent = level
            )
            .unwrap();
            writeln!(
                writer,
                "{:indent$}<property name=\"ServiceUUIDs\" type=\"{}\" access=\"read\"/>",
                crate::UuidArray::signature(),
                indent = level
            )
            .unwrap();
            writeln!(
                writer,
                "{:indent$}<property name=\"Data\" type=\"{}\" access=\"read\"/>",
                self.data().signature(),
                indent = level
            )
            .unwrap();
            writeln!(
                writer,
                "{:indent$}<property name=\"Type\" type=\"{}\" access=\"read\"/>",
                AdvertisementType::signature(),
                indent = level
            )
            .unwrap();
            writeln!(
                writer,
                "{:indent$}<property name=\"SolicitUUIDs\" type=\"{}\" access=\"read\"/>",
                crate::UuidArray::signature(),
                indent = level
            )
            .unwrap();
            writeln!(
                writer,
                "{:indent$}<property name=\"ServiceData\" type=\"{}\" access=\"read\"/>",
                self.service_data().signature(),
                indent = level
            )
            .unwrap();
            writeln!(
                writer,
                "{:indent$}<property name=\"Discoverable\" type=\"{}\" access=\"read\"/>",
                bool::signature(),
                indent = level
            )
            .unwrap();
            writeln!(
                writer,
                "{:indent$}<property name=\"Includes\" type=\"{}\" access=\"read\"/>",
                <bluez::ZvariantableArray<SystemInclude>>::signature(),
                indent = level
            )
            .unwrap();
            writeln!(
                writer,
                "{:indent$}<property name=\"LocalName\" type=\"{}\" access=\"read\"/>",
                <&str>::signature(),
                indent = level
            )
            .unwrap();
        }
        writeln!(writer, r#"{:indent$}</interface>"#, "", indent = level).unwrap();
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, zvariant_derive::Type)]
pub struct AdvertisingManager {
    object_path: String,
}

#[zbus::dbus_proxy(
    interface = "org.bluez.LEAdvertisingManager1",
    default_service = "org.bluez",
    default_path = "/org/bluez/hci0"
)]
#[derive(Debug, Clone, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
pub trait AdvertisingManager {
    fn register_advertisement(
        &self,
        advertisement: Advertisement,
        options: std::collections::HashMap<String, String>,
    ) -> zbus::Result<()>;
    fn unregister_advertisement(&self, advertisement: Advertisement) -> zbus::Result<()>;

    #[dbus_proxy(property)]
    fn active_instances(&self) -> zbus::fdo::Result<u8>;
    #[dbus_proxy(property)]
    fn supported_instances(&self) -> zbus::fdo::Result<u8>;
    #[dbus_proxy(property)]
    fn supported_includes(&self) -> zbus::fdo::Result<bluez::ZvariantableArray<SystemInclude>>;
    #[dbus_proxy(property)]
    fn supported_secondary_channels(
        &self,
    ) -> zbus::fdo::Result<bluez::ZvariantableArray<SecondaryChannel>>;
}
