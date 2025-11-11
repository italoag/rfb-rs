use rfb_rs::download::{DownloadConfig, FederalRevenue};
use rfb_rs::transform::TransformConfig;

#[test]
fn test_federal_revenue_urls() {
    let urls = FederalRevenue::file_urls();
    assert_eq!(urls.len(), 37);

    // Check that all URLs start with the correct base
    for url in &urls {
        assert!(url
            .starts_with("https://arquivos.receitafederal.gov.br/dados/cnpj/dados_abertos_cnpj/"));
    }

    // Check specific file types exist
    assert!(urls.iter().any(|u| u.contains("Estabelecimentos")));
    assert!(urls.iter().any(|u| u.contains("Empresas")));
    assert!(urls.iter().any(|u| u.contains("Socios")));
    assert!(urls.iter().any(|u| u.contains("Simples")));
    assert!(urls.iter().any(|u| u.contains("Cnaes")));
}

#[test]
fn test_download_config_default() {
    let config = DownloadConfig::default();
    assert_eq!(config.data_dir, "data");
    assert_eq!(config.max_parallel, 4);
    assert_eq!(config.max_retries, 3);
    assert_eq!(config.chunk_size, 10_485_760);
}

#[test]
fn test_transform_config_default() {
    let config = TransformConfig::default();
    assert_eq!(config.data_dir, "data");
    assert_eq!(config.output_dir, "output");
    assert!(!config.privacy_mode);
}

#[test]
fn test_filename_extraction() {
    let url = "https://arquivos.receitafederal.gov.br/dados/cnpj/dados_abertos_cnpj/2025-11/Estabelecimentos0.zip";
    let filename = FederalRevenue::filename_from_url(url);
    assert_eq!(filename, Some("Estabelecimentos0.zip".to_string()));
}

#[test]
fn test_privacy_mode_config() {
    let config = TransformConfig {
        privacy_mode: true,
        ..Default::default()
    };
    assert!(config.privacy_mode);
}
