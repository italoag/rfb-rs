mod cnae;
mod company;
mod lookups;
mod partner;
mod tax_regime;
mod transformer;

pub use cnae::CNAE;
pub use company::Company;
pub use lookups::Lookups;
pub use partner::Partner;
pub use tax_regime::TaxRegime;
pub use transformer::Transformer;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum TransformError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("CSV parse error: {0}")]
    CsvError(String),

    #[error("Polars error: {0}")]
    PolarsError(#[from] polars::prelude::PolarsError),

    #[error("Invalid data format: {0}")]
    InvalidFormat(String),

    #[error("Missing required field: {0}")]
    MissingField(String),

    #[error("ZIP extraction error: {0}")]
    ZipError(#[from] zip::result::ZipError),
}

pub type Result<T> = std::result::Result<T, TransformError>;

#[derive(Debug, Clone)]
pub struct TransformConfig {
    pub data_dir: String,
    pub output_dir: String,
    pub privacy_mode: bool,
}

impl Default for TransformConfig {
    fn default() -> Self {
        Self {
            data_dir: "data".to_string(),
            output_dir: "output".to_string(),
            privacy_mode: false,
        }
    }
}
