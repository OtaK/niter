pub trait ObexAgentDelegate: zvariant::Type + serde::Serialize + 'static {
    fn release(&mut self);
    fn authorize_push(&self, transfer: ()) -> String;
    fn cancel(&mut self);
}

#[derive(Debug, Clone, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
pub struct ObexAgent<T: ObexAgentDelegate> {
    object_path: String,
    service_name: String,
    delegate: T,
}

#[zbus::dbus_interface(name = "org.bluez.obex.Agent1")]
impl<T: ObexAgentDelegate> ObexAgent<T> {
    fn release(&mut self) {
        self.delegate.release()
    }
    fn authorize_push(&self, transfer: ()) -> String {
        #[allow(clippy::unit_arg)]
        self.delegate.authorize_push(transfer)
    }
    fn cancel(&mut self) {
        self.delegate.cancel()
    }
}

#[zbus::dbus_proxy(
    interface = "org.bluez.obex.AgentManager1",
    default_service = "org.bluez.obex",
    default_path = "/org/bluez/obex"
)]
pub trait ObexAgentManager {
    fn register_agent(&self, agent: ObexAgent<impl ObexAgentDelegate>) -> zbus::Result<()>;
    fn unregister_agent(&self, agent: ObexAgent<impl ObexAgentDelegate>) -> zbus::Result<()>;
}
