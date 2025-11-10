/// SQL schema for creating tables
pub fn create_schema() -> &'static str {
    r#"
    -- Companies table (Estabelecimentos)
    CREATE TABLE IF NOT EXISTS companies (
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
    );

    -- Partners table (Sócios)
    CREATE TABLE IF NOT EXISTS partners (
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
        faixa_etaria VARCHAR(50),
        FOREIGN KEY (cnpj) REFERENCES companies(cnpj)
    );

    -- Create indexes
    CREATE INDEX IF NOT EXISTS idx_companies_cnpj ON companies(cnpj);
    CREATE INDEX IF NOT EXISTS idx_companies_razao_social ON companies(razao_social);
    CREATE INDEX IF NOT EXISTS idx_companies_nome_fantasia ON companies(nome_fantasia);
    CREATE INDEX IF NOT EXISTS idx_companies_uf ON companies(uf);
    CREATE INDEX IF NOT EXISTS idx_companies_municipio ON companies(codigo_municipio);
    CREATE INDEX IF NOT EXISTS idx_companies_cnae ON companies(cnae_fiscal);
    CREATE INDEX IF NOT EXISTS idx_partners_cnpj ON partners(cnpj);
    CREATE INDEX IF NOT EXISTS idx_partners_nome ON partners(nome_socio);
    "#
}
