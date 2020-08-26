bitflags::bitflags! {
    pub struct GattDescriptorMethodsEnabled: u8 {
        const READ = 0b0000_0001;
        const WRITE = 0b0000_0010;
        const RW = Self::READ.bits | Self::WRITE.bits;
    }
}

pub trait GattDescriptorDelegate {
    const UUID: u16;
    const METHODS_ENABLED: GattDescriptorMethodsEnabled;

    fn needs_loop(&self) -> bool {
        !Self::METHODS_ENABLED.is_empty()
    }

    fn initial_value(&self) -> Option<Vec<u8>> {
        None
    }

    fn read(&self) -> crate::NiterResult<Vec<u8>> {
        if Self::METHODS_ENABLED.contains(GattDescriptorMethodsEnabled::READ) {
            return Err(crate::NiterGattError::DelegateNotImplemented(Self::METHODS_ENABLED.bits).into());
        }

        self.read()
    }

    fn write(&mut self, data: Vec<u8>, offset: usize) -> crate::NiterResult<Option<Vec<u8>>> {
        if Self::METHODS_ENABLED.contains(GattDescriptorMethodsEnabled::WRITE) {
            return Err(crate::NiterGattError::DelegateNotImplemented(Self::METHODS_ENABLED.bits).into());
        }

        self.write(data, offset)
    }
}
