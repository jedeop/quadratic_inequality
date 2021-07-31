use nom::error::{ErrorKind, FromExternalError, ParseError};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum Error<'a> {
    #[error("{0} is not an inequality sign")]
    InvalidIneqSign(String),
    #[error("invalid quadratic")]
    InvalidQuadratic,
    #[error("invalid character: expected {expected}, found {found}")]
    InvalidCharacter { expected: String, found: String },
    #[error("parser error: input: {0}, kind: {1:?}")]
    Nom(&'a str, ErrorKind),
}
impl<'a> ParseError<&'a str> for Error<'a> {
    fn from_error_kind(input: &'a str, kind: nom::error::ErrorKind) -> Self {
        Error::Nom(input, kind)
    }

    fn append(_: &str, _: nom::error::ErrorKind, other: Self) -> Self {
        other
    }
}
impl<'a> FromExternalError<&str, Error<'a>> for Error<'a> {
    fn from_external_error(_: &str, _: ErrorKind, e: Error<'a>) -> Self {
        e
    }
}

pub type Result<'a, T> = std::result::Result<T, Error<'a>>;
