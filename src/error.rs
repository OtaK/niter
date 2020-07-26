#[derive(Debug, thiserror::Error)]
pub enum NiterError {
    #[error(transparent)]
    DbusError(#[from] zbus::Error),
    #[error(transparent)]
    ZvariantError(#[from] zvariant::Error),
    #[error(transparent)]
    Other(#[from] anyhow::Error)
}

pub type NiterResult<T> = Result<T, NiterError>;
