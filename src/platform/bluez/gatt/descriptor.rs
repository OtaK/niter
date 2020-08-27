use crate::platform::bluez;
use crate::peripheral::{GattDescriptorFlags, GattDescriptorDelegate};

#[derive(Debug, Clone, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
pub struct GattDescriptorReadOptions {
    offset: u16,
    device: crate::platform::bluez::device::Device,
    link: super::GattLinkType,
}

#[derive(Debug, Clone, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
pub struct GattDescriptorWriteOptions {
    offset: u16,
    device: crate::platform::bluez::device::Device,
    link: super::GattLinkType,
    #[serde(rename = "prepare-authorize")]
    prepare_authorize: bool,
}

#[zbus::dbus_proxy(
    interface = "org.bluez.GattDescriptor1",
    default_service = "org.bluez",
    // default_path = "[variable prefix]/{hci0,hci1,...}/dev_XX_XX_XX_XX_XX_XX/serviceXX/charYYYY/descriptorZZZ"
)]
#[derive(Debug, Clone, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
pub trait GattDescriptor {
    fn read_value(&self, flags: GattDescriptorReadOptions) -> zbus::Result<Vec<u8>>;
    fn write_value(&self, value: Vec<u8>, flags: GattDescriptorWriteOptions) -> zbus::Result<()>;

    #[dbus_proxy(property, name = "UUID")]
    fn uuid(&self) -> zbus::fdo::Result<crate::UuidArray>;
    #[dbus_proxy(property)]
    fn characteristic(&self) -> zbus::fdo::Result<super::GattCharacteristic>;
    #[dbus_proxy(property)]
    fn value(&self) -> zbus::fdo::Result<Vec<u8>>;
    #[dbus_proxy(property)]
    fn flags(&self) -> zbus::fdo::Result<crate::platform::bluez::ZvariantableArray<GattDescriptorFlags>>;
    #[dbus_proxy(property)]
    fn handle(&self) -> zbus::fdo::Result<u16>;
}

#[derive(Debug, Clone, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
pub struct GattDescriptor<D: GattDescriptorDelegate> {
    object_path: String,
    characteristic_object_path: String,
    inner: crate::peripheral::GattDescriptor<D>,
    handle: u16,
}

#[zbus::dbus_interface(name = "org.bluez.GattDescriptor1")]
impl<D: GattDescriptorDelegate> GattDescriptor<D> {
    fn read_value(&self, flags: GattDescriptorReadOptions) -> zbus::fdo::Result<Vec<u8>> {
        let value = self.inner.read().map_err(Into::<zbus::fdo::Error>::into)?;
        Ok(value[flags.offset as usize..].into())
    }

    fn write_value(&mut self, value: Vec<u8>, flags: GattDescriptorWriteOptions) -> zbus::fdo::Result<()> {
        self.inner.write(value, flags.offset as usize).map_err(Into::<zbus::fdo::Error>::into)?;
        Ok(())
    }

    #[dbus_interface(property, name = "UUID")]
    fn uuid(&self) -> zvariant::Value<'_> {
        zvariant::Value::Str(self.inner.uuid().to_hyphenated().to_string().into())
    }

    #[dbus_interface(property)]
    fn characteristic(&self) -> zvariant::ObjectPath {
        zvariant::ObjectPath::from_str_unchecked(&self.characteristic_object_path)
    }

    #[dbus_interface(property)]
    fn value(&self) -> &[u8] {
        self.inner.value()
    }

    #[dbus_interface(property)]
    fn flags(&self) -> bluez::ZvariantableArray<GattDescriptorFlags> {
        self.inner.dbus_descriptor_flags().into()
    }

    #[dbus_interface(property)]
    fn handle(&self) -> u16 {
        self.handle
    }

    #[dbus_interface(property)]
    fn set_handle(&mut self, handle: u16) {
        self.handle = handle;
    }
}
