use std::collections::HashSet;

use crate::GattCharacteristic;

pub trait GattServiceDelegate {
    const UUID: u16;
    const IS_PRIMARY: bool;
    fn create() -> Self;
    fn characteristics(&mut self) -> HashSet<GattCharacteristic>;
}
