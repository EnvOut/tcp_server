use std::collections::TryReserveError;
use std::convert::Infallible;
use std::net::AddrParseError;

use cbor4ii::{DecodeError, EncodeError};

use common_pow::errors::CommonPowErrors;

#[allow(clippy::enum_variant_names)]
#[derive(thiserror::Error, Debug)]
pub enum ClientErrors {
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
    #[error(transparent)]
    AddrParseError(#[from] AddrParseError),

    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    CborError(#[from] EncodeError<TryReserveError>),

    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),
    #[error(transparent)]
    CborDecodeError(#[from] DecodeError<Infallible>),

    #[error(transparent)]
    CommonPowErrors(#[from] CommonPowErrors),
}

pub(crate) type ClientResult<T> = Result<T, ClientErrors>;
