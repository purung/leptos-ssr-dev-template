use thiserror::Error;

#[derive(Debug, Error)]
pub enum EyeError {
    #[error("Problem med lagring")]
    StorageError,
    #[error("Problem med upplägget")]
    ConfigError,
}

#[cfg(feature = "ssr")]
impl From<sqlx::Error> for EyeError {
    fn from(_: sqlx::Error) -> Self {
        EyeError::StorageError
    }
}
