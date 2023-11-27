use thiserror::Error;

#[derive(Debug, Error)]
pub enum EyeError {
    #[error("Problem med lagring")]
    StorageError,
    #[error("Problem med upplägget")]
    ConfigError,
    #[error("Problem med inloggning")]
    AuthError
}

#[cfg(feature = "ssr")]
impl From<sqlx::Error> for EyeError {
    fn from(_: sqlx::Error) -> Self {
        EyeError::StorageError
    }
}

#[cfg(feature = "ssr")]
impl From<crate::auth::AuthError> for EyeError {
    fn from(_: crate::auth::AuthError) -> Self {
        EyeError::AuthError
    }
}
