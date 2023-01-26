use std::fmt::{self, Debug, Display};

/// Errors that can occur when serializing or deserializing TOML.
pub struct Error(ErrorInner);

pub(crate) enum ErrorInner {
    Ser(crate::ser::Error),
    De(crate::de::Error),
}

impl From<crate::ser::Error> for Error {
    fn from(error: crate::ser::Error) -> Self {
        Error(ErrorInner::Ser(error))
    }
}

impl From<crate::de::Error> for Error {
    fn from(error: crate::de::Error) -> Self {
        Error(ErrorInner::De(error))
    }
}

impl Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            ErrorInner::Ser(error) => Display::fmt(error, formatter),
            ErrorInner::De(error) => Display::fmt(error, formatter),
        }
    }
}

impl Debug for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            ErrorInner::Ser(error) => Debug::fmt(error, formatter),
            ErrorInner::De(error) => Debug::fmt(error, formatter),
        }
    }
}

impl std::error::Error for Error {}
