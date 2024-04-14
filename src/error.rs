use std::fmt::{self, Debug, Display, Formatter};

#[derive(Clone, Debug, thiserror::Error)]
pub enum Error {
    TrailingCharacters,
    ExpectedColon,
    ExpectedMapColon,
    ExpectedMapComma,
    ExpectedMap,
    ExpectedMapEnd,
    ExpectedArrayComma,
    ExpectedArray,
    ExpectedArrayEnd,
    ExpectedEnum,
    ExpectedEnd,
    ExpectedNull,
    ExpectedString,
    ExpectedInteger,
    ExpectedBoolean,
    Eof,
    Syntax,
    Custom(String),
}

pub type Result<T> = core::result::Result<T, Error>;

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl serde::ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Error::Custom(msg.to_string())
    }
}

impl serde::de::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Error::Custom(msg.to_string())
    }
}
