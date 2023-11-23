use thiserror::Error;

#[derive(Debug, Error)]
pub enum EyeError {
    #[error("Problem med lagring")]
    StorageError(#[from] sqlx::Error),
    #[error("Problem med upplägget")]
    ConfigError
}
