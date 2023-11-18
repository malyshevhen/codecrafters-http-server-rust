use std::fmt::Display;

use crate::http;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    InvalidRequest,
    InvalidURL,

    // -- Modules
    Io(std::io::Error),
    Http(http::Error),
}

impl From<http::error::Error> for Error {
    fn from(v: http::error::Error) -> Self {
        Self::Http(v)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl From<std::io::Error> for Error {
    fn from(v: std::io::Error) -> Self {
        Self::Io(v)
    }
}

impl std::error::Error for Error {}
