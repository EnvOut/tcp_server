#[allow(clippy::enum_variant_names)]
#[derive(thiserror::Error, Debug)]
pub enum ProtocolErrors {
    #[error("Walled for user {0:?} is empty")]
    WalletNotInitialized(String),

    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

pub(crate) type ProtocolResult<T> = Result<T, ProtocolErrors>;
