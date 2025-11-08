/// Federal Revenue data source URLs and file information
pub struct FederalRevenue;

impl FederalRevenue {
    /// Base URL for Federal Revenue data
    const BASE_URL: &'static str = "https://dadosabertos.rfb.gov.br/CNPJ/";

    /// Get list of all file URLs to download
    pub fn file_urls() -> Vec<String> {
        let mut urls = Vec::new();
        
        // Companies files (Estabelecimentos)
        for i in 0..10 {
            urls.push(format!("{}Estabelecimentos{}.zip", Self::BASE_URL, i));
        }
        
        // Companies base data (Empresas)
        for i in 0..10 {
            urls.push(format!("{}Empresas{}.zip", Self::BASE_URL, i));
        }
        
        // Partners (Socios)
        for i in 0..10 {
            urls.push(format!("{}Socios{}.zip", Self::BASE_URL, i));
        }
        
        // Lookup tables
        urls.push(format!("{}Cnaes.zip", Self::BASE_URL));
        urls.push(format!("{}Motivos.zip", Self::BASE_URL));
        urls.push(format!("{}Municipios.zip", Self::BASE_URL));
        urls.push(format!("{}Naturezas.zip", Self::BASE_URL));
        urls.push(format!("{}Paises.zip", Self::BASE_URL));
        urls.push(format!("{}Qualificacoes.zip", Self::BASE_URL));
        urls.push(format!("{}Simples.zip", Self::BASE_URL));
        
        urls
    }
    
    /// Get filename from URL
    pub fn filename_from_url(url: &str) -> Option<String> {
        url.split('/').last().map(|s| s.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_urls_count() {
        let urls = FederalRevenue::file_urls();
        // 10 Estabelecimentos + 10 Empresas + 10 Socios + 7 lookup tables
        assert_eq!(urls.len(), 37);
    }

    #[test]
    fn test_filename_from_url() {
        let url = "https://dadosabertos.rfb.gov.br/CNPJ/Estabelecimentos0.zip";
        assert_eq!(
            FederalRevenue::filename_from_url(url),
            Some("Estabelecimentos0.zip".to_string())
        );
    }
}
