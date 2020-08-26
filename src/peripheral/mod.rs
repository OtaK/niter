mod descriptor;
pub use self::descriptor::*;

pub(crate) trait PlatformPeripheral: std::fmt::Debug {
    type Service;
    type Characteristic;
    type Descriptor;

    fn address(&self) -> String;
    fn is_connected(&self) -> bool;
    fn connect(&self) -> crate::NiterResult<()>;
    fn disconnect(&self) -> crate::NiterResult<()>;
    fn characteristics(&self) -> crate::NiterResult<Vec<Self::Characteristic>>;
    fn services(&self) -> crate::NiterResult<Vec<Self::Service>>;
    fn descriptors(&self) -> crate::NiterResult<Vec<Self::Descriptor>>;
    fn register_service(&self) -> crate::NiterResult<()>;
}
