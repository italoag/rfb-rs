/// Integration tests for database module
use rfb_rs::db::{Database, PostgresDatabase};

#[test]
fn test_postgres_database_creation() {
    let db = PostgresDatabase::new(
        "postgres://localhost/test_db".to_string(),
        "public".to_string(),
    );
    
    // Just verify it was created successfully
    // Cannot access private fields, so we just test creation
    let _ = db;
}

#[test]
fn test_postgres_different_schema() {
    let db = PostgresDatabase::new(
        "postgres://localhost/test_db".to_string(),
        "rfb_schema".to_string(),
    );
    
    // Just verify it was created successfully
    let _ = db;
}

// Note: The following tests would require a real PostgreSQL instance
// They are marked as ignored and can be run with --ignored flag when DB is available

#[test]
#[ignore]
fn test_postgres_create_tables() {
    // Set DATABASE_URL environment variable before running
    let db = PostgresDatabase::from_env().unwrap();
    let result = db.create();
    
    assert!(result.is_ok());
}

#[test]
#[ignore]
fn test_postgres_drop_tables() {
    let db = PostgresDatabase::from_env().unwrap();
    let result = db.drop();
    
    assert!(result.is_ok());
}

#[test]
#[ignore]
fn test_postgres_create_and_drop_cycle() {
    let db = PostgresDatabase::from_env().unwrap();
    
    // Create tables
    let create_result = db.create();
    assert!(create_result.is_ok());
    
    // Drop tables
    let drop_result = db.drop();
    assert!(drop_result.is_ok());
    
    // Create again to verify it works after drop
    let create_again = db.create();
    assert!(create_again.is_ok());
}
