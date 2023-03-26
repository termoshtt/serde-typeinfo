use serde::ser;
use std::fmt::{self, Display};

#[derive(Debug)]
pub struct Error {}

impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        todo!()
    }
}

impl Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        todo!()
    }
}

impl std::error::Error for Error {}

pub type Result<T> = ::std::result::Result<T, Error>;
