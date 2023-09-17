use thiserror::Error;

#[derive(Error, Debug)]
pub enum FaissError {
    #[error("invalid arguments: {0}")]
    InvalidArguments(String),
}

pub type Result<T> = std::result::Result<T, FaissError>;