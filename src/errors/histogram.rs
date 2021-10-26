// was Exception
#[derive(thiserror::Error)]
pub enum DynaHistError {
    #[error("{0}")]
    ArithmeticError(String),
    #[error(transparent)]
    DataFormatError(#[from] anyhow::Error),
    #[error(transparent)]
    IllegalArgumentError(#[from] anyhow::Error),
    #[error(transparent)]
    IOError(#[from] anyhow::Error),
    #[error(transparent)]
    NoSuchElementError(#[from] anyhow::Error),
    #[error(transparent)]
    UncheckedIOError(#[from] anyhow::Error),
    #[error(transparent)]
    UnsupportedOperationError(#[from] anyhow::Error),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}
