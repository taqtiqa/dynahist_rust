use thiserror::Error;
// was Exception
#[derive(Debug, Error)]
pub enum DynaHistError {
    #[error("{0}")]
    ArithmeticError(String),
    // #[error(transparent)]
    // DataFormatError(#[from] anyhow::Error),
    #[error("Illegal argument error")]
    IllegalArgumentError { source: std::io::Error },
    #[error("IO error")]
    IOError(#[source] anyhow::Error),
    #[error("IO error (Unchecked)")]
    IOErrorUnchecked(#[source] anyhow::Error),
    #[error("No such element error")]
    NoSuchElementError(#[source] anyhow::Error),
    #[error("Unsupported operation error")]
    UnsupportedOperationError { source: std::io::Error },
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}
