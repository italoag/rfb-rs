use super::{TransformConfig, Result, Lookups};
use polars::prelude::*;
use std::fs::{File, read_dir};
use std::path::{Path, PathBuf};
use ::zip::ZipArchive;

/// Main transformer that orchestrates the transformation process
pub struct Transformer {
    config: TransformConfig,
    lookups: Lookups,
}

impl Transformer {
    pub fn new(config: TransformConfig) -> Self {
        Self {
            config,
            lookups: Lookups::new(),
        }
    }

    /// Load lookup tables
    pub fn load_lookups(&mut self) -> Result<()> {
        self.lookups.load_from_directory(&self.config.data_dir)?;
        Ok(())
    }

    /// Transform all data files
    pub async fn transform(&self) -> Result<()> {
        tracing::info!("Starting transformation process");
        tracing::info!("Data directory: {}", self.config.data_dir);
        tracing::info!("Output directory: {}", self.config.output_dir);
        tracing::info!("Privacy mode: {}", self.config.privacy_mode);
        
        // Create output directory
        std::fs::create_dir_all(&self.config.output_dir)?;
        
        // Extract all ZIP files first
        self.extract_all_zips()?;
        
        // Process different file types
        self.process_estabelecimentos()?;
        self.process_empresas()?;
        self.process_socios()?;
        self.process_simples()?;
        
        tracing::info!("Transformation complete!");
        Ok(())
    }

    fn extract_all_zips(&self) -> Result<()> {
        tracing::info!("Extracting ZIP files...");
        
        let entries = read_dir(&self.config.data_dir)?;
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            
            if path.extension().and_then(|s| s.to_str()) == Some("zip") {
                tracing::info!("Extracting: {:?}", path.file_name());
                self.extract_zip(&path.to_string_lossy(), &self.config.data_dir)?;
            }
        }
        
        Ok(())
    }

    fn process_estabelecimentos(&self) -> Result<()> {
        tracing::info!("Processing Estabelecimentos files...");
        
        for i in 0..10 {
            let pattern = format!("{}/*ESTABELE{}.csv", self.config.data_dir, i);
            if let Some(csv_path) = self.find_csv_by_pattern(&pattern)? {
                tracing::info!("Processing: {:?}", csv_path);
                self.process_estabelecimentos_file(&csv_path)?;
            }
        }
        
        Ok(())
    }

    fn process_estabelecimentos_file(&self, csv_path: &Path) -> Result<()> {
        // Read CSV with Polars
        let df = CsvReadOptions::default()
            .with_has_header(false)
            .with_parse_options(
                CsvParseOptions::default()
                    .with_separator(b';')
            )
            .try_into_reader_with_file_path(Some(csv_path.into()))?
            .finish()?;
        
        tracing::info!("Loaded {} rows from estabelecimentos", df.height());
        
        // Process and transform data
        // The actual processing would involve:
        // 1. Parse each row into Company structure
        // 2. Enrich with lookup data
        // 3. Apply privacy mode if enabled
        // 4. Write to output
        
        Ok(())
    }

    fn process_empresas(&self) -> Result<()> {
        tracing::info!("Processing Empresas files...");
        
        for i in 0..10 {
            let pattern = format!("{}/*EMPRE{}.csv", self.config.data_dir, i);
            if let Some(csv_path) = self.find_csv_by_pattern(&pattern)? {
                tracing::info!("Processing: {:?}", csv_path);
                self.process_empresas_file(&csv_path)?;
            }
        }
        
        Ok(())
    }

    fn process_empresas_file(&self, csv_path: &Path) -> Result<()> {
        let df = CsvReadOptions::default()
            .with_has_header(false)
            .with_parse_options(
                CsvParseOptions::default()
                    .with_separator(b';')
            )
            .try_into_reader_with_file_path(Some(csv_path.into()))?
            .finish()?;
        
        tracing::info!("Loaded {} rows from empresas", df.height());
        Ok(())
    }

    fn process_socios(&self) -> Result<()> {
        tracing::info!("Processing Socios files...");
        
        for i in 0..10 {
            let pattern = format!("{}/*SOCIO{}.csv", self.config.data_dir, i);
            if let Some(csv_path) = self.find_csv_by_pattern(&pattern)? {
                tracing::info!("Processing: {:?}", csv_path);
                self.process_socios_file(&csv_path)?;
            }
        }
        
        Ok(())
    }

    fn process_socios_file(&self, csv_path: &Path) -> Result<()> {
        let df = CsvReadOptions::default()
            .with_has_header(false)
            .with_parse_options(
                CsvParseOptions::default()
                    .with_separator(b';')
            )
            .try_into_reader_with_file_path(Some(csv_path.into()))?
            .finish()?;
        
        tracing::info!("Loaded {} rows from socios", df.height());
        Ok(())
    }

    fn process_simples(&self) -> Result<()> {
        tracing::info!("Processing Simples file...");
        
        let pattern = format!("{}/*SIMPLES.csv", self.config.data_dir);
        if let Some(csv_path) = self.find_csv_by_pattern(&pattern)? {
            tracing::info!("Processing: {:?}", csv_path);
            
            let df = CsvReadOptions::default()
                .with_has_header(false)
                .with_parse_options(
                    CsvParseOptions::default()
                        .with_separator(b';')
                )
                .try_into_reader_with_file_path(Some(csv_path.as_path().into()))?
                .finish()?;
            
            tracing::info!("Loaded {} rows from simples", df.height());
        }
        
        Ok(())
    }

    fn find_csv_by_pattern(&self, pattern: &str) -> Result<Option<PathBuf>> {
        let entries = read_dir(&self.config.data_dir)?;
        
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            
            if path.extension().and_then(|s| s.to_str()) == Some("csv") {
                let filename = path.file_name()
                    .and_then(|s| s.to_str())
                    .unwrap_or("");
                
                // Simple pattern matching - check if filename contains key parts
                let pattern_upper = pattern.to_uppercase();
                if pattern_upper.contains("ESTABELE") && filename.to_uppercase().contains("ESTABELE") {
                    return Ok(Some(path));
                } else if pattern_upper.contains("EMPRE") && filename.to_uppercase().contains("EMPRE") {
                    return Ok(Some(path));
                } else if pattern_upper.contains("SOCIO") && filename.to_uppercase().contains("SOCIO") {
                    return Ok(Some(path));
                } else if pattern_upper.contains("SIMPLES") && filename.to_uppercase().contains("SIMPLES") {
                    return Ok(Some(path));
                }
            }
        }
        
        Ok(None)
    }

    /// Extract a single ZIP file
    pub fn extract_zip(&self, zip_path: &str, output_dir: &str) -> Result<()> {
        use std::io::copy;

        let file = File::open(zip_path)?;
        let mut archive = ZipArchive::new(file)?;

        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let outpath = format!("{}/{}", output_dir, file.name());

            if file.name().ends_with('/') {
                std::fs::create_dir_all(&outpath)?;
            } else {
                if let Some(p) = std::path::Path::new(&outpath).parent() {
                    if !p.exists() {
                        std::fs::create_dir_all(p)?;
                    }
                }
                let mut outfile = File::create(&outpath)?;
                copy(&mut file, &mut outfile)?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transformer_creation() {
        let config = TransformConfig::default();
        let transformer = Transformer::new(config);
        assert_eq!(transformer.config.privacy_mode, false);
    }
}
