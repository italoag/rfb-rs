/// Integration tests for download module
use rfb_rs::download::{Downloader, DownloadConfig, FederalRevenue, check_zip_integrity};
use tempfile::TempDir;

#[tokio::test]
async fn test_downloader_initialization() {
    let temp_dir = TempDir::new().unwrap();
    let config = DownloadConfig {
        data_dir: temp_dir.path().to_str().unwrap().to_string(),
        timeout_secs: 60,
        max_retries: 2,
        max_parallel: 2,
        chunk_size: 1_048_576,
        skip_existing: false,
        restart: false,
    };
    
    let downloader = Downloader::new(config);
    let urls = downloader.list_urls().await.unwrap();
    
    // Should list all 37 files
    assert_eq!(urls.len(), 37);
}

#[tokio::test]
async fn test_downloader_skip_existing() {
    let temp_dir = TempDir::new().unwrap();
    
    // Create a dummy file
    let dummy_file = temp_dir.path().join("Estabelecimentos0.zip");
    std::fs::write(&dummy_file, b"test").unwrap();
    
    let config = DownloadConfig {
        data_dir: temp_dir.path().to_str().unwrap().to_string(),
        skip_existing: true,
        ..Default::default()
    };
    
    let downloader = Downloader::new(config);
    let urls = downloader.list_urls().await.unwrap();
    
    // Should skip the existing file
    assert_eq!(urls.len(), 36);
}

#[test]
fn test_federal_revenue_all_files() {
    let urls = FederalRevenue::file_urls();
    
    // Verify exact count
    assert_eq!(urls.len(), 37);
    
    // Count by type
    let estabelecimentos = urls.iter().filter(|u| u.contains("Estabelecimentos")).count();
    let empresas = urls.iter().filter(|u| u.contains("Empresas")).count();
    let socios = urls.iter().filter(|u| u.contains("Socios")).count();
    let simples = urls.iter().filter(|u| u.contains("Simples")).count();
    
    assert_eq!(estabelecimentos, 10);
    assert_eq!(empresas, 10);
    assert_eq!(socios, 10);
    assert_eq!(simples, 1);
    
    // Verify lookup files
    assert!(urls.iter().any(|u| u.contains("Cnaes")));
    assert!(urls.iter().any(|u| u.contains("Motivos")));
    assert!(urls.iter().any(|u| u.contains("Municipios")));
    assert!(urls.iter().any(|u| u.contains("Naturezas")));
    assert!(urls.iter().any(|u| u.contains("Paises")));
    assert!(urls.iter().any(|u| u.contains("Qualificacoes")));
}

#[test]
fn test_zip_integrity_invalid_file() {
    let temp_dir = TempDir::new().unwrap();
    let invalid_zip = temp_dir.path().join("invalid.zip");
    std::fs::write(&invalid_zip, b"not a zip file").unwrap();
    
    let result = check_zip_integrity(&invalid_zip);
    assert!(result.is_err());
}
