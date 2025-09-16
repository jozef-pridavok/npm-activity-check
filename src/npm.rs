use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use reqwest::Client;

use crate::types::{NpmDownloadStats, NpmPackageInfo};

pub struct NpmClient {
    client: Client,
}

impl NpmClient {
    pub fn new() -> Result<Self> {
        let client = Client::builder()
            .user_agent("npm-activity-check/0.1.0")
            .build()
            .context("Failed to create HTTP client")?;

        Ok(Self { client })
    }

    pub async fn get_package_info(&self, package_name: &str) -> Result<NpmPackageInfo> {
        let url = format!("https://registry.npmjs.org/{}", package_name);
        
        let response = self
            .client
            .get(&url)
            .send()
            .await
            .context("Failed to fetch package info")?;

        if !response.status().is_success() {
            anyhow::bail!("Package '{}' not found or API error: {}", package_name, response.status());
        }

        let package_info: NpmPackageInfo = response
            .json()
            .await
            .context("Failed to parse package info JSON")?;

        Ok(package_info)
    }

    pub async fn get_download_stats(&self, package_name: &str, period: &str) -> Result<NpmDownloadStats> {
        let url = format!("https://api.npmjs.org/downloads/point/{}/{}", period, package_name);
        
        let response = self
            .client
            .get(&url)
            .send()
            .await
            .context("Failed to fetch download stats")?;

        if !response.status().is_success() {
            return Ok(NpmDownloadStats {
                downloads: 0,
                start: "".to_string(),
                end: "".to_string(),
                package: package_name.to_string(),
            });
        }

        let stats: NpmDownloadStats = response
            .json()
            .await
            .context("Failed to parse download stats JSON")?;

        Ok(stats)
    }

    pub async fn get_weekly_downloads(&self, package_name: &str) -> Result<u64> {
        let stats = self.get_download_stats(package_name, "last-week").await?;
        Ok(stats.downloads)
    }

    pub async fn get_monthly_downloads(&self, package_name: &str) -> Result<u64> {
        let stats = self.get_download_stats(package_name, "last-month").await?;
        Ok(stats.downloads)
    }

    pub fn get_latest_version_info(&self, package_info: &NpmPackageInfo) -> Option<(String, DateTime<Utc>)> {
        // Get the latest version from dist-tags
        let latest_version = package_info.dist_tags.get("latest")?;
        let publish_date = package_info.time.get(latest_version)?.clone();
        
        Some((latest_version.clone(), publish_date))
    }

}