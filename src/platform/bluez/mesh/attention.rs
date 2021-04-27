pub trait MeshAttentionDelegate: zvariant::Type + 'static {
    fn set_timer(&self, element_index: u8, time: u16);
    fn get_timer(&self, element_index: u8) -> u16;
}

#[allow(dead_code)]
pub struct MeshAttention<T: MeshAttentionDelegate> {
    object_path: String,
    service_name: String,
    delegate: T,
}

#[zbus::dbus_interface(name = "org.bluez.mesh.Attention1")]
impl<T: MeshAttentionDelegate> MeshAttention<T> {
    fn set_timer(&self, element_index: u8, time: u16) {
        self.delegate.set_timer(element_index, time)
    }
    fn get_timer(&self, element_index: u8) -> u16 {
        self.delegate.get_timer(element_index)
    }
}
