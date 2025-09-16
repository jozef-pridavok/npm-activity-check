use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NpmPackageInfo {
    pub name: String,
    pub description: Option<String>,
    #[serde(rename = "dist-tags")]
    pub dist_tags: std::collections::HashMap<String, String>,
    pub versions: std::collections::HashMap<String, serde_json::Value>,
    pub time: std::collections::HashMap<String, DateTime<Utc>>,
    pub maintainers: Option<serde_json::Value>,
    pub keywords: Option<Vec<String>>,
    pub homepage: Option<String>,
    pub repository: Option<serde_json::Value>,
    pub license: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NpmDownloadStats {
    pub downloads: u64,
    pub start: String,
    pub end: String,
    pub package: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageReport {
    pub name: String,
    pub latest_version: String,
    pub total_versions: usize,
    pub last_publish_date: DateTime<Utc>,
    pub downloads_last_week: u64,
    pub downloads_last_month: u64,
    pub maintainers_count: usize,
    pub has_recent_activity: bool,
    pub package_alive: bool,
    pub description: Option<String>,
    pub homepage: Option<String>,
    pub repository_url: Option<String>,
    pub license: Option<String>,
    pub keywords: Option<Vec<String>>,
}