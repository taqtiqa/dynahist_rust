// was Exception
#[derive(Debug, thiserror::Error)]
pub enum DynaHistError {
    #[error("{0}")]
    ArithmeticError(String),
    // #[error(transparent)]
    // DataFormatError(#[from] anyhow::Error),
    #[error("Illegal argument error")]
    IllegalArgumentError(#[source] anyhow::Error),
    #[error("transparent")]
    IOError(#[source] anyhow::Error),
    #[error("transparent")]
    NoSuchElementError(#[source] anyhow::Error),
    #[error("transparent")]
    UncheckedIOError(#[source] anyhow::Error),
    #[error("transparent")]
    UnsupportedOperationError(#[source] anyhow::Error),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}
