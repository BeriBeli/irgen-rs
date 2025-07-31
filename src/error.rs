#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Calamine error: {0}")]
    CalamineError(#[from] calamine::Error),

    #[error("Xlsx error: {0}")]
    XlsxError(#[from] calamine::XlsxError),

    #[error("Polars error: {0}")]
    PolarsError(#[from] polars::prelude::PolarsError),

    #[error("XML error: {0}")]
    XmlSerializeError(#[from] quick_xml::SeError),

    #[error("Json error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error(transparent)]
    AnyhowError(#[from] anyhow::Error),

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
