use super::{Database, Result, DatabaseError};
use postgres::{Client, NoTls};

/// PostgreSQL database implementation
pub struct PostgresDatabase {
    connection_string: String,
    schema: String,
}

impl PostgresDatabase {
    pub fn new(connection_string: String, schema: String) -> Self {
        Self {
            connection_string,
            schema,
        }
    }

    pub fn from_env() -> Result<Self> {
        let connection_string = std::env::var("DATABASE_URL")
            .map_err(|_| DatabaseError::ConnectionError("DATABASE_URL not set".to_string()))?;
        let schema = std::env::var("POSTGRES_SCHEMA").unwrap_or_else(|_| "public".to_string());
        
        Ok(Self::new(connection_string, schema))
    }

    fn get_client(&self) -> Result<Client> {
        Client::connect(&self.connection_string, NoTls)
            .map_err(|e| DatabaseError::ConnectionError(e.to_string()))
    }
}

impl Database for PostgresDatabase {
    fn create(&self) -> Result<()> {
        tracing::info!("Creating database schema: {}", self.schema);
        
        let mut client = self.get_client()?;
        
        // Set schema
        client.execute(&format!("SET search_path TO {}", self.schema), &[])
            .map_err(|e| DatabaseError::QueryError(e.to_string()))?;
        
        // Create companies table
        client.execute(
            "CREATE TABLE IF NOT EXISTS companies (
                cnpj VARCHAR(14) PRIMARY KEY,
                identificador_matriz_filial INTEGER,
                descricao_identificador_matriz_filial VARCHAR(50),
                nome_fantasia TEXT,
                situacao_cadastral INTEGER,
                descricao_situacao_cadastral VARCHAR(50),
                data_situacao_cadastral DATE,
                motivo_situacao_cadastral INTEGER,
                descricao_motivo_situacao_cadastral TEXT,
                nome_cidade_no_exterior TEXT,
                codigo_pais INTEGER,
                pais VARCHAR(100),
                data_inicio_atividade DATE,
                cnae_fiscal INTEGER,
                cnae_fiscal_descricao TEXT,
                descricao_tipo_de_logradouro VARCHAR(100),
                logradouro TEXT,
                numero VARCHAR(20),
                complemento TEXT,
                bairro TEXT,
                cep VARCHAR(8),
                uf VARCHAR(2),
                codigo_municipio INTEGER,
                codigo_municipio_ibge INTEGER,
                municipio VARCHAR(100),
                ddd_telefone_1 VARCHAR(20),
                ddd_telefone_2 VARCHAR(20),
                ddd_fax VARCHAR(20),
                email TEXT,
                situacao_especial TEXT,
                data_situacao_especial DATE,
                opcao_pelo_simples BOOLEAN,
                data_opcao_pelo_simples DATE,
                data_exclusao_do_simples DATE,
                opcao_pelo_mei BOOLEAN,
                data_opcao_pelo_mei DATE,
                data_exclusao_do_mei DATE,
                razao_social TEXT,
                codigo_natureza_juridica INTEGER,
                natureza_juridica TEXT,
                qualificacao_do_responsavel INTEGER,
                capital_social NUMERIC(20, 2),
                codigo_porte INTEGER,
                porte VARCHAR(50),
                ente_federativo_responsavel TEXT
            )",
            &[],
        ).map_err(|e| DatabaseError::QueryError(e.to_string()))?;
        
        // Create partners table
        client.execute(
            "CREATE TABLE IF NOT EXISTS partners (
                id SERIAL PRIMARY KEY,
                cnpj VARCHAR(14) NOT NULL,
                identificador_socio INTEGER,
                nome_socio TEXT,
                cnpj_cpf_socio VARCHAR(14),
                codigo_qualificacao_socio INTEGER,
                qualificacao_socio TEXT,
                data_entrada_sociedade DATE,
                codigo_pais INTEGER,
                pais VARCHAR(100),
                cpf_representante_legal VARCHAR(11),
                nome_representante_legal TEXT,
                codigo_qualificacao_representante_legal INTEGER,
                qualificacao_representante_legal TEXT,
                codigo_faixa_etaria INTEGER,
                faixa_etaria VARCHAR(50)
            )",
            &[],
        ).map_err(|e| DatabaseError::QueryError(e.to_string()))?;
        
        // Create indexes
        let indexes = vec![
            "CREATE INDEX IF NOT EXISTS idx_companies_cnpj ON companies(cnpj)",
            "CREATE INDEX IF NOT EXISTS idx_companies_razao_social ON companies(razao_social)",
            "CREATE INDEX IF NOT EXISTS idx_companies_nome_fantasia ON companies(nome_fantasia)",
            "CREATE INDEX IF NOT EXISTS idx_companies_uf ON companies(uf)",
            "CREATE INDEX IF NOT EXISTS idx_companies_municipio ON companies(codigo_municipio)",
            "CREATE INDEX IF NOT EXISTS idx_companies_cnae ON companies(cnae_fiscal)",
            "CREATE INDEX IF NOT EXISTS idx_partners_cnpj ON partners(cnpj)",
            "CREATE INDEX IF NOT EXISTS idx_partners_nome ON partners(nome_socio)",
        ];
        
        for idx_sql in indexes {
            client.execute(idx_sql, &[])
                .map_err(|e| DatabaseError::QueryError(e.to_string()))?;
        }
        
        tracing::info!("Database tables and indexes created successfully");
        Ok(())
    }

    fn drop(&self) -> Result<()> {
        tracing::info!("Dropping database schema: {}", self.schema);
        
        let mut client = self.get_client()?;
        
        // Set schema
        client.execute(&format!("SET search_path TO {}", self.schema), &[])
            .map_err(|e| DatabaseError::QueryError(e.to_string()))?;
        
        // Drop tables
        client.execute("DROP TABLE IF EXISTS partners CASCADE", &[])
            .map_err(|e| DatabaseError::QueryError(e.to_string()))?;
        
        client.execute("DROP TABLE IF EXISTS companies CASCADE", &[])
            .map_err(|e| DatabaseError::QueryError(e.to_string()))?;
        
        tracing::info!("Database tables dropped successfully");
        Ok(())
    }

    fn close(&self) -> Result<()> {
        // Connection cleanup happens automatically
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_postgres_creation() {
        let db = PostgresDatabase::new(
            "postgres://user:pass@localhost/db".to_string(),
            "public".to_string(),
        );
        assert_eq!(db.schema, "public");
    }
}
