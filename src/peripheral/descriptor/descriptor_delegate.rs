use crate::error::*;

bitflags::bitflags! {
    pub struct GattDescriptorMethodsEnabled: u8 {
        const READ = 0b0000_0001;
        const WRITE = 0b0000_0010;
        const RW = Self::READ.bits | Self::WRITE.bits;
    }
}

pub trait GattDescriptorDelegateInner: Send + Sync + 'static {
    const UUID: u16;
    const METHODS_ENABLED: GattDescriptorMethodsEnabled;
    const ENCRYPT_READ: bool = false;
    const ENCRYPT_AUTH_READ: bool = false;
    const ENCRYPT_WRITE: bool = false;
    const ENCRYPT_AUTH_WRITE: bool = false;
    const AUTHORIZE: bool = false;

    fn needs_loop(&self) -> bool {
        !Self::METHODS_ENABLED.is_empty()
    }

    fn value(&self) -> &[u8];

    fn read(&self) -> NiterGattResult<Vec<u8>> {
        if Self::METHODS_ENABLED.contains(GattDescriptorMethodsEnabled::READ) {
            return Err(NiterGattError::DelegateNotImplemented(Self::METHODS_ENABLED.bits));
        }

        self.read()
    }

    fn write(&mut self, data: Vec<u8>, offset: usize) -> NiterGattResult<()> {
        if Self::METHODS_ENABLED.contains(GattDescriptorMethodsEnabled::WRITE) {
            return Err(NiterGattError::DelegateNotImplemented(Self::METHODS_ENABLED.bits).into());
        }

        self.write(data, offset)
    }
}

#[cfg(target_os = "linux")]
pub trait GattDescriptorDelegateInnerDbusExt: GattDescriptorDelegateInner {
    #[cfg(target_os = "linux")]
    fn dbus_descriptor_flags(&self) -> Vec<super::GattDescriptorFlags> {
        let mut ret = vec![];
        if Self::METHODS_ENABLED.contains(GattDescriptorMethodsEnabled::READ) {
            ret.push(super::GattDescriptorFlags::Read);
        }
        if Self::METHODS_ENABLED.contains(GattDescriptorMethodsEnabled::WRITE) {
            ret.push(super::GattDescriptorFlags::Write);
        }
        if Self::ENCRYPT_READ {
            ret.push(super::GattDescriptorFlags::EncryptRead);
        }
        if Self::ENCRYPT_WRITE {
            ret.push(super::GattDescriptorFlags::EncryptWrite);
        }
        if Self::ENCRYPT_AUTH_READ {
            ret.push(super::GattDescriptorFlags::EncryptAuthenticatedRead);
        }
        if Self::ENCRYPT_AUTH_WRITE {
            ret.push(super::GattDescriptorFlags::EncryptAuthenticatedWrite);
        }
        if Self::AUTHORIZE {
            ret.push(super::GattDescriptorFlags::Authorize);
        }
        ret
    }
}

#[cfg(target_os = "linux")]
pub trait GattDescriptorDelegate:
    GattDescriptorDelegateInner +
    GattDescriptorDelegateInnerDbusExt +
    zvariant::Type
{}

#[cfg(not(target_os = "linux"))]
pub trait GattDescriptorDelegate: GattDescriptorDelegateInner {}
