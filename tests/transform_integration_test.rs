/// Integration tests for transform module
use rfb_rs::transform::{CNAE, Company, Lookups, Partner, TaxRegime, TransformConfig, Transformer};
use std::fs;
use tempfile::TempDir;

#[test]
fn test_transformer_initialization() {
    let config = TransformConfig {
        data_dir: "test_data".to_string(),
        output_dir: "test_output".to_string(),
        privacy_mode: false,
    };

    let transformer = Transformer::new(config);
    // Just verify it was created successfully
    let _ = transformer;
}

#[test]
fn test_transformer_with_privacy_mode() {
    let config = TransformConfig {
        data_dir: "data".to_string(),
        output_dir: "output".to_string(),
        privacy_mode: true,
    };

    let transformer = Transformer::new(config);
    // Just verify it was created successfully
    let _ = transformer;
}

#[test]
fn test_lookups_initialization() {
    let lookups = Lookups::new();

    assert_eq!(lookups.countries.len(), 0);
    assert_eq!(lookups.cities.len(), 0);
    assert_eq!(lookups.legal_natures.len(), 0);
    assert_eq!(lookups.qualifications.len(), 0);
    assert_eq!(lookups.cnaes.len(), 0);
    assert_eq!(lookups.motives.len(), 0);
}

#[test]
fn test_lookups_data_retrieval() {
    let mut lookups = Lookups::new();

    // Insert test data
    lookups.countries.insert(76, "Brasil".to_string());
    lookups.cities.insert(3550308, "São Paulo".to_string());
    lookups
        .cnaes
        .insert(4751201, "Comércio varejista".to_string());

    // Test retrieval
    assert_eq!(lookups.get_country(76), Some(&"Brasil".to_string()));
    assert_eq!(lookups.get_city(3550308), Some(&"São Paulo".to_string()));
    assert_eq!(
        lookups.get_cnae(4751201),
        Some(&"Comércio varejista".to_string())
    );

    // Test non-existent
    assert_eq!(lookups.get_country(999), None);
}

#[test]
fn test_company_situacao_cadastral_parsing() {
    assert_eq!(Company::parse_situacao_cadastral(1), Some("NULA"));
    assert_eq!(Company::parse_situacao_cadastral(2), Some("ATIVA"));
    assert_eq!(Company::parse_situacao_cadastral(3), Some("SUSPENSA"));
    assert_eq!(Company::parse_situacao_cadastral(4), Some("INAPTA"));
    assert_eq!(Company::parse_situacao_cadastral(8), Some("BAIXADA"));
    assert_eq!(Company::parse_situacao_cadastral(99), None);
}

#[test]
fn test_company_matriz_filial_parsing() {
    assert_eq!(Company::parse_matriz_filial(1), Some("MATRIZ"));
    assert_eq!(Company::parse_matriz_filial(2), Some("FILIAL"));
    assert_eq!(Company::parse_matriz_filial(3), None);
}

#[test]
fn test_company_name_cleanup() {
    let name_with_cpf = "JOAO SILVA 12345678901";
    let cleaned = Company::clean_name(name_with_cpf);

    // Should mask CPF digits
    assert!(cleaned.contains("***"));
    assert!(!cleaned.contains("12345678901"));
}

#[test]
fn test_partner_identificador_parsing() {
    assert_eq!(
        Partner::parse_identificador_socio(1),
        Some("PESSOA JURÍDICA")
    );
    assert_eq!(Partner::parse_identificador_socio(2), Some("PESSOA FÍSICA"));
    assert_eq!(Partner::parse_identificador_socio(3), Some("ESTRANGEIRO"));
    assert_eq!(Partner::parse_identificador_socio(99), None);
}

#[test]
fn test_partner_faixa_etaria_parsing() {
    assert_eq!(Partner::parse_faixa_etaria(1), Some("0 a 12 anos"));
    assert_eq!(Partner::parse_faixa_etaria(5), Some("41 a 50 anos"));
    assert_eq!(Partner::parse_faixa_etaria(9), Some("Maiores de 80 anos"));
    assert_eq!(Partner::parse_faixa_etaria(99), None);
}

#[test]
fn test_cnae_creation() {
    let cnae = CNAE::new(
        4751201,
        "Comércio varejista de produtos de panificação".to_string(),
    );

    assert_eq!(cnae.codigo, 4751201);
    assert_eq!(
        cnae.descricao,
        "Comércio varejista de produtos de panificação"
    );
}

#[test]
fn test_tax_regime_creation() {
    let regime = TaxRegime::new("12345678000195".to_string());

    assert_eq!(regime.cnpj, "12345678000195");
    assert_eq!(regime.opcao_simples, None);
    assert_eq!(regime.opcao_mei, None);
}

#[tokio::test]
async fn test_zip_extraction() {
    let temp_dir = TempDir::new().unwrap();
    let config = TransformConfig {
        data_dir: temp_dir.path().to_str().unwrap().to_string(),
        output_dir: temp_dir.path().to_str().unwrap().to_string(),
        privacy_mode: false,
    };

    let transformer = Transformer::new(config);

    // Create a simple test ZIP file
    let zip_path = temp_dir.path().join("test.zip");
    let file = fs::File::create(&zip_path).unwrap();
    let mut zip = zip::ZipWriter::new(file);

    zip.start_file::<&str, ()>("test.txt", zip::write::FileOptions::default())
        .unwrap();
    std::io::Write::write_all(&mut zip, b"test content").unwrap();
    zip.finish().unwrap();

    // Test extraction
    let result = transformer.extract_zip(
        zip_path.to_str().unwrap(),
        temp_dir.path().to_str().unwrap(),
    );

    assert!(result.is_ok());
    assert!(temp_dir.path().join("test.txt").exists());
}
