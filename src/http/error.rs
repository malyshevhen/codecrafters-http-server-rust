use std::fmt::Display;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    InvalidRequest,
    MethodNotSupported,
    MethodWasNotReadCorrectly,
    CanNotParseHeader,
    NoUrl,

    // -- Modules
    Io(std::io::Error),
}

impl From<std::io::Error> for Error {
    fn from(v: std::io::Error) -> Self {
        Self::Io(v)
    }
}

impl Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
