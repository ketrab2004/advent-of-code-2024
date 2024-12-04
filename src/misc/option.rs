use std::{error::Error, fmt::Display};

#[derive(Debug, Clone)]
pub struct OptionError;

impl Error for OptionError {}
impl Display for OptionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("called `Option::unwrap_or_err()` on a `None` value")
    }
}

pub trait OptionExt<T> {
    fn unwrap_or_err(self) -> Result<T, OptionError>;
}

impl<T> OptionExt<T> for Option<T> {
    fn unwrap_or_err(self) -> Result<T, OptionError> {
        match self {
            Some(value) => Ok(value),
            None => Err(OptionError)
        }
    }
}
