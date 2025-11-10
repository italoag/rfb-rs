use std::collections::HashMap;
use std::fs::read_dir;
use std::path::Path;
use polars::prelude::*;

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
        tracing::info!("Loading lookup tables from {}", dir);
        
        // Find and load each lookup file
        let entries = read_dir(dir)?;
        
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            
            if path.extension().and_then(|s| s.to_str()) == Some("csv") {
                let filename = path.file_name()
                    .and_then(|s| s.to_str())
                    .unwrap_or("");
                
                let filename_upper = filename.to_uppercase();
                
                if filename_upper.contains("PAIS") {
                    self.load_countries(&path)?;
                } else if filename_upper.contains("MUNIC") {
                    self.load_cities(&path)?;
                } else if filename_upper.contains("NATUREZA") || filename_upper.contains("NATJU") {
                    self.load_legal_natures(&path)?;
                } else if filename_upper.contains("QUALIF") || filename_upper.contains("QUALS") {
                    self.load_qualifications(&path)?;
                } else if filename_upper.contains("CNAE") {
                    self.load_cnaes(&path)?;
                } else if filename_upper.contains("MOTIV") {
                    self.load_motives(&path)?;
                }
            }
        }
        
        tracing::info!("Loaded {} countries", self.countries.len());
        tracing::info!("Loaded {} cities", self.cities.len());
        tracing::info!("Loaded {} legal natures", self.legal_natures.len());
        tracing::info!("Loaded {} qualifications", self.qualifications.len());
        tracing::info!("Loaded {} CNAEs", self.cnaes.len());
        tracing::info!("Loaded {} motives", self.motives.len());
        
        Ok(())
    }

    fn load_countries(&mut self, path: &Path) -> super::Result<()> {
        let df = CsvReadOptions::default()
            .with_has_header(false)
            .with_parse_options(
                CsvParseOptions::default()
                    .with_separator(b';')
            )
            .try_into_reader_with_file_path(Some(path.into()))?
            .finish()?;
        
        if df.width() >= 2 {
            let codes = df.column("column_1")?.i32()?;
            let names = df.column("column_2")?.str()?;
            
            for (code_opt, name_opt) in codes.into_iter().zip(names.into_iter()) {
                if let (Some(code), Some(name)) = (code_opt, name_opt) {
                    self.countries.insert(code, name.to_string());
                }
            }
        }
        
        Ok(())
    }

    fn load_cities(&mut self, path: &Path) -> super::Result<()> {
        let df = CsvReadOptions::default()
            .with_has_header(false)
            .with_parse_options(
                CsvParseOptions::default()
                    .with_separator(b';')
            )
            .try_into_reader_with_file_path(Some(path.into()))?
            .finish()?;
        
        if df.width() >= 2 {
            let codes = df.column("column_1")?.i32()?;
            let names = df.column("column_2")?.str()?;
            
            for (code_opt, name_opt) in codes.into_iter().zip(names.into_iter()) {
                if let (Some(code), Some(name)) = (code_opt, name_opt) {
                    self.cities.insert(code, name.to_string());
                }
            }
        }
        
        Ok(())
    }

    fn load_legal_natures(&mut self, path: &Path) -> super::Result<()> {
        let df = CsvReadOptions::default()
            .with_has_header(false)
            .with_parse_options(
                CsvParseOptions::default()
                    .with_separator(b';')
            )
            .try_into_reader_with_file_path(Some(path.into()))?
            .finish()?;
        
        if df.width() >= 2 {
            let codes = df.column("column_1")?.i32()?;
            let names = df.column("column_2")?.str()?;
            
            for (code_opt, name_opt) in codes.into_iter().zip(names.into_iter()) {
                if let (Some(code), Some(name)) = (code_opt, name_opt) {
                    self.legal_natures.insert(code, name.to_string());
                }
            }
        }
        
        Ok(())
    }

    fn load_qualifications(&mut self, path: &Path) -> super::Result<()> {
        let df = CsvReadOptions::default()
            .with_has_header(false)
            .with_parse_options(
                CsvParseOptions::default()
                    .with_separator(b';')
            )
            .try_into_reader_with_file_path(Some(path.into()))?
            .finish()?;
        
        if df.width() >= 2 {
            let codes = df.column("column_1")?.i32()?;
            let names = df.column("column_2")?.str()?;
            
            for (code_opt, name_opt) in codes.into_iter().zip(names.into_iter()) {
                if let (Some(code), Some(name)) = (code_opt, name_opt) {
                    self.qualifications.insert(code, name.to_string());
                }
            }
        }
        
        Ok(())
    }

    fn load_cnaes(&mut self, path: &Path) -> super::Result<()> {
        let df = CsvReadOptions::default()
            .with_has_header(false)
            .with_parse_options(
                CsvParseOptions::default()
                    .with_separator(b';')
            )
            .try_into_reader_with_file_path(Some(path.into()))?
            .finish()?;
        
        if df.width() >= 2 {
            let codes = df.column("column_1")?.i32()?;
            let names = df.column("column_2")?.str()?;
            
            for (code_opt, name_opt) in codes.into_iter().zip(names.into_iter()) {
                if let (Some(code), Some(name)) = (code_opt, name_opt) {
                    self.cnaes.insert(code, name.to_string());
                }
            }
        }
        
        Ok(())
    }

    fn load_motives(&mut self, path: &Path) -> super::Result<()> {
        let df = CsvReadOptions::default()
            .with_has_header(false)
            .with_parse_options(
                CsvParseOptions::default()
                    .with_separator(b';')
            )
            .try_into_reader_with_file_path(Some(path.into()))?
            .finish()?;
        
        if df.width() >= 2 {
            let codes = df.column("column_1")?.i32()?;
            let names = df.column("column_2")?.str()?;
            
            for (code_opt, name_opt) in codes.into_iter().zip(names.into_iter()) {
                if let (Some(code), Some(name)) = (code_opt, name_opt) {
                    self.motives.insert(code, name.to_string());
                }
            }
        }
        
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
