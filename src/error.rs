#[derive(Debug, thiserror::Error)]
pub enum NiterError {
    #[error(transparent)]
    DbusError(#[from] zbus::Error),
    #[error("BlueZ Error: {0}")]
    BlueZError(#[from] BlueZError),
    #[error(transparent)]
    ZvariantError(#[from] zvariant::Error),
    #[error(transparent)]
    UuidError(#[from] uuid::Error),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum BlueZError {
    #[error("org.bluez.Error.Failed")]
    Failed,
    #[error("org.bluez.Error.InProgress")]
    InProgress,
    #[error("org.bluez.Error.NotPermitted")]
    NotPermitted,
    #[error("org.bluez.Error.NotAuthorized")]
    NotAuthorized,
    #[error("org.bluez.Error.InvalidOffset")]
    InvalidOffset,
    #[error("org.bluez.Error.NotSupported")]
    NotSupported,
}

pub type NiterResult<T> = Result<T, NiterError>;
