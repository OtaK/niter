use crate::media::transport::{MediaTransport, MediaTransportConfiguration};

pub type MediaEndpointCapabilities = Vec<u8>;

#[derive(Debug, Clone, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
pub struct MediaEndpointProperties {
    capabilities: MediaEndpointCapabilities
}

pub trait MediaEndpointDelegate<E: std::error::Error>: zvariant::Type {
    fn set_configuration(&mut self, transport: MediaTransport, properties: MediaEndpointProperties) -> Result<(), E>;
    fn select_configuration(&mut self, capabilities: MediaEndpointCapabilities) -> Result<MediaTransportConfiguration, E>;
    fn clear_configuration(&mut self, transport: MediaTransport) -> Result<(), E>;
    fn release(&mut self) -> Result<(), E>;

    fn uuid(&self) -> &str;
    fn codec(&self) -> u8;
    fn capabilities(&self) -> MediaEndpointCapabilities;
    fn device(&self) -> zvariant::Value;
}

#[derive(Debug, Clone, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
pub struct MediaEndpointServer<T: MediaEndpointDelegate<zbus::fdo::Error>> {
    object_path: String,
    delegate: T,
}

#[zbus::dbus_interface(name = "org.bluez.MediaEndpoint1")]
impl<T: MediaEndpointDelegate<zbus::fdo::Error>> MediaEndpointServer<T> {
    fn set_configuration(&mut self, transport: MediaTransport, properties: MediaEndpointProperties) -> zbus::fdo::Result<()> {
        self.delegate.set_configuration(transport, properties)
    }
    fn select_configuration(&mut self, capabilities: MediaEndpointCapabilities) -> zbus::fdo::Result<MediaTransportConfiguration> {
        self.delegate.select_configuration(capabilities)
    }
    fn clear_configuration(&mut self, transport: MediaTransport) -> zbus::fdo::Result<()> {
        self.delegate.clear_configuration(transport)
    }
    fn release(&mut self) -> zbus::fdo::Result<()> {
        self.delegate.release()
    }

    #[dbus_interface(property, name = "UUID")]
    fn uuid(&self) -> &str {
        self.delegate.uuid()
    }

    #[dbus_interface(property)]
    fn codec(&self) -> u8 {
        self.delegate.codec()
    }

    #[dbus_interface(property)]
    fn capabilities(&self) -> MediaEndpointCapabilities {
        self.delegate.capabilities()
    }

    #[dbus_interface(property)]
    fn device(&self) -> zvariant::Value {
        self.delegate.device()
    }
}

#[zbus::dbus_proxy(
    interface = "org.bluez.MediaEndpoint1",
    default_service = "org.bluez",
    //default_path = "[variable prefix]/{hci0,hci1,...}/dev_XX_XX_XX_XX_XX_XX/sepX"
)]
pub trait MediaEndpointClient {
    fn set_configuration(&self, transport: MediaTransport, properties: MediaEndpointProperties) -> zbus::Result<()>;
    fn select_configuration(&self, capabilities: MediaEndpointCapabilities) -> zbus::Result<MediaTransportConfiguration>;
    fn clear_configuration(&self, transport: MediaTransport) -> zbus::Result<()>;
    fn release(&self) -> zbus::Result<()>;

    #[dbus_proxy(property, name = "UUID")]
    fn uuid(&self) -> zbus::fdo::Result<String>;
    #[dbus_proxy(property)]
    fn codec(&self) -> zbus::fdo::Result<u8>;
    #[dbus_proxy(property)]
    fn capabilities(&self) -> zbus::fdo::Result<MediaEndpointCapabilities>;
    #[dbus_proxy(property)]
    fn device(&self) -> zbus::fdo::Result<crate::device::Device>;
}
