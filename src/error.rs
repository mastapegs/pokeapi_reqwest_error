use std::fmt;

#[derive(Debug)]
pub enum Kind<'a> {
    RequestError(&'a str),
}

impl fmt::Display for Kind<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Kind::RequestError(message) => write!(f, "{message}"),
        }
    }
}

#[derive(Debug)]
pub struct Error {
    kind: Kind<'static>,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind)
    }
}

impl std::error::Error for Error {}

impl From<reqwest::Error> for Error {
    fn from(_error: reqwest::Error) -> Self {
        Self {
            kind: Kind::RequestError("Reqwest Error"),
        }
    }
}
