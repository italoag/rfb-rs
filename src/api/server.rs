use super::{ApiConfig, Result, handlers};
use actix_web::{web, App, HttpServer, middleware};

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
        
        let bind_addr = format!("{}:{}", self.config.host, self.config.port);
        let db_url = self.config.database_url.clone();
        
        HttpServer::new(move || {
            App::new()
                .app_data(web::Data::new(db_url.clone()))
                .wrap(middleware::Logger::default())
                .wrap(middleware::Compress::default())
                .service(
                    web::scope("/api")
                        .route("/health", web::get().to(handlers::health_check))
                        .route("/metrics", web::get().to(handlers::metrics))
                        .route("/cnpj/{cnpj}", web::get().to(handlers::get_company_handler))
                )
                .route("/health", web::get().to(handlers::health_check))
                .route("/metrics", web::get().to(handlers::metrics))
        })
        .bind(&bind_addr)
        .map_err(|e| super::ApiError::ServerError(e.to_string()))?
        .run()
        .await
        .map_err(|e| super::ApiError::ServerError(e.to_string()))?;
        
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
