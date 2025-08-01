#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Calamine error: {0}")]
    Calamine(#[from] calamine::Error),

    #[error("Xlsx error: {0}")]
    Xlsx(#[from] calamine::XlsxError),

    #[error("Polars error: {0}")]
    Polars(#[from] polars::prelude::PolarsError),

    #[error("XML Serialization error: {0}")]
    XmlSe(#[from] quick_xml::SeError),

    #[error("Json error: {0}")]
    Json(#[from] serde_json::Error),

    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),

    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),

    #[error("String error: {0}")]
    StringError(String),

    #[error("HashMap not found for key {0}")]
    NotFound(String),
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
