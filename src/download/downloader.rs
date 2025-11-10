use super::{DownloadConfig, Result, FederalRevenue};
use indicatif::{ProgressBar, ProgressStyle, MultiProgress};
use reqwest::{Client, header};
use std::path::{Path, PathBuf};
use std::fs::{File, create_dir_all};
use std::io::Write;
use std::time::Duration;
use futures::stream::{self, StreamExt};
use tokio::time::sleep;

/// Maximum exponent for exponential backoff retry delay (2^5 = 32 seconds)
const MAX_BACKOFF_EXPONENT: u32 = 5;

/// Main downloader that orchestrates the download process
pub struct Downloader {
    config: DownloadConfig,
    client: Client,
}

impl Downloader {
    pub fn new(config: DownloadConfig) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(config.timeout_secs))
            .user_agent("rfb-rs/0.1.0")
            .build()
            .expect("Failed to build HTTP client");
        
        Self { config, client }
    }

    /// Download all required files from Federal Revenue
    pub async fn download(&self) -> Result<()> {
        tracing::info!("Starting download process");
        tracing::info!("Data directory: {}", self.config.data_dir);
        
        // Create data directory if it doesn't exist
        create_dir_all(&self.config.data_dir)?;
        
        // Get list of files to download
        let urls = FederalRevenue::file_urls();
        let mut to_download = Vec::new();
        
        for url in urls {
            let filename = FederalRevenue::filename_from_url(&url)
                .ok_or_else(|| super::DownloadError::InvalidUrl(url.clone()))?;
            let filepath = PathBuf::from(&self.config.data_dir).join(&filename);
            
            if self.config.skip_existing && filepath.exists() {
                tracing::info!("Skipping existing file: {}", filename);
                continue;
            }
            
            to_download.push((url, filepath));
        }
        
        if to_download.is_empty() {
            tracing::info!("No files to download");
            return Ok(());
        }
        
        tracing::info!("Files to download: {}", to_download.len());
        
        // Download files with parallelism
        let multi = MultiProgress::new();
        let results: Vec<Result<()>> = stream::iter(to_download)
            .map(|(url, filepath)| {
                let client = self.client.clone();
                let config = self.config.clone();
                let pb = multi.add(ProgressBar::new(0));
                pb.set_style(
                    ProgressStyle::default_bar()
                        .template("{msg} [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
                        .unwrap()
                        .progress_chars("#>-")
                );
                
                async move {
                    download_file(&client, &url, &filepath, &config, pb).await
                }
            })
            .buffer_unordered(self.config.max_parallel)
            .collect()
            .await;
        
        // Check for errors
        for result in results {
            result?;
        }
        
        tracing::info!("Download complete!");
        Ok(())
    }

    /// List URLs of all files that need to be downloaded
    pub async fn list_urls(&self) -> Result<Vec<String>> {
        let mut urls = FederalRevenue::file_urls();
        
        if self.config.skip_existing {
            urls.retain(|url| {
                if let Some(filename) = FederalRevenue::filename_from_url(url) {
                    let filepath = PathBuf::from(&self.config.data_dir).join(&filename);
                    !filepath.exists()
                } else {
                    true
                }
            });
        }
        
        Ok(urls)
    }
}

async fn download_file(
    client: &Client,
    url: &str,
    filepath: &Path,
    config: &DownloadConfig,
    pb: ProgressBar,
) -> Result<()> {
    let filename = filepath.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown");
    
    pb.set_message(format!("Downloading {}", filename));
    
    // Get file size first
    let head_response = client.head(url).send().await?;
    let content_length = head_response
        .headers()
        .get(header::CONTENT_LENGTH)
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(0);
    
    pb.set_length(content_length);
    
    // Check if server supports range requests
    let supports_range = head_response
        .headers()
        .get(header::ACCEPT_RANGES)
        .and_then(|v| v.to_str().ok())
        .map(|s| s == "bytes")
        .unwrap_or(false);
    
    if supports_range && content_length > 0 {
        // Download with chunking
        download_chunked(client, url, filepath, content_length, config, pb).await
    } else {
        // Simple download without chunking
        download_simple(client, url, filepath, pb).await
    }
}

async fn download_chunked(
    client: &Client,
    url: &str,
    filepath: &Path,
    total_size: u64,
    config: &DownloadConfig,
    pb: ProgressBar,
) -> Result<()> {
    let mut file = File::create(filepath)?;
    let mut downloaded = 0u64;
    
    while downloaded < total_size {
        let chunk_size = std::cmp::min(config.chunk_size as u64, total_size - downloaded);
        let range_end = downloaded + chunk_size - 1;
        
        let mut retries = 0;
        let chunk_data = loop {
            match download_chunk(client, url, downloaded, range_end).await {
                Ok(data) => break data,
                Err(e) => {
                    retries += 1;
                    if retries >= config.max_retries {
                        return Err(super::DownloadError::MaxRetriesExceeded(retries));
                    }
                    tracing::warn!("Retry {}/{} for chunk: {}", retries, config.max_retries, e);
                    sleep(Duration::from_secs(2u64.pow(retries.min(MAX_BACKOFF_EXPONENT)))).await;
                }
            }
        };
        
        file.write_all(&chunk_data)?;
        downloaded += chunk_data.len() as u64;
        pb.set_position(downloaded);
    }
    
    pb.finish_with_message(format!("Downloaded {}", filepath.file_name().unwrap().to_str().unwrap()));
    Ok(())
}

async fn download_chunk(
    client: &Client,
    url: &str,
    start: u64,
    end: u64,
) -> Result<Vec<u8>> {
    let response = client
        .get(url)
        .header(header::RANGE, format!("bytes={}-{}", start, end))
        .send()
        .await?;
    
    if !response.status().is_success() {
        let status = response.status();
        return Err(super::DownloadError::HttpError(
            response.error_for_status()
                .expect_err(&format!("Expected error status but got success for status {}", status))
        ));
    }
    
    let bytes = response.bytes().await?;
    Ok(bytes.to_vec())
}

async fn download_simple(
    client: &Client,
    url: &str,
    filepath: &Path,
    pb: ProgressBar,
) -> Result<()> {
    let response = client.get(url).send().await?;
    
    if !response.status().is_success() {
        let status = response.status();
        return Err(super::DownloadError::HttpError(
            response.error_for_status()
                .expect_err(&format!("Expected error status but got success for status {}", status))
        ));
    }
    
    let bytes = response.bytes().await?;
    let mut file = File::create(filepath)?;
    file.write_all(&bytes)?;
    
    pb.set_length(bytes.len() as u64);
    pb.set_position(bytes.len() as u64);
    pb.finish_with_message(format!("Downloaded {}", filepath.file_name().unwrap().to_str().unwrap()));
    
    Ok(())
}
