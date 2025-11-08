/// Integration tests for API module
use rfb_rs::api::{ApiConfig, ApiServer};

#[test]
fn test_api_config_default() {
    let config = ApiConfig::default();
    
    assert_eq!(config.host, "127.0.0.1");
    assert_eq!(config.port, 8080);
}

#[test]
fn test_api_config_custom() {
    let config = ApiConfig {
        host: "0.0.0.0".to_string(),
        port: 3000,
        database_url: "postgres://localhost/rfb".to_string(),
    };
    
    assert_eq!(config.host, "0.0.0.0");
    assert_eq!(config.port, 3000);
    assert_eq!(config.database_url, "postgres://localhost/rfb");
}

#[test]
fn test_api_server_creation() {
    let config = ApiConfig {
        host: "127.0.0.1".to_string(),
        port: 8080,
        database_url: "postgres://localhost/rfb".to_string(),
    };
    
    let server = ApiServer::new(config);
    // Just verify it was created successfully
    let _ = server;
}

// Note: The following tests would require a running server
// They are integration tests that can be run with a test database

#[tokio::test]
#[ignore]
async fn test_api_server_startup() {
    let config = ApiConfig {
        host: "127.0.0.1".to_string(),
        port: 8081, // Use different port for testing
        database_url: std::env::var("DATABASE_URL").unwrap_or("postgres://localhost/rfb_test".to_string()),
    };
    
    let server = ApiServer::new(config);
    
    // This would start the server - in real tests, we'd spawn it in a separate task
    // and make HTTP requests to it
    // let result = server.start().await;
    // assert!(result.is_ok());
}

#[tokio::test]
#[ignore]
async fn test_health_endpoint() {
    // This test would make an HTTP request to the health endpoint
    // let response = reqwest::get("http://localhost:8081/health").await.unwrap();
    // assert_eq!(response.status(), 200);
    
    // let body: serde_json::Value = response.json().await.unwrap();
    // assert_eq!(body["status"], "OK");
}

#[tokio::test]
#[ignore]
async fn test_metrics_endpoint() {
    // This test would make an HTTP request to the metrics endpoint
    // let response = reqwest::get("http://localhost:8081/metrics").await.unwrap();
    // assert_eq!(response.status(), 200);
    
    // let text = response.text().await.unwrap();
    // assert!(text.contains("rfb_info"));
}

#[tokio::test]
#[ignore]
async fn test_cnpj_endpoint_not_found() {
    // This test would make an HTTP request to a non-existent CNPJ
    // let response = reqwest::get("http://localhost:8081/api/cnpj/99999999999999").await.unwrap();
    // assert_eq!(response.status(), 404);
}
