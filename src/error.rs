use std::error::Error as StdError;
use std::fmt::{self, Display};
use std::result::Result as StdResult;

use crate::Acc;

pub type Result = StdResult<Acc, Error>;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ErrorKind {
    Io,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Error {
    pub message: String,
    pub kind: ErrorKind,
}

impl Error {
    pub fn new(message: &str, kind: ErrorKind) -> Error {
        Error {
            message: String::from(message),
            kind,
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.message)
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::new(&error.to_string(), ErrorKind::Io)
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        &*self.message
    }
}
