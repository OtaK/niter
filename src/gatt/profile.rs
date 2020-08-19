pub trait GattProfileImpl: zvariant::Type {
    fn release(&mut self);
}

#[derive(Debug, Clone, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
pub struct GattProfile<T: GattProfileImpl> {
    uuids: crate::UuidArray,
    profile_impl: T,
}

#[zbus::dbus_interface(name = "org.bluez.GattProfile1")]
impl<T: GattProfileImpl> GattProfile<T> {
    fn release(&mut self) -> zbus::fdo::Result<()> {
        // Letting the profile impl know that cleanup has started and do its thing
        self.profile_impl.release();
        // We're dropping self since that's a profile release signal
        // Side effect is that it'll drop the inner profile_impl
        Ok(())
    }

    #[dbus_interface(property, name = "UUIDs")]
    fn uuids(&self) -> crate::UuidArray {
        self.uuids.clone()
    }
}
