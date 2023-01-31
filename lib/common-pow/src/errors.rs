use std::num::ParseIntError;
use std::string::FromUtf8Error;

#[allow(clippy::enum_variant_names)]
#[derive(thiserror::Error, Debug)]
pub enum CommonPowErrors {
    #[error("Wrong HashCash, {0}")]
    WrongHashCash(String),
    #[error("Error in pow processing. Counter: {1:?}, error: {0:?}")]
    PowError(String, u128),
    #[error(transparent)]
    RegexError(#[from] regex::Error),
    #[error(transparent)]
    ParseIntError(#[from] ParseIntError),
    #[error(transparent)]
    Base64Error(#[from] base64::DecodeError),
    #[error(transparent)]
    FromUtf8Error(#[from] FromUtf8Error),

    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

pub(crate) type CrateResult<T> = Result<T, CommonPowErrors>;
