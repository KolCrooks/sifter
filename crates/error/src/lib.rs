use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CustomErrors {
    #[error(transparent)]
    Io(#[from] io::Error),

    #[error("invalid arguments: {0}")]
    InvalidArguments(String),

    #[error("invalid state: {0}")]
    InvalidState(String),
}

pub type Error = anyhow::Error;

pub type Result<T> = anyhow::Result<T>;
