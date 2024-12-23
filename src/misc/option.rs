use error_rules::Error;


#[derive(Debug, Clone, Error)]
pub enum OptionError {
    #[error_kind("Option: called `Option::unwrap_or_err()` on a `None` value")]
    None,
    #[error_kind("Option: called `Option::is_none_or_err()` on a `Some` value")]
    Some
}

#[allow(dead_code)]
pub trait OptionExt<T> {
    fn unwrap_or_err(self) -> Result<T, OptionError>;
    fn is_none_or_err(self) -> Result<bool, OptionError>;
}

impl<T> OptionExt<T> for Option<T> {
    fn unwrap_or_err(self) -> Result<T, OptionError> {
        match self {
            Some(value) => Ok(value),
            None => Err(OptionError::None)
        }
    }
    fn is_none_or_err(self) -> Result<bool, OptionError> {
        match self {
            Some(_) => Err(OptionError::Some),
            None => Ok(true)
        }
    }
}
