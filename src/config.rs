use anyhow::{Context, Result};
use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug, Clone)]
#[command(name = "npm-activity-check")]
#[command(about = "Check if NPM packages are actively maintained")]
#[command(version)]
pub struct Config {
    #[arg(help = "Package name to check")]
    pub package: String,

    #[arg(long, help = "Output format: default, json, field:name")]
    pub format: Option<String>,

    #[arg(long, help = "Load settings from TOML file")]
    pub config_file: Option<String>,

    #[arg(long, help = "Save/load run history")]
    pub history: Option<String>,

    #[arg(long, help = "Check field changes (sets exit code)")]
    pub check: Option<String>,

    #[arg(long, default_value = "90", help = "Maximum days since last publish (default: 90)")]
    pub max_days: i64,

    #[arg(long, default_value = "1000", help = "Minimum weekly downloads threshold (default: 1000)")]
    pub min_weekly_downloads: u64,

    #[arg(long, default_value = "5000", help = "Minimum monthly downloads threshold (default: 5000)")]
    pub min_monthly_downloads: u64,

    #[arg(long, default_value = "10", help = "Minimum total versions threshold (default: 10)")]
    pub min_versions: usize,

    #[arg(long, default_value = "1", help = "Minimum maintainers threshold (default: 1)")]
    pub min_maintainers: usize,

    #[arg(long, help = "Show detailed output")]
    pub verbose: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileConfig {
    pub format: Option<String>,
    pub max_days: Option<i64>,
    pub min_weekly_downloads: Option<u64>,
    pub min_monthly_downloads: Option<u64>,
    pub min_versions: Option<usize>,
    pub min_maintainers: Option<usize>,
}

impl Config {
    pub fn validate(&self) -> Result<()> {
        if self.package.trim().is_empty() {
            anyhow::bail!("Package name cannot be empty");
        }

        if let Some(format) = &self.format {
            if !format.starts_with("field:") && format != "json" && format != "default" {
                anyhow::bail!("Invalid format. Use 'default', 'json', or 'field:FIELD_NAME'");
            }
        }

        Ok(())
    }

    pub fn from_toml(path: &str) -> Result<FileConfig> {
        let content = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read config file: {}", path))?;
        
        let config: FileConfig = toml::from_str(&content)
            .with_context(|| format!("Failed to parse config file: {}", path))?;
        
        Ok(config)
    }

    pub fn merge(mut self, file_config: FileConfig) -> Self {
        if self.format.is_none() {
            self.format = file_config.format;
        }
        if let Some(max_days) = file_config.max_days {
            self.max_days = max_days;
        }
        if let Some(min_weekly) = file_config.min_weekly_downloads {
            self.min_weekly_downloads = min_weekly;
        }
        if let Some(min_monthly) = file_config.min_monthly_downloads {
            self.min_monthly_downloads = min_monthly;
        }
        if let Some(min_versions) = file_config.min_versions {
            self.min_versions = min_versions;
        }
        if let Some(min_maintainers) = file_config.min_maintainers {
            self.min_maintainers = min_maintainers;
        }
        
        self
    }

    pub fn with_defaults(mut self) -> Self {
        if self.format.is_none() {
            self.format = Some("default".to_string());
        }
        self
    }

    pub fn get_package(&self) -> &str {
        &self.package
    }
}