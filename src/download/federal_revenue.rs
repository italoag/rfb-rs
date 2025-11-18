/// Federal Revenue data source URLs and file information
pub struct FederalRevenue;

impl FederalRevenue {
    /// Base URL for Federal Revenue data
    const BASE_URL: &'static str =
        "https://arquivos.receitafederal.gov.br/dados/cnpj/dados_abertos_cnpj/";

    /// Get the current year-month directory (YYYY-MM format)
    fn get_year_month() -> String {
        use chrono::Datelike;
        let now = chrono::Utc::now();
        format!("{}-{:02}", now.year(), now.month())
    }

    /// Get the full base URL with year-month directory
    pub fn get_base_url_with_date() -> String {
        format!("{}{}/", Self::BASE_URL, Self::get_year_month())
    }

    /// Get list of all file URLs to download
    pub fn file_urls() -> Vec<String> {
        let base = Self::get_base_url_with_date();
        let mut urls = Vec::new();

        // Companies files (Estabelecimentos)
        for i in 0..10 {
            urls.push(format!("{}Estabelecimentos{}.zip", base, i));
        }

        // Companies base data (Empresas)
        for i in 0..10 {
            urls.push(format!("{}Empresas{}.zip", base, i));
        }

        // Partners (Socios)
        for i in 0..10 {
            urls.push(format!("{}Socios{}.zip", base, i));
        }

        // Lookup tables
        urls.push(format!("{}Cnaes.zip", base));
        urls.push(format!("{}Motivos.zip", base));
        urls.push(format!("{}Municipios.zip", base));
        urls.push(format!("{}Naturezas.zip", base));
        urls.push(format!("{}Paises.zip", base));
        urls.push(format!("{}Qualificacoes.zip", base));
        urls.push(format!("{}Simples.zip", base));

        urls
    }

    /// Get filename from URL
    pub fn filename_from_url(url: &str) -> Option<String> {
        url.split('/').next_back().map(|s| s.to_string())
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
    fn test_base_url_format() {
        let base_url = FederalRevenue::get_base_url_with_date();
        assert!(
            base_url.starts_with(
                "https://arquivos.receitafederal.gov.br/dados/cnpj/dados_abertos_cnpj/"
            )
        );
        // Should contain YYYY-MM format
        assert!(base_url.contains("-"));
    }

    #[test]
    fn test_year_month_format() {
        let year_month = FederalRevenue::get_year_month();
        // Should be in YYYY-MM format (e.g., "2025-11")
        assert_eq!(year_month.len(), 7);
        assert_eq!(year_month.chars().nth(4), Some('-'));
    }

    #[test]
    fn test_filename_from_url() {
        let url = "https://arquivos.receitafederal.gov.br/dados/cnpj/dados_abertos_cnpj/2025-11/Estabelecimentos0.zip";
        assert_eq!(
            FederalRevenue::filename_from_url(url),
            Some("Estabelecimentos0.zip".to_string())
        );
    }

    #[test]
    fn test_urls_use_new_base() {
        let urls = FederalRevenue::file_urls();
        // All URLs should use the new base URL
        for url in &urls {
            assert!(url.starts_with(
                "https://arquivos.receitafederal.gov.br/dados/cnpj/dados_abertos_cnpj/"
            ));
        }
    }
}
