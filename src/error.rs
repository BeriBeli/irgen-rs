
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Calamine error: {0}")]
    CalamineError(#[from] calamine::Error),

    #[error("Xlsx error: {0}")]
    XlsxError(#[from] calamine::XlsxError),
    
    #[error("Polars error: {0}")]
    PolarsError(#[from] polars::prelude::PolarsError),

    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),

    #[error("String error: {0}")]
    StringError(String),

    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),
}

impl From<&str> for Error {
    fn from(s: &str) -> Self {
        Error::StringError(s.to_string())
    }
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Error::StringError(s)
    }
}