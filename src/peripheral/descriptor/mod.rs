mod descriptor_delegate;
pub use self::descriptor_delegate::*;

// use GattDescriptorDelegate as _;

#[derive(
    Debug,
    Clone,
    Copy,
    strum::Display,
    strum::EnumString
)]
#[cfg_attr(target_os = "linux", derive(
    zvariant_derive::Type,
    serde::Serialize,
    serde::Deserialize,
))]
#[strum(serialize_all = "kebab-case")]
pub enum GattDescriptorFlags {
    Read,
    Write,
    EncryptRead,
    EncryptWrite,
    EncryptAuthenticatedRead,
    EncryptAuthenticatedWrite,
    SecureRead,
    SecureWrite,
    Authorize,
}

#[cfg(target_os = "linux")]
crate::impl_tryfrom_zvariant!(GattDescriptorFlags);
#[cfg(target_os = "linux")]
crate::impl_enumto_zstruct!(GattDescriptorFlags);

#[derive(Debug, Clone)]
#[cfg_attr(target_os = "linux", derive(zvariant_derive::Type, serde::Serialize, serde::Deserialize))]
pub struct GattDescriptor<T: GattDescriptorDelegate> {
    delegate: T,
    uuid: crate::Uuid,
}

impl<T: GattDescriptorDelegate> std::ops::Deref for GattDescriptor<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.delegate
    }
}

impl<T: GattDescriptorDelegate> std::ops::DerefMut for GattDescriptor<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.delegate
    }
}

#[allow(dead_code)]
impl<T: GattDescriptorDelegate> GattDescriptor<T> {
    pub fn delegate(&self) -> &T {
        &self.delegate
    }

    pub fn uuid(&self) -> &crate::Uuid {
        &self.uuid
    }

    pub(crate) fn register(delegate: T) -> crate::NiterResult<async_std::task::JoinHandle<Self>> {
        use crate::uuid::SdpShortUUID as _;
        let ret = Self {
            uuid: crate::Uuid::from(uuid::Uuid::from_sdp_short_uuid(T::UUID)?),
            delegate,
        };
        Ok(async_std::task::spawn(ret.run()))
    }

    async fn run(self) -> Self {
        let _flags = T::METHODS_ENABLED;
        let _uuid = self.uuid.clone();
        let _needs_loop = self.delegate.needs_loop();

        // let mut maybe_rx = if needs_loop {
        //     None
        //     // TODO:
        // } else {
        //     None
        // };
        // TODO:
        self
    }
}
