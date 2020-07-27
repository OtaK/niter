mod error;
pub use self::error::*;

mod uuid;
pub use self::uuid::*;

pub mod adapter;
pub mod advertisement_monitor;
pub mod advertising;
pub mod agent;
pub mod assigned_numbers;
pub mod device;
pub mod gatt;
pub mod profile;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
