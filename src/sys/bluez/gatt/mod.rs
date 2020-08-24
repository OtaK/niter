mod manager;
pub use self::manager::*;
mod service;
pub use self::service::*;
mod profile;
pub use self::profile::*;
mod characteristic;
pub use self::characteristic::*;
mod descriptor;
pub use self::descriptor::*;

#[derive(Debug, Clone, zvariant_derive::Type, serde::Serialize, serde::Deserialize)]
pub enum GattLinkType {
    Todo,
}
