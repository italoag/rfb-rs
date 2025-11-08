use super::{ApiConfig, Result};

/// API server
pub struct ApiServer {
    config: ApiConfig,
}

impl ApiServer {
    pub fn new(config: ApiConfig) -> Self {
        Self { config }
    }

    /// Start the API server
    pub async fn start(&self) -> Result<()> {
        tracing::info!("Starting API server on {}:{}", self.config.host, self.config.port);
        
        // TODO: Implement HTTP server using actix-web or similar
        // Endpoints:
        // - GET /cnpj/{cnpj} - Get company by CNPJ
        // - GET /health - Health check
        // - GET /metrics - Prometheus metrics
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_server_creation() {
        let config = ApiConfig::default();
        let server = ApiServer::new(config);
        assert_eq!(server.config.port, 8080);
    }
}
