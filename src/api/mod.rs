mod handlers;
mod server;

pub use server::ApiServer;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Invalid CNPJ: {0}")]
    InvalidCnpj(String),

    #[error("Server error: {0}")]
    ServerError(String),
}

pub type Result<T> = std::result::Result<T, ApiError>;

#[derive(Debug, Clone)]
pub struct ApiConfig {
    pub host: String,
    pub port: u16,
    pub database_url: String,
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 8080,
            database_url: std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgres://localhost/rfb".to_string()),
        }
    }
}
