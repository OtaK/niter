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
pub enum DisplayNumericKind {
    Blink,
    Beep,
    Vibrate,
    OutNumeric,
    Push,
    Twist,
}

crate::impl_tryfrom_zvariant!(DisplayNumericKind);

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
pub enum PromptNumericKind {
    Blink,
    Beep,
    Vibrate,
    InNumeric,
    Push,
    Twist,
}

crate::impl_tryfrom_zvariant!(PromptNumericKind);

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
pub enum PromptStaticKind {
    StaticOob,
    InAlpha,
}

crate::impl_tryfrom_zvariant!(PromptStaticKind);

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
pub enum DisplayCapability {
    Blink,
    Beep,
    Vibrate,
    OutNumeric,
    OutAlpha,
    Push,
    Twist,
    InNumeric,
    InAlpha,
    StaticOob,
    PublicOob,
}

crate::impl_tryfrom_zvariant!(DisplayCapability);

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
pub enum OutOfBandInfoAvailability {
    Other,
    Uri,
    MachineCode2D,
    BarCode,
    Nfc,
    Number,
    String,
    OnBox,
    InBox,
    OnPaper,
    InManual,
    OnDevice,
}

crate::impl_tryfrom_zvariant!(OutOfBandInfoAvailability);

pub trait ProvisioningAgentDelegate: zvariant::Type + 'static {
    fn private_key(&self) -> Vec<u8>;
    fn public_key(&self) -> Vec<u8>;
    fn display_string(&self, value: String);
    fn display_numeric(&self, display_kind: DisplayNumericKind, number: u32);
    fn prompt_numeric(&self, display_kind: PromptNumericKind) -> u32;
    fn prompt_static(&self, display_kind: PromptStaticKind) -> [u8; 16];
    fn cancel(&mut self);
}

#[derive(Debug, Clone, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
pub struct ProvisioningAgent<T: ProvisioningAgentDelegate> {
    capabilities: Vec<DisplayCapability>,
    oob_info: Vec<OutOfBandInfoAvailability>,
    uri: String,
    service_name: String,
    object_path: String,
    delegate: T,
}

#[zbus::dbus_interface(name = "org.bluez.mesh.ProvisionAgent1")]
impl<'a, T: ProvisioningAgentDelegate> ProvisioningAgent<T> {
    fn private_key(&self) -> Vec<u8> {
        self.delegate.private_key()
    }
    fn public_key(&self) -> Vec<u8> {
        self.delegate.public_key()
    }
    fn display_string(&self, value: String) {
        self.delegate.display_string(value)
    }
    fn display_numeric(&self, display_kind: DisplayNumericKind, number: u32) {
        self.delegate.display_numeric(display_kind, number)
    }
    fn prompt_numeric(&self, display_kind: PromptNumericKind) -> u32 {
        self.delegate.prompt_numeric(display_kind)
    }
    fn prompt_static(&self, display_kind: PromptStaticKind) -> [u8; 16] {
        self.delegate.prompt_static(display_kind)
    }
    fn cancel(&mut self) {
        self.delegate.cancel()
    }

    #[dbus_interface(property)]
    fn capabilities(&self) -> zvariant::Value<'_> {
        use std::string::ToString as _;
        use zvariant::Type as _;
        let mut arr: zvariant::Array = zvariant::Array::new(String::signature());
        for item in &self.capabilities {
            let _ = arr.append(zvariant::Value::Str(item.to_string().into()));
        }
        zvariant::Value::Array(arr)
    }

    #[dbus_interface(property)]
    fn out_of_band_info(&self) -> zvariant::Value<'_> {
        use std::string::ToString as _;
        use zvariant::Type as _;
        let mut arr: zvariant::Array = zvariant::Array::new(String::signature());
        for item in &self.oob_info {
            let _ = arr.append(zvariant::Value::Str(item.to_string().into()));
        }
        zvariant::Value::Array(arr)
    }

    #[dbus_interface(property, name = "URI")]
    fn uri(&self) -> &str {
        &self.uri
    }
}
