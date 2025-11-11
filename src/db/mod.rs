mod postgres;
mod schema;

pub use postgres::PostgresDatabase;
pub use schema::create_schema;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Database connection error: {0}")]
    ConnectionError(String),

    #[error("Query error: {0}")]
    QueryError(String),

    #[error("Migration error: {0}")]
    MigrationError(String),

    #[error("Diesel error: {0}")]
    DieselError(String),
}

pub type Result<T> = std::result::Result<T, DatabaseError>;

/// Database trait for different database backends
pub trait Database {
    /// Create tables and indexes
    fn create(&self) -> Result<()>;

    /// Drop all tables
    fn drop(&self) -> Result<()>;

    /// Close database connection
    fn close(&self) -> Result<()>;
}
