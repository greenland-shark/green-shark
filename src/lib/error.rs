use std::fmt::Debug;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to parse json.")]
    SerdeJsonError(#[from] serde_json::Error),

    #[error("failed to read file.")]
    FileReadError(#[source] std::io::Error),

    #[error("failed to write file.")]
    FileWriteError(#[source] std::io::Error),

    #[error("failed to create file.")]
    FileCreateError(#[source] std::io::Error),
}
