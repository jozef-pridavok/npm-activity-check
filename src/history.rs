use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;

use crate::types::PackageReport;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryData {
    pub last_data: PackageReport,
}

impl HistoryData {
    pub fn load(path: &str, verbose: bool) -> Result<Option<Self>> {
        if !Path::new(path).exists() {
            if verbose {
                eprintln!("[VERBOSE] History file does not exist: {}", path);
            }
            return Ok(None);
        }

        let content = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read history file: {}", path))?;

        let history: HistoryData = serde_json::from_str(&content)
            .with_context(|| format!("Failed to parse history file: {}", path))?;

        if verbose {
            eprintln!("[VERBOSE] Loaded history from: {}", path);
        }

        Ok(Some(history))
    }

    pub fn save(&self, path: &str, verbose: bool) -> Result<()> {
        let content = serde_json::to_string_pretty(self)
            .context("Failed to serialize history data")?;

        std::fs::write(path, content)
            .with_context(|| format!("Failed to write history file: {}", path))?;

        if verbose {
            eprintln!("[VERBOSE] Saved history to: {}", path);
        }

        Ok(())
    }

    pub fn calculate_change(&self, current_report: &PackageReport, field_name: &str) -> Result<u64> {
        match field_name {
            "total_versions" => {
                let old_count = self.last_data.total_versions;
                let new_count = current_report.total_versions;
                Ok(if new_count > old_count { new_count - old_count } else { 0 } as u64)
            }
            "downloads_last_week" => {
                let old_downloads = self.last_data.downloads_last_week;
                let new_downloads = current_report.downloads_last_week;
                Ok(if new_downloads > old_downloads { 
                    new_downloads - old_downloads 
                } else if old_downloads > new_downloads {
                    old_downloads - new_downloads
                } else { 
                    0 
                })
            }
            "downloads_last_month" => {
                let old_downloads = self.last_data.downloads_last_month;
                let new_downloads = current_report.downloads_last_month;
                Ok(if new_downloads > old_downloads { 
                    new_downloads - old_downloads 
                } else if old_downloads > new_downloads {
                    old_downloads - new_downloads
                } else { 
                    0 
                })
            }
            "maintainers_count" => {
                let old_count = self.last_data.maintainers_count;
                let new_count = current_report.maintainers_count;
                Ok(if new_count != old_count { 1 } else { 0 })
            }
            "latest_version" => {
                Ok(if self.last_data.latest_version != current_report.latest_version { 1 } else { 0 })
            }
            "package_alive" => {
                Ok(if self.last_data.package_alive != current_report.package_alive { 1 } else { 0 })
            }
            "has_recent_activity" => {
                Ok(if self.last_data.has_recent_activity != current_report.has_recent_activity { 1 } else { 0 })
            }
            "last_publish_date" => {
                let days_diff = (current_report.last_publish_date - self.last_data.last_publish_date).num_days().abs();
                Ok(days_diff as u64)
            }
            "name" => {
                Ok(if self.last_data.name != current_report.name { 1 } else { 0 })
            }
            "description" => {
                Ok(if self.last_data.description != current_report.description { 1 } else { 0 })
            }
            "homepage" => {
                Ok(if self.last_data.homepage != current_report.homepage { 1 } else { 0 })
            }
            "repository_url" => {
                Ok(if self.last_data.repository_url != current_report.repository_url { 1 } else { 0 })
            }
            "license" => {
                Ok(if self.last_data.license != current_report.license { 1 } else { 0 })
            }
            "keywords" => {
                Ok(if self.last_data.keywords != current_report.keywords { 1 } else { 0 })
            }
            _ => anyhow::bail!("Unknown field for change calculation: {}", field_name),
        }
    }
}