mod descriptor_delegate;
pub use self::descriptor_delegate::*;

use GattDescriptorDelegate as _;

#[derive(Debug)]
pub struct GattDescriptor<T: 'static + GattDescriptorDelegate + Sync + Send> {
    delegate: Option<T>,
    uuid: crate::Uuid,
    value: Option<Vec<u8>>,
}

#[allow(dead_code)]
impl<T: 'static + GattDescriptorDelegate + Sync + Send> GattDescriptor<T> {
    pub fn delegate(&self) -> Option<&T> {
        self.delegate.as_ref()
    }

    pub(crate) fn register(delegate: T) -> crate::NiterResult<async_std::task::JoinHandle<Self>> {
        use crate::uuid::SdpShortUUID as _;
        let ret = Self {
            uuid: crate::Uuid::from(uuid::Uuid::from_sdp_short_uuid(T::UUID)?),
            value: delegate.initial_value(),
            delegate: Some(delegate),
        };
        Ok(async_std::task::spawn(ret.run()))
    }

    async fn run(self) -> Self {
        let _flags = T::METHODS_ENABLED;
        let _uuid = self.uuid.clone();
        let needs_loop = self.delegate.as_ref().map(|d| d.needs_loop()).unwrap_or_default();

        let mut maybe_rx = if needs_loop {
            None
            // TODO:
        } else {
            None
        };
        // TODO:
        self
    }
}
