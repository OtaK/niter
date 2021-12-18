#[derive(Debug, thiserror::Error)]
pub enum NiterError {
    #[cfg(target_os = "linux")]
    #[error(transparent)]
    DbusError(#[from] zbus::Error),
    #[cfg(target_os = "linux")]
    #[error(transparent)]
    DbusFdoError(#[from] zbus::fdo::Error),
    #[cfg(target_os = "linux")]
    #[error(transparent)]
    BlueZError(#[from] BlueZError),
    #[cfg(target_os = "linux")]
    #[error(transparent)]
    ZvariantError(#[from] zvariant::Error),
    #[error(transparent)]
    StrumParseError(#[from] strum::ParseError),
    #[error(transparent)]
    UuidError(#[from] uuid::Error),
    #[error(transparent)]
    GattError(#[from] NiterGattError),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum NiterGattError {
    #[error("The delegated call (flag: {0:b}) has not been implemented on this delegate")]
    DelegateNotImplemented(u8),
}

#[cfg(target_os = "linux")]
impl Into<zbus::fdo::Error> for NiterGattError {
    fn into(self) -> zbus::fdo::Error {
        zbus::fdo::Error::NotSupported(format!("{}", self))
    }
}

#[cfg(target_os = "linux")]
#[derive(Debug, zbus::DBusError)]
#[dbus_error(prefix = "org.bluez.Error")]
pub enum BlueZError {
    ZBus(zbus::Error),
    InvalidArguments(String),
    Failed(String),
    InProgress,
    NotPermitted,
    NotAuthorized,
    InvalidOffset,
    NotSupported,
}

pub type NiterResult<T> = Result<T, NiterError>;
pub type NiterGattResult<T> = Result<T, NiterGattError>;
