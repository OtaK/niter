#[derive(Debug, thiserror::Error)]
pub enum NiterError {
    #[error(transparent)]
    DbusError(#[from] zbus::Error),
    #[error(transparent)]
    DbusFdoError(#[from] zbus::fdo::Error),
    #[error(transparent)]
    BlueZError(#[from] BlueZError),
    #[error(transparent)]
    ZvariantError(#[from] zvariant::Error),
    #[error(transparent)]
    StrumParseError(#[from] strum::ParseError),
    #[error(transparent)]
    UuidError(#[from] uuid::Error),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

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
