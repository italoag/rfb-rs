use std::collections::HashMap;

/// Lookup tables for enriching company data
#[derive(Debug, Clone)]
pub struct Lookups {
    pub countries: HashMap<i32, String>,
    pub cities: HashMap<i32, String>,
    pub legal_natures: HashMap<i32, String>,
    pub qualifications: HashMap<i32, String>,
    pub cnaes: HashMap<i32, String>,
    pub motives: HashMap<i32, String>,
}

impl Lookups {
    pub fn new() -> Self {
        Self {
            countries: HashMap::new(),
            cities: HashMap::new(),
            legal_natures: HashMap::new(),
            qualifications: HashMap::new(),
            cnaes: HashMap::new(),
            motives: HashMap::new(),
        }
    }

    /// Load lookup tables from CSV files
    pub fn load_from_directory(&mut self, dir: &str) -> super::Result<()> {
        // TODO: Implement loading from CSV files
        // - Paises.csv -> countries
        // - Municipios.csv -> cities
        // - Naturezas.csv -> legal_natures
        // - Qualificacoes.csv -> qualifications
        // - Cnaes.csv -> cnaes
        // - Motivos.csv -> motives
        
        tracing::info!("Loading lookup tables from {}", dir);
        Ok(())
    }

    pub fn get_country(&self, code: i32) -> Option<&String> {
        self.countries.get(&code)
    }

    pub fn get_city(&self, code: i32) -> Option<&String> {
        self.cities.get(&code)
    }

    pub fn get_legal_nature(&self, code: i32) -> Option<&String> {
        self.legal_natures.get(&code)
    }

    pub fn get_qualification(&self, code: i32) -> Option<&String> {
        self.qualifications.get(&code)
    }

    pub fn get_cnae(&self, code: i32) -> Option<&String> {
        self.cnaes.get(&code)
    }

    pub fn get_motive(&self, code: i32) -> Option<&String> {
        self.motives.get(&code)
    }
}

impl Default for Lookups {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lookups_creation() {
        let lookups = Lookups::new();
        assert_eq!(lookups.countries.len(), 0);
        assert_eq!(lookups.cities.len(), 0);
    }

    #[test]
    fn test_lookups_get() {
        let mut lookups = Lookups::new();
        lookups.countries.insert(76, "Brasil".to_string());
        
        assert_eq!(lookups.get_country(76), Some(&"Brasil".to_string()));
        assert_eq!(lookups.get_country(999), None);
    }
}
