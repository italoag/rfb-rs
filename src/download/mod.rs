mod federal_revenue;
mod downloader;
mod check;

pub use downloader::Downloader;
pub use federal_revenue::FederalRevenue;
pub use check::check_zip_integrity;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum DownloadError {
    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("ZIP error: {0}")]
    ZipError(#[from] zip::result::ZipError),
    
    #[error("File not found: {0}")]
    FileNotFound(String),
    
    #[error("Download failed after {0} retries")]
    MaxRetriesExceeded(u32),
    
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),
    
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),
}

pub type Result<T> = std::result::Result<T, DownloadError>;

// Default constants for download configuration
pub const DEFAULT_TIMEOUT_SECS: u64 = 300;
pub const DEFAULT_MAX_RETRIES: u32 = 3;
pub const DEFAULT_MAX_PARALLEL: usize = 4;
pub const DEFAULT_CHUNK_SIZE: i64 = 10_485_760; // 10 MB

#[derive(Debug, Clone)]
pub struct DownloadConfig {
    pub data_dir: String,
    pub timeout_secs: u64,
    pub max_retries: u32,
    pub max_parallel: usize,
    pub chunk_size: i64,
    pub skip_existing: bool,
    pub restart: bool,
}

impl DownloadConfig {
    /// Validate the configuration
    pub fn validate(&self) -> Result<()> {
        if self.max_parallel == 0 {
            return Err(DownloadError::InvalidConfig("max_parallel must be at least 1".to_string()));
        }
        if self.max_retries == 0 {
            return Err(DownloadError::InvalidConfig("max_retries must be at least 1".to_string()));
        }
        Ok(())
    }
}

impl Default for DownloadConfig {
    fn default() -> Self {
        Self {
            data_dir: "data".to_string(),
            timeout_secs: DEFAULT_TIMEOUT_SECS,
            max_retries: DEFAULT_MAX_RETRIES,
            max_parallel: DEFAULT_MAX_PARALLEL,
            chunk_size: DEFAULT_CHUNK_SIZE,
            skip_existing: false,
            restart: false,
        }
    }
}
