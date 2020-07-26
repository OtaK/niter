#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Adapter {

}

#[zbus::dbus_interface(name = "org.bluez.Adapter1")]
impl Adapter {
    #[zbus::dbus_interface(property, name = "Address")]
    fn address(&self) -> &str;
}
