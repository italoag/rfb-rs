use super::Result;
use std::path::Path;

/// Check the integrity of downloaded ZIP files
pub fn check_zip_integrity(path: &Path) -> Result<bool> {
    use std::fs::File;
    use zip::ZipArchive;

    let file = File::open(path)?;
    let mut archive = ZipArchive::new(file)?;
    
    // Try to read all entries
    for i in 0..archive.len() {
        let _ = archive.by_index(i)?;
    }
    
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_check_invalid_zip() {
        let temp_dir = std::env::temp_dir();
        let test_file = temp_dir.join("test_invalid.zip");
        
        // Create an invalid zip file
        let mut file = std::fs::File::create(&test_file).unwrap();
        file.write_all(b"This is not a zip file").unwrap();
        
        let result = check_zip_integrity(&test_file);
        assert!(result.is_err());
        
        // Cleanup
        let _ = std::fs::remove_file(&test_file);
    }
}
