mod descriptor;
pub use self::descriptor::*;

mod characteristic;
pub use self::characteristic::*;

mod service;
pub use self::service::*;

pub trait Peripheral: std::fmt::Debug {
    fn find_adapter() -> crate::NiterResult<Self> where Self: Sized;
    fn is_powered(&self) -> crate::NiterResult<bool>;
    fn is_advertising(&self) -> crate::NiterResult<bool>;
    fn start_advertising(&self, name: &str) -> crate::NiterResult<()>;
    fn stop_advertising(&self) -> crate::NiterResult<()>;
    fn register_gatt(&self) -> crate::NiterResult<()>;
    fn unregister_gatt(&self) -> crate::NiterResult<()>;

    fn add_service<S: GattServiceDelegate + 'static>(&mut self, service: GattService<S>) -> crate::NiterResult<()>;
}
