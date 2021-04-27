#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum MeshMessageDestination {
    Unicast(u16),
    VirtualLabel(Vec<u8>),
}

impl zvariant::Type for MeshMessageDestination {
    fn signature() -> zvariant::Signature<'static> {
        zvariant::Value::signature()
    }
}

impl<'a> Into<zvariant::Value<'a>> for MeshMessageDestination {
    fn into(self) -> zvariant::Value<'a> {
        match self {
            Self::Unicast(addr) => zvariant::Value::U16(addr),
            Self::VirtualLabel(addr) => zvariant::Value::Array(addr.into()),
        }
    }
}

#[derive(Debug, Clone, Copy, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
pub struct ModelCaps {
    publish: bool,
    subscribe: bool,
}

impl<'a> Into<zvariant::Value<'a>> for ModelCaps {
    fn into(self) -> zvariant::Value<'a> {
        use zvariant::Type as _;
        let mut zvdict = zvariant::Dict::new(String::signature(), bool::signature());
        let _ = zvdict.add("Publish", self.publish);
        let _ = zvdict.add("Subscribe", self.subscribe);
        zvariant::Value::Dict(zvdict)
    }
}

#[derive(Debug, Clone, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
pub struct ModelConfiguration {
    bindings: Vec<u16>,
    publication_period: u32,
    vendor: u16,
    subscriptions: Vec<MeshMessageDestination>,
}

impl<'a> Into<zvariant::Value<'a>> for ModelConfiguration {
    fn into(self) -> zvariant::Value<'a> {
        use zvariant::Type as _;
        let mut zvdict = zvariant::Dict::new(String::signature(), zvariant::Value::signature());
        let _ = zvdict.add("Bindings", self.bindings);
        let _ = zvdict.add("PublicationPeriod", self.publication_period);
        let _ = zvdict.add("Vendor", self.vendor);
        let _ = zvdict.add("Subscriptions", self.subscriptions);
        zvariant::Value::Dict(zvdict)
    }
}

pub trait MeshElementDelegate: zvariant::Type + 'static {
    fn message_received(
        &mut self,
        source: u16,
        key_index: u16,
        destination: MeshMessageDestination,
        data: Vec<u8>,
    );
    fn dev_key_message_received(
        &mut self,
        source: u16,
        remote: bool,
        net_index: u16,
        data: Vec<u8>,
    );
    fn update_model_configuration(
        &mut self,
        model_id: u16,
        config: std::collections::HashMap<String, zvariant::Value<'_>>,
    );
}

#[derive(Debug, Clone, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
pub struct MeshElement<T: MeshElementDelegate> {
    index: u8,
    models: Vec<(u16, ModelCaps)>,
    vendor_models: Vec<(u16, u16, ModelCaps)>,
    location: u16,
    service_name: String,
    object_path: String,
    delegate: T,
}

#[zbus::dbus_interface(name = "org.bluez.mesh.Element1")]
impl<T: MeshElementDelegate> MeshElement<T> {
    fn message_received(
        &mut self,
        source: u16,
        key_index: u16,
        destination: MeshMessageDestination,
        data: Vec<u8>,
    ) {
        self.delegate
            .message_received(source, key_index, destination, data)
    }
    fn dev_key_message_received(
        &mut self,
        source: u16,
        remote: bool,
        net_index: u16,
        data: Vec<u8>,
    ) {
        self.delegate
            .dev_key_message_received(source, remote, net_index, data)
    }
    fn update_model_configuration(
        &mut self,
        model_id: u16,
        config: std::collections::HashMap<String, zvariant::Value<'_>>,
    ) {
        self.delegate.update_model_configuration(model_id, config)
    }

    #[dbus_interface(property)]
    fn index(&self) -> u8 {
        self.index
    }
    #[dbus_interface(property)]
    fn models(&self) -> zvariant::Value<'_> {
        use zvariant::Type as _;
        let mut zvarray = zvariant::Array::new(<(u16, ModelCaps)>::signature());
        for item in &self.models {
            let _ = zvarray.append((*item).into());
        }
        zvariant::Value::Array(zvarray)
    }
    #[dbus_interface(property)]
    fn vendor_models(&self) -> zvariant::Value<'_> {
        use zvariant::Type as _;
        let mut zvarray = zvariant::Array::new(<(u16, u16, ModelCaps)>::signature());
        for item in &self.vendor_models {
            let _ = zvarray.append((*item).into());
        }
        zvariant::Value::Array(zvarray)
    }
    #[dbus_interface(property)]
    fn location(&self) -> u16 {
        self.location
    }
}
