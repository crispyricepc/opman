use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum ErrorKind {
    Ok = 0,
    NotYetImplemented,
    PackageNotFound,
    Unknown,
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorKind::Ok => write!(f, "Ok"),
            ErrorKind::NotYetImplemented => write!(f, "Not yet implemented"),
            ErrorKind::PackageNotFound => write!(f, "Package not found"),
            ErrorKind::Unknown => write!(f, "Unknown error"),
        }
    }
}

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub parent: Option<Box<dyn std::error::Error>>,
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Self {
        Self { kind, parent: None }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.parent {
            Some(e) => write!(f, "{}: {}", self.kind, e),
            None => write!(f, "{}", self.kind),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
