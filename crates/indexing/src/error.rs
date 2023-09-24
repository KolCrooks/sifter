use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FaissError {
    #[error(transparent)]
    Io(#[from] io::Error),

    #[error("invalid arguments: {0}")]
    InvalidArguments(String),
}

pub type Result<T> = anyhow::Result<T>;
