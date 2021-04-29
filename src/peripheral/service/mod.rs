mod service_delegate;
pub use self::service_delegate::*;

#[derive(Debug)]
pub struct GattService<S: GattServiceDelegate + 'static> {
    delegate: S,
    // inner: Service,
    uuid: uuid::Uuid,
}

// impl<S: GattServiceDelegate + 'static> std::ops::Deref for GattService<S> {
//     type Target = Service;
//     fn deref(&self) -> &Self::Target {
//         self.as_service()
//     }
// }
