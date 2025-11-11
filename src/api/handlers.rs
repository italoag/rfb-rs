use super::Result;
use crate::transform::Company;
use actix_web::{web, HttpResponse, Responder};
use postgres::{Client, NoTls};
use serde_json::json;

/// Prometheus exposition format version
const PROMETHEUS_CONTENT_TYPE: &str = "text/plain; version=0.0.4";

/// Handler for getting company by CNPJ
pub async fn get_company_handler(
    cnpj: web::Path<String>,
    db_url: web::Data<String>,
) -> impl Responder {
    match get_company(&cnpj, &db_url).await {
        Ok(Some(company)) => HttpResponse::Ok().json(company),
        Ok(None) => HttpResponse::NotFound().json(json!({
            "error": "Company not found",
            "cnpj": cnpj.as_str()
        })),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "error": format!("Database error: {}", e)
        })),
    }
}

/// Handler for getting company by CNPJ
pub async fn get_company(cnpj: &str, db_url: &str) -> Result<Option<Company>> {
    // Validate and clean CNPJ (remove non-digits)
    let clean_cnpj: String = cnpj.chars().filter(|c| c.is_ascii_digit()).collect();

    if clean_cnpj.len() != 14 {
        return Err(super::ApiError::InvalidCnpj(cnpj.to_string()));
    }

    tracing::info!("Getting company with CNPJ: {}", clean_cnpj);

    // Connect to database
    let mut client = Client::connect(db_url, NoTls)
        .map_err(|e| super::ApiError::DatabaseError(e.to_string()))?;

    // Query company
    let row = client
        .query_opt(
            "SELECT cnpj, razao_social, nome_fantasia, situacao_cadastral, 
                descricao_situacao_cadastral, data_situacao_cadastral,
                uf, municipio, cnae_fiscal, cnae_fiscal_descricao,
                logradouro, numero, complemento, bairro, cep,
                email, codigo_natureza_juridica, capital_social
         FROM companies WHERE cnpj = $1",
            &[&clean_cnpj],
        )
        .map_err(|e| super::ApiError::DatabaseError(e.to_string()))?;

    if let Some(row) = row {
        let company = Company {
            cnpj: row.get(0),
            razao_social: row.get(1),
            nome_fantasia: row.get(2),
            situacao_cadastral: row.get(3),
            descricao_situacao_cadastral: row.get(4),
            data_situacao_cadastral: row.get::<_, Option<String>>(5),
            uf: row.get(6),
            municipio: row.get(7),
            cnae_fiscal: row.get(8),
            cnae_fiscal_descricao: row.get(9),
            logradouro: row.get(10),
            numero: row.get(11),
            complemento: row.get(12),
            bairro: row.get(13),
            cep: row.get(14),
            email: row.get(15),
            codigo_natureza_juridica: row.get(16),
            capital_social: row.get(17),
            // Fill remaining fields with defaults for now
            identificador_matriz_filial: None,
            descricao_identificador_matriz_filial: None,
            motivo_situacao_cadastral: None,
            descricao_motivo_situacao_cadastral: None,
            nome_cidade_no_exterior: String::new(),
            codigo_pais: None,
            pais: None,
            data_inicio_atividade: None,
            descricao_tipo_de_logradouro: String::new(),
            ddd_telefone_1: String::new(),
            ddd_telefone_2: String::new(),
            ddd_fax: String::new(),
            situacao_especial: String::new(),
            data_situacao_especial: None,
            opcao_pelo_simples: None,
            data_opcao_pelo_simples: None,
            data_exclusao_do_simples: None,
            opcao_pelo_mei: None,
            data_opcao_pelo_mei: None,
            data_exclusao_do_mei: None,
            natureza_juridica: None,
            qualificacao_do_responsavel: None,
            codigo_porte: None,
            porte: None,
            ente_federativo_responsavel: String::new(),
            codigo_municipio: None,
            codigo_municipio_ibge: None,
        };

        Ok(Some(company))
    } else {
        Ok(None)
    }
}

/// Handler for health check
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "status": "OK",
        "service": "rfb-rs",
        "version": env!("CARGO_PKG_VERSION")
    }))
}

/// Handler for metrics
pub async fn metrics() -> impl Responder {
    // Basic Prometheus-compatible metrics
    let metrics_text = format!(
        "# HELP rfb_info Information about rfb-rs\n\
         # TYPE rfb_info gauge\n\
         rfb_info{{version=\"{}\"}} 1\n\
         # HELP rfb_requests_total Total requests\n\
         # TYPE rfb_requests_total counter\n\
         rfb_requests_total 0\n",
        env!("CARGO_PKG_VERSION")
    );

    HttpResponse::Ok()
        .content_type(PROMETHEUS_CONTENT_TYPE)
        .body(metrics_text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_health_check() {
        let _response = health_check().await;
        // Response is an impl Responder, can't directly test here
        // In integration tests, we would test the actual HTTP response
    }
}
