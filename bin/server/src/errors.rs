use std::collections::TryReserveError;
use std::convert::Infallible;

use cbor4ii::{DecodeError, EncodeError};
use magic_crypt::MagicCryptError;

use common_pow::errors::CommonPowErrors;

#[allow(clippy::enum_variant_names)]
#[derive(thiserror::Error, Debug)]
pub enum ServerErrors {
    #[error("Hashcash is invalid")]
    HashcashInvalid,

    #[error("Hashcash is invalid. Expected starting from: {0:?}, hash: {1:?}, answer: {2:?}")]
    IncorrectAnswer(String, String, String),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    CborDecodeError(#[from] DecodeError<Infallible>),

    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),
    #[error(transparent)]
    CommonPowErrors(#[from] CommonPowErrors),

    #[error(transparent)]
    MagicCryptError(#[from] MagicCryptError),

    #[error(transparent)]
    CborError(#[from] EncodeError<TryReserveError>),

    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

pub(crate) type ServerResult<T> = Result<T, ServerErrors>;
