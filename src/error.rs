
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Calamine error: {0}")]
    CalamineError(#[from] calamine::Error),

    #[error("Xlsx error: {0}")]
    XlsxError(#[from] calamine::XlsxError),
    
    #[error("Polars error: {0}")]
    PolarsError(#[from] polars::prelude::PolarsError),

}