use crate::error::*;

bitflags::bitflags! {
    pub struct GattCharacteristicMethodsEnabled: u8 {
        const READ = 0b0000_0001;
        const WRITE = 0b0000_0010;
        const NOTIFY = 0b0000_0100;
        const INDICATE = 0b0000_1000;
        const RW = Self::READ.bits | Self::WRITE.bits;
        const RN = Self::READ.bits | Self::NOTIFY.bits;
        const RI = Self::READ.bits | Self::INDICATE.bits;
        const NI = Self::NOTIFY.bits | Self::INDICATE.bits;
        const RWN = Self::READ.bits | Self::WRITE.bits | Self::NOTIFY.bits;
        const RWNI = Self::RW.bits | Self::NI.bits;
    }
}

pub trait GattCharacteristicDelegate: Send + Sync + Sized + 'static {
    const UUID: u16;
    const METHODS_ENABLED: GattCharacteristicMethodsEnabled;
    const SECURE_READ: bool = true;
    const WRITE_WITH_RESPONSE: bool = false;
    const WRITE_WITH_RESPONSE_SECURE: bool = true;
    const NOTIFY_POLL_INTERVAL_MS: u64 = 1000;

    // fn descriptors(&mut self) -> std::collections::HashSet<Descriptor> {
    //     std::collections::HashSet::new()
    // }

    fn needs_loop(&self) -> bool {
        !Self::METHODS_ENABLED.is_empty()
    }

    fn initial_value(&self) -> Option<Vec<u8>> {
        None
    }

    fn read_fn(&self) -> NiterGattResult<Vec<u8>> {
        if Self::METHODS_ENABLED.contains(GattCharacteristicMethodsEnabled::READ) {
            return Err(NiterGattError::DelegateNotImplemented(Self::METHODS_ENABLED.bits));
        }

        self.read_fn()
    }

    fn write_fn(&mut self, data: Vec<u8>, offset: usize) -> NiterGattResult<Option<Vec<u8>>> {
        if Self::METHODS_ENABLED.contains(GattCharacteristicMethodsEnabled::WRITE) {
            return Err(NiterGattError::DelegateNotImplemented(Self::METHODS_ENABLED.bits));
        }

        self.write_fn(data, offset)
    }

    fn notify_fn(&self) -> NiterGattResult<Vec<u8>> {
        if Self::METHODS_ENABLED.contains(GattCharacteristicMethodsEnabled::NOTIFY) {
            return Err(NiterGattError::DelegateNotImplemented(Self::METHODS_ENABLED.bits));
        }

        self.notify_fn()
    }

    fn indicate_fn(&self) -> NiterGattResult<Vec<u8>> {
        if Self::METHODS_ENABLED.contains(GattCharacteristicMethodsEnabled::INDICATE) {
            return Err(NiterGattError::DelegateNotImplemented(Self::METHODS_ENABLED.bits));
        }

        self.indicate_fn()
    }
}
