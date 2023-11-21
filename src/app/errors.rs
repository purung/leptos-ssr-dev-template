use thiserror::Error;

#[derive(Debug, Error)]
pub enum EyeError {
    #[error("Problem med lagring")]
    StorageError
}
